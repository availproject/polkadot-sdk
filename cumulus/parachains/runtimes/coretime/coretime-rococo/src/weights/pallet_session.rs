// Copyright Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_session`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-31, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm4`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("coretime-rococo-dev"), DB CACHE: 1024

// Executed Command:
// ./artifacts/polkadot-parachain
// benchmark
// pallet
// --chain=coretime-rococo-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_session
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./cumulus/parachains/runtimes/coretime/coretime-rococo/src/weights/pallet_session.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_session`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_session::WeightInfo for WeightInfo<T> {
	/// Storage: Session NextKeys (r:1 w:1)
	/// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	/// Storage: Session KeyOwner (r:1 w:1)
	/// Proof Skipped: Session KeyOwner (max_values: None, max_size: None, mode: Measured)
	fn set_keys() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `297`
		//  Estimated: `3762`
		// Minimum execution time: 17_809_000 picoseconds.
		Weight::from_parts(18_215_000, 0)
			.saturating_add(Weight::from_parts(0, 3762))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Session NextKeys (r:1 w:1)
	/// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	/// Storage: Session KeyOwner (r:0 w:1)
	/// Proof Skipped: Session KeyOwner (max_values: None, max_size: None, mode: Measured)
	fn purge_keys() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `279`
		//  Estimated: `3744`
		// Minimum execution time: 13_565_000 picoseconds.
		Weight::from_parts(13_841_000, 0)
			.saturating_add(Weight::from_parts(0, 3744))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
