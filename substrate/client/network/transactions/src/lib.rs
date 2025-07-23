// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Transactions handling to plug on top of the network service.
//!
//! Usage:
//!
//! - Use [`TransactionsHandlerPrototype::new`] to create a prototype.
//! - Pass the `NonDefaultSetConfig` returned from [`TransactionsHandlerPrototype::new`] to the
//!   network configuration as an extra peers set.
//! - Use [`TransactionsHandlerPrototype::build`] then [`TransactionsHandler::run`] to obtain a
//! `Future` that processes transactions.

use crate::config::*;

use codec::{Decode, Encode};
use futures::{prelude::*, stream::FuturesUnordered};
use libp2p::{multiaddr, PeerId};
use log::{debug, trace, warn};

use prometheus_endpoint::{register, Counter, PrometheusError, Registry, U64};
use sc_consensus::authority_discovery::AuthorityDiscoveryForTxPool;
use sc_network::{
	config::{NonDefaultSetConfig, NonReservedPeerMode, ProtocolId, SetConfig},
	error,
	service::traits::{NotificationEvent, NotificationService, ValidationResult},
	types::ProtocolName,
	utils::{interval, LruHashSet},
	NetworkEventStream, NetworkNotification, NetworkPeers,
};
use sc_network_common::{role::ObservedRole, ExHashT};
use sc_network_sync::{SyncEvent, SyncEventStream};
use sc_utils::mpsc::{tracing_unbounded, TracingUnboundedReceiver, TracingUnboundedSender};
use sp_runtime::{traits::Block as BlockT, transaction_validity::TransactionSummary};

use std::{
	collections::{hash_map::Entry, HashMap},
	iter,
	num::NonZeroUsize,
	pin::Pin,
	sync::Arc,
	task::Poll,
};

pub mod config;

/// A set of transactions.
pub type Transactions<E> = Vec<E>;

/// A transaction handler messages
#[derive(Debug, Clone, Encode, Decode)]
pub enum TransactionMessage<E, H: ExHashT> {
	/// A list of full transactions
	#[codec(index = 0)]
	Transactions(Vec<E>),

	/// Lightweight transaction summary announcements
	#[codec(index = 1)]
	TransactionAnnouncement(Vec<TransactionSummary<H>>),

	/// Requests for full transactions by their hashes
	#[codec(index = 2)]
	TransactionRequest(Vec<H>),
}

mod rep {
	use sc_network::ReputationChange as Rep;
	/// Reputation change when a peer sends us any transaction.
	///
	/// This forces node to verify it, thus the negative value here. Once transaction is verified,
	/// reputation change should be refunded with `ANY_TRANSACTION_REFUND`
	pub const ANY_TRANSACTION: Rep = Rep::new(-(1 << 4), "Any transaction");
	/// Reputation change when a peer sends us any transaction that is not invalid.
	pub const ANY_TRANSACTION_REFUND: Rep = Rep::new(1 << 4, "Any transaction (refund)");
	/// Reputation change when a peer sends us an transaction that we didn't know about.
	pub const GOOD_TRANSACTION: Rep = Rep::new(1 << 7, "Good transaction");
	/// Reputation change when a peer sends us a bad transaction.
	pub const BAD_TRANSACTION: Rep = Rep::new(-(1 << 12), "Bad transaction");
}

struct Metrics {
	propagated_transactions: Counter<U64>,
}

impl Metrics {
	fn register(r: &Registry) -> Result<Self, PrometheusError> {
		Ok(Self {
			propagated_transactions: register(
				Counter::new(
					"substrate_sync_propagated_transactions",
					"Number of transactions propagated to at least one peer",
				)?,
				r,
			)?,
		})
	}
}

struct PendingTransaction<H> {
	validation: TransactionImportFuture,
	tx_hash: H,
}

impl<H> Unpin for PendingTransaction<H> {}

impl<H: ExHashT> Future for PendingTransaction<H> {
	type Output = (H, TransactionImport);

	fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
		if let Poll::Ready(import_result) = self.validation.poll_unpin(cx) {
			return Poll::Ready((self.tx_hash.clone(), import_result));
		}

