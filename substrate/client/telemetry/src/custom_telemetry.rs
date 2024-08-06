// Execute the following command to run the tests:
//
// cd polkadot-sdk/substrate/client/telemetry/src
// cargo test -- --test-threads 1
//

use std::{
	collections::HashMap,
	sync::Mutex,
	time::{Duration, SystemTimeError},
};

use libp2p::PeerId;
use serde::Serialize;
use wasm_timer::{SystemTime, UNIX_EPOCH};

use crate::custom_telemetry::external::BlockIntervalFromNode;
use crate::{telemetry, TelemetryHandle, SUBSTRATE_INFO};

///
#[repr(u8)]
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub enum IntervalKind {
	/// Tells us how long it took us to produce a block. Basically it's all about
	/// taking transactions from the mem pool and executing them.
	Proposal = 0,
	/// Tells us how long it took to get a block from someone.
	Sync = 1,
	/// Tells us how long it took to import a block.
	/// Import is measured for the node that produced the block as well as for the
	/// node that requested that block.
	Import = 2,
}

///
#[derive(Debug, Clone, Default)]
pub struct BlockIntervals {
	proposal: Option<IntervalDetailsProposal>,
	import: Option<IntervalDetailsImport>,
	sync: Option<IntervalDetailsSync>,
	partial_syncs: Vec<IntervalDetailsPartialSync>,
}

///
#[derive(Debug, Clone)]
pub enum IntervalDetails {
	///
	Proposal(IntervalDetailsProposal),
	///
	Import(IntervalDetailsImport),
	///
	Sync(IntervalDetailsSync),
	///
	PartialSync(IntervalDetailsPartialSync),
}

impl From<IntervalDetailsProposal> for IntervalDetails {
	fn from(value: IntervalDetailsProposal) -> Self {
		Self::Proposal(value)
	}
}

impl From<IntervalDetailsImport> for IntervalDetails {
	fn from(value: IntervalDetailsImport) -> Self {
		Self::Import(value)
	}
}

impl From<IntervalDetailsPartialSync> for IntervalDetails {
	fn from(value: IntervalDetailsPartialSync) -> Self {
		Self::PartialSync(value)
	}
}

///
#[derive(Debug, Clone)]
pub struct IntervalDetailsProposal {
	///
	pub start_timestamp: u64,
	///
	pub end_timestamp: u64,
}

///
#[derive(Debug, Clone)]
pub struct IntervalDetailsImport {
	///
	pub peer_id: Option<PeerId>,
	///
	pub start_timestamp: u64,
	///
	pub end_timestamp: u64,
}

///
#[derive(Debug, Clone)]
pub struct IntervalDetailsSync {
	///
	pub peer_id: PeerId,
	///
	pub start_timestamp: u64,
	///
	pub end_timestamp: u64,
}

///
#[derive(Debug, Clone)]
pub struct IntervalDetailsPartialSync {
	///
	pub peer_id: PeerId,
	///
	pub start_timestamp: Option<u64>,
	///
	pub end_timestamp: Option<u64>,
}

///
#[derive(Serialize, Debug)]
pub struct BlockRequestsDetail {
	///
	pub current_queue_size: u32,
	///
	pub requests_handled: u32,
	///
	pub time_frame: u64,
}

const MAX_BLOCKS_PER_HEIGHT: usize = 50;

///
#[derive(Default, Debug)]
pub struct BlockMetrics {
	///
	intervals: Option<HashMap<u64, HashMap<String, BlockIntervals>>>,
	///
	block_requests: Vec<BlockRequestsDetail>,
	///
	max_interval_buffer_size: usize,
	///
	max_block_request_buffer_size: usize,
}

static BLOCK_METRICS: Mutex<BlockMetrics> = Mutex::new(BlockMetrics::new());

impl BlockMetrics {
	///
	pub const fn new() -> Self {
		Self {
			intervals: None,
			block_requests: Vec::new(),
			max_interval_buffer_size: 0,
			max_block_request_buffer_size: 0,
		}
	}

