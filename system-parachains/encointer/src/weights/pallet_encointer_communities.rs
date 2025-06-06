// Copyright (C) Parity Technologies and the various Polkadot contributors, see Contributions.md
// for a list of specific contributors.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_encointer_communities`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2025-01-05, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ggwpez-ref-hw`, CPU: `AMD EPYC 7232P 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("./encointer-kusama-chain-spec.json")`, DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot-parachain
// benchmark
// pallet
// --chain=./encointer-kusama-chain-spec.json
// --steps=50
// --repeat=20
// --pallet=pallet_encointer_communities
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./encointer-kusama-weights/
// --header=./file_header.txt

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_encointer_communities`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_encointer_communities::WeightInfo for WeightInfo<T> {
	/// Storage: `EncointerCommunities::CommunityIdentifiers` (r:1 w:1)
	/// Proof: `EncointerCommunities::CommunityIdentifiers` (`max_values`: Some(1), `max_size`: Some(90002), added: 90497, mode: `MaxEncodedLen`)
	/// Storage: `EncointerCommunities::MaxSpeedMps` (r:1 w:0)
	/// Proof: `EncointerCommunities::MaxSpeedMps` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `EncointerCommunities::MinSolarTripTimeS` (r:1 w:0)
	/// Proof: `EncointerCommunities::MinSolarTripTimeS` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `EncointerCommunities::CommunityIdentifiersByGeohash` (r:1 w:1)
	/// Proof: `EncointerCommunities::CommunityIdentifiersByGeohash` (`max_values`: None, `max_size`: Some(90007), added: 92482, mode: `MaxEncodedLen`)
	/// Storage: `EncointerCommunities::Locations` (r:1 w:1)
	/// Proof: `EncointerCommunities::Locations` (`max_values`: None, `max_size`: Some(320032), added: 322507, mode: `MaxEncodedLen`)
	/// Storage: `EncointerCommunities::NominalIncome` (r:0 w:1)
	/// Proof: `EncointerCommunities::NominalIncome` (`max_values`: None, `max_size`: Some(41), added: 2516, mode: `MaxEncodedLen`)
	/// Storage: `EncointerCommunities::CommunityMetadata` (r:0 w:1)
	/// Proof: `EncointerCommunities::CommunityMetadata` (`max_values`: None, `max_size`: Some(1352), added: 3827, mode: `MaxEncodedLen`)
	/// Storage: `EncointerCommunities::Bootstrappers` (r:0 w:1)
	/// Proof: `EncointerCommunities::Bootstrappers` (`max_values`: None, `max_size`: Some(320027), added: 322502, mode: `MaxEncodedLen`)
	/// Storage: `EncointerBalances::DemurragePerBlock` (r:0 w:1)
	/// Proof: `EncointerBalances::DemurragePerBlock` (`max_values`: None, `max_size`: Some(41), added: 2516, mode: `MaxEncodedLen`)
	fn new_community() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6448`
		//  Estimated: `323497`
		// Minimum execution time: 4_977_894_000 picoseconds.
		Weight::from_parts(4_992_334_000, 0)
			.saturating_add(Weight::from_parts(0, 323497))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: `EncointerScheduler::CurrentPhase` (r:1 w:0)
	/// Proof: `EncointerScheduler::CurrentPhase` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `EncointerCommunities::CommunityIdentifiers` (r:1 w:0)
	/// Proof: `EncointerCommunities::CommunityIdentifiers` (`max_values`: Some(1), `max_size`: Some(90002), added: 90497, mode: `MaxEncodedLen`)
	/// Storage: `EncointerCommunities::MaxSpeedMps` (r:1 w:0)
	/// Proof: `EncointerCommunities::MaxSpeedMps` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `EncointerCommunities::MinSolarTripTimeS` (r:1 w:0)
	/// Proof: `EncointerCommunities::MinSolarTripTimeS` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `EncointerCommunities::CommunityIdentifiersByGeohash` (r:1 w:0)
	/// Proof: `EncointerCommunities::CommunityIdentifiersByGeohash` (`max_values`: None, `max_size`: Some(90007), added: 92482, mode: `MaxEncodedLen`)
	/// Storage: `EncointerCommunities::Locations` (r:1 w:1)
	/// Proof: `EncointerCommunities::Locations` (`max_values`: None, `max_size`: Some(320032), added: 322507, mode: `MaxEncodedLen`)
	fn add_location() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6521`
		//  Estimated: `323497`
		// Minimum execution time: 4_970_953_000 picoseconds.
		Weight::from_parts(4_984_214_000, 0)
			.saturating_add(Weight::from_parts(0, 323497))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `EncointerScheduler::CurrentPhase` (r:1 w:0)
	/// Proof: `EncointerScheduler::CurrentPhase` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `EncointerCommunities::CommunityIdentifiers` (r:1 w:0)
	/// Proof: `EncointerCommunities::CommunityIdentifiers` (`max_values`: Some(1), `max_size`: Some(90002), added: 90497, mode: `MaxEncodedLen`)
	/// Storage: `EncointerCommunities::Locations` (r:1 w:1)
	/// Proof: `EncointerCommunities::Locations` (`max_values`: None, `max_size`: Some(320032), added: 322507, mode: `MaxEncodedLen`)
	fn remove_location() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6500`
		//  Estimated: `323497`
		// Minimum execution time: 37_240_000 picoseconds.
		Weight::from_parts(37_850_000, 0)
			.saturating_add(Weight::from_parts(0, 323497))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `EncointerCommunities::CommunityIdentifiers` (r:1 w:0)
	/// Proof: `EncointerCommunities::CommunityIdentifiers` (`max_values`: Some(1), `max_size`: Some(90002), added: 90497, mode: `MaxEncodedLen`)
	/// Storage: `EncointerCommunities::CommunityMetadata` (r:0 w:1)
	/// Proof: `EncointerCommunities::CommunityMetadata` (`max_values`: None, `max_size`: Some(1352), added: 3827, mode: `MaxEncodedLen`)
	fn update_community_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `221`
		//  Estimated: `91487`
		// Minimum execution time: 20_720_000 picoseconds.
		Weight::from_parts(21_220_000, 0)
			.saturating_add(Weight::from_parts(0, 91487))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `EncointerCommunities::CommunityIdentifiers` (r:1 w:0)
	/// Proof: `EncointerCommunities::CommunityIdentifiers` (`max_values`: Some(1), `max_size`: Some(90002), added: 90497, mode: `MaxEncodedLen`)
	/// Storage: `EncointerBalances::DemurragePerBlock` (r:0 w:1)
	/// Proof: `EncointerBalances::DemurragePerBlock` (`max_values`: None, `max_size`: Some(41), added: 2516, mode: `MaxEncodedLen`)
	fn update_demurrage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `221`
		//  Estimated: `91487`
		// Minimum execution time: 18_360_000 picoseconds.
		Weight::from_parts(18_981_000, 0)
			.saturating_add(Weight::from_parts(0, 91487))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `EncointerCommunities::CommunityIdentifiers` (r:1 w:0)
	/// Proof: `EncointerCommunities::CommunityIdentifiers` (`max_values`: Some(1), `max_size`: Some(90002), added: 90497, mode: `MaxEncodedLen`)
	/// Storage: `EncointerCommunities::NominalIncome` (r:0 w:1)
	/// Proof: `EncointerCommunities::NominalIncome` (`max_values`: None, `max_size`: Some(41), added: 2516, mode: `MaxEncodedLen`)
	fn update_nominal_income() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `221`
		//  Estimated: `91487`
		// Minimum execution time: 18_300_000 picoseconds.
		Weight::from_parts(18_740_000, 0)
			.saturating_add(Weight::from_parts(0, 91487))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `EncointerCommunities::MinSolarTripTimeS` (r:0 w:1)
	/// Proof: `EncointerCommunities::MinSolarTripTimeS` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn set_min_solar_trip_time_s() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_080_000 picoseconds.
		Weight::from_parts(9_470_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `EncointerCommunities::MaxSpeedMps` (r:0 w:1)
	/// Proof: `EncointerCommunities::MaxSpeedMps` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn set_max_speed_mps() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_120_000 picoseconds.
		Weight::from_parts(9_391_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `EncointerCommunities::Locations` (r:2 w:1)
	/// Proof: `EncointerCommunities::Locations` (`max_values`: None, `max_size`: Some(320032), added: 322507, mode: `MaxEncodedLen`)
	/// Storage: `EncointerCommunities::CommunityIdentifiersByGeohash` (r:1 w:1)
	/// Proof: `EncointerCommunities::CommunityIdentifiersByGeohash` (`max_values`: None, `max_size`: Some(90007), added: 92482, mode: `MaxEncodedLen`)
	/// Storage: `EncointerCommunities::CommunityIdentifiers` (r:1 w:1)
	/// Proof: `EncointerCommunities::CommunityIdentifiers` (`max_values`: Some(1), `max_size`: Some(90002), added: 90497, mode: `MaxEncodedLen`)
	/// Storage: `EncointerCommunities::NominalIncome` (r:0 w:1)
	/// Proof: `EncointerCommunities::NominalIncome` (`max_values`: None, `max_size`: Some(41), added: 2516, mode: `MaxEncodedLen`)
	/// Storage: `EncointerCommunities::CommunityMetadata` (r:0 w:1)
	/// Proof: `EncointerCommunities::CommunityMetadata` (`max_values`: None, `max_size`: Some(1352), added: 3827, mode: `MaxEncodedLen`)
	/// Storage: `EncointerCommunities::Bootstrappers` (r:0 w:1)
	/// Proof: `EncointerCommunities::Bootstrappers` (`max_values`: None, `max_size`: Some(320027), added: 322502, mode: `MaxEncodedLen`)
	fn purge_community() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `482`
		//  Estimated: `646004`
		// Minimum execution time: 59_971_000 picoseconds.
		Weight::from_parts(60_821_000, 0)
			.saturating_add(Weight::from_parts(0, 646004))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(6))
	}
}
