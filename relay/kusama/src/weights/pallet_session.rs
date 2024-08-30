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

//! Autogenerated weights for `pallet_session`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-15, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ggwpez-ref-hw`, CPU: `AMD EPYC 7232P 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("./kusama-chain-spec.json")`, DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=./kusama-chain-spec.json
// --steps=50
// --repeat=20
// --pallet=pallet_session
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./kusama-weights/
// --header=./file_header.txt

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_session`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_session::WeightInfo for WeightInfo<T> {
	/// Storage: `Staking::Ledger` (r:1 w:0)
	/// Proof: `Staking::Ledger` (`max_values`: None, `max_size`: Some(1091), added: 3566, mode: `MaxEncodedLen`)
	/// Storage: `Session::NextKeys` (r:1 w:1)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Session::KeyOwner` (r:6 w:6)
	/// Proof: `Session::KeyOwner` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_keys() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2111`
		//  Estimated: `17951`
		// Minimum execution time: 58_610_000 picoseconds.
		Weight::from_parts(59_801_000, 0)
			.saturating_add(Weight::from_parts(0, 17951))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: `Staking::Ledger` (r:1 w:0)
	/// Proof: `Staking::Ledger` (`max_values`: None, `max_size`: Some(1091), added: 3566, mode: `MaxEncodedLen`)
	/// Storage: `Session::NextKeys` (r:1 w:1)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Session::KeyOwner` (r:0 w:6)
	/// Proof: `Session::KeyOwner` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn purge_keys() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1911`
		//  Estimated: `5376`
		// Minimum execution time: 41_440_000 picoseconds.
		Weight::from_parts(42_101_000, 0)
			.saturating_add(Weight::from_parts(0, 5376))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(7))
	}
}
