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

use crate as pallet_ambassador_governance;
use codec::MaxEncodedLen;
use frame_support::{
	pallet_prelude::*,
	parameter_types,
	traits::{
		tokens::{Balance as BalanceTrait, WithdrawReasons},
		ConstU16, ConstU32, ConstU64, EnsureOrigin, Everything, Get, OnUnbalanced,
		PalletInfoAccess,
	},
	PalletId, Parameter, StorageMap,
};
use frame_system::{self as system, RawOrigin};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};
use sp_weights::Weight;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub BlockWeights: frame_system::limits::BlockWeights = frame_system::limits::BlockWeights::with_sensible_defaults(Weight::from_parts(2_000_000_000_000, u64::MAX), sp_runtime::Perbill::from_percent(75));
	pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength::max_with_normal_ratio(64 * 1024, sp_runtime::Perbill::from_percent(75));
	pub const AmbassadorGovernancePalletId: PalletId = PalletId(*b"ambgov!!");
}

// Mock pallet_balances implementation
type AccountId = u64;
type Balance = u128;

parameter_types! {
	pub const ExistentialDeposit: Balance = 1;
}

// Configure a mock runtime to test the pallet
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Test;

// Construct mock runtime
frame_support::construct_runtime!(pub enum Runtime where
	Block = Block,
	NodeBlock = Block,
	UncheckedExtrinsic = UncheckedExtrinsic
{
	System: frame_system,
	Balances: pallet_balances,
	AmbassadorGovernance: pallet_ambassador_governance,
});

// Type aliases for the runtime
pub type Call = RuntimeCall;

// Helper function to create signed origins correctly
pub fn signed_origin(who: u64) -> RuntimeOrigin {
	frame_system::RawOrigin::Signed(who).into()
}
pub type Event = RuntimeEvent;

// Define the Block type
pub type Block = frame_system::mocking::MockBlock<Runtime>;

// Define the Nonce type
pub type Nonce = u32;

// Define UncheckedExtrinsic type
pub type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;

// Define type aliases for BlockNumber
pub type BlockNumber = u64;

// PalletInfo is implemented for Runtime by the construct_runtime macro

impl system::Config for Runtime {
	type BaseCallFilter = Everything;
	type BlockWeights = BlockWeights;
	type BlockLength = BlockLength;
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
	type Block = Block;
	type Nonce = Nonce;
	type MultiBlockMigrator = ();
	type PreInherents = ();
	type PostInherents = ();
	type PostTransactions = ();
	type RuntimeTask = ();
	type SingleBlockMigrations = ();
	type ExtensionsWeightInfo = ();
}

impl pallet_balances::Config for Runtime {
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type RuntimeHoldReason = ();
	type RuntimeFreezeReason = ();
	type DoneSlashHandler = ();
}

