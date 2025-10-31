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

//! Identity verification for the Ambassador Program.

use pallet_ambassador_governance::IdentityVerifier;
use pallet_ambassador_governance::RankChecker;
use pallet_identity::{self, Judgement};
use sp_std::marker::PhantomData;
use frame_support::traits::RankedMembers;

/// An identity verifier that uses the identity pallet to check if an account has a verified identity.
/// and checks if the account has a positive judgement from any registrar.
pub struct AmbassadorIdentityVerifier<T>(PhantomData<T>);

impl<T> Default for AmbassadorIdentityVerifier<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> IdentityVerifier<crate::AccountId> for AmbassadorIdentityVerifier<T> 
where
    T: pallet_identity::Config<AccountId = crate::AccountId>,
{
    /// Returns true if the account has a verified identity according to the identity pallet
    /// and checks if the account has received at least a "Reasonable" judgement from any registrar.
    fn has_identity(who: &crate::AccountId) -> bool {
        // Get the identity information for the account
        if let Some(registration) = pallet_identity::IdentityOf::<T>::get(who) {
            // Check if the account has at least a "Reasonable" judgement from any registrar
            for (_index, judgement) in registration.judgements.iter() {
                // Compare judgement with Reasonable using match instead of >= operator
                match *judgement {
                    Judgement::Reasonable | Judgement::KnownGood | Judgement::OutOfDate => return true,
                    _ => continue,
                }
            }
        }
        
        // No identity or no positive judgement from any registrar
        false
    }
}

/// Implementation of RankChecker for the Ambassador Collective
/// and checks if an account has at least the specified minimum rank in the ambassador collective.
pub struct AmbassadorRankChecker;

impl RankChecker<crate::AccountId> for AmbassadorRankChecker {
    /// Returns true if the account has at least the specified minimum rank.
    /// Checks the actual rank of an ambassador in the collective using the RankedMembers trait.
    fn has_minimum_rank(who: &crate::AccountId, min_rank: u16) -> bool {
        // Check if the account is a member of the ambassador collective with at least the specified rank
        // by using the RankedMembers trait to get the ambassador's rank
        if let Some(rank) = <crate::AmbassadorCollective as RankedMembers>::rank_of(who) {
            // Check if the ambassador's rank is at least the minimum required rank
            rank >= min_rank
        } else {
            // Account is not a member of the ambassador collective
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::{assert_ok, parameter_types, traits::ConstU32};
    use sp_core::H256;
    use sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup, Verify},
        BuildStorage, MultiSignature,
    };
    use frame_system as system;
    use pallet_identity::legacy::IdentityInfo;

    type Block = frame_system::mocking::MockBlock<Test>;
    type Signature = MultiSignature;

    // Configure a mock runtime to test the pallet.
    frame_support::construct_runtime!(
        pub enum Test
        {
            System: frame_system,
            Identity: pallet_identity,
            AmbassadorCollective: pallet_ranked_collective_ambassador,
        }
    );

    parameter_types! {
        pub const BlockHashCount: u64 = 250;
        pub const SS58Prefix: u8 = 42;
    }

    impl system::Config for Test {
        type BaseCallFilter = frame_support::traits::Everything;
        type BlockWeights = ();
        type BlockLength = ();
        type DbWeight = ();
        type RuntimeOrigin = RuntimeOrigin;
        type RuntimeCall = RuntimeCall;
        type Nonce = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Block = Block;
        type RuntimeEvent = RuntimeEvent;
        type BlockHashCount = BlockHashCount;
        type Version = ();
        type PalletInfo = PalletInfo;
        type AccountData = ();
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type SystemWeightInfo = ();
        type SS58Prefix = SS58Prefix;
        type OnSetCode = ();
        type MaxConsumers = frame_support::traits::ConstU32<16>;
    }