	///
	pub fn observe_interval(block_number: u64, block_hash: String, value: IntervalDetails) {
		let Ok(mut lock) = BLOCK_METRICS.lock() else {
			return;
		};
		let max_buffer_size = lock.max_interval_buffer_size;

		let intervals = lock.intervals.get_or_insert(HashMap::new());
		if intervals.len() >= max_buffer_size {
			return;
		}

		let block_height = intervals.entry(block_number).or_default();
		if block_height.len() >= MAX_BLOCKS_PER_HEIGHT {
			return;
		}

		let block = block_height.entry(block_hash.clone()).or_default();

		match value {
			IntervalDetails::Proposal(v) => {
				if block.proposal.is_some() {
					return;
				}
				block.proposal = Some(v);
			},
			IntervalDetails::Import(v) => {
				if block.import.is_some() {
					return;
				}
				let peer_id = v.peer_id.clone();
				block.import = Some(v);

				if let Some(sync) =
					block.partial_syncs.iter().find(|ps| Some(ps.peer_id) == peer_id)
				{
					if let (Some(start_timestamp), Some(end_timestamp)) =
						(sync.start_timestamp, sync.end_timestamp)
					{
						block.sync = Some(IntervalDetailsSync {
							peer_id: sync.peer_id,
							start_timestamp,
							end_timestamp,
						})
					}
				}
			},
			IntervalDetails::PartialSync(v) => {
				// If we already have sync that is paired with import, then don't do anything.
				if block.sync.is_some() {
					return;
				}

				if let Some(p_sync) =
					block.partial_syncs.iter_mut().find(|s| s.peer_id == v.peer_id)
				{
					p_sync.start_timestamp = match v.start_timestamp {
						Some(s) => Some(s),
						_ => p_sync.start_timestamp,
					};

					p_sync.end_timestamp = match v.end_timestamp {
						Some(s) => Some(s),
						_ => p_sync.end_timestamp,
					};
				} else {
					block.partial_syncs.push(v);
				}
			},
			_ => (),
		};
	}

	///
	pub fn observe_block_request(value: BlockRequestsDetail) {
		let Ok(mut lock) = BLOCK_METRICS.lock() else {
			return;
		};

		lock.block_requests.push(value);

		if lock.block_requests.len() > lock.max_block_request_buffer_size {
			lock.block_requests.remove(0);
		}
	}

	///
	pub fn take_metrics() -> Option<BlockMetrics> {
		let Ok(mut lock) = BLOCK_METRICS.lock() else {
			return None;
		};

		let metrics = std::mem::take(&mut *lock);
		lock.max_interval_buffer_size = metrics.max_interval_buffer_size;
		lock.max_block_request_buffer_size = metrics.max_block_request_buffer_size;

		Some(metrics)
	}

	///
	pub fn get_current_timestamp_in_ms_or_default() -> u64 {
		Self::get_current_timestamp_in_ms().map(|v| v as u64).unwrap_or(0u64)
	}

	fn get_current_timestamp_in_ms() -> Result<u128, SystemTimeError> {
		let start = SystemTime::now();
		start.duration_since(UNIX_EPOCH).map(|f| f.as_millis())
	}
}

/// This will be send to the telemetry backend
pub mod external {
	use super::*;

	///
	#[derive(Debug, Serialize, Clone)]
	pub struct IntervalFromNode {
		///
		pub peer_id: Option<String>,
		///
		pub kind: IntervalKind,
		///
		pub start_timestamp: u64,
		///
		pub end_timestamp: u64,
	}

	impl From<IntervalDetailsProposal> for IntervalFromNode {
		fn from(value: IntervalDetailsProposal) -> Self {
			Self {
				peer_id: None,
				kind: IntervalKind::Proposal,
				start_timestamp: value.start_timestamp,
				end_timestamp: value.end_timestamp,
			}
		}
	}

	impl From<IntervalDetailsImport> for IntervalFromNode {
		fn from(value: IntervalDetailsImport) -> Self {
			let peer_id = value.peer_id.and_then(|p| Some(p.to_string()));
			Self {
				peer_id,
				kind: IntervalKind::Import,
				start_timestamp: value.start_timestamp,
				end_timestamp: value.end_timestamp,
			}
		}
	}