		Poll::Pending
	}
}

/// Prototype for a [`TransactionsHandler`].
pub struct TransactionsHandlerPrototype {
	/// Name of the transaction protocol.
	protocol_name: ProtocolName,

	/// Handle that is used to communicate with `sc_network::Notifications`.
	notification_service: Box<dyn NotificationService>,
}

impl TransactionsHandlerPrototype {
	/// Create a new instance.
	pub fn new<Hash: AsRef<[u8]>>(
		protocol_id: ProtocolId,
		genesis_hash: Hash,
		fork_id: Option<&str>,
	) -> (Self, NonDefaultSetConfig) {
		let genesis_hash = genesis_hash.as_ref();
		let protocol_name: ProtocolName = if let Some(fork_id) = fork_id {
			format!("/{}/{}/transactions/1", array_bytes::bytes2hex("", genesis_hash), fork_id)
		} else {
			format!("/{}/transactions/1", array_bytes::bytes2hex("", genesis_hash))
		}
		.into();
		let (config, notification_service) = NonDefaultSetConfig::new(
			protocol_name.clone(),
			vec![format!("/{}/transactions/1", protocol_id.as_ref()).into()],
			MAX_TRANSACTIONS_SIZE,
			None,
			SetConfig {
				in_peers: 0,
				out_peers: 0,
				reserved_nodes: Vec::new(),
				non_reserved_mode: NonReservedPeerMode::Deny,
			},
		);

		(Self { protocol_name, notification_service }, config)
	}

	/// Turns the prototype into the actual handler. Returns a controller that allows controlling
	/// the behaviour of the handler while it's running.
	///
	/// Important: the transactions handler is initially disabled and doesn't gossip transactions.
	/// Gossiping is enabled when major syncing is done.
	pub fn build<
		B: BlockT + 'static,
		H: ExHashT,
		N: NetworkPeers + NetworkEventStream + NetworkNotification,
		S: SyncEventStream + sp_consensus::SyncOracle,
		AD: AuthorityDiscoveryForTxPool,
	>(
		self,
		network: N,
		sync: S,
		transaction_pool: Arc<dyn TransactionPool<H, B>>,
		authority_discovery: Arc<AD>,
		metrics_registry: Option<&Registry>,
	) -> error::Result<(TransactionsHandler<B, H, N, S, AD>, TransactionsHandlerController<H>)> {
		let sync_event_stream = sync.event_stream("transactions-handler-sync");
		let (to_handler, from_controller) = tracing_unbounded("mpsc_transactions_handler", 100_000);

		let handler = TransactionsHandler {
			protocol_name: self.protocol_name,
			notification_service: self.notification_service,
			propagate_timeout: (Box::pin(interval(PROPAGATE_TIMEOUT))
				as Pin<Box<dyn Stream<Item = ()> + Send>>)
				.fuse(),
			pending_transactions: FuturesUnordered::new(),
			pending_transactions_peers: HashMap::new(),
			network,
			sync,
			sync_event_stream: sync_event_stream.fuse(),
			peers: HashMap::new(),
			transaction_pool,
			from_controller,
			metrics: if let Some(r) = metrics_registry {
				Some(Metrics::register(r)?)
			} else {
				None
			},
			authority_discovery,
		};

		let controller = TransactionsHandlerController { to_handler };

		Ok((handler, controller))
	}
}

/// Controls the behaviour of a [`TransactionsHandler`] it is connected to.
pub struct TransactionsHandlerController<H: ExHashT> {
	to_handler: TracingUnboundedSender<ToHandler<H>>,
}

impl<H: ExHashT> TransactionsHandlerController<H> {
	/// You may call this when new transactions are imported by the transaction pool.
	///
	/// All transactions will be fetched from the `TransactionPool` that was passed at
	/// initialization as part of the configuration and propagated to peers.
	pub fn propagate_transactions(&self) {
		let _ = self.to_handler.unbounded_send(ToHandler::PropagateTransactions);
	}

