// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod signed;
mod staking;
// mod unsigned;

use frame_election_provider_support::{bounds::ElectionBounds, onchain, SequentialPhragmen};
pub use staking::*;

use crate::{
	self as epm,
	signed::{self as signed_pallet},
	verifier::{self as verifier_pallet},
	Config, *,
};
use frame_support::{derive_impl, pallet_prelude::*, parameter_types};
use sp_runtime::{BuildStorage, Perbill};

frame_support::construct_runtime!(
	pub struct Runtime {
		System: frame_system,
		Balances: pallet_balances,
		MultiPhase: epm,
		VerifierPallet: verifier_pallet,
		SignedPallet: signed_pallet,
	}
);

pub type AccountId = u64;
pub type Balance = u128;
pub type BlockNumber = u64;
pub type VoterIndex = u32;
pub type TargetIndex = u16;
pub type T = Runtime;
pub type Block = frame_system::mocking::MockBlock<Runtime>;

frame_election_provider_support::generate_solution_type!(
	#[compact]
	pub struct TestNposSolution::<
		VoterIndex = VoterIndex,
		TargetIndex = TargetIndex,
		Accuracy = sp_runtime::PerU16,
		MaxVoters = frame_support::traits::ConstU32::<2_000>
	>(16)
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Runtime {
	type Block = Block;
	type AccountData = pallet_balances::AccountData<Balance>;
}

parameter_types! {
	pub const ExistentialDeposit: Balance = 1;
}

impl pallet_balances::Config for Runtime {
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = ();
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type RuntimeHoldReason = RuntimeHoldReason;
	type MaxHolds = ConstU32<1>;
	type RuntimeFreezeReason = ();
}

parameter_types! {
	pub static SignedPhase: BlockNumber = 3;
	pub static UnsignedPhase: BlockNumber = 5;
	pub static SignedValidationPhase: BlockNumber = Pages::get().into();
	pub static Lookhaead: BlockNumber = 0;
	pub static VoterSnapshotPerBlock: VoterIndex = 4;
	pub static TargetSnapshotPerBlock: TargetIndex = 8;
	pub static Pages: PageIndex = 3;
}

impl Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type SignedPhase = SignedPhase;
	type UnsignedPhase = UnsignedPhase;
	type SignedValidationPhase = SignedValidationPhase;
	type Lookhaead = Lookhaead;
	type VoterSnapshotPerBlock = VoterSnapshotPerBlock;
	type TargetSnapshotPerBlock = TargetSnapshotPerBlock;
	type Pages = Pages;
	type DataProvider = MockStaking;
	type Solution = TestNposSolution;
	type Fallback = MockFallback;
	type Verifier = VerifierPallet;
}

parameter_types! {
	pub static SolutionImprovementThreshold: Perbill = Perbill::zero();
	pub static MaxWinnersPerPage: u32 = 10;
	pub static MaxBackersPerWinner: u32 = (staking::Targets::get().len() as u32).min(staking::DesiredTargets::get());
}

impl crate::verifier::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
	type SolutionImprovementThreshold = SolutionImprovementThreshold;
	type MaxBackersPerWinner = MaxBackersPerWinner;
	type MaxWinnersPerPage = MaxWinnersPerPage;
	type SolutionDataProvider = SignedPallet;
	type WeightInfo = ();
}

parameter_types! {
	pub static DepositBase: Balance = 10;
	pub static DepositPerPage: Balance = 1;
	pub static Reward: Balance = 10;
	pub static MaxSubmissions: u32 = 5;
}

impl crate::signed::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type EstimateCallFee = ConstU32<8>;
	type OnSlash = (); // burn
	type DepositBase = ConstDepositBase;
	type DepositPerPage = DepositPerPage;
	type Reward = Reward;
	type MaxSubmissions = MaxSubmissions;
	type RuntimeHoldReason = RuntimeHoldReason;
	type WeightInfo = ();
}

