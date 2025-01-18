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

//! Autogenerated weights for `polkadot_runtime_common::claims`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2025-01-06, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ggwpez-ref-hw`, CPU: `AMD EPYC 7232P 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("./polkadot-chain-spec.json")`, DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=./polkadot-chain-spec.json
// --steps=50
// --repeat=20
// --pallet=polkadot_runtime_common::claims
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./polkadot-weights/
// --header=./file_header.txt

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `polkadot_runtime_common::claims`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> polkadot_runtime_common::claims::WeightInfo for WeightInfo<T> {
	/// Storage: `Claims::Claims` (r:1 w:1)
	/// Proof: `Claims::Claims` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Signing` (r:1 w:1)
	/// Proof: `Claims::Signing` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Total` (r:1 w:1)
	/// Proof: `Claims::Total` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Vesting` (r:1 w:1)
	/// Proof: `Claims::Vesting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Vesting::Vesting` (r:1 w:1)
	/// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	fn claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `612`
		//  Estimated: `4764`
		// Minimum execution time: 191_971_000 picoseconds.
		Weight::from_parts(196_681_000, 0)
			.saturating_add(Weight::from_parts(0, 4764))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `Claims::Total` (r:1 w:1)
	/// Proof: `Claims::Total` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Vesting` (r:0 w:1)
	/// Proof: `Claims::Vesting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Claims` (r:0 w:1)
	/// Proof: `Claims::Claims` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Signing` (r:0 w:1)
	/// Proof: `Claims::Signing` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn mint_claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `182`
		//  Estimated: `1667`
		// Minimum execution time: 14_371_000 picoseconds.
		Weight::from_parts(14_751_000, 0)
			.saturating_add(Weight::from_parts(0, 1667))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Claims::Claims` (r:1 w:1)
	/// Proof: `Claims::Claims` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Signing` (r:1 w:1)
	/// Proof: `Claims::Signing` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Total` (r:1 w:1)
	/// Proof: `Claims::Total` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Vesting` (r:1 w:1)
	/// Proof: `Claims::Vesting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Vesting::Vesting` (r:1 w:1)
	/// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	fn claim_attest() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `612`
		//  Estimated: `4764`
		// Minimum execution time: 197_341_000 picoseconds.
		Weight::from_parts(202_321_000, 0)
			.saturating_add(Weight::from_parts(0, 4764))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `Claims::Preclaims` (r:1 w:1)
	/// Proof: `Claims::Preclaims` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Signing` (r:1 w:1)
	/// Proof: `Claims::Signing` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Claims` (r:1 w:1)
	/// Proof: `Claims::Claims` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Total` (r:1 w:1)
	/// Proof: `Claims::Total` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Vesting` (r:1 w:1)
	/// Proof: `Claims::Vesting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Vesting::Vesting` (r:1 w:1)
	/// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(193), added: 2668, mode: `MaxEncodedLen`)
	fn attest() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `686`
		//  Estimated: `4764`
		// Minimum execution time: 87_791_000 picoseconds.
		Weight::from_parts(96_440_000, 0)
			.saturating_add(Weight::from_parts(0, 4764))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: `Claims::Claims` (r:1 w:2)
	/// Proof: `Claims::Claims` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Vesting` (r:1 w:2)
	/// Proof: `Claims::Vesting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Signing` (r:1 w:2)
	/// Proof: `Claims::Signing` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Claims::Preclaims` (r:1 w:1)
	/// Proof: `Claims::Preclaims` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn move_claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `406`
		//  Estimated: `3871`
		// Minimum execution time: 35_420_000 picoseconds.
		Weight::from_parts(37_110_000, 0)
			.saturating_add(Weight::from_parts(0, 3871))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(7))
	}
}