	/// You must call when new a transaction is imported by the transaction pool.
	///
	/// This transaction will be fetched from the `TransactionPool` that was passed at
	/// initialization as part of the configuration and propagated to peers.
	pub fn propagate_transaction(&self, hash: H) {
		let _ = self.to_handler.unbounded_send(ToHandler::PropagateTransaction(hash));
	}

	/// You must call when new transactions are imported by the transaction pool.
	///
	/// This will announce all transactions that are ready to be included in the block.
	/// This will not propagate transactions to peers but instead just sends a summary of the
	/// trasnaction.
	pub fn announce_summaries(&self) {
		let _ = self.to_handler.unbounded_send(ToHandler::AnnounceSummaries);
	}

	pub fn announce_summary(&self, hash: H) {
		let _ = self.to_handler.unbounded_send(ToHandler::AnnounceSummary(hash));
	}
}

enum ToHandler<H: ExHashT> {
	PropagateTransactions,
	PropagateTransaction(H),
	AnnounceSummaries,
	AnnounceSummary(H),
}

/// Handler for transactions. Call [`TransactionsHandler::run`] to start the processing.
pub struct TransactionsHandler<
	B: BlockT + 'static,
	H: ExHashT,
	N: NetworkPeers + NetworkEventStream + NetworkNotification,
	S: SyncEventStream + sp_consensus::SyncOracle,
	AD: AuthorityDiscoveryForTxPool,
> {
	protocol_name: ProtocolName,
	/// Interval at which we call `propagate_transactions`.
	propagate_timeout: stream::Fuse<Pin<Box<dyn Stream<Item = ()> + Send>>>,
	/// Pending transactions verification tasks.
	pending_transactions: FuturesUnordered<PendingTransaction<H>>,
	/// As multiple peers can send us the same transaction, we group
	/// these peers using the transaction hash while the transaction is
	/// imported. This prevents that we import the same transaction
	/// multiple times concurrently.
	pending_transactions_peers: HashMap<H, Vec<PeerId>>,
	/// Network service to use to send messages and manage peers.
	network: N,
	/// Syncing service.
	sync: S,
	/// Receiver for syncing-related events.
	sync_event_stream: stream::Fuse<Pin<Box<dyn Stream<Item = SyncEvent> + Send>>>,
	// All connected peers
	peers: HashMap<PeerId, Peer<H>>,
	transaction_pool: Arc<dyn TransactionPool<H, B>>,
	from_controller: TracingUnboundedReceiver<ToHandler<H>>,
	/// Prometheus metrics.
	metrics: Option<Metrics>,
	/// Handle that is used to communicate with `sc_network::Notifications`.
	notification_service: Box<dyn NotificationService>,
	/// Authority discovery.
	authority_discovery: Arc<AD>,
}

/// Peer information
#[derive(Debug)]
struct Peer<H: ExHashT> {
	/// Holds a set of transactions known to this peer.
	known_transactions: LruHashSet<H>,
	role: ObservedRole,
}