// Mock origin types for different ambassador ranks
pub struct MockEmergencyOrigin;
impl EnsureOrigin<RuntimeOrigin> for MockEmergencyOrigin {
	type Success = AccountId;
	fn try_origin(o: RuntimeOrigin) -> Result<Self::Success, RuntimeOrigin> {
		let o_clone = o.clone();
		let result = system::ensure_signed(o);
		result.map_err(|_| o_clone)
	}
	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<RuntimeOrigin, ()> {
		Ok(signed_origin(1))
	}
}
pub struct EnsureRankI;
impl EnsureOrigin<RuntimeOrigin> for EnsureRankI {
	type Success = u64;
	fn try_origin(o: RuntimeOrigin) -> Result<Self::Success, RuntimeOrigin> {
		let o_clone = o.clone();
		match <RuntimeOrigin as Into<Result<frame_system::RawOrigin<u64>, RuntimeOrigin>>>::into(
			o_clone,
		)
		.map_err(|_| o.clone())?
		{
			frame_system::RawOrigin::Signed(who) if who == 1 => Ok(who),
			_ => Err(o),
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<RuntimeOrigin, ()> {
		Ok(signed_origin(1))
	}
}

pub struct EnsureRankII;
impl EnsureOrigin<RuntimeOrigin> for EnsureRankII {
	type Success = u64;
	fn try_origin(o: RuntimeOrigin) -> Result<Self::Success, RuntimeOrigin> {
		let o_clone = o.clone();
		match <RuntimeOrigin as Into<Result<frame_system::RawOrigin<u64>, RuntimeOrigin>>>::into(
			o_clone,
		)
		.map_err(|_| o.clone())?
		{
			frame_system::RawOrigin::Signed(who) if who == 2 => Ok(who),
			_ => Err(o),
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<RuntimeOrigin, ()> {
		Ok(signed_origin(2))
	}
}

pub struct EnsureRankIII;
impl EnsureOrigin<RuntimeOrigin> for EnsureRankIII {
	type Success = u64;
	fn try_origin(o: RuntimeOrigin) -> Result<Self::Success, RuntimeOrigin> {
		let o_clone = o.clone();
		match <RuntimeOrigin as Into<Result<frame_system::RawOrigin<u64>, RuntimeOrigin>>>::into(
			o_clone,
		)
		.map_err(|_| o.clone())?
		{
			frame_system::RawOrigin::Signed(who) if who == 3 => Ok(who),
			_ => Err(o),
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<RuntimeOrigin, ()> {
		Ok(signed_origin(3))
	}
}

pub struct EnsureRankIV;
impl EnsureOrigin<RuntimeOrigin> for EnsureRankIV {
	type Success = u64;
	fn try_origin(o: RuntimeOrigin) -> Result<Self::Success, RuntimeOrigin> {
		let o_clone = o.clone();
		match <RuntimeOrigin as Into<Result<frame_system::RawOrigin<u64>, RuntimeOrigin>>>::into(
			o_clone,
		)
		.map_err(|_| o.clone())?
		{
			frame_system::RawOrigin::Signed(who) if who == 4 => Ok(who),
			_ => Err(o),
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<RuntimeOrigin, ()> {
		Ok(signed_origin(4))
	}
}

pub struct EnsureRankV;
impl EnsureOrigin<RuntimeOrigin> for EnsureRankV {
	type Success = u64;
	fn try_origin(o: RuntimeOrigin) -> Result<Self::Success, RuntimeOrigin> {
		let o_clone = o.clone();
		match <RuntimeOrigin as Into<Result<frame_system::RawOrigin<u64>, RuntimeOrigin>>>::into(
			o_clone,
		)
		.map_err(|_| o.clone())?
		{
			frame_system::RawOrigin::Signed(who) if who == 5 => Ok(who),
			_ => Err(o),
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<RuntimeOrigin, ()> {
		Ok(signed_origin(5))
	}
}

pub struct EnsureRankVI;
impl EnsureOrigin<RuntimeOrigin> for EnsureRankVI {
	type Success = u64;
	fn try_origin(o: RuntimeOrigin) -> Result<Self::Success, RuntimeOrigin> {
		let o_clone = o.clone();
		match <RuntimeOrigin as Into<Result<frame_system::RawOrigin<u64>, RuntimeOrigin>>>::into(
			o_clone,
		)
		.map_err(|_| o.clone())?
		{
			frame_system::RawOrigin::Signed(who) if who == 6 => Ok(who),
			_ => Err(o),
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<RuntimeOrigin, ()> {
		Ok(signed_origin(6))
	}
}

pub struct MockCommitteeFormationOrigin;
impl EnsureOrigin<RuntimeOrigin> for MockCommitteeFormationOrigin {
	type Success = AccountId;
	fn try_origin(o: RuntimeOrigin) -> Result<Self::Success, RuntimeOrigin> {
		let o_clone = o.clone();
		let result = system::ensure_signed(o);
		result.map_err(|_| o_clone)
	}
	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<RuntimeOrigin, ()> {
		Ok(signed_origin(1))
	}
}

pub struct MockEmergencyResolutionOrigin;
impl EnsureOrigin<RuntimeOrigin> for MockEmergencyResolutionOrigin {
	type Success = AccountId;
	fn try_origin(o: RuntimeOrigin) -> Result<Self::Success, RuntimeOrigin> {
		let o_clone = o.clone();
		let result = system::ensure_signed(o);
		result.map_err(|_| o_clone)
	}
	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<RuntimeOrigin, ()> {
		Ok(signed_origin(1))
	}
}

pub struct MockAppealSubmissionOrigin;
impl EnsureOrigin<RuntimeOrigin> for MockAppealSubmissionOrigin {
	type Success = AccountId;
	fn try_origin(o: RuntimeOrigin) -> Result<Self::Success, RuntimeOrigin> {
		let o_clone = o.clone();
		let result = system::ensure_signed(o);
		result.map_err(|_| o_clone)
	}
	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<RuntimeOrigin, ()> {
		Ok(signed_origin(1))
	}
}

pub struct MockAppealCommitteeOrigin;
impl EnsureOrigin<RuntimeOrigin> for MockAppealCommitteeOrigin {
	type Success = AccountId;
	fn try_origin(o: RuntimeOrigin) -> Result<Self::Success, RuntimeOrigin> {
		let o_clone = o.clone();
		let result = system::ensure_signed(o);
		result.map_err(|_| o_clone)
	}
	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<RuntimeOrigin, ()> {
		Ok(signed_origin(1))
	}
}

pub struct MockAppealDecisionOrigin;
impl EnsureOrigin<RuntimeOrigin> for MockAppealDecisionOrigin {
	type Success = AccountId;
	fn try_origin(o: RuntimeOrigin) -> Result<Self::Success, RuntimeOrigin> {
		let o_clone = o.clone();
		let result = system::ensure_signed(o);
		result.map_err(|_| o_clone)
	}
	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<RuntimeOrigin, ()> {
		Ok(signed_origin(1))
	}
}

pub struct MockIntegrationOrigin;
impl EnsureOrigin<RuntimeOrigin> for MockIntegrationOrigin {
	type Success = AccountId;
	fn try_origin(o: RuntimeOrigin) -> Result<Self::Success, RuntimeOrigin> {
		let o_clone = o.clone();
		let result = system::ensure_signed(o);
		result.map_err(|_| o_clone)
	}
	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<RuntimeOrigin, ()> {
		Ok(signed_origin(1))
	}
}

parameter_types! {
	pub const MaxJustificationLength: u32 = 1000;
	pub const MaxDescriptionLength: u32 = 1000;
	pub const MaxCommitteeMembers: u32 = 5;
	pub const MaxParticipants: u32 = 10;
	pub const MinRankForProviderRegistry: u16 = 2; // Rank II
	pub const MinRankForReferral: u16 = 2; // Rank II
	pub const MinRankToActivateEmergencyProtocol: u16 = 3; // Rank III (Senior Ambassador)
	pub const MinRankToFormEmergencyCommittee: u16 = 3; // Rank III (Senior Ambassador)
	pub const MinRankToSubmitAppeal: u16 = 1; // Rank I (Junior Ambassador)
	pub const MinRankToFormAppealCommittee: u16 = 3; // Rank III (Senior Ambassador)
	pub const MinRankToEstablishIntegration: u16 = 2; // Rank II (Ambassador)
}

impl pallet_ambassador_governance::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type BlockNumber = u64;
	type EmergencyOrigin = MockEmergencyOrigin;
	type CommitteeFormationOrigin = MockCommitteeFormationOrigin;
	type EmergencyResolutionOrigin = MockEmergencyResolutionOrigin;
	type AppealSubmissionOrigin = MockAppealSubmissionOrigin;
	type AppealCommitteeOrigin = MockAppealCommitteeOrigin;
	type AppealDecisionOrigin = MockAppealDecisionOrigin;
	type IntegrationOrigin = MockIntegrationOrigin;
	type MaxJustificationLength = MaxJustificationLength;
	type MaxDescriptionLength = MaxDescriptionLength;
	type MaxCommitteeMembers = MaxCommitteeMembers;
	type MaxParticipants = MaxParticipants;
	type WeightInfo = ();
	type IdentityRegistrar = MockIdentityVerifier;
	type RankChecker = MockRankChecker;
	type MinRankForProviderRegistry = MinRankForProviderRegistry;
	type MinRankForReferral = MinRankForReferral;
	type MinRankToActivateEmergencyProtocol = MinRankToActivateEmergencyProtocol;
	type MinRankToFormEmergencyCommittee = MinRankToFormEmergencyCommittee;
	type MinRankToSubmitAppeal = MinRankToSubmitAppeal;
	type MinRankToFormAppealCommittee = MinRankToFormAppealCommittee;
	type MinRankToEstablishIntegration = MinRankToEstablishIntegration;
}

// Mock implementation of IdentityVerifier trait
pub struct MockIdentityVerifier;

impl crate::IdentityVerifier<AccountId> for MockIdentityVerifier {
	fn has_identity(who: &AccountId) -> bool {
		// For testing purposes:
		// - Accounts 1-10 have verified identities
		// - Account 11 and above do NOT have verified identities
		// This allows rank check tests to work properly (which use accounts 1-4)
		// while still enabling identity verification tests (which use account 11)
		*who >= 1 && *who <= 10
	}
}

// Mock implementation of RankChecker trait
pub struct MockRankChecker;

impl crate::RankChecker<AccountId> for MockRankChecker {
	fn has_minimum_rank(who: &AccountId, min_rank: u16) -> bool {
		// For testing purposes:
		// Account 1 has rank 3 (Senior Ambassador)
		// Account 2 has rank 2 (Ambassador)
		// Account 3 has rank 1 (Junior Ambassador)
		// All other accounts have rank 0 (not ranked)
		match *who {
			1 => min_rank <= 3,
			2 => min_rank <= 2,
			3 => min_rank <= 1,
			_ => min_rank == 0,
		}
	}
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<Runtime>::default().build_storage().unwrap();

	let balances = vec![(1, 100), (2, 100), (3, 100), (4, 100), (5, 100)];
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		frame_system::Pallet::<Runtime>::set_block_number(1);
	});
	ext
}

// Helper function to advance blocks
pub(crate) fn run_to_block(n: u64) {
	while frame_system::Pallet::<Runtime>::block_number() < n {
		frame_system::Pallet::<Runtime>::set_block_number(
			frame_system::Pallet::<Runtime>::block_number() + 1,
		);
		frame_system::Pallet::<Runtime>::on_initialize(
			frame_system::Pallet::<Runtime>::block_number(),
		);
		pallet_ambassador_governance::Pallet::<Runtime>::on_initialize(frame_system::Pallet::<
			Runtime,
		>::block_number());
	}
}