    parameter_types! {
        pub const BasicDeposit: u64 = 10;
        pub const ByteDeposit: u64 = 10;
        pub const SubAccountDeposit: u64 = 10;
        pub const MaxSubAccounts: u32 = 2;
        pub const MaxRegistrars: u32 = 20;
        pub const PendingUsernameExpiration: u32 = 100;
        pub const MaxSuffixLength: u32 = 7;
        pub const MaxUsernameLength: u32 = 32;
        pub const UsernameGracePeriod: u32 = 10;
    }

    impl pallet_identity::Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type Currency = ();
        type BasicDeposit = BasicDeposit;
        type ByteDeposit = ByteDeposit;
        type SubAccountDeposit = SubAccountDeposit;
        type MaxSubAccounts = MaxSubAccounts;
        type MaxRegistrars = MaxRegistrars;
        type Slashed = ();
        type ForceOrigin = frame_system::EnsureRoot<u64>;
        type RegistrarOrigin = frame_system::EnsureRoot<u64>;
        type WeightInfo = ();
        type IdentityInformation = IdentityInfo<()>;
        type OffchainSignature = Signature;
        type SigningPublicKey = <Signature as Verify>::Signer;
        type UsernameAuthorityOrigin = frame_system::EnsureRoot<u64>;
        type PendingUsernameExpiration = PendingUsernameExpiration;
        type MaxSuffixLength = MaxSuffixLength;
        type MaxUsernameLength = MaxUsernameLength;
        type UsernameDeposit = BasicDeposit;
        type UsernameGracePeriod = UsernameGracePeriod;
    }

    parameter_types! {
        pub const MaxMembers: u32 = 100;
    }

    impl pallet_ranked_collective_ambassador::Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type WeightInfo = ();
        type MaxMembers = MaxMembers;
    }

    // Build genesis storage according to the mock runtime.
    pub fn new_test_ext() -> sp_io::TestExternalities {
        system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
    }

    #[test]
    fn identity_verifier_works() {
        new_test_ext().execute_with(|| {
            // Setup
            let account_id = 1;
            let registrar = 2;
            
            // Register identity registrar
            assert_ok!(Identity::add_registrar(RuntimeOrigin::root(), registrar));
            
            // Register identity for account_id
            let info = IdentityInfo {
                additional: vec![],
                display: Default::default(),
                legal: Default::default(),
                web: Default::default(),
                riot: Default::default(),
                email: Default::default(),
                pgp_fingerprint: None,
                image: Default::default(),
                twitter: Default::default(),
            };
            
            assert_ok!(Identity::set_identity(RuntimeOrigin::signed(account_id), Box::new(info)));
            
            // Account has no judgement yet, should return false
            assert!(!AmbassadorIdentityVerifier::<Test>::has_identity(&account_id));
            
            // Provide a "Reasonable" judgement
            assert_ok!(Identity::provide_judgement(
                RuntimeOrigin::signed(registrar),
                0,
                account_id,
                Judgement::Reasonable,
                H256::default()
            ));
            
            // Now the account should have a verified identity
            assert!(AmbassadorIdentityVerifier::<Test>::has_identity(&account_id));
        });
    }

    #[test]
    fn rank_checker_works() {
        new_test_ext().execute_with(|| {
            // Setup
            let account_id = 1;
            
            // Account is not a member yet, should return false
            assert!(!AmbassadorRankChecker::has_minimum_rank(&account_id, 0));
            
            // Add account as a member with rank 0
            assert_ok!(AmbassadorCollective::add_member(RuntimeOrigin::root(), account_id));
            
            // Now account should have rank 0
            assert!(AmbassadorRankChecker::has_minimum_rank(&account_id, 0));
            assert!(!AmbassadorRankChecker::has_minimum_rank(&account_id, 1));
            
            // Promote account to rank 1
            assert_ok!(AmbassadorCollective::promote_member(RuntimeOrigin::root(), account_id));
            
            // Now account should have rank 1
            assert!(AmbassadorRankChecker::has_minimum_rank(&account_id, 0));
            assert!(AmbassadorRankChecker::has_minimum_rank(&account_id, 1));
            assert!(!AmbassadorRankChecker::has_minimum_rank(&account_id, 2));
        });
    }
}
