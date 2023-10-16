// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for pallet_asset_conversion
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-18, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-gghbxkbs-project-145-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: ``, WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// target/production/substrate
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=/builds/parity/mirrors/substrate/.git/.artifacts/bench.json
// --pallet=pallet_asset_conversion
// --chain=dev
// --header=./HEADER-APACHE2
// --output=./frame/asset-conversion/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_asset_conversion.
pub trait WeightInfo {
	fn create_pool() -> Weight;
	fn start_destroy_pool() -> Weight;
	fn destroy_lp_token_accounts() -> Weight;
	fn destroy_lp_token_approvals() -> Weight;
	fn finish_destroy_pool() -> Weight;
	fn add_liquidity() -> Weight;
	fn remove_liquidity() -> Weight;
	fn swap_exact_tokens_for_tokens() -> Weight;
	fn swap_tokens_for_exact_tokens() -> Weight;
}

/// Weights for pallet_asset_conversion using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `AssetConversion::Pools` (r:1 w:1)
	/// Proof: `AssetConversion::Pools` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `AssetConversion::NextPoolAssetId` (r:1 w:1)
	/// Proof: `AssetConversion::NextPoolAssetId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Asset` (r:1 w:1)
	/// Proof: `PoolAssets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Account` (r:1 w:1)
	/// Proof: `PoolAssets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	fn create_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `729`
		//  Estimated: `6196`
		// Minimum execution time: 131_688_000 picoseconds.
		Weight::from_parts(134_092_000, 6196)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}

	fn destroy_lp_token_accounts() -> Weight {
			Weight::from_parts(134_092_000, 6196)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}

	fn destroy_lp_token_approvals() -> Weight {
			Weight::from_parts(134_092_000, 6196)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}

	fn start_destroy_pool() -> Weight {
			Weight::from_parts(134_092_000, 6196)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	fn finish_destroy_pool() -> Weight {
			Weight::from_parts(134_092_000, 6196)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: `AssetConversion::Pools` (r:1 w:0)
	/// Proof: `AssetConversion::Pools` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Asset` (r:1 w:1)
	/// Proof: `PoolAssets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Account` (r:2 w:2)
	/// Proof: `PoolAssets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	fn add_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1382`
		//  Estimated: `6208`
		// Minimum execution time: 157_310_000 picoseconds.
		Weight::from_parts(161_547_000, 6208)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: `AssetConversion::Pools` (r:1 w:0)
	/// Proof: `AssetConversion::Pools` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Asset` (r:1 w:1)
	/// Proof: `PoolAssets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Account` (r:1 w:1)
	/// Proof: `PoolAssets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	fn remove_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1371`
		//  Estimated: `6208`
		// Minimum execution time: 142_769_000 picoseconds.
		Weight::from_parts(145_139_000, 6208)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:3 w:3)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:6 w:6)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	fn swap_exact_tokens_for_tokens() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1738`
		//  Estimated: `16644`
		// Minimum execution time: 213_186_000 picoseconds.
		Weight::from_parts(217_471_000, 16644)
			.saturating_add(T::DbWeight::get().reads(10_u64))
			.saturating_add(T::DbWeight::get().writes(10_u64))
	}
	/// Storage: `Assets::Asset` (r:3 w:3)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:6 w:6)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn swap_tokens_for_exact_tokens() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1738`
		//  Estimated: `16644`
		// Minimum execution time: 213_793_000 picoseconds.
		Weight::from_parts(218_584_000, 16644)
			.saturating_add(T::DbWeight::get().reads(10_u64))
			.saturating_add(T::DbWeight::get().writes(10_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `AssetConversion::Pools` (r:1 w:1)
	/// Proof: `AssetConversion::Pools` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `AssetConversion::NextPoolAssetId` (r:1 w:1)
	/// Proof: `AssetConversion::NextPoolAssetId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Asset` (r:1 w:1)
	/// Proof: `PoolAssets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Account` (r:1 w:1)
	/// Proof: `PoolAssets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	fn create_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `729`
		//  Estimated: `6196`
		// Minimum execution time: 131_688_000 picoseconds.
		Weight::from_parts(134_092_000, 6196)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(8_u64))
	}

	fn start_destroy_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `729`
		//  Estimated: `6196`
		// Minimum execution time: 131_688_000 picoseconds.
		Weight::from_parts(134_092_000, 6196)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(8_u64))
	}
	fn destroy_lp_token_accounts() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `729`
		//  Estimated: `6196`
		// Minimum execution time: 131_688_000 picoseconds.
		Weight::from_parts(134_092_000, 6196)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(8_u64))
	}
	fn destroy_lp_token_approvals() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `729`
		//  Estimated: `6196`
		// Minimum execution time: 131_688_000 picoseconds.
		Weight::from_parts(134_092_000, 6196)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(8_u64))
	}
	fn finish_destroy_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `729`
		//  Estimated: `6196`
		// Minimum execution time: 131_688_000 picoseconds.
		Weight::from_parts(134_092_000, 6196)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(8_u64))
	}
	/// Storage: `AssetConversion::Pools` (r:1 w:0)
	/// Proof: `AssetConversion::Pools` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Asset` (r:1 w:1)
	/// Proof: `PoolAssets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Account` (r:2 w:2)
	/// Proof: `PoolAssets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	fn add_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1382`
		//  Estimated: `6208`
		// Minimum execution time: 157_310_000 picoseconds.
		Weight::from_parts(161_547_000, 6208)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	/// Storage: `AssetConversion::Pools` (r:1 w:0)
	/// Proof: `AssetConversion::Pools` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Asset` (r:1 w:1)
	/// Proof: `PoolAssets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Account` (r:1 w:1)
	/// Proof: `PoolAssets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	fn remove_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1371`
		//  Estimated: `6208`
		// Minimum execution time: 142_769_000 picoseconds.
		Weight::from_parts(145_139_000, 6208)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:3 w:3)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:6 w:6)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	fn swap_exact_tokens_for_tokens() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1738`
		//  Estimated: `16644`
		// Minimum execution time: 213_186_000 picoseconds.
		Weight::from_parts(217_471_000, 16644)
			.saturating_add(RocksDbWeight::get().reads(10_u64))
			.saturating_add(RocksDbWeight::get().writes(10_u64))
	}
	/// Storage: `Assets::Asset` (r:3 w:3)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:6 w:6)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn swap_tokens_for_exact_tokens() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1738`
		//  Estimated: `16644`
		// Minimum execution time: 213_793_000 picoseconds.
		Weight::from_parts(218_584_000, 16644)
			.saturating_add(RocksDbWeight::get().reads(10_u64))
			.saturating_add(RocksDbWeight::get().writes(10_u64))
	}
}