impl<B, H, N, S, AD> TransactionsHandler<B, H, N, S, AD>
where
	B: BlockT + 'static,
	H: ExHashT,
	N: NetworkPeers + NetworkEventStream + NetworkNotification,
	S: SyncEventStream + sp_consensus::SyncOracle,
	AD: AuthorityDiscoveryForTxPool,
	TransactionMessage<B::Extrinsic, H>: Encode + Decode,
{
	/// Turns the [`TransactionsHandler`] into a future that should run forever and not be
	/// interrupted.
	pub async fn run(mut self) {
		loop {
			futures::select! {
				_ = self.propagate_timeout.next() => {
					// Maybe we can have a flag to decide whetehr to only announce the transation summary or propagate transactions
					self.propagate_transactions();
					// self.announce_transactions();
				},
				(tx_hash, result) = self.pending_transactions.select_next_some() => {
					if let Some(peers) = self.pending_transactions_peers.remove(&tx_hash) {
						peers.into_iter().for_each(|p| self.on_handle_transaction_import(p, result));
					} else {
						warn!(target: "sub-libp2p", "Inconsistent state, no peers for pending transaction!");
					}
				},
				sync_event = self.sync_event_stream.next() => {
					if let Some(sync_event) = sync_event {
						self.handle_sync_event(sync_event);
					} else {
						// Syncing has seemingly closed. Closing as well.
						return;
					}
				}
				message = self.from_controller.select_next_some() => {
					match message {
						ToHandler::PropagateTransaction(hash) => self.propagate_transaction(&hash),
						ToHandler::PropagateTransactions => self.propagate_transactions(),
						ToHandler::AnnounceSummaries => self.announce_transactions(),
						// To keep the transaction status events in-order, we need the broadcasted info, so using announce_transaction
						ToHandler::AnnounceSummary(hash) => self.announce_transaction(&hash),
					}
				},
				event = self.notification_service.next_event().fuse() => {
					if let Some(event) = event {
						self.handle_notification_event(event)
					} else {
						// `Notifications` has seemingly closed. Closing as well.
						return
					}
				}
			}
		}
	}

	fn handle_notification_event(&mut self, event: NotificationEvent) {
		match event {
			NotificationEvent::ValidateInboundSubstream { peer, handshake, result_tx, .. } => {
				// only accept peers whose role can be determined
				let result = self
					.network
					.peer_role(peer, handshake)
					.map_or(ValidationResult::Reject, |_| ValidationResult::Accept);
				let _ = result_tx.send(result);
			},
			NotificationEvent::NotificationStreamOpened { peer, handshake, .. } => {
				let Some(role) = self.network.peer_role(peer, handshake) else {
					log::debug!(target: "sub-libp2p", "role for {peer} couldn't be determined");
					return;
				};

				let _was_in = self.peers.insert(
					peer,
					Peer {
						known_transactions: LruHashSet::new(
							NonZeroUsize::new(MAX_KNOWN_TRANSACTIONS).expect("Constant is nonzero"),
						),
						role,
					},
				);
				debug_assert!(_was_in.is_none());
			},
			NotificationEvent::NotificationStreamClosed { peer } => {
				let _peer = self.peers.remove(&peer);
				debug_assert!(_peer.is_some());
			},
			NotificationEvent::NotificationReceived { peer, notification } => {
				if let Ok(m) =
					<Transactions<B::Extrinsic> as Decode>::decode(&mut notification.as_ref())
				{
					self.on_transactions(peer, m);
				} else {
					warn!(target: "sub-libp2p", "Failed to decode transactions list");
				}
				// Decode the incoming message to TransactionMessage type & handle each type
				// For now, not handling the existing message types
				// if let Ok(message) =
				// 	TransactionMessage::<B::Extrinsic, H>::decode(&mut notification.as_ref())
				// {
				// 	match message {
				// 		TransactionMessage::Transactions(transactions) => {
				// 			self.on_transactions(peer, transactions);
				// 		},
				// 		TransactionMessage::TransactionAnnouncement(summaries) => {
				// 			self.handle_announcement(peer, summaries);
				// 		},
				// 		TransactionMessage::TransactionRequest(hashes) => {
				// 			self.handle_request(peer, hashes);
				// 		},
				// 	}
				// } else {
				// 	warn!(target: "sub-libp2p", "Failed to decode TransactionMessage");
				// }
			},
		}
	}

	fn handle_sync_event(&mut self, event: SyncEvent) {
		match event {
			SyncEvent::PeerConnected(remote) => {
				let addr = iter::once(multiaddr::Protocol::P2p(remote.into()))
					.collect::<multiaddr::Multiaddr>();
				let result = self.network.add_peers_to_reserved_set(
					self.protocol_name.clone(),
					iter::once(addr).collect(),
				);
				if let Err(err) = result {
					log::error!(target: "sync", "Add reserved peer failed: {}", err);
				}
			},
			SyncEvent::PeerDisconnected(remote) => {
				let result = self.network.remove_peers_from_reserved_set(
					self.protocol_name.clone(),
					iter::once(remote).collect(),
				);
				if let Err(err) = result {
					log::error!(target: "sync", "Remove reserved peer failed: {}", err);
				}
			},
		}
	}

	/// Called when peer sends us new transactions
	fn on_transactions(&mut self, who: PeerId, transactions: Transactions<B::Extrinsic>) {
		// Accept transactions only when node is not major syncing
		if self.sync.is_major_syncing() {
			trace!(target: "sync", "{} Ignoring transactions while major syncing", who);
			return;
		}

		trace!(target: "sync", "Received {} transactions from {}", transactions.len(), who);
		if let Some(ref mut peer) = self.peers.get_mut(&who) {
			for t in transactions {
				if self.pending_transactions.len() > MAX_PENDING_TRANSACTIONS {
					debug!(
						target: "sync",
						"Ignoring any further transactions that exceed `MAX_PENDING_TRANSACTIONS`({}) limit",
						MAX_PENDING_TRANSACTIONS,
					);
					break;
				}

				let hash = self.transaction_pool.hash_of(&t);
				peer.known_transactions.insert(hash.clone());

				self.network.report_peer(who, rep::ANY_TRANSACTION);

				match self.pending_transactions_peers.entry(hash.clone()) {
					Entry::Vacant(entry) => {
						self.pending_transactions.push(PendingTransaction {
							validation: self.transaction_pool.import(t),
							tx_hash: hash,
						});
						entry.insert(vec![who]);
					},
					Entry::Occupied(mut entry) => {
						entry.get_mut().push(who);
					},
				}
			}
		}
	}

	// TODO: Add reputation system to handle announcement & requests
	fn handle_announcement(&mut self, peer: PeerId, summaries: Vec<TransactionSummary<H>>) {
		if self.sync.is_major_syncing() {
			trace!(target: "sync", "Ignoring announcements from {} while major syncing", peer);
			return;
		}

		trace!(target: "sync", "Received {} transaction summaries from {}", summaries.len(), peer);
		// Filter transactions we dont have in our pool
		let hashes: Vec<H> = summaries
			.iter()
			.filter(|s| !self.transaction_pool.transaction(&s.hash).is_some())
			.map(|s| s.hash.clone())
			.collect();

		// TODO: we can have a more sophisticated logic here to determine if this client need these txs
		// or not, for now we're only checking if they exist in client's pool or not & if client is authoring the blocks in near future
		if !hashes.is_empty() && self.authority_discovery.will_author_in_next_slots(SLOTS_TO_CHECK){
			trace!(target: "sync", "Requesting {} transactions from {}", hashes.len(), peer);
			self.notification_service.send_sync_notification(
				&peer,
				TransactionMessage::TransactionRequest(hashes).encode(),
			);
		}
	}

	fn handle_request(&mut self, peer: PeerId, hashes: Vec<H>) {
		if self.sync.is_major_syncing() {
			trace!(target: "sync", "Ignoring transaction requests from {} while major syncing", peer);
			return;
		}

		trace!(target: "sync", "Received {} transaction requests from {}", hashes.len(), peer);
		let transactions: Vec<_> =
			hashes.iter().filter_map(|h| self.transaction_pool.transaction(h)).collect();

		if !transactions.is_empty() {
			self.notification_service.send_sync_notification(
				&peer,
				TransactionMessage::Transactions(transactions).encode(),
			);
		}
	}

	fn on_handle_transaction_import(&mut self, who: PeerId, import: TransactionImport) {
		match import {
			TransactionImport::KnownGood =>
				self.network.report_peer(who, rep::ANY_TRANSACTION_REFUND),
			TransactionImport::NewGood => self.network.report_peer(who, rep::GOOD_TRANSACTION),
			TransactionImport::Bad => self.network.report_peer(who, rep::BAD_TRANSACTION),
			TransactionImport::None => {},
		}
	}

	/// Propagate one transaction.
	pub fn propagate_transaction(&mut self, hash: &H) {
		// Accept transactions only when node is not major syncing
		if self.sync.is_major_syncing() {
			return;
		}

		debug!(target: "sync", "Propagating transaction [{:?}]", hash);
		if let Some(transaction) = self.transaction_pool.transaction(hash) {
			let propagated_to = self.do_propagate_transactions(&[(hash.clone(), transaction)]);
			self.transaction_pool.on_broadcasted(propagated_to);
		}
	}

	fn do_propagate_transactions(
		&mut self,
		transactions: &[(H, B::Extrinsic)],
	) -> HashMap<H, Vec<String>> {
		let mut propagated_to = HashMap::<_, Vec<_>>::new();
		let mut propagated_transactions = 0;

		for (who, peer) in self.peers.iter_mut() {
			// never send transactions to the light node
			if matches!(peer.role, ObservedRole::Light) {
				continue;
			}

			let (hashes, to_send): (Vec<_>, Vec<_>) = transactions
				.iter()
				.filter(|(hash, _)| peer.known_transactions.insert(hash.clone()))
				.cloned()
				.unzip();

			propagated_transactions += hashes.len();

			if !to_send.is_empty() {
				for hash in hashes {
					propagated_to.entry(hash).or_default().push(who.to_base58());
				}
				trace!(target: "sync", "Sending {} transactions to {}", to_send.len(), who);
				// Historically, the format of a notification of the transactions protocol
				// consisted in a (SCALE-encoded) `Vec<Transaction>`.
				// After RFC 56, the format was modified in a backwards-compatible way to be
				// a (SCALE-encoded) tuple `(Compact(1), Transaction)`, which is the same encoding
				// as a `Vec` of length one. This is no coincidence, as the change was
				// intentionally done in a backwards-compatible way.
				// In other words, the `Vec` that is sent below **must** always have only a single
				// element in it.
				// See <https://github.com/polkadot-fellows/RFCs/blob/main/text/0056-one-transaction-per-notification.md>
				for to_send in to_send {
					let _ = self
						.notification_service
						.send_sync_notification(who, vec![to_send].encode());
				}
			}
		}

		if let Some(ref metrics) = self.metrics {
			metrics.propagated_transactions.inc_by(propagated_transactions as _)
		}

		propagated_to
	}

	/// Call when we must announce a single ready transaction summary to peers.
	fn announce_transaction(&mut self, hash: &H) {
		// announce only when we're not major syncing
		if self.sync.is_major_syncing() {
			return;
		}

		trace!(target: "sync", "Announcing transaction summary");
		if let Some(summary) = self.transaction_pool.summary(&hash) {
			trace!(target: "sync", "Announcing transaction summary {:?}", summary);
			let propagated_to = self.do_propagate_announcements(&[summary]);
			self.transaction_pool.on_broadcasted(propagated_to);
		}
	}

	/// Call when we must announce ready transactions summary to peers.
	fn announce_transactions(&mut self) {
		// Accept transactions only when node is not major syncing
		if self.sync.is_major_syncing() {
			return;
		}

		trace!(target: "sync", "Announcing transaction summaries");
		let summaries = self.transaction_pool.summaries();
		let propagated_to = self.do_propagate_announcements(&summaries);
		self.transaction_pool.on_broadcasted(propagated_to);
	}

	fn do_propagate_announcements(
		&mut self,
		summaries: &[TransactionSummary<H>],
	) -> HashMap<H, Vec<String>> {
		let mut propagated_to: HashMap<H, Vec<String>> = HashMap::new();

		let announcements: Vec<_> = summaries.to_vec();
		if announcements.is_empty() {
			return propagated_to;
		}

		for (who, _peer) in self.peers.iter() {
			for summary in &announcements {
				propagated_to.entry(summary.hash.clone()).or_default().push(who.to_base58());
			}

			let msg: TransactionMessage<B::Extrinsic, H> =
				TransactionMessage::TransactionAnnouncement(announcements.clone());
			self.notification_service.send_sync_notification(who, msg.encode());
		}

		propagated_to
	}

	/// Call when we must propagate ready transactions to peers.
	fn propagate_transactions(&mut self) {
		// Accept transactions only when node is not major syncing
		if self.sync.is_major_syncing() {
			return;
		}

		debug!(target: "sync", "Propagating transactions");
		let transactions = self.transaction_pool.transactions();
		let propagated_to = self.do_propagate_transactions(&transactions);
		self.transaction_pool.on_broadcasted(propagated_to);
	}
}
