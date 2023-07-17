// This file is part of Metaverse.Network & Bit.Country.

// Copyright (C) 2020-2022 Metaverse.Network & Bit.Country .
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

//! Autogenerated weights for metaverse
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-17, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/metaverse-node
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// metaverse
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --template=./template/weight-template.hbs
// --output
// ./pallets/metaverse/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for metaverse.
pub trait WeightInfo {	fn create_metaverse() -> Weight;	fn transfer_metaverse() -> Weight;	fn freeze_metaverse() -> Weight;	fn unfreeze_metaverse() -> Weight;	fn destroy_metaverse() -> Weight;	fn update_metaverse_listing_fee() -> Weight;	fn withdraw_from_metaverse_fund() -> Weight;}

/// Weights for metaverse using the for collator node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {	// Storage: System Account (r:3 w:3)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Metaverse NextMetaverseId (r:1 w:1)
	// Proof Skipped: Metaverse NextMetaverseId (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: OrmlNFT NextClassId (r:1 w:1)
	// Proof Skipped: OrmlNFT NextClassId (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Nft GroupCollections (r:1 w:0)
	// Proof Skipped: Nft GroupCollections (max_values: None, max_size: None, mode: Measured)
	// Storage: Metaverse AllMetaversesCount (r:1 w:1)
	// Proof Skipped: Metaverse AllMetaversesCount (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: OrmlNFT Classes (r:0 w:2)
	// Proof Skipped: OrmlNFT Classes (max_values: None, max_size: None, mode: Measured)
	// Storage: Metaverse MetaverseOwner (r:0 w:1)
	// Proof Skipped: Metaverse MetaverseOwner (max_values: None, max_size: None, mode: Measured)
	// Storage: Metaverse Metaverses (r:0 w:1)
	// Proof Skipped: Metaverse Metaverses (max_values: None, max_size: None, mode: Measured)
	// Storage: Nft ClassDataCollection (r:0 w:2)
	// Proof Skipped: Nft ClassDataCollection (max_values: None, max_size: None, mode: Measured)
	fn create_metaverse() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1949`
		//  Estimated: `27361`
		// Minimum execution time: 141_186 nanoseconds.
		Weight::from_parts(144_874_000, 27361)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(12))
	}
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Metaverse MetaverseOwner (r:1 w:2)
	// Proof Skipped: Metaverse MetaverseOwner (max_values: None, max_size: None, mode: Measured)
	// Storage: Metaverse Metaverses (r:1 w:1)
	// Proof Skipped: Metaverse Metaverses (max_values: None, max_size: None, mode: Measured)
	fn transfer_metaverse() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1852`
		//  Estimated: `13860`
		// Minimum execution time: 51_299 nanoseconds.
		Weight::from_parts(53_412_000, 13860)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: Metaverse Metaverses (r:1 w:1)
	// Proof Skipped: Metaverse Metaverses (max_values: None, max_size: None, mode: Measured)
	fn freeze_metaverse() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1247`
		//  Estimated: `3722`
		// Minimum execution time: 25_115 nanoseconds.
		Weight::from_parts(26_870_000, 3722)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Metaverse Metaverses (r:1 w:1)
	// Proof Skipped: Metaverse Metaverses (max_values: None, max_size: None, mode: Measured)
	fn unfreeze_metaverse() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1247`
		//  Estimated: `3722`
		// Minimum execution time: 25_233 nanoseconds.
		Weight::from_parts(27_836_000, 3722)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Metaverse Metaverses (r:1 w:1)
	// Proof Skipped: Metaverse Metaverses (max_values: None, max_size: None, mode: Measured)
	// Storage: Metaverse MetaverseOwner (r:0 w:1)
	// Proof Skipped: Metaverse MetaverseOwner (max_values: None, max_size: None, mode: Measured)
	fn destroy_metaverse() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1247`
		//  Estimated: `4969`
		// Minimum execution time: 27_011 nanoseconds.
		Weight::from_parts(28_903_000, 4969)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Metaverse MetaverseOwner (r:1 w:0)
	// Proof Skipped: Metaverse MetaverseOwner (max_values: None, max_size: None, mode: Measured)
	// Storage: Metaverse Metaverses (r:1 w:1)
	// Proof Skipped: Metaverse Metaverses (max_values: None, max_size: None, mode: Measured)
	fn update_metaverse_listing_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1292`
		//  Estimated: `7534`
		// Minimum execution time: 28_885 nanoseconds.
		Weight::from_parts(30_034_000, 7534)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Metaverse MetaverseOwner (r:1 w:0)
	// Proof Skipped: Metaverse MetaverseOwner (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn withdraw_from_metaverse_fund() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1763`
		//  Estimated: `9444`
		// Minimum execution time: 46_756 nanoseconds.
		Weight::from_parts(47_894_000, 9444)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {	fn create_metaverse() -> Weight {
		Weight::from_parts(144_874_000, 27361)
			.saturating_add(RocksDbWeight::get().reads(7))
			.saturating_add(RocksDbWeight::get().writes(12))
	}
	fn transfer_metaverse() -> Weight {
		Weight::from_parts(53_412_000, 13860)
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(5))
	}
	fn freeze_metaverse() -> Weight {
		Weight::from_parts(26_870_000, 3722)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	fn unfreeze_metaverse() -> Weight {
		Weight::from_parts(27_836_000, 3722)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	fn destroy_metaverse() -> Weight {
		Weight::from_parts(28_903_000, 4969)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	fn update_metaverse_listing_fee() -> Weight {
		Weight::from_parts(30_034_000, 7534)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	fn withdraw_from_metaverse_fund() -> Weight {
		Weight::from_parts(47_894_000, 9444)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
}