	impl From<IntervalDetailsSync> for IntervalFromNode {
		fn from(value: IntervalDetailsSync) -> Self {
			Self {
				peer_id: Some(value.peer_id.to_string()),
				kind: IntervalKind::Sync,
				start_timestamp: value.start_timestamp,
				end_timestamp: value.end_timestamp,
			}
		}
	}

	///
	#[derive(Debug, Default, Serialize, Clone)]
	pub struct BlockIntervalFromNode {
		///
		pub block_number: u64,
		///
		pub block_hash: String,
		///
		pub proposal: Option<IntervalFromNode>,
		///
		pub import: Option<IntervalFromNode>,
		///
		pub sync: Option<IntervalFromNode>,
	}

	///
	pub fn prepare_data(
		value: Option<HashMap<u64, HashMap<String, BlockIntervals>>>,
	) -> Vec<BlockIntervalFromNode> {
		let Some(block_heights) = value else {
			return Vec::new();
		};

		let mut processed_blocks: Vec<BlockIntervalFromNode> = Vec::new();

		for (block_number, forks) in block_heights {
			for (block_hash, data) in forks {
				let block = BlockIntervalFromNode {
					block_number,
					block_hash: block_hash.clone(),
					proposal: data.proposal.and_then(|p| Some(p.into())),
					import: data.import.and_then(|p| Some(p.into())),
					sync: data.sync.and_then(|p| Some(p.into())),
				};

				processed_blocks.push(block);
			}
		}

		processed_blocks
	}
}

///
pub struct CustomTelemetryWorker {
	///
	pub handle: Option<TelemetryHandle>,
	///
	pub sampling_interval_ms: u128,
	///
	pub max_interval_buffer_size: usize,
	///
	pub max_block_request_buffer_size: usize,
}

impl CustomTelemetryWorker {
	///
	pub async fn run(
		self,
		filter_intervals: Option<fn(Vec<BlockIntervalFromNode>) -> Vec<BlockIntervalFromNode>>,
		filter_block_requests: Option<fn(Vec<BlockRequestsDetail>) -> Vec<BlockRequestsDetail>>,
	) {
		const SLEEP_DURATION: Duration = Duration::from_millis(250);

		if let Ok(mut lock) = BLOCK_METRICS.lock() {
			lock.max_interval_buffer_size = self.max_interval_buffer_size;
			lock.max_block_request_buffer_size = self.max_block_request_buffer_size;
		}

		let mut start = std::time::Instant::now();
		loop {
			if start.elapsed().as_millis() >= self.sampling_interval_ms {
				self.send_telemetry(filter_intervals, filter_block_requests).await;
				start = std::time::Instant::now();
			}

			tokio::time::sleep(SLEEP_DURATION).await;
		}
	}

	///
	pub async fn send_telemetry(
		&self,
		filter_intervals: Option<fn(Vec<BlockIntervalFromNode>) -> Vec<BlockIntervalFromNode>>,
		filter_block_requests: Option<fn(Vec<BlockRequestsDetail>) -> Vec<BlockRequestsDetail>>,
	) {
		let (block_intervals, block_requests) =
			Self::get_and_filter_data(filter_intervals, filter_block_requests);

		if block_intervals.len() > 0 || block_requests.len() > 0 {
			telemetry!(
				self.handle;
				SUBSTRATE_INFO;
				"block.metrics";
				"block_intervals" => block_intervals,
				"block_requests" => block_requests,
			);
		}
	}

	pub(crate) fn get_and_filter_data(
		filter_intervals: Option<fn(Vec<BlockIntervalFromNode>) -> Vec<BlockIntervalFromNode>>,
		filter_block_requests: Option<fn(Vec<BlockRequestsDetail>) -> Vec<BlockRequestsDetail>>,
	) -> (Vec<BlockIntervalFromNode>, Vec<BlockRequestsDetail>) {
		let metrics = BlockMetrics::take_metrics().unwrap_or_default();
		dbg!(&metrics);

		let block_intervals = external::prepare_data(metrics.intervals);
		let block_intervals = match filter_intervals {
			Some(f) => f(block_intervals),
			_ => block_intervals,
		};

		let block_requests = metrics.block_requests;
		let block_requests = match filter_block_requests {
			Some(f) => f(block_requests),
			_ => block_requests,
		};

		dbg!(&block_intervals);

		(block_intervals, block_requests)
	}
}