pub struct ConstDepositBase;
impl sp_runtime::traits::Convert<usize, Balance> for ConstDepositBase {
	fn convert(_a: usize) -> Balance {
		DepositBase::get()
	}
}

parameter_types! {
	pub static OnChainElectionBounds: ElectionBounds = ElectionBoundsBuilder::default().build();
	 pub static MaxVotesPerVoter: u32 = <TestNposSolution as NposSolution>::LIMIT as u32;
	pub static FallbackEnabled: bool = true;
}

impl onchain::Config for Runtime {
	type System = Runtime;
	type Solver = SequentialPhragmen<AccountId, Perbill, ()>;
	type MaxWinnersPerPage = MaxWinnersPerPage;
	type MaxBackersPerWinner = MaxBackersPerWinner;
	type Bounds = OnChainElectionBounds;
	type DataProvider = MockStaking;
	type WeightInfo = ();
}

pub struct MockFallback;
impl ElectionProvider for MockFallback {
	type AccountId = AccountId;
	type BlockNumber = BlockNumberFor<Runtime>;
	type Error = &'static str;
	type DataProvider = MockStaking;
	type Pages = ConstU32<1>;
	type MaxWinnersPerPage = MaxWinnersPerPage;
	type MaxBackersPerWinner = MaxBackersPerWinner;

	fn elect(remaining: PageIndex) -> Result<BoundedSupportsOf<Self>, Self::Error> {
		if FallbackEnabled::get() {
			onchain::OnChainExecution::<Runtime>::elect(remaining)
				.map_err(|_| "fallback election failed")
		} else {
			Err("fallback election failed (forced in mock)")
		}
	}
}

#[derive(Default)]
pub struct ExtBuilder;
impl ExtBuilder {
	pub(crate) fn pages(self, pages: u32) -> Self {
		Pages::set(pages);
		self
	}

	pub(crate) fn signed_phase(self, blocks: BlockNumber) -> Self {
		SignedPhase::set(blocks);
		self
	}

	pub(crate) fn validate_signed_phase(self, blocks: BlockNumber) -> Self {
		SignedValidationPhase::set(blocks);
		self
	}

	pub(crate) fn unsigned_phase(self, blocks: BlockNumber) -> Self {
		UnsignedPhase::set(blocks);
		self
	}

	pub(crate) fn lookahead(self, blocks: BlockNumber) -> Self {
		Lookhaead::set(blocks);
		self
	}

	pub(crate) fn build(self) -> sp_io::TestExternalities {
		sp_tracing::try_init_simple();

		let mut storage = frame_system::GenesisConfig::<T>::default().build_storage().unwrap();
		let _ = pallet_balances::GenesisConfig::<T> {
			balances: vec![
				// account for submitting stuff only.
				(91, 100),
				(92, 100),
				(93, 100),
				(94, 100),
				(95, 100),
				(96, 100),
				(97, 100),
				(99, 100),
				(999, 100),
				(9999, 100),
			],
		}
		.assimilate_storage(&mut storage);

		sp_io::TestExternalities::from(storage)
	}
	pub(crate) fn build_and_execute(self, test: impl FnOnce() -> ()) {
		let mut ext = self.build();
		ext.execute_with(test);
		// TODO(gpestana): add sanity checks and try_runtimes
	}
}

pub(crate) fn roll_to(n: BlockNumber) {
	let now = System::block_number();
	for bn in now + 1..n {
		System::set_block_number(bn);

		let election_prediction =
			<<Runtime as Config>::DataProvider as ElectionDataProvider>::next_election_prediction(
				bn,
			);

		log!(
			info,
			"Phase: {:?}, Round: {:?}, Election at {:?}",
			<CurrentPhase<T>>::get(),
			<Round<T>>::get(),
			election_prediction
		);
		MultiPhase::on_initialize(bn);
		VerifierPallet::on_initialize(bn);
		//SignedPallet::on_initialize(bn);
		//UnsignedPallet::on_initialize(bn);

		// TODO(gpestana): implement sanity check for all pallets.
		//all_pallets_sanity_checks();
	}
}
