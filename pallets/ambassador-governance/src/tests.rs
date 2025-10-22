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

use crate::{
	mock::*, AppealCommittees, AppealDecision, AppealStatus, ConflictType, Conflicts, DisciplineLevel,
	Disciplines, EmergencyRole, EmergencySeverity, EmergencyType, Error, Event, IntegrationMechanism,
	Pallet as AmbassadorGovernance, ProfessionalServiceType, RemarkCategory, TargetCollective,
	TransitionType,
};
use frame_support::{assert_noop, assert_ok, BoundedVec};
use sp_core::{H256, blake2_256};
use sp_std::vec;
use codec::{Decode, Encode};

// Helper function to create a random H256 hash
fn random_hash() -> H256 {
	H256::random()
}

fn create_description(len: usize) -> BoundedVec<u8, crate::mock::MaxDescriptionLength> {
	let max_len = crate::mock::MaxDescriptionLength::get() as usize;
	let actual_len = core::cmp::min(len, max_len);
	BoundedVec::try_from(vec![0u8; actual_len]).expect("Should not fail with length <= max")
}

// Helper function to create a justification string
fn create_justification(len: usize) -> BoundedVec<u8, crate::mock::MaxJustificationLength> {
	let max_len = crate::mock::MaxJustificationLength::get() as usize;
	let actual_len = core::cmp::min(len, max_len);
	BoundedVec::try_from(vec![0u8; actual_len]).expect("Should not fail with length <= max")
}

fn create_resolution(len: usize) -> BoundedVec<u8, crate::mock::MaxResolutionLength> {
	let max_len = crate::mock::MaxResolutionLength::get() as usize;
	let actual_len = core::cmp::min(len, max_len);
	BoundedVec::try_from(vec![0u8; actual_len]).expect("Should not fail with length <= max")
}

/// Helper function to extract the emergency ID from the most recent EmergencyActivated event
fn get_last_emergency_id() -> H256 {
	let events = frame_system::Pallet::<Runtime>::events();
	for event in events.iter() {
		if let RuntimeEvent::AmbassadorGovernance(Event::EmergencyActivated {
			emergency_id, ..
		}) = &event.event
		{
			return emergency_id.clone();
		}
	}
	panic!("EmergencyActivated event should be emitted");
}

/// Helper function to extract the appeal ID from the most recent AppealSubmitted event
fn get_last_appeal_id() -> H256 {
	let events = frame_system::Pallet::<Runtime>::events();
	for event in events.iter() {
		if let RuntimeEvent::AmbassadorGovernance(Event::AppealSubmitted { appeal_id, .. }) =
			&event.event
		{
			return appeal_id.clone();
		}
	}
	panic!("AppealSubmitted event should be emitted");
}

fn create_evidence_info(len: usize) -> BoundedVec<u8, crate::mock::MaxEvidenceInfoLength> {
	let max_len = crate::mock::MaxEvidenceInfoLength::get() as usize;
	let actual_len = core::cmp::min(len, max_len);
	BoundedVec::try_from(vec![0u8; actual_len]).expect("Should not fail with length <= max")
}

// Helper function to create remark content
fn create_remark_content(len: usize) -> BoundedVec<u8, crate::mock::MaxRemarkContentLength> {
	let max_len = crate::mock::MaxRemarkContentLength::get() as usize;
	let actual_len = core::cmp::min(len, max_len);
	let content = format!("Test remark content (evidence: ipfs://Qm123456) with length {}", actual_len);
	let bytes = content.into_bytes();
	let padded_bytes = if bytes.len() < actual_len {
		let mut padded = bytes.clone();
		padded.extend(vec![0u8; actual_len - bytes.len()]);
		padded
	} else {
		bytes[..actual_len].to_vec()
	};

	BoundedVec::try_from(padded_bytes).expect("Should not fail with length <= max")
}

// Helper function to create an account for testing
fn account(name: &'static str, index: u8, seed: u8) -> u64 {
	let entropy = (name, index, seed).using_encoded(blake2_256);
	u64::decode(&mut &entropy[..]).unwrap_or_default()
}

// Helper function to set up identity for an account
fn setup_identity_for_account(who: u64) {
	// Mock implementation - in the real tests this would interact with the Identity pallet
	// For our mock, we'll just assume all accounts with ID >= 1 have verified identity
	assert!(who >= 1, "Account ID must be >= 1 to have verified identity");
}

// Helper function to assign ambassador rank.
// Refer also to account IDs and associated ranks in MockRankChecker
fn assign_ambassador_rank(who: u64, rank: u16) {
	// Mock implementation - in the real tests this would interact with the RankedCollective pallet
	// For our mock, we'll just assume all accounts with ID >= 1 can be assigned ranks
	assert!(who >= 1, "Account ID must be >= 1 to be assigned a rank");
	assert!(rank <= 5, "Rank must be <= 5");
}

#[test]
fn activate_emergency_protocol_works() {
	new_test_ext().execute_with(|| {
		// Setup
		let emergency_type = EmergencyType::SecurityVulnerability;
		let severity = EmergencySeverity::Critical;
		let justification = create_justification(100);

		// Execute using account 3 which has Senior Ambassador rank (3)
		// MinRankToActivateEmergencyProtocol is set to 3 (Senior Ambassador) in mock.rs
		assert_ok!(AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
			signed_origin(3),
			emergency_type.clone(),
			severity.clone(),
			justification.clone(),
			None
		));

		// Get the emergency ID from the emitted event
		let emergency_id = get_last_emergency_id();

		let emergency = AmbassadorGovernance::<Runtime>::emergencies(emergency_id).unwrap();
		assert_eq!(emergency.emergency_type, emergency_type);
		assert_eq!(emergency.severity, severity);
		assert_eq!(emergency.initiator, 3); // Senior Ambassador (rank 3)
		assert_eq!(emergency.resolved_at, None); // Check resolved_at is None instead of resolved field

		// Check event was emitted
		frame_system::Pallet::<Runtime>::assert_has_event(RuntimeEvent::AmbassadorGovernance(
			Event::EmergencyActivated { emergency_id, emergency_type, severity, initiator: 3 },
		));
	});
}

#[test]
fn activate_emergency_protocol_fails_with_invalid_origin() {
	new_test_ext().execute_with(|| {
		// Setup
		let emergency_type = EmergencyType::SecurityVulnerability;
		let severity = EmergencySeverity::Critical;
		let justification = create_justification(100);

		// Execute and verify failure with unsigned origin
		assert_noop!(
			AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
				frame_system::RawOrigin::None.into(),
				emergency_type.clone(),
				severity.clone(),
				justification.clone(),
				None
			),
			sp_runtime::traits::BadOrigin
		);
	});
}

#[test]
fn activate_emergency_protocol_success_with_longest_possible_justification() {
	new_test_ext().execute_with(|| {
		// Setup
		let emergency_type = EmergencyType::SecurityVulnerability;
		let severity = EmergencySeverity::Critical;
    // Create a vector of max length
    let justification_max_length_vec = BoundedVec::try_from(vec![0u8; MaxJustificationLength::get() as usize]).unwrap();

		// Execute and verify success with longest possible justification
		assert_ok!(
			AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
				RuntimeOrigin::signed(3),
				emergency_type.clone(),
				severity.clone(),
				justification_max_length_vec,
				None
			)
		);
	});
}

#[test]
fn form_emergency_committee_works() {
	new_test_ext().execute_with(|| {
		// Setup by first activating an emergency
		let emergency_type = EmergencyType::SecurityVulnerability;
		let severity = EmergencySeverity::Critical;
		let justification = create_justification(100);

		assert_ok!(AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
			signed_origin(3), // Senior Ambassador (rank 3)
			emergency_type.clone(),
			severity.clone(),
			justification.clone(),
			None
		));

		let emergency_id = get_last_emergency_id();

		// Create committee members with roles
		let mut members_vec = Vec::new();
		members_vec.push((5, EmergencyRole::TechnicalLead));
		members_vec.push((4, EmergencyRole::GovernanceRepresentative));
		members_vec.push((3, EmergencyRole::IndependentExpert));
		let members = BoundedVec::try_from(members_vec).unwrap();

		// Execute
		assert_ok!(AmbassadorGovernance::<Runtime>::form_emergency_committee(
			signed_origin(3), // Senior Ambassador (rank 3)
			emergency_id,
			members.clone()
		));

		// Verify
		let committee =
			AmbassadorGovernance::<Runtime>::emergency_committees(emergency_id).unwrap();
		assert_eq!(committee.len(), 3);
		assert!(committee.contains(&(5, EmergencyRole::TechnicalLead)));
		assert!(committee.contains(&(4, EmergencyRole::GovernanceRepresentative)));
		assert!(committee.contains(&(3, EmergencyRole::IndependentExpert)));

		// Check event was emitted
		frame_system::Pallet::<Runtime>::assert_has_event(RuntimeEvent::AmbassadorGovernance(
			Event::EmergencyCommitteeFormed { emergency_id, members },
		));
	});
}

#[test]
fn form_emergency_committee_fails_with_insufficient_rank() {
	new_test_ext().execute_with(|| {
		// Setup by first activating an emergency
		let emergency_type = EmergencyType::SecurityVulnerability;
		let severity = EmergencySeverity::Critical;
		let justification = create_justification(100);

		assert_ok!(AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
			RuntimeOrigin::signed(3), // Senior Ambassador (rank 3)
			emergency_type.clone(),
			severity.clone(),
			justification.clone(),
			None
		));

		let emergency_id = get_last_emergency_id();

		// Create committee members with roles
		let mut members_vec = Vec::new();
		members_vec.push((5, EmergencyRole::TechnicalLead));
		members_vec.push((4, EmergencyRole::GovernanceRepresentative));
		members_vec.push((3, EmergencyRole::IndependentExpert));
		let members = BoundedVec::try_from(members_vec).unwrap();

		// Try to form committee with account 2 (insufficient rank)
		assert_noop!(
			AmbassadorGovernance::<Runtime>::form_emergency_committee(
				RuntimeOrigin::signed(2),
				emergency_id,
				members.clone()
			),
			Error::<Runtime>::InsufficientRank
		);
	});
}

#[test]
fn form_emergency_committee_fails_with_nonexistent_emergency() {
	new_test_ext().execute_with(|| {
		// Setup
		let emergency_id = H256::from_low_u64_be(999); // Non-existent emergency

		// Create committee members with roles
		let mut members_vec = Vec::new();
		members_vec.push((5, EmergencyRole::TechnicalLead));
		members_vec.push((4, EmergencyRole::GovernanceRepresentative));
		members_vec.push((3, EmergencyRole::IndependentExpert));
		let members = BoundedVec::try_from(members_vec).unwrap();

		// Execute and verify failure
		assert_noop!(
			AmbassadorGovernance::<Runtime>::form_emergency_committee(
				RuntimeOrigin::signed(3),
				emergency_id,
				members
			),
			Error::<Runtime>::EmergencyNotFound
		);
	});
}

#[test]
fn form_emergency_committee_fails_with_already_formed_committee() {
	new_test_ext().execute_with(|| {
		// Setup by first activating an emergency
		let emergency_type = EmergencyType::SecurityVulnerability;
		let severity = EmergencySeverity::Critical;
		let justification = create_justification(100);

		assert_ok!(AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
			RuntimeOrigin::signed(3),
			emergency_type.clone(),
			severity.clone(),
			justification.clone(),
			None
		));

		let emergency_id = get_last_emergency_id();

		// Create committee members with roles
		let mut members_vec = Vec::new();
		members_vec.push((5, EmergencyRole::TechnicalLead));
		members_vec.push((4, EmergencyRole::GovernanceRepresentative));
		members_vec.push((3, EmergencyRole::IndependentExpert));
		let members = BoundedVec::try_from(members_vec).unwrap();

		// Form committee first time
		assert_ok!(AmbassadorGovernance::<Runtime>::form_emergency_committee(
			RuntimeOrigin::signed(3),
			emergency_id,
			members.clone()
		));

		// Try to form committee again
		assert_noop!(
			AmbassadorGovernance::<Runtime>::form_emergency_committee(
				RuntimeOrigin::signed(3),
				emergency_id,
				members
			),
			Error::<Runtime>::CommitteeAlreadyFormed
		);
	});
}

#[test]
fn form_emergency_committee_fails_with_too_many_members() {
	new_test_ext().execute_with(|| {
		// Setup by first activating an emergency
		let emergency_type = EmergencyType::SecurityVulnerability;
		let severity = EmergencySeverity::Critical;
		let justification = create_justification(100);

		assert_ok!(AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
			RuntimeOrigin::signed(3),
			emergency_type.clone(),
			severity.clone(),
			justification.clone(),
			None
		));

		let emergency_id = get_last_emergency_id();

		// Create too many committee members (MaxEmergencyCommitteeMembers is 5)
		let mut members_vec = Vec::new();
		members_vec.push((5, EmergencyRole::TechnicalLead));
		members_vec.push((4, EmergencyRole::GovernanceRepresentative));
		members_vec.push((3, EmergencyRole::IndependentExpert));
		members_vec.push((2, EmergencyRole::IndependentExpert));
		members_vec.push((1, EmergencyRole::IndependentExpert));
		members_vec.push((6, EmergencyRole::IndependentExpert)); // Extra member exceeding limit

		// Try to convert to BoundedVec should fail
		assert!(BoundedVec::<_, MaxEmergencyCommitteeMembers>::try_from(members_vec.clone()).is_err());

		// Remove the extra member to make it valid
		members_vec.pop();
		let members = BoundedVec::try_from(members_vec).unwrap();

		// Formation should now succeed with exactly MaxEmergencyCommitteeMembers
		assert_ok!(
			AmbassadorGovernance::<Runtime>::form_emergency_committee(
				RuntimeOrigin::signed(3),
				emergency_id,
				members
			)
		);
	});
}

#[test]
fn form_emergency_committee_fails_with_missing_required_roles() {
	new_test_ext().execute_with(|| {
		// Setup by first activating an emergency
		let emergency_type = EmergencyType::SecurityVulnerability;
		let severity = EmergencySeverity::Critical;
		let justification = create_justification(100);

		assert_ok!(AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
			RuntimeOrigin::signed(3),
			emergency_type.clone(),
			severity.clone(),
			justification.clone(),
			None
		));

		let emergency_id = get_last_emergency_id();

		// Create committee members with missing required roles (missing IndependentExpert)
		let mut members_vec = Vec::new();
		members_vec.push((5, EmergencyRole::TechnicalLead));
		members_vec.push((4, EmergencyRole::GovernanceRepresentative));
		let members = BoundedVec::try_from(members_vec).unwrap();

		// Execute and verify failure
		assert_noop!(
			AmbassadorGovernance::<Runtime>::form_emergency_committee(
				RuntimeOrigin::signed(3),
				emergency_id,
				members
			),
			Error::<Runtime>::MissingRequiredRole
		);
	});
}

#[test]
fn resolve_emergency_works() {
	new_test_ext().execute_with(|| {
		// Setup by first activating an emergency and forming committee
		let emergency_type = EmergencyType::SecurityVulnerability;
		let severity = EmergencySeverity::Critical;
		let justification = create_justification(100);

		assert_ok!(AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
			RuntimeOrigin::signed(3),
			emergency_type.clone(),
			severity.clone(),
			justification.clone(),
			None
		));

		let emergency_id = get_last_emergency_id();

		// Create committee members with roles
			let mut members_vec = Vec::new();
			members_vec.push((5, EmergencyRole::TechnicalLead));
			members_vec.push((4, EmergencyRole::GovernanceRepresentative));
			members_vec.push((3, EmergencyRole::IndependentExpert));
			let members = BoundedVec::try_from(members_vec).unwrap();

		assert_ok!(AmbassadorGovernance::<Runtime>::form_emergency_committee(
			signed_origin(3),
			emergency_id,
			members.clone()
		));

		// Resolve emergency
		let resolution_summary = create_resolution(100);
		let abuse_detected = false;

		// Execute as committee member
		assert_ok!(AmbassadorGovernance::<Runtime>::resolve_emergency(
			signed_origin(4), // Committee member
			emergency_id,
			abuse_detected,
			resolution_summary.clone(),
			None
		));

		// Verify
		let emergency = AmbassadorGovernance::<Runtime>::emergencies(emergency_id).unwrap();
		assert!(emergency.resolved_at.is_some());
		assert_eq!(emergency.abuse_detected, abuse_detected);

		// Check event was emitted
		frame_system::Pallet::<Runtime>::assert_has_event(RuntimeEvent::AmbassadorGovernance(
			Event::EmergencyResolved {
				emergency_id,
				abuse_detected,
				resolution_summary,
			},
		));
	});
}

#[test]
fn activate_emergency_protocol_fails_with_insufficient_rank() {
    new_test_ext().execute_with(|| {
        // Setup
        let emergency_type = EmergencyType::SecurityVulnerability;
        let severity = EmergencySeverity::Critical;
        let justification = create_justification(100);

        // Try with account 2 (Lead Ambassador rank 2)
        // MinRankToActivateEmergencyProtocol is set to 3 (Senior Ambassador)
        assert_noop!(
            AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
                signed_origin(2),
                emergency_type.clone(),
                severity.clone(),
                justification.clone(),
                None
            ),
            Error::<Runtime>::InsufficientRank
        );
    });
}

#[test]
fn activate_emergency_protocol_fails_with_rank_2_when_rank_3_required() {
	new_test_ext().execute_with(|| {
		// Try to activate emergency protocol as account 2 (has Rank 2, regular Ambassador)
		// In the mock runtime, MinRankToActivateEmergencyProtocol is set to 3 (Senior Ambassador)
		assert_noop!(
			AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
				RuntimeOrigin::signed(2),
				EmergencyType::SecurityVulnerability,
				EmergencySeverity::Critical,
				BoundedVec::try_from(b"Critical security vulnerability found in XYZ component - evidence stored at ipfs://QmHash456".to_vec()).unwrap(),
				None
			),
			Error::<Runtime>::InsufficientRank
		);
	});
}

#[test]
fn resolve_emergency_works_with_principal_ambassador() {
	new_test_ext().execute_with(|| {
		// Setup by first activating an emergency
		let emergency_type = EmergencyType::SecurityVulnerability;
		let severity = EmergencySeverity::Critical;
		let justification = create_justification(100);

		assert_ok!(AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
			RuntimeOrigin::signed(4), // Using account 4 (Principal Ambassador) instead of account 1
			emergency_type.clone(),
			severity.clone(),
			justification.clone(),
			None
		));

		let emergency_id = get_last_emergency_id();

		// Create committee members with roles including the Principal Ambassador
		let members = BoundedVec::try_from(vec![
			(5, EmergencyRole::TechnicalLead), // Using account 4 (Principal Ambassador)
			(4, EmergencyRole::GovernanceRepresentative),
			(3, EmergencyRole::IndependentExpert),
		]).unwrap();

		assert_ok!(AmbassadorGovernance::<Runtime>::form_emergency_committee(
			RuntimeOrigin::signed(3), // Using account 3 (Senior Ambassador)
			emergency_id,
			members.clone()
		));

		// Resolve emergency without forming committee (as Principal Ambassador)
		let resolution_summary = create_resolution(100);
		let abuse_detected = false;

		// Execute as Principal Ambassador
		assert_ok!(AmbassadorGovernance::<Runtime>::resolve_emergency(
			RuntimeOrigin::signed(4), // Using account 4 (Principal Ambassador)
			emergency_id,
			abuse_detected,
			resolution_summary.clone(),
			None
		));

		// Verify
		let emergency = AmbassadorGovernance::<Runtime>::emergencies(emergency_id).unwrap();
		assert!(emergency.resolved_at.is_some());
		assert_eq!(emergency.abuse_detected, abuse_detected);
	});
}

#[test]
fn resolve_emergency_fails_with_nonexistent_emergency() {
	new_test_ext().execute_with(|| {
		// Setup by first activating an emergency
		let emergency_id_non_existent = H256::from_low_u64_be(999); // Non-existent emergency
		let resolution_summary = create_resolution(100);
		let abuse_detected = false;
		let emergency_type = EmergencyType::SecurityVulnerability;
		let severity = EmergencySeverity::Critical;
		let justification = create_justification(100);

		assert_ok!(AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
			RuntimeOrigin::signed(3),
			emergency_type.clone(),
			severity.clone(),
			justification.clone(),
			None
		));

		let emergency_id = get_last_emergency_id();

		// Create committee members with roles
		let members = BoundedVec::try_from(vec![
			(5, EmergencyRole::TechnicalLead),
			(4, EmergencyRole::GovernanceRepresentative),
			(3, EmergencyRole::IndependentExpert),
		]).unwrap();

		assert_ok!(AmbassadorGovernance::<Runtime>::form_emergency_committee(
			RuntimeOrigin::signed(3),
			emergency_id,
			members.clone()
		));

		// Execute and verify failure
		assert_noop!(
			AmbassadorGovernance::<Runtime>::resolve_emergency(
				RuntimeOrigin::signed(4),
				emergency_id_non_existent,
				abuse_detected,
				resolution_summary,
				None
			),
			Error::<Runtime>::EmergencyNotFound
		);
	});
}

#[test]
fn resolve_emergency_fails_with_already_resolved_emergency() {
	new_test_ext().execute_with(|| {
		// Setup by first activating an emergency
		let emergency_type = EmergencyType::SecurityVulnerability;
		let severity = EmergencySeverity::Critical;
		let justification = create_justification(100);

		assert_ok!(AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
			RuntimeOrigin::signed(3),
			emergency_type.clone(),
			severity.clone(),
			justification.clone(),
			None
		));

		let emergency_id = get_last_emergency_id();

		// Create committee members with roles including the Senior Ambassador
		let members = BoundedVec::try_from(vec![
			(5, EmergencyRole::TechnicalLead),
			(4, EmergencyRole::GovernanceRepresentative),
			(3, EmergencyRole::IndependentExpert),
		]).unwrap();

		assert_ok!(AmbassadorGovernance::<Runtime>::form_emergency_committee(
			RuntimeOrigin::signed(3),
			emergency_id,
			members.clone()
		));

		// Resolve emergency first time
		let resolution_summary = create_resolution(100);
		let abuse_detected = false;

		assert_ok!(AmbassadorGovernance::<Runtime>::resolve_emergency(
			RuntimeOrigin::signed(4),
			emergency_id,
			abuse_detected,
			resolution_summary.clone(),
			None
		));

		// Try to resolve again
		assert_noop!(
			AmbassadorGovernance::<Runtime>::resolve_emergency(
				RuntimeOrigin::signed(4),
				emergency_id,
				abuse_detected,
				resolution_summary,
				None
			),
			Error::<Runtime>::EmergencyAlreadyResolved
		);
	});
}

#[test]
fn resolve_emergency_fails_with_unauthorized_account() {
	new_test_ext().execute_with(|| {
		// Setup by first activating an emergency and forming committee
		let emergency_type = EmergencyType::SecurityVulnerability;
		let severity = EmergencySeverity::Critical;
		let justification = create_justification(100);

		assert_ok!(AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
			RuntimeOrigin::signed(3),
			emergency_type.clone(),
			severity.clone(),
			justification.clone(),
			None
		));

		let emergency_id = get_last_emergency_id();

		// Create committee members with roles
		let members = BoundedVec::try_from(vec![
			(5, EmergencyRole::TechnicalLead),
			(4, EmergencyRole::GovernanceRepresentative),
			(3, EmergencyRole::IndependentExpert),
		]).unwrap();

		assert_ok!(AmbassadorGovernance::<Runtime>::form_emergency_committee(
			RuntimeOrigin::signed(3),
			emergency_id,
			members.clone()
		));

		// Try to resolve with unauthorized account (not committee member or Senior Ambassador)
		let resolution_summary = create_resolution(100);
		let abuse_detected = false;

		assert_noop!(
			AmbassadorGovernance::<Runtime>::resolve_emergency(
				RuntimeOrigin::signed(6), // Not a committee member
				emergency_id,
				abuse_detected,
				resolution_summary,
				None
			),
			Error::<Runtime>::NotCommitteeMember
		);
	});
}

#[test]
fn resolve_emergency_fails_when_already_resolved() {
	new_test_ext().execute_with(|| {
		// Create an emergency
		let emergency_type = EmergencyType::TechnicalFailure;
		let severity = EmergencySeverity::Critical;
		let justification = create_justification(100);

		assert_ok!(AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
			signed_origin(3),
			emergency_type,
			severity,
			justification.clone(),
			None
		));

		let emergency_id = get_last_emergency_id();

		// Create committee members with roles
		let members = BoundedVec::try_from(vec![
			(5, EmergencyRole::TechnicalLead),
			(4, EmergencyRole::GovernanceRepresentative),
			(3, EmergencyRole::IndependentExpert),
		]).unwrap();

		assert_ok!(AmbassadorGovernance::<Runtime>::form_emergency_committee(
			signed_origin(3),
			emergency_id,
			members.clone()
		));

		// First resolution should succeed
		let resolution_summary = create_resolution(100);
		let abuse_detected = false;

		assert_ok!(AmbassadorGovernance::<Runtime>::resolve_emergency(
			signed_origin(4), // Committee member
			emergency_id,
			abuse_detected,
			resolution_summary.clone(),
			None
		));

		// Second resolution should fail
		assert_noop!(
			AmbassadorGovernance::<Runtime>::resolve_emergency(
				signed_origin(4),
				emergency_id,
				abuse_detected,
				resolution_summary,
				None
			),
			Error::<Runtime>::EmergencyAlreadyResolved
		);
	});
}

#[test]
fn submit_appeal_works() {
	new_test_ext().execute_with(|| {
		// Setup
		let original_decision = BoundedVec::try_from(b"Original decision text".to_vec()).unwrap();
		let justification = create_justification(100);

		// Execute
		assert_ok!(AmbassadorGovernance::<Runtime>::submit_appeal(
			RuntimeOrigin::signed(1),
			original_decision.clone(),
			justification.clone(),
			None
		));

		// Verify
		let appeal_id = get_last_appeal_id();
		let appeal = AmbassadorGovernance::<Runtime>::appeals(appeal_id).unwrap();
		assert_eq!(appeal.appellant, 1);
		assert_eq!(appeal.status, AppealStatus::Submitted);
		assert_eq!(appeal.decision, None);

		// Check event was emitted
		frame_system::Pallet::<Runtime>::assert_has_event(RuntimeEvent::AmbassadorGovernance(
			Event::AppealSubmitted {
				appeal_id,
				appellant: 1,
				original_decision: original_decision.clone(),
			},
		));
	});
}

#[test]
fn submit_appeal_fails_with_invalid_origin() {
	new_test_ext().execute_with(|| {
		// Setup
		let original_decision = BoundedVec::try_from(b"Original decision text".to_vec()).unwrap();
		let justification = create_justification(100);

		// Execute and verify failure with unsigned origin
		assert_noop!(
			AmbassadorGovernance::<Runtime>::submit_appeal(
				frame_system::RawOrigin::None.into(),
				original_decision.clone(),
				justification.clone(),
				None
			),
			sp_runtime::traits::BadOrigin
		);
	});
}

#[test]
fn submit_appeal_fails_with_insufficient_rank() {
	new_test_ext().execute_with(|| {
		// Setup
		let original_decision = BoundedVec::try_from(b"Original decision text".to_vec()).unwrap();
		let justification = create_justification(100);

		// Try to submit appeal as account 0 (has Rank 0, non-ambassador)
		// In the mock runtime, the AppealSubmissionOrigin is restricted to specific accounts
		assert_noop!(
			AmbassadorGovernance::<Runtime>::submit_appeal(
				RuntimeOrigin::signed(0),
				original_decision.clone(),
				justification.clone(),
				None
			),
			Error::<Runtime>::InsufficientRank
		);
	});
}

#[test]
fn submit_appeal_success_with_longest_possible_justification() {
	new_test_ext().execute_with(|| {
		// Setup
		let original_decision = BoundedVec::try_from(b"Original decision text".to_vec()).unwrap();
		let justification = create_justification(MaxJustificationLength::get() as usize);

		// Execute and verify success with longest possible justification
		assert_ok!(
			AmbassadorGovernance::<Runtime>::submit_appeal(
				RuntimeOrigin::signed(1),
				original_decision.clone(),
				justification,
				None
			)
		);
	});
}

#[test]
fn form_appeal_committee_works() {
	new_test_ext().execute_with(|| {
		// Setup by first submitting an appeal
		let original_decision = BoundedVec::try_from(b"Original decision text".to_vec()).unwrap();
		let justification = create_justification(100);

		assert_ok!(AmbassadorGovernance::<Runtime>::submit_appeal(
			RuntimeOrigin::signed(1),
			original_decision.clone(),
			justification.clone(),
			None
		));

		let appeal_id = get_last_appeal_id();

		// Create committee members
			let mut members_vec = Vec::new();
			members_vec.push(5);
			members_vec.push(4);
			members_vec.push(3);
			let members = BoundedVec::try_from(members_vec).unwrap();

		// Execute
		assert_ok!(AmbassadorGovernance::<Runtime>::form_appeal_committee(
			RuntimeOrigin::signed(3),
			appeal_id,
			members.clone()
		));

		// Verify
		let committee = AmbassadorGovernance::<Runtime>::appeal_committees(appeal_id).unwrap();
		assert_eq!(committee.len(), 3);
		assert!(committee.contains(&5));
		assert!(committee.contains(&4));
		assert!(committee.contains(&3));

		// Check appeal status updated
		let appeal = AmbassadorGovernance::<Runtime>::appeals(appeal_id).unwrap();
		assert_eq!(appeal.status, AppealStatus::UnderReview);

		// Check event was emitted
		frame_system::Pallet::<Runtime>::assert_has_event(RuntimeEvent::AmbassadorGovernance(
			Event::AppealCommitteeFormed { appeal_id, members },
		));
	});
}

#[test]
fn form_appeal_committee_fails_with_insufficient_rank() {
	new_test_ext().execute_with(|| {
		// Setup by first submitting an appeal
		let original_decision = BoundedVec::try_from(b"Original decision text".to_vec()).unwrap();
		let justification = create_justification(100);

		assert_ok!(AmbassadorGovernance::<Runtime>::submit_appeal(
			RuntimeOrigin::signed(1),
			original_decision.clone(),
			justification.clone(),
			None
		));

		let appeal_id = get_last_appeal_id();

		// Create committee members
		let members = BoundedVec::try_from(vec![5, 4, 3]).unwrap();

		// Try to form appeal committee as account 2 (less than MinRankToFormAppealCommittee)
		assert_noop!(
			AmbassadorGovernance::<Runtime>::form_appeal_committee(
				RuntimeOrigin::signed(2),
				appeal_id,
				members.clone()
			),
			Error::<Runtime>::InsufficientRank
		);
	});
}

#[test]
fn form_appeal_committee_fails_with_nonexistent_appeal() {
	new_test_ext().execute_with(|| {
		// Setup
		let appeal_id = H256::from_low_u64_be(999); // Non-existent appeal

		// Create committee members
		let members = BoundedVec::try_from(vec![5, 4, 3]).unwrap();

		// Execute and verify failure
		assert_noop!(
			AmbassadorGovernance::<Runtime>::form_appeal_committee(
				RuntimeOrigin::signed(3),
				appeal_id,
				members
			),
			Error::<Runtime>::AppealNotFound
		);
	});
}

#[test]
fn form_appeal_committee_fails_with_already_formed_committee() {
	new_test_ext().execute_with(|| {
		// Setup by first submitting an appeal
		let original_decision = BoundedVec::try_from(b"Original decision text".to_vec()).unwrap();
		let justification = create_justification(100);

		assert_ok!(AmbassadorGovernance::<Runtime>::submit_appeal(
			RuntimeOrigin::signed(1),
			original_decision.clone(),
			justification.clone(),
			None
		));

		let appeal_id = get_last_appeal_id();

		// Create committee members
		let members = BoundedVec::try_from(vec![5, 4, 3]).unwrap();

		// Form committee first time
		assert_ok!(AmbassadorGovernance::<Runtime>::form_appeal_committee(
			RuntimeOrigin::signed(3),
			appeal_id,
			members.clone()
		));

		// Try to form committee again
		assert_noop!(
			AmbassadorGovernance::<Runtime>::form_appeal_committee(
				RuntimeOrigin::signed(3),
				appeal_id,
				members
			),
			Error::<Runtime>::AppealCommitteeAlreadyFormed
		);
	});
}

#[test]
fn form_appeal_committee_fails_with_too_many_members() {
	new_test_ext().execute_with(|| {
		// Setup by first submitting an appeal
		let original_decision = BoundedVec::try_from(b"Original decision text".to_vec()).unwrap();
		let justification = create_justification(100);

		assert_ok!(AmbassadorGovernance::<Runtime>::submit_appeal(
			RuntimeOrigin::signed(1),
			original_decision.clone(),
			justification.clone(),
			None
		));

		let appeal_id = get_last_appeal_id();

		// Create too many committee members (MaxAppealCommitteeMembers is 5)
		let mut members_vec = Vec::new();
		members_vec.push(5);
		members_vec.push(4);
		members_vec.push(3);
		members_vec.push(2);
		members_vec.push(1);
		members_vec.push(6); // Extra member exceeding limit

		// Try to convert to BoundedVec - this should fail
		assert!(BoundedVec::<_, MaxAppealCommitteeMembers>::try_from(members_vec.clone()).is_err());

		// Remove the extra member to make it valid
		members_vec.pop();
		let members = BoundedVec::try_from(members_vec).unwrap();

		// Now the formation should succeed with exactly MaxAppealCommitteeMembers
		assert_ok!(
			AmbassadorGovernance::<Runtime>::form_appeal_committee(
				RuntimeOrigin::signed(3),
				appeal_id,
				members
			)
		);
	});
}

#[test]
fn decide_appeal_works() {
	new_test_ext().execute_with(|| {
		// Setup by first submitting an appeal and forming committee
		let original_decision = BoundedVec::try_from(b"Original decision text".to_vec()).unwrap();
		let justification = create_justification(100);

		assert_ok!(AmbassadorGovernance::<Runtime>::submit_appeal(
			RuntimeOrigin::signed(1),
			original_decision.clone(),
			justification.clone(),
			None
		));

		let appeal_id = get_last_appeal_id();

		// Create appeal committee members
		let members = BoundedVec::try_from(vec![5, 4, 3]).unwrap();

		assert_ok!(AmbassadorGovernance::<Runtime>::form_appeal_committee(
			RuntimeOrigin::signed(3),
			appeal_id,
			members.clone()
		));

		// Create decision on appeal
		let decision = AppealDecision::Modified;
		let decider = 3;
		let decision_justification = BoundedVec::try_from(b"Justification for decision".to_vec()).unwrap();
		let evidence_hash = Some(H256::from_low_u64_be(42));

		// Execute as committee member
		assert_ok!(AmbassadorGovernance::<Runtime>::decide_appeal(
			RuntimeOrigin::signed(decider),
			appeal_id,
			decision.clone(),
			decision_justification.clone(),
			evidence_hash
		));

		// Verify
		let appeal = AmbassadorGovernance::<Runtime>::appeals(appeal_id).unwrap();
		assert_eq!(appeal.status, AppealStatus::Decided);
		assert_eq!(appeal.decision, Some(decision.clone()));
		assert_eq!(appeal.justification, decision_justification.clone());

		// Check event was emitted
		frame_system::Pallet::<Runtime>::assert_has_event(RuntimeEvent::AmbassadorGovernance(
			Event::AppealDecided { appeal_id, decider, decision, justification: decision_justification, evidence_hash },
		));
	});
}

#[test]
fn decide_appeal_fails_with_nonexistent_appeal() {
	new_test_ext().execute_with(|| {
		// Setup
		let appeal_id = H256::from_low_u64_be(999); // Non-existent appeal
		let decision = AppealDecision::Upheld;

		// Execute and verify failure
		assert_noop!(
			AmbassadorGovernance::<Runtime>::decide_appeal(signed_origin(3), appeal_id, decision.clone(), BoundedVec::try_from(b"Justification for decision".to_vec()).unwrap(), Some(H256::from_low_u64_be(42))),
			Error::<Runtime>::AppealCommitteeNotFormed
		);
	});
}

#[test]
fn decide_appeal_fails_with_no_committee_formed() {
	new_test_ext().execute_with(|| {
		// Setup by first submitting an appeal but don't form committee
		let original_decision = BoundedVec::try_from(b"Original decision text".to_vec()).unwrap();
		let justification = create_justification(100);

		assert_ok!(AmbassadorGovernance::<Runtime>::submit_appeal(
			RuntimeOrigin::signed(1),
			original_decision.clone(),
			justification.clone(),
			None
		));

		let appeal_id = get_last_appeal_id();

		// Try to decide without forming committee
		let decision = AppealDecision::Upheld;

		// Execute and verify failure
		assert_noop!(
			AmbassadorGovernance::<Runtime>::decide_appeal(
				signed_origin(3),
				appeal_id,
				decision.clone(),
				BoundedVec::try_from(b"Justification for decision".to_vec()).unwrap(),
				Some(H256::from_low_u64_be(42))
			),
			Error::<Runtime>::AppealCommitteeNotFormed
		);
	});
}

#[test]
fn decide_appeal_fails_with_unauthorized_account() {
	new_test_ext().execute_with(|| {
		// Setup by first submitting an appeal and forming committee
		let original_decision = BoundedVec::try_from(b"Original decision text".to_vec()).unwrap();
		let justification = create_justification(100);

		assert_ok!(AmbassadorGovernance::<Runtime>::submit_appeal(
			RuntimeOrigin::signed(1),
			original_decision.clone(),
			justification.clone(),
			None
		));

		let appeal_id = get_last_appeal_id();

		// Create committee members
		let members = BoundedVec::try_from(vec![5, 4, 3]).unwrap();

		assert_ok!(AmbassadorGovernance::<Runtime>::form_appeal_committee(
			RuntimeOrigin::signed(3),
			appeal_id,
			members.clone()
		));

		// Try to decide with unauthorized account (not committee member)
		let decision = AppealDecision::Upheld;

		assert_noop!(
			AmbassadorGovernance::<Runtime>::decide_appeal(
				signed_origin(6), // Not a committee member
				appeal_id,
				decision.clone(),
				BoundedVec::try_from(b"Justification for decision".to_vec()).unwrap(),
				Some(H256::from_low_u64_be(42))
			),
			Error::<Runtime>::NotCommitteeMember
		);
	});
}

#[test]
fn decide_appeal_fails_with_already_decided_appeal() {
	new_test_ext().execute_with(|| {
		// Setup by first submitting an appeal and form committee
		let original_decision = BoundedVec::try_from(b"Original decision text".to_vec()).unwrap();
		let justification = create_justification(100);

		assert_ok!(AmbassadorGovernance::<Runtime>::submit_appeal(
			RuntimeOrigin::signed(1),
			original_decision.clone(),
			justification.clone(),
			None
		));

		let appeal_id = get_last_appeal_id();

		// Create committee members
		let members = BoundedVec::try_from(vec![5, 4, 3]).unwrap();

		assert_ok!(AmbassadorGovernance::<Runtime>::form_appeal_committee(
			RuntimeOrigin::signed(3),
			appeal_id,
			members.clone()
		));

		// Decide on appeal first time
		let decision = AppealDecision::Modified;

		assert_ok!(AmbassadorGovernance::<Runtime>::decide_appeal(
			RuntimeOrigin::signed(3),
			appeal_id,
			decision.clone(),
			BoundedVec::try_from(b"Justification for decision".to_vec()).unwrap(),
			Some(H256::from_low_u64_be(42))
		));

		// Try to decide again
		assert_noop!(
			AmbassadorGovernance::<Runtime>::decide_appeal(
				RuntimeOrigin::signed(3),
				appeal_id,
				AppealDecision::Rejected,
				BoundedVec::try_from(b"Justification for decision".to_vec()).unwrap(),
				Some(H256::from_low_u64_be(42))
			),
			Error::<Runtime>::AppealAlreadyDecided
		);
	});
}

#[test]
fn establish_integration_works() {
	new_test_ext().execute_with(|| {
		// Setup
		let mechanism = IntegrationMechanism::JointGovernanceCouncil;
		let target_collective =
			TargetCollective::Other(b"Technical Committee".to_vec().try_into().unwrap());
		let description = create_description(100);
		let ambassador_participants = BoundedVec::try_from(vec![3, 4]).unwrap();
		let target_participants = BoundedVec::try_from(vec![1, 2]).unwrap();
		let evidence_hash = Some(random_hash());

		// Execute
		assert_ok!(AmbassadorGovernance::<Runtime>::establish_integration(
			RuntimeOrigin::signed(2),
			mechanism.clone(),
			target_collective.clone(),
			description.clone(),
			ambassador_participants.clone(),
			target_participants.clone(),
			evidence_hash
		));

		// Verify integration was created
		// Note: Since we can't directly check the integration_id since it's a hash of the details,
		// we'll instead verify an event was emitted
		// Check for IntegrationEstablished event (using wildcard for integration_id since we don't know its value)
		let events = frame_system::Pallet::<Runtime>::events();
		let mut found_integration = false;

		for event in &events {
			if let RuntimeEvent::AmbassadorGovernance(Event::IntegrationEstablished {
				integration_id: _,
				mechanism: m,
				target_collective: tc,
				..
			}) = &event.event
			{
				if *m == mechanism && *tc == target_collective {
					found_integration = true;
					break;
				}
			}
		}

		assert!(found_integration, "IntegrationEstablished event not found");
	});
}

#[test]
fn establish_integration_fails_with_invalid_origin() {
	new_test_ext().execute_with(|| {
		// Setup
		let mechanism = IntegrationMechanism::JointGovernanceCouncil;
		let target_collective =
			TargetCollective::Other(b"Technical Committee".to_vec().try_into().unwrap());
		let description = create_description(100);
		let ambassador_participants = BoundedVec::try_from(vec![3, 4]).unwrap();
		let target_participants = BoundedVec::try_from(vec![1, 2]).unwrap();
		let evidence_hash = Some(random_hash());

		// Execute and verify failure with unsigned origin
		assert_noop!(
			AmbassadorGovernance::<Runtime>::establish_integration(
				RuntimeOrigin::none(),
				mechanism.clone(),
				target_collective.clone(),
				description.clone(),
				ambassador_participants.clone(),
				target_participants.clone(),
				evidence_hash
			),
			sp_runtime::traits::BadOrigin
		);
	});
}

#[test]
fn establish_integration_fails_with_insufficient_rank() {
	new_test_ext().execute_with(|| {
		// Setup
		let mechanism = IntegrationMechanism::JointGovernanceCouncil;
		let target_collective =
			TargetCollective::Other(b"Technical Committee".to_vec().try_into().unwrap());
		let description = create_description(100);
		let ambassador_participants = BoundedVec::try_from(vec![3, 4]).unwrap();
		let target_participants = BoundedVec::try_from(vec![1, 2]).unwrap();
		let evidence_hash = Some(random_hash());

		// Try to establish integration without satisfying MinRankToEstablishIntegration
		assert_noop!(
			AmbassadorGovernance::<Runtime>::establish_integration(
				RuntimeOrigin::signed(1),
				mechanism.clone(),
				target_collective.clone(),
				description.clone(),
				ambassador_participants.clone(),
				target_participants.clone(),
				evidence_hash
			),
			Error::<Runtime>::InsufficientRank
		);
	});
}

#[test]
fn establish_integration_success_with_longest_possible_description() {
	new_test_ext().execute_with(|| {
		// Setup
		let mechanism = IntegrationMechanism::JointGovernanceCouncil;
		let target_collective =
			TargetCollective::Other(b"Technical Committee".to_vec().try_into().unwrap());
		let description = create_description(MaxDescriptionLength::get() as usize);
		let ambassador_participants = BoundedVec::try_from(vec![3, 4]).unwrap();
		let target_participants = BoundedVec::try_from(vec![1, 2]).unwrap();
		let evidence_hash = Some(random_hash());

		// Execute and verify success with longest possible description
		assert_ok!(
			AmbassadorGovernance::<Runtime>::establish_integration(
				RuntimeOrigin::signed(2),
				mechanism.clone(),
				target_collective.clone(),
				description,
				ambassador_participants.clone(),
				target_participants.clone(),
				evidence_hash
			)
		);
	});
}

#[test]
fn establish_integration_success_with_max_participants() {
	new_test_ext().execute_with(|| {
		// Setup
		let mechanism = IntegrationMechanism::JointGovernanceCouncil;
		let target_collective =
			TargetCollective::Other(b"Technical Committee".to_vec().try_into().unwrap());
		let description = create_description(100);

		// Create ambassador participants with valid ranks (3-6)
		let mut ambassador_participants_vec = Vec::new();
		// Use accounts 3-6 which have sufficient ranks
		ambassador_participants_vec.push(3); // Senior Ambassador
		ambassador_participants_vec.push(4); // Principal Ambassador
		ambassador_participants_vec.push(5); // Global Ambassador
		ambassador_participants_vec.push(6); // Global Head Ambassador
		let ambassador_participants = BoundedVec::try_from(ambassador_participants_vec).unwrap();

		let target_participants = BoundedVec::try_from(vec![3, 4]).unwrap();
		let evidence_hash = Some(random_hash());

		// Execute and verify success with max participants
		assert_ok!(
			AmbassadorGovernance::<Runtime>::establish_integration(
				RuntimeOrigin::signed(2),
				mechanism.clone(),
				target_collective.clone(),
				description.clone(),
				ambassador_participants,
				target_participants.clone(),
				evidence_hash
			)
		);
	});
}

#[test]
fn emergency_workflow_end_to_end() {
	new_test_ext().execute_with(|| {
		// Activate emergency protocol
		let emergency_type = EmergencyType::SecurityVulnerability;
		let severity = EmergencySeverity::Critical;
		let justification = create_justification(100);

		assert_ok!(AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
			RuntimeOrigin::signed(3),
			emergency_type.clone(),
			severity.clone(),
			justification.clone(),
			None
		));

		let emergency_id = get_last_emergency_id();

		// Form emergency committee with required roles
		let members = BoundedVec::try_from(vec![
			(5, EmergencyRole::TechnicalLead),
			(4, EmergencyRole::GovernanceRepresentative),
			(3, EmergencyRole::IndependentExpert),
		]).unwrap();

		assert_ok!(AmbassadorGovernance::<Runtime>::form_emergency_committee(
			RuntimeOrigin::signed(3),
			emergency_id,
			members.clone()
		));

		// Resolve emergency as committee member
		let resolution_summary = create_resolution(100);
		let abuse_detected = true; // Detected abuse

		// MinRankForEmergencyResponseAuthority
		assert_ok!(AmbassadorGovernance::<Runtime>::resolve_emergency(
			RuntimeOrigin::signed(4), // Committee member of at least MinRankForEmergencyResponseAuthority resolves
			emergency_id,
			abuse_detected,
			resolution_summary.clone(),
			None
		));

		// Verify final state
		let emergency = AmbassadorGovernance::<Runtime>::emergencies(emergency_id).unwrap();
		assert!(emergency.resolved_at.is_some());
		assert_eq!(emergency.abuse_detected, abuse_detected);

		// Verify events were emitted in correct order
		let events = frame_system::Pallet::<Runtime>::events();
		let mut found_activate = false;
		let mut found_committee = false;
		let mut found_resolve = false;

		for event in &events {
			match &event.event {
				RuntimeEvent::AmbassadorGovernance(Event::EmergencyActivated { .. }) => {
					found_activate = true;
					assert!(!found_committee && !found_resolve, "Events out of order");
				},
				RuntimeEvent::AmbassadorGovernance(Event::EmergencyCommitteeFormed { .. }) => {
					found_committee = true;
					assert!(found_activate && !found_resolve, "Events out of order");
				},
				RuntimeEvent::AmbassadorGovernance(Event::EmergencyResolved { .. }) => {
					found_resolve = true;
					assert!(found_activate && found_committee, "Events out of order");
				},
				_ => {},
			}
		}

		assert!(found_activate && found_committee && found_resolve, "Not all events were emitted");
	});
}

#[test]
fn appeal_workflow_end_to_end() {
	new_test_ext().execute_with(|| {
		// Submit appeal
		let original_decision = BoundedVec::try_from(b"Original decision text".to_vec()).unwrap();
		let justification = create_justification(100);

		assert_ok!(AmbassadorGovernance::<Runtime>::submit_appeal(
			RuntimeOrigin::signed(1),
			original_decision.clone(),
			justification.clone(),
			None
		));

		let appeal_id = get_last_appeal_id();

		// Form appeal committee
		let members = BoundedVec::try_from(vec![5, 4, 3]).unwrap();

		assert_ok!(AmbassadorGovernance::<Runtime>::form_appeal_committee(
			RuntimeOrigin::signed(3),
			appeal_id,
			members
		));

		// Decide on appeal
		let decision = AppealDecision::Modified;

		assert_ok!(AmbassadorGovernance::<Runtime>::decide_appeal(
			RuntimeOrigin::signed(4),
			appeal_id,
			decision.clone(),
			BoundedVec::try_from(b"Justification for decision".to_vec()).unwrap(),
			Some(H256::from_low_u64_be(42))
		));

		// Verify final state
		let appeal = AmbassadorGovernance::<Runtime>::appeals(appeal_id).unwrap();
		assert_eq!(appeal.status, AppealStatus::Decided);
		assert_eq!(appeal.decision, Some(decision));

		// Verify events were emitted in correct order
		let events = frame_system::Pallet::<Runtime>::events();
		let mut found_submit = false;
		let mut found_committee = false;
		let mut found_decide = false;

		for event in &events {
			match &event.event {
				RuntimeEvent::AmbassadorGovernance(Event::AppealSubmitted { .. }) => {
					found_submit = true;
					assert!(!found_committee && !found_decide, "Events out of order");
				},
				RuntimeEvent::AmbassadorGovernance(Event::AppealCommitteeFormed { .. }) => {
					found_committee = true;
					assert!(found_submit && !found_decide, "Events out of order");
				},
				RuntimeEvent::AmbassadorGovernance(Event::AppealDecided { .. }) => {
					found_decide = true;
					assert!(found_submit && found_committee, "Events out of order");
				},
				_ => {},
			}
		}

		assert!(found_submit && found_committee && found_decide, "Not all events were emitted");
	});
}

#[test]
fn set_service_provider_works() {
	new_test_ext().execute_with(|| {
		// Setup
		let provider_account = 3; // Account with verified identity
		let mut service_types = Vec::new();
		service_types.push(ProfessionalServiceType::LegalFinancial);
		service_types.push(ProfessionalServiceType::TechnicalDevelopment);
		let service_types = BoundedVec::<ProfessionalServiceType, MaxServiceTypes>::try_from(service_types).unwrap();
		let evidence_info = create_evidence_info(100); // Include reference to off-chain evidence
		let evidence_hash = Some(H256::random());

		// Execute as account 2 (has Rank II)
		assert_ok!(AmbassadorGovernance::<Runtime>::set_service_provider(
			RuntimeOrigin::signed(2),
			provider_account,
			service_types.clone(),
			evidence_info.clone(),
			evidence_hash,
		));

		// Verify event was emitted
		let events = frame_system::Pallet::<Runtime>::events();
		let mut found_event = false;

		for event in &events {
			if let RuntimeEvent::AmbassadorGovernance(Event::ServiceProviderSet {
				provider_account: id,
				service_types: types,
				evidence_info: info,
				evidence_hash: hash,
				last_updated: _,
			}) = &event.event
			{
				assert_eq!(id, &provider_account);
				assert_eq!(types, &service_types);
				assert_eq!(info, &evidence_info);
				assert_eq!(hash, &evidence_hash);
				found_event = true;
				break;
			}
		}

		assert!(found_event, "ServiceProviderSet event should be emitted");

		// Verify the provider was stored correctly
		let provider = AmbassadorGovernance::<Runtime>::service_providers(&provider_account).unwrap();
		assert_eq!(provider.provider_account, provider_account);
		assert_eq!(provider.service_types, service_types);
		assert_eq!(provider.evidence_info, evidence_info);
		assert_eq!(provider.evidence_hash, evidence_hash);
	});
}

#[test]
fn set_service_provider_fails_with_insufficient_rank() {
	new_test_ext().execute_with(|| {
		// Setup
		let provider_account = 3; // Account with verified identity
		let service_types = BoundedVec::<ProfessionalServiceType, MaxServiceTypes>::try_from(vec![ProfessionalServiceType::LegalFinancial]).unwrap();
		let evidence_info = create_evidence_info(100); // Include reference to off-chain evidence
		let evidence_hash = Some(H256::random());

		assert_noop!(
			AmbassadorGovernance::<Runtime>::set_service_provider(
				RuntimeOrigin::signed(1),
				provider_account,
				service_types,
				evidence_info,
				evidence_hash
			),
			Error::<Runtime>::InsufficientRank
		);
	});
}

#[test]
fn set_service_provider_fails_with_unverified_identity() {
	new_test_ext().execute_with(|| {
		// Setup
		let provider_account = 11; // Account WITHOUT verified identity
		let service_types = BoundedVec::<ProfessionalServiceType, MaxServiceTypes>::try_from(vec![ProfessionalServiceType::LegalFinancial]).unwrap();
		let evidence_info = create_evidence_info(100); // Include reference to off-chain evidence
		let evidence_hash = Some(H256::random());

		// Execute as account 2 (has sufficient rank to satisfy MinRankForProviderRegistry)
		assert_noop!(
			AmbassadorGovernance::<Runtime>::set_service_provider(
				RuntimeOrigin::signed(2),
				provider_account,
				service_types,
				evidence_info,
				evidence_hash
			),
			Error::<Runtime>::IdentityNotVerified
		);
	});
}

#[test]
fn set_service_referral_works() {
	new_test_ext().execute_with(|| {
		// First register a service provider
		let provider_account = 3;
		let service_types = BoundedVec::<ProfessionalServiceType, MaxServiceTypes>::try_from(vec![
			ProfessionalServiceType::LegalFinancial,
			ProfessionalServiceType::TechnicalDevelopment,
		]).unwrap();
		let evidence_info = create_evidence_info(100); // Include reference to off-chain evidence
		let evidence_hash = Some(H256::random());

		assert_ok!(AmbassadorGovernance::<Runtime>::set_service_provider(
			RuntimeOrigin::signed(2),
			provider_account,
			service_types.clone(),
			evidence_info.clone(),
			evidence_hash,
		));

		// Create referral
		let description = create_description(100); // Include reference to off-chain evidence
		let compensation_disclosed = true;
		let compensation_details = Some(BoundedVec::try_from(b"Received 10 tokens for this referral".to_vec()).unwrap());
		let evidence_hash = Some(H256::random());

		// Clear events
		frame_system::Pallet::<Runtime>::reset_events();

		assert_ok!(AmbassadorGovernance::<Runtime>::set_service_referral(
			RuntimeOrigin::signed(2),
			provider_account,
			BoundedVec::try_from(vec![ProfessionalServiceType::LegalFinancial]).unwrap(),
			description.clone(),
			compensation_disclosed,
			compensation_details.clone(),
			evidence_hash,
		));

		// Verify event was emitted
		let events = frame_system::Pallet::<Runtime>::events();
		let mut found_event = false;
		let mut referral_id = None;

		for event in &events {
			if let RuntimeEvent::AmbassadorGovernance(Event::ServiceReferralSet {
				referral_id: r_id,
				referrer,
				provider_account: p_acct,
				service_types,
				description: desc,
				compensation_disclosed: c_disclosed,
				compensation_details: c_details,
				evidence_hash: hash,
				last_updated: _,
			}) = &event.event
			{
				found_event = true;
				referral_id = Some(r_id.clone());
				assert_eq!(*referrer, 2);
				assert_eq!(*p_acct, provider_account);
				assert_eq!(*service_types, BoundedVec::<ProfessionalServiceType, MaxServiceTypes>::try_from(vec![ProfessionalServiceType::LegalFinancial]).unwrap());
				assert_eq!(*desc, description);
				assert_eq!(*c_disclosed, compensation_disclosed);
				assert_eq!(*hash, evidence_hash);
				break;
			}
		}

		assert!(found_event, "ServiceReferralSet event should be emitted");

		// Verify the referral was stored correctly
		let referral_id = referral_id.unwrap();
		let referral = AmbassadorGovernance::<Runtime>::service_referrals(referral_id).unwrap();
		assert_eq!(referral.referrer, 2);
		assert_eq!(referral.provider_account, provider_account);
		assert_eq!(referral.service_types, BoundedVec::<ProfessionalServiceType, MaxServiceTypes>::try_from(vec![ProfessionalServiceType::LegalFinancial]).unwrap());
		assert_eq!(referral.description, description);
		assert_eq!(referral.compensation_disclosed, compensation_disclosed);
		assert_eq!(referral.compensation_details, compensation_details);
		assert_eq!(referral.evidence_hash, evidence_hash);
	});
}

#[test]
fn set_service_referral_fails_with_insufficient_rank() {
	new_test_ext().execute_with(|| {
		// First register a service provider
		let provider_account = 3;
		let service_types = BoundedVec::<ProfessionalServiceType, MaxServiceTypes>::try_from(vec![ProfessionalServiceType::LegalFinancial]).unwrap();
		let evidence_info = create_evidence_info(100); // Include reference to off-chain evidence
		let evidence_hash = Some(H256::random());

		assert_ok!(AmbassadorGovernance::<Runtime>::set_service_provider(
			RuntimeOrigin::signed(2),
			provider_account,
			service_types.clone(),
			evidence_info.clone(),
			evidence_hash,
		));

		let description = create_description(100);
		let compensation_disclosed = false;
		let compensation_details = None;
		let evidence_hash = Some(H256::random());

		assert_noop!(
			AmbassadorGovernance::<Runtime>::set_service_referral(
				RuntimeOrigin::signed(1), // Insufficient rank to satisfy MinRankForReferral
				provider_account,
				service_types.clone(),
				description,
				compensation_disclosed,
				compensation_details,
				evidence_hash
			),
			Error::<Runtime>::InsufficientRank
		);
	});
}

#[test]
fn set_service_referral_fails_with_nonexistent_provider() {
	new_test_ext().execute_with(|| {
		// Try to create referral for a non-existent provider
		let non_existent_provider_account = 12;
		let service_types = BoundedVec::<ProfessionalServiceType, MaxServiceTypes>::try_from(vec![ProfessionalServiceType::LegalFinancial]).unwrap();
		let description = create_description(100);
		let compensation_disclosed = false;
		let compensation_details = None;
		let evidence_hash = Some(H256::random());

		assert_noop!(
			AmbassadorGovernance::<Runtime>::set_service_referral(
				RuntimeOrigin::signed(2), // Sufficient rank to satisfy MinRankForReferral
				non_existent_provider_account,
				service_types,
				description,
				compensation_disclosed,
				compensation_details,
				evidence_hash
			),
			Error::<Runtime>::ServiceProviderNotFound
		);
	});
}

#[test]
fn set_service_referral_fails_with_missing_compensation_disclosure() {
	new_test_ext().execute_with(|| {
		// First register a service provider
		let provider_account = 3;
		let service_types = BoundedVec::<ProfessionalServiceType, MaxServiceTypes>::try_from(vec![ProfessionalServiceType::LegalFinancial]).unwrap();
		let description = create_description(100);
		let compensation_disclosed = true;
		let compensation_details = None; // Missing details
		let evidence_info = create_evidence_info(100); // Include reference to off-chain evidence
		let evidence_hash = Some(H256::random());

		assert_ok!(AmbassadorGovernance::<Runtime>::set_service_provider(
			RuntimeOrigin::signed(2), // Sufficient rank to satisfy MinRankForProviderRegistry
			provider_account,
			service_types.clone(),
			evidence_info.clone(),
			evidence_hash,
		));

		assert_noop!(
			AmbassadorGovernance::<Runtime>::set_service_referral(
				RuntimeOrigin::signed(2), // Sufficient rank to satisfy MinRankForReferral
				provider_account,
				service_types.clone(),
				description,
				compensation_disclosed,
				compensation_details,
				evidence_hash
			),
			Error::<Runtime>::MissingCompensationDisclosure
		);
	});
}

#[test]
fn professional_services_workflow_end_to_end() {
	new_test_ext().execute_with(|| {
		// Register service provider
		let provider_account = 3; // Account with verified identity
		let service_types = BoundedVec::<ProfessionalServiceType, MaxServiceTypes>::try_from(vec![
			ProfessionalServiceType::LegalFinancial,
			ProfessionalServiceType::TechnicalDevelopment,
		]).unwrap();
		let evidence_info = create_evidence_info(100); // Include reference to off-chain evidence
		let evidence_hash = Some(H256::random());

		assert_ok!(AmbassadorGovernance::<Runtime>::set_service_provider(
			RuntimeOrigin::signed(2), // Sufficient rank to satisfy MinRankForProviderRegistry
			provider_account,
			service_types.clone(),
			evidence_info.clone(),
			evidence_hash,
		));

		// Create a referral with compensation disclosure
		let description = create_description(100); // Include reference to off-chain evidence
		let compensation_disclosed = true;
		let compensation_details = Some(BoundedVec::try_from(b"Received 5% commission for this referral".to_vec()).unwrap());
		let evidence_hash = Some(H256::random());

		// Clear events
		frame_system::Pallet::<Runtime>::reset_events();

		assert_ok!(AmbassadorGovernance::<Runtime>::set_service_referral(
			RuntimeOrigin::signed(2), // Sufficient rank to satisfy MinRankForReferral
			provider_account,
			BoundedVec::<ProfessionalServiceType, MaxServiceTypes>::try_from(vec![ProfessionalServiceType::LegalFinancial]).unwrap(),
			description.clone(),
			compensation_disclosed,
			compensation_details.clone(),
			evidence_hash,
		));

		// Create another referral without compensation
		let description2 = create_description(80);
		let compensation_disclosed2 = false;
		let compensation_details2 = None;
		let evidence_hash2 = Some(H256::random());

		assert_ok!(AmbassadorGovernance::<Runtime>::set_service_referral(
			RuntimeOrigin::signed(2), // Sufficient rank to satisfy MinRankForReferral
			provider_account,
			BoundedVec::<ProfessionalServiceType, MaxServiceTypes>::try_from(vec![ProfessionalServiceType::LegalFinancial]).unwrap(),
			description2.clone(),
			compensation_disclosed2,
			compensation_details2,
			evidence_hash2,
		));

		// Verify events were emitted
		let events = frame_system::Pallet::<Runtime>::events();
		let mut found_referrals = 0;

		for event in &events {
			if let RuntimeEvent::AmbassadorGovernance(Event::ServiceReferralSet { .. }) =
				&event.event
			{
				found_referrals += 1;
			}
		}

		assert_eq!(found_referrals, 2, "Two ServiceReferralSet events should be emitted");
	});
}

#[test]
fn identity_verification_is_enforced() {
	new_test_ext().execute_with(|| {
		// Account 3 has a verified identity (according to MockIdentityVerifier)
		let account_with_identity = 3;

		// Account 11 does not have a verified identity (according to MockIdentityVerifier)
		let account_without_identity = 11;

		// Test with account that has verified identity - should succeed
		assert_ok!(AmbassadorGovernance::<Runtime>::submit_appeal(
			signed_origin(account_with_identity),
			BoundedVec::try_from(b"Appeal decision hash".to_vec()).unwrap(),
			BoundedVec::try_from(b"Appeal justification with reference to off-chain evidence location".to_vec()).unwrap(),
			None
		));

		// Test with account that doesn't have verified identity and it should fail with IdentityNotVerified error
		assert_noop!(
			AmbassadorGovernance::<Runtime>::submit_appeal(
				signed_origin(account_without_identity),
				BoundedVec::try_from(b"Appeal decision hash".to_vec()).unwrap(),
				BoundedVec::try_from(b"Appeal justification with reference to off-chain evidence location".to_vec()).unwrap(),
				None
			),
			crate::Error::<Runtime>::IdentityNotVerified
		);

		// Test another extrinsic with identity verification
		assert_ok!(AmbassadorGovernance::<Runtime>::set_conflict_of_interest(
			signed_origin(account_with_identity),
			crate::ConflictType::FinancialCritical,
			BoundedVec::try_from(b"Conflict".to_vec()).unwrap(),
			Some(BoundedVec::try_from(b"Related matter".to_vec()).unwrap()),
			None,
			None,
			Some(H256::from_low_u64_be(42)),
			None
		));

		// Test the same extrinsic with account without identity and it should fail
		assert_noop!(
			AmbassadorGovernance::<Runtime>::set_conflict_of_interest(
				signed_origin(account_without_identity),
				crate::ConflictType::FinancialCritical,
				BoundedVec::try_from(b"Conflict".to_vec()).unwrap(),
				Some(BoundedVec::try_from(b"Related matter".to_vec()).unwrap()),
				None,
				None,
				Some(H256::from_low_u64_be(42)),
				None
			),
			crate::Error::<Runtime>::IdentityNotVerified
		);
	});
}

#[test]
fn register_rank_transition_works() {
	new_test_ext().execute_with(|| {
		// Setup
		let member = 3; // Account with verified identity
		let transition_type = TransitionType::Promotion;
		let previous_rank = 2;
		let new_rank = 3;
		let justification: BoundedVec<u8, <Runtime as crate::Config>::MaxJustificationLength> =
			BoundedVec::try_from(b"Excellent contributions to the ecosystem".to_vec()).unwrap();
		let effective_at = 100;
		let successor = None;
		let evidence_hash = Some(H256::random());

		// Clear events
		frame_system::Pallet::<Runtime>::reset_events();

		assert_ok!(AmbassadorGovernance::<Runtime>::register_rank_transition(
			RuntimeOrigin::signed(4), // Sufficient rank to satisfy MinRankForRoleFulfillmentContingency
			member,
			transition_type.clone(),
			previous_rank,
			new_rank,
			justification.clone(),
			effective_at,
			successor.clone(),
			evidence_hash,
		));

		// Verify event was emitted
		let events = frame_system::Pallet::<Runtime>::events();
		let mut found_event = false;
		let mut transition_id = None;

		for event in &events {
			if let RuntimeEvent::AmbassadorGovernance(Event::RankTransitionRegistered {
				transition_id: id,
				member: m,
				transition_type: t_type,
				previous_rank: p_rank,
				new_rank: n_rank,
				effective_at: e_at,
				evidence_hash: e_hash,
			}) = &event.event
			{
				transition_id = Some(id.clone());
				assert_eq!(m, &member);
				assert_eq!(t_type, &transition_type);
				assert_eq!(p_rank, &previous_rank);
				assert_eq!(n_rank, &new_rank);
				assert_eq!(e_at, &effective_at);
				assert_eq!(e_hash, &evidence_hash);
				found_event = true;
				break;
			}
		}

		assert!(found_event, "RankTransitionRegistered event not found");

		// Verify transition was stored
		let transition_id = transition_id.unwrap();
		let transition = AmbassadorGovernance::<Runtime>::transitions(transition_id).unwrap();
		assert_eq!(transition.member, member);
		assert_eq!(transition.transition_type, transition_type);
		assert_eq!(transition.previous_rank, previous_rank);
		assert_eq!(transition.new_rank, new_rank);
		assert_eq!(transition.justification, justification);
		assert_eq!(transition.effective_at, effective_at);
		assert_eq!(transition.successor, successor);
		assert_eq!(transition.evidence_hash, evidence_hash);
		assert_eq!(transition.knowledge_transfer_complete, false);
	});
}

#[test]
fn register_rank_transition_without_evidence_hash_works() {
	new_test_ext().execute_with(|| {
		// Setup
		let member = 3; // Account with verified identity
		let transition_type = TransitionType::Demotion;
		let previous_rank = 3;
		let new_rank = 2;
		let justification: BoundedVec<u8, <Runtime as crate::Config>::MaxJustificationLength> =
			BoundedVec::try_from(b"Performance issues".to_vec()).unwrap();
		let effective_at = 100;
		let successor = None;
		let evidence_hash = None; // No evidence hash provided

		// Clear events
		frame_system::Pallet::<Runtime>::reset_events();

		// Execute as account 1 (has sufficient rank)
		assert_ok!(AmbassadorGovernance::<Runtime>::register_rank_transition(
			RuntimeOrigin::signed(4), // Sufficient rank to satisfy MinRankForRoleFulfillmentContingency
			member,
			transition_type.clone(),
			previous_rank,
			new_rank,
			justification.clone(),
			effective_at,
			successor.clone(),
			evidence_hash,
		));

		// Verify event was emitted
		let events = frame_system::Pallet::<Runtime>::events();
		let mut found_event = false;
		let mut transition_id = None;

		for event in &events {
			if let RuntimeEvent::AmbassadorGovernance(Event::RankTransitionRegistered {
				transition_id: id,
				member: m,
				transition_type: t_type,
				previous_rank: p_rank,
				new_rank: n_rank,
				effective_at: e_at,
				evidence_hash: e_hash,
			}) = &event.event
			{
				transition_id = Some(id.clone());
				assert_eq!(m, &member);
				assert_eq!(t_type, &transition_type);
				assert_eq!(p_rank, &previous_rank);
				assert_eq!(n_rank, &new_rank);
				assert_eq!(e_at, &effective_at);
				assert_eq!(e_hash, &evidence_hash);
				found_event = true;
				break;
			}
		}

		assert!(found_event, "RankTransitionRegistered event not found");

		// Verify transition was stored
		let transition_id = transition_id.unwrap();
		let transition = AmbassadorGovernance::<Runtime>::transitions(transition_id).unwrap();
		assert_eq!(transition.member, member);
		assert_eq!(transition.transition_type, transition_type);
		assert_eq!(transition.previous_rank, previous_rank);
		assert_eq!(transition.new_rank, new_rank);
		assert_eq!(transition.justification, justification);
		assert_eq!(transition.effective_at, effective_at);
		assert_eq!(transition.successor, successor);
		assert_eq!(transition.evidence_hash, evidence_hash);
		assert_eq!(transition.knowledge_transfer_complete, false);
	});
}

#[test]
fn register_rank_transition_fails_with_insufficient_rank() {
	new_test_ext().execute_with(|| {
		// Setup
		let member = 3; // Account with verified identity
		let transition_type = TransitionType::Promotion;
		let previous_rank = 2;
		let new_rank = 3;
		let justification: BoundedVec<u8, <Runtime as crate::Config>::MaxJustificationLength> =
			BoundedVec::try_from(b"Excellent contributions to the ecosystem".to_vec()).unwrap();
		let effective_at = 100;
		let successor = None;
		let evidence_hash = Some(H256::random());

		assert_noop!(
			AmbassadorGovernance::<Runtime>::register_rank_transition(
				RuntimeOrigin::signed(3), // Insufficient rank to satisfy MinRankForRoleFulfillmentContingency
				member,
				transition_type,
				previous_rank,
				new_rank,
				justification,
				effective_at,
				successor,
				evidence_hash
			),
			Error::<Runtime>::InsufficientRank
		);
	});
}

#[test]
fn register_rank_transition_fails_with_unverified_identity() {
	new_test_ext().execute_with(|| {
		// Setup
		let member = 3; // Account with verified identity
		let transition_type = TransitionType::Promotion;
		let previous_rank = 2;
		let new_rank = 3;
		let justification: BoundedVec<u8, <Runtime as crate::Config>::MaxJustificationLength> =
			BoundedVec::try_from(b"Excellent contributions to the ecosystem".to_vec()).unwrap();
		let effective_at = 100;
		let successor = None;
		let evidence_hash = Some(H256::random());

		// Execute as account 11 (without verified identity)
		assert_noop!(
			AmbassadorGovernance::<Runtime>::register_rank_transition(
				RuntimeOrigin::signed(11),
				member,
				transition_type,
				previous_rank,
				new_rank,
				justification,
				effective_at,
				successor,
				evidence_hash
			),
			Error::<Runtime>::IdentityNotVerified
		);
	});
}

#[test]
fn register_rank_transition_with_successor_works() {
	new_test_ext().execute_with(|| {
		// Setup
		let member = 3; // Account with verified identity
		let transition_type = TransitionType::Resignation;
		let previous_rank = 3;
		let new_rank = 0;
		let justification: BoundedVec<u8, <Runtime as crate::Config>::MaxJustificationLength> =
			BoundedVec::try_from(b"Moving to a different role".to_vec()).unwrap();
		let effective_at = 100;
		let successor = Some(2); // Account 2 will take over
		let evidence_hash = Some(H256::random());

		// Clear events
		frame_system::Pallet::<Runtime>::reset_events();

		assert_ok!(AmbassadorGovernance::<Runtime>::register_rank_transition(
			RuntimeOrigin::signed(4), // Sufficient rank to satisfy MinRankForRoleFulfillmentContingency
			member,
			transition_type.clone(),
			previous_rank,
			new_rank,
			justification.clone(),
			effective_at,
			successor.clone(),
			evidence_hash,
		));

		// Verify event was emitted
		let events = frame_system::Pallet::<Runtime>::events();
		let mut found_event = false;
		let mut transition_id = None;

		for event in &events {
			if let RuntimeEvent::AmbassadorGovernance(Event::RankTransitionRegistered {
				transition_id: id,
				member: m,
				transition_type: t_type,
				previous_rank: p_rank,
				new_rank: n_rank,
				effective_at: e_at,
				evidence_hash: e_hash,
			}) = &event.event
			{
				found_event = true;
				transition_id = Some(id.clone());
				assert_eq!(m, &member);
				assert_eq!(t_type, &transition_type);
				assert_eq!(p_rank, &previous_rank);
				assert_eq!(n_rank, &new_rank);
				assert_eq!(e_at, &effective_at);
				assert_eq!(e_hash, &evidence_hash);
				break;
			}
		}

		assert!(found_event, "RankTransitionRegistered event not found");

		// Verify transition was stored
		let transition_id = transition_id.unwrap();
		let transition = AmbassadorGovernance::<Runtime>::transitions(transition_id).unwrap();
		assert_eq!(transition.member, member);
		assert_eq!(transition.transition_type, transition_type);
		assert_eq!(transition.previous_rank, previous_rank);
		assert_eq!(transition.new_rank, new_rank);
		assert_eq!(transition.justification, justification);
		assert_eq!(transition.effective_at, effective_at);
		assert_eq!(transition.successor, successor);
		assert_eq!(transition.evidence_hash, evidence_hash);
		assert_eq!(transition.knowledge_transfer_complete, false);
	});
}

#[test]
fn set_remark_works() {
	new_test_ext().execute_with(|| {
		// Setup
		let category = RemarkCategory::Governance;
		let content = create_remark_content(100); // Include reference to off-chain evidence
		let evidence_hash = Some(H256::random());

		// Execute as account 2 (has Rank II)
		assert_ok!(AmbassadorGovernance::<Runtime>::set_remark(
			RuntimeOrigin::signed(2),
			category.clone(),
			content.clone(),
			evidence_hash,
			None // Create a new remark
		));

		// Verify event was emitted
		let events = frame_system::Pallet::<Runtime>::events();
		let mut found_event = false;
		let mut nonce_used = 0;

		for event in &events {
			if let RuntimeEvent::AmbassadorGovernance(Event::RemarkSet {
				author,
				category: event_category,
				content: event_content,
				evidence_hash: event_hash,
				nonce,
				last_updated: _,
			}) = &event.event
			{
				assert_eq!(author, &2);
				assert_eq!(event_category, &category);
				assert_eq!(event_content, &content);
				assert_eq!(event_hash, &evidence_hash);
				nonce_used = *nonce;
				found_event = true;
				break;
			}
		}

		assert!(found_event, "RemarkSet event should be emitted");

		// Verify the remark was stored correctly
		let remark = AmbassadorGovernance::<Runtime>::remarks((2, category.clone(), nonce_used)).unwrap();
		assert_eq!(remark.content, content);
		assert_eq!(remark.evidence_hash, evidence_hash);

		// Test updating an existing remark
		let updated_content = create_remark_content(150);
		let updated_evidence_hash = Some(H256::random());

		assert_ok!(AmbassadorGovernance::<Runtime>::set_remark(
			RuntimeOrigin::signed(2),
			category.clone(),
			updated_content.clone(),
			updated_evidence_hash,
			Some(nonce_used) // Update the existing remark
		));

		// Verify the remark was updated correctly
		let updated_remark = AmbassadorGovernance::<Runtime>::remarks((2, category.clone(), nonce_used)).unwrap();
		assert_eq!(updated_remark.content, updated_content);
		assert_eq!(updated_remark.evidence_hash, updated_evidence_hash);
	});
}

#[test]
fn set_remark_fails_with_insufficient_rank() {
	new_test_ext().execute_with(|| {
		// Setup
		let category = RemarkCategory::Governance;
		let content = create_remark_content(100);
		let evidence_hash = Some(H256::random());

		// Execute as account 0 (has Rank 0, which should be insufficient)
		// MinRankToSetRemark is set to 1 (Rank I) in mock.rs
		assert_noop!(
			AmbassadorGovernance::<Runtime>::set_remark(
				RuntimeOrigin::signed(0),
				category,
				content,
				evidence_hash,
				None
			),
			Error::<Runtime>::InsufficientRank
		);
	});
}

#[test]
fn set_remark_fails_with_nonexistent_nonce() {
	new_test_ext().execute_with(|| {
		// Setup
		let category = RemarkCategory::Governance;
		let content = create_remark_content(100);
		let evidence_hash = Some(H256::random());
		let nonexistent_nonce = 999; // A nonce that doesn't exist

		// Try to update a remark with a nonexistent nonce
		assert_noop!(
			AmbassadorGovernance::<Runtime>::set_remark(
				RuntimeOrigin::signed(2),
				category,
				content,
				evidence_hash,
				Some(nonexistent_nonce)
			),
			Error::<Runtime>::RemarkNotFound
		);
	});
}

#[test]
fn set_remark_fails_with_no_identity() {
	new_test_ext().execute_with(|| {
		// Setup
		let category = RemarkCategory::Governance;
		let content = create_remark_content(100);
		let evidence_hash = Some(H256::random());

		// Account 99 has no identity
		assert_noop!(
			AmbassadorGovernance::<Runtime>::set_remark(
				RuntimeOrigin::signed(99),
				category,
				content,
				evidence_hash,
				None
			),
			Error::<Runtime>::IdentityNotVerified
		);
	});
}

#[test]
fn set_conflict_of_interest_works() {
	new_test_ext().execute_with(|| {
		// Setup
		let conflict_type = ConflictType::FinancialCritical;
		let description = BoundedVec::try_from(b"Financial interest in related project. Details at: ipfs://QmConflictDetails789".to_vec()).unwrap();
		let relates_to = Some(BoundedVec::try_from(b"Integration proposal #42".to_vec()).unwrap());
		let start_block = Some(100u64);
		let end_block = Some(500u64);
		let evidence_hash = Some(H256::random());

		// Execute as account 2 (has Rank II, which should be sufficient)
		assert_ok!(AmbassadorGovernance::<Runtime>::set_conflict_of_interest(
			RuntimeOrigin::signed(2),
			conflict_type.clone(),
			description.clone(),
			relates_to.clone(),
			start_block,
			end_block,
			evidence_hash,
			None // Create a new conflict
		));

		// Verify event was emitted
		let events = frame_system::Pallet::<Runtime>::events();
		let mut found_event = false;
		let mut nonce_used = 0;

		for event in &events {
			if let RuntimeEvent::AmbassadorGovernance(Event::ConflictOfInterestSet {
				member,
				conflict_type: event_conflict_type,
				description: event_description,
				nonce,
				..
			}) = &event.event
			{
				found_event = true;
				nonce_used = nonce.unwrap();
				assert_eq!(member, &2);
				assert_eq!(event_conflict_type, &conflict_type);
				assert_eq!(event_description, &description);
				break;
			}
		}

		assert!(found_event, "ConflictOfInterestSet event should be emitted");

		// Verify the conflict was stored correctly
		let conflict = AmbassadorGovernance::<Runtime>::conflicts((2, conflict_type.clone(), nonce_used)).unwrap();
		assert_eq!(conflict.member, 2);
		assert_eq!(conflict.conflict_type, conflict_type);
		assert_eq!(conflict.description, description);
		assert_eq!(conflict.relates_to, relates_to);
		assert_eq!(conflict.start_block, start_block);
		assert_eq!(conflict.end_block, end_block);
		assert_eq!(conflict.evidence_hash, evidence_hash);

		// Test updating an existing conflict
		let updated_description = BoundedVec::try_from(b"Updated financial interest. New details at: ipfs://QmUpdatedDetails".to_vec()).unwrap();
		let updated_relates_to = Some(BoundedVec::try_from(b"Updated integration proposal #42".to_vec()).unwrap());
		let updated_end_block = Some(600u64);
		let updated_evidence_hash = Some(H256::random());

		assert_ok!(AmbassadorGovernance::<Runtime>::set_conflict_of_interest(
			RuntimeOrigin::signed(2),
			conflict_type.clone(),
			updated_description.clone(),
			updated_relates_to.clone(),
			start_block,
			updated_end_block,
			updated_evidence_hash,
			Some(nonce_used) // Update the existing conflict
		));

		// Verify the conflict was updated correctly
		let updated_conflict = AmbassadorGovernance::<Runtime>::conflicts((2, conflict_type.clone(), nonce_used)).unwrap();
		assert_eq!(updated_conflict.description, updated_description);
		assert_eq!(updated_conflict.relates_to, updated_relates_to);
		assert_eq!(updated_conflict.end_block, updated_end_block);
		assert_eq!(updated_conflict.evidence_hash, updated_evidence_hash);
	});
}

#[test]
fn set_conflict_of_interest_with_default_start_block_works() {
	new_test_ext().execute_with(|| {
		// Setup
		let conflict_type = ConflictType::ProfessionalCritical;
		let description = BoundedVec::try_from(b"Professional conflict. Details at: ipfs://QmProfDetails".to_vec()).unwrap();
		let relates_to = None; // No specific related matter
		let start_block = None; // Should default to current block
		let end_block = None; // No end date
		let evidence_hash = Some(H256::random());

		// Get current block before execution
		let current_block = frame_system::Pallet::<Runtime>::block_number();

		// Execute
		assert_ok!(AmbassadorGovernance::<Runtime>::set_conflict_of_interest(
			RuntimeOrigin::signed(3),
			conflict_type.clone(),
			description.clone(),
			relates_to.clone(),
			start_block,
			end_block,
			evidence_hash,
			None
		));

		// Find the nonce used
		let events = frame_system::Pallet::<Runtime>::events();
		let mut nonce_used = 0;

		for event in &events {
			if let RuntimeEvent::AmbassadorGovernance(Event::ConflictOfInterestSet { nonce, .. }) = &event.event {
				nonce_used = nonce.unwrap();
				break;
			}
		}

		// Verify the conflict was stored with current block as start_block
		let conflict = AmbassadorGovernance::<Runtime>::conflicts((3, conflict_type, nonce_used)).unwrap();
		assert_eq!(conflict.start_block, Some(current_block));
	});
}

#[test]
fn set_conflict_of_interest_fails_with_nonexistent_conflict() {
	new_test_ext().execute_with(|| {
		// Setup
		let conflict_type = ConflictType::IdeologicalNonCritical;
		let description = BoundedVec::try_from(b"Ideological bias regarding implementation approach. Details at: ipfs://QmConflictDetails123".to_vec()).unwrap();
		let relates_to = Some(BoundedVec::try_from(b"Some related matter".to_vec()).unwrap());
		let start_block = Some(100u64);
		let end_block = Some(500u64);
		let evidence_hash = Some(H256::random());
		let nonexistent_nonce = 999; // A nonce that doesn't exist

		// Try to update a conflict of interest with a nonexistent nonce
		assert_noop!(
			AmbassadorGovernance::<Runtime>::set_conflict_of_interest(
				RuntimeOrigin::signed(2),
				conflict_type,
				description,
				relates_to,
				start_block,
				end_block,
				evidence_hash,
				Some(nonexistent_nonce)
			),
			Error::<Runtime>::ConflictOfInterestNotFound
		);
	});
}

#[test]
fn set_conflict_of_interest_fails_with_insufficient_rank() {
	new_test_ext().execute_with(|| {
		// Setup
		let conflict_type = ConflictType::FinancialCritical;
		let description = BoundedVec::try_from(b"Financial conflict. Details at: ipfs://QmConflictDetails456".to_vec()).unwrap();
		let relates_to = Some(BoundedVec::try_from(b"Some related matter".to_vec()).unwrap());
		let start_block = Some(100u64);
		let end_block = Some(500u64);
		let evidence_hash = Some(H256::random());

		// Execute as account 10 (has verified identity but no specific rank assigned)
		// MinRankToSetConflictOfInterest is set to 0 (All ranks) in mock.rs
		assert_noop!(
			AmbassadorGovernance::<Runtime>::set_conflict_of_interest(
				RuntimeOrigin::signed(10),
				conflict_type,
				description,
				relates_to,
				start_block,
				end_block,
				evidence_hash,
				None
			),
			Error::<Runtime>::InsufficientRank
		);
	});
}

#[test]
fn set_conflict_of_interest_multiple_conflicts_works() {
	new_test_ext().execute_with(|| {
		// Setup
		let conflict_type1 = ConflictType::FinancialCritical;
		let description1 = BoundedVec::try_from(b"Financial interest in outcome. Details at: ipfs://QmConflictDetails789".to_vec()).unwrap();
		let relates_to1 = Some(BoundedVec::try_from(b"Related matter 1".to_vec()).unwrap());
		let evidence_hash1 = Some(H256::random());

		// Create first conflict
		assert_ok!(AmbassadorGovernance::<Runtime>::set_conflict_of_interest(
			RuntimeOrigin::signed(2),
			conflict_type1.clone(),
			description1.clone(),
			relates_to1.clone(),
			None,
			None,
			evidence_hash1,
			None
		));

		// Setup for second conflict of same type
		let description2 = BoundedVec::try_from(b"Financial interest in outcome. Details at: ipfs://QmConflictDetails456".to_vec()).unwrap();
		let relates_to2 = Some(BoundedVec::try_from(b"Related matter 2".to_vec()).unwrap());
		let evidence_hash2 = Some(H256::random());

		// Create second conflict of same type
		assert_ok!(AmbassadorGovernance::<Runtime>::set_conflict_of_interest(
			RuntimeOrigin::signed(2),
			conflict_type1.clone(),
			description2.clone(),
			relates_to2.clone(),
			None,
			None,
			evidence_hash2,
			None
		));

		// Verify both conflicts exist with different nonces
		let events = frame_system::Pallet::<Runtime>::events();
		let mut nonce1 = None;
		let mut nonce2 = None;

		for event in &events {
			if let RuntimeEvent::AmbassadorGovernance(Event::ConflictOfInterestSet {
				member,
				conflict_type,
				description,
				nonce,
				..
			}) = &event.event
			{
				if member == &2 && conflict_type == &conflict_type1 {
					if description == &description1 {
						nonce1 = *nonce;
					} else if description == &description2 {
						nonce2 = *nonce;
					}
				}
			}
		}

		assert!(nonce1.is_some(), "First conflict should have a nonce");
		assert!(nonce2.is_some(), "Second conflict should have a nonce");
		assert_ne!(nonce1, nonce2, "Conflicts should have different nonces");

		// Verify both conflicts are stored correctly
		assert!(AmbassadorGovernance::<Runtime>::conflicts((2, conflict_type1.clone(), nonce1.unwrap())).is_some());
		assert!(AmbassadorGovernance::<Runtime>::conflicts((2, conflict_type1.clone(), nonce2.unwrap())).is_some());
	});
}

#[test]
fn register_conflict_of_interest_for_emergency_works() {
	new_test_ext().execute_with(|| {
		// Setup: Create accounts with verified identities
		let initiator = 3;
		let committee_member = 4;

		// Assign sufficient rank according to MinRankToActivateEmergencyProtocol
		assign_ambassador_rank(initiator, 3);
		// Assign sufficient rank according to MinRankToFormEmergencyCommittee
		assign_ambassador_rank(committee_member, 3);

		// Step 1: Activate an emergency
		let emergency_type = EmergencyType::SecurityVulnerability;
		let severity = EmergencySeverity::Critical;
		let justification: BoundedVec<u8, <Runtime as crate::Config>::MaxJustificationLength> =
			BoundedVec::try_from(b"Critical security vulnerability detected in runtime".to_vec()).unwrap();
		let evidence_hash = H256::from_low_u64_be(1);

		assert_ok!(AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
			RuntimeOrigin::signed(initiator.clone()),
			emergency_type.clone(),
			severity.clone(),
			justification.clone(),
			Some(evidence_hash)
		));

		// Get the emergency_id from events
		let emergency_id = match System::events().last().unwrap().event {
			RuntimeEvent::AmbassadorGovernance(
				Event::EmergencyActivated { emergency_id, .. }
			) => emergency_id,
			_ => panic!("Expected EmergencyActivated event"),
		};

		// Step 2: Register a conflict of interest related to the emergency
		let emergency_id_str = format!("{:?}", emergency_id);
		let relates_to: BoundedVec<u8, <Runtime as crate::Config>::MaxDescriptionLength> = emergency_id_str.as_bytes().to_vec().try_into().unwrap();
		let description: BoundedVec<u8, <Runtime as crate::Config>::MaxDescriptionLength> = format!("I have a professional relationship with the affected system vendor (evidence: ipfs://QmConflictOfInterest123)")
			.as_bytes().to_vec().try_into().unwrap();
		let conflict_evidence_hash = sp_core::blake2_256("QmConflictOfInterest123".as_bytes()).into();
		let conflict_type = ConflictType::ProfessionalCritical;

		assert_ok!(AmbassadorGovernance::<Runtime>::set_conflict_of_interest(
			RuntimeOrigin::signed(committee_member.clone()),
			conflict_type.clone(),
			description.clone(),
			Some(relates_to.clone()),
			None, // start_block (defaults to current)
			None, // end_block
			Some(conflict_evidence_hash),
			None, // nonce
		));

		// Verify the conflict of interest was registered correctly
		let nonce = match System::events().last().unwrap().event {
			RuntimeEvent::AmbassadorGovernance(
				Event::ConflictOfInterestSet { nonce, .. }
			) => nonce,
			_ => panic!("Expected ConflictOfInterestSet event"),
		};

		// Verify the conflict of interest details in storage
		let conflict = Conflicts::<Runtime>::get((committee_member.clone(), conflict_type.clone(), nonce.unwrap_or(0))).unwrap();
		assert_eq!(conflict.member, committee_member);
		assert_eq!(conflict.conflict_type, conflict_type.clone());
		assert_eq!(conflict.description, description);
		assert_eq!(conflict.relates_to, Some(relates_to));
		assert_eq!(conflict.evidence_hash, Some(conflict_evidence_hash));

		// Step 3: Attempt to form an emergency committee with the conflicted member
		let expert = 3;
		assign_ambassador_rank(expert.clone(), 3);

		let members = vec![
			(initiator.clone(), EmergencyRole::TechnicalLead),
			(committee_member.clone(), EmergencyRole::GovernanceRepresentative), // Member with a conflict of interest
			(expert.clone(), EmergencyRole::IndependentExpert),
		];
		let bounded_members = BoundedVec::<_, _>::try_from(members).unwrap();

		assert_noop!(
			AmbassadorGovernance::<Runtime>::form_emergency_committee(
				RuntimeOrigin::signed(initiator.clone()),
				emergency_id,
				bounded_members
			),
			Error::<Runtime>::ConflictOfInterestDetected
		);
	});
}

#[test]
fn register_disciplinary_action_works() {
	new_test_ext().execute_with(|| {
		// Setup
		let issuer = 4; // An account ID with sufficient rank in MockRankChecker
		let subject = 2; // Any account ID rank

		// Assign sufficient rank to issuer according to MinRankForDisciplinaryActionEnforcement
		assign_ambassador_rank(issuer, 4);

		// Prepare disciplinary action parameters
		let level = DisciplineLevel::Formal;
		let reason: BoundedVec<u8, <Runtime as crate::Config>::MaxJustificationLength> = BoundedVec::try_from(b"Violation of code of conduct. Details at: ipfs://QmDisciplinaryEvidence123".to_vec()).unwrap();
		let duration = Some(100u64);
		let evidence_hash = Some(sp_core::blake2_256("QmDisciplinaryEvidence123".as_bytes()).into());

		// Register disciplinary action
		assert_ok!(AmbassadorGovernance::<Runtime>::register_disciplinary_action(
			RuntimeOrigin::signed(issuer.clone()),
			subject.clone(),
			level.clone(),
			reason.clone(),
			duration,
			evidence_hash,
		));

		// Verify the disciplinary action was registered correctly
		let discipline_id = match System::events().last().unwrap().event {
			RuntimeEvent::AmbassadorGovernance(
				Event::DisciplinaryActionRegistered { discipline_id, .. }
			) => discipline_id,
			_ => panic!("Expected DisciplinaryActionRegistered event"),
		};

		// Verify the disciplinary action details in storage
		let discipline = Disciplines::<Runtime>::get(discipline_id).unwrap();
		assert_eq!(discipline.subject, subject);
		assert_eq!(discipline.issuer, issuer);
		assert_eq!(discipline.level, level);
		assert_eq!(discipline.reason, reason);
		assert_eq!(discipline.duration, duration);
		assert_eq!(discipline.evidence_hash, evidence_hash);
		assert_eq!(discipline.active, true); // Should be active when first registered
	});
}

#[test]
fn register_disciplinary_action_fails_with_insufficient_rank() {
	new_test_ext().execute_with(|| {
		// Setup
		let low_rank_issuer = 1;
		let subject = 2;

		// Assign insufficient rank to issuer
		assign_ambassador_rank(low_rank_issuer.clone(), 1);

		// Prepare disciplinary action parameters
		let level = DisciplineLevel::Formal;
		let reason: BoundedVec<u8, <Runtime as crate::Config>::MaxJustificationLength> = BoundedVec::try_from(b"Violation of code of conduct. Details at: ipfs://QmDisciplinaryEvidence123".to_vec()).unwrap();
		let duration = Some(100u64);
		let evidence_hash = Some(sp_core::blake2_256("QmDisciplinaryEvidence123".as_bytes()).into());

		// Attempt to register disciplinary action should fail
		assert_noop!(
			AmbassadorGovernance::<Runtime>::register_disciplinary_action(
				RuntimeOrigin::signed(low_rank_issuer),
				subject,
				level.clone(),
				reason.clone(),
				duration,
				evidence_hash
			),
			Error::<Runtime>::InsufficientRank
		);
	});
}

#[test]
fn resolve_disciplinary_action_works() {
	new_test_ext().execute_with(|| {
		// Setup
		let issuer = 4; // An account ID with sufficient rank in MockRankChecker
		let subject = 2; // Any account ID rank
		let resolver = 4; // An account ID with sufficient rank in MockRankChecker

		// Assign ranks
		assign_ambassador_rank(issuer, 4); // Sufficient for issuing
		assign_ambassador_rank(resolver, 4);

		// Prepare disciplinary action parameters
		let level = DisciplineLevel::Formal;
		let reason: BoundedVec<u8, <Runtime as crate::Config>::MaxJustificationLength> = BoundedVec::try_from(b"Violation of code of conduct. Details at: ipfs://QmDisciplinaryEvidence123".to_vec()).unwrap();
		let duration = Some(100u64);
		let evidence_hash = Some(sp_core::blake2_256("QmDisciplinaryEvidence123".as_bytes()).into());

		// Register disciplinary action
		assert_ok!(AmbassadorGovernance::<Runtime>::register_disciplinary_action(
			RuntimeOrigin::signed(issuer),
			subject.clone(),
			level.clone(),
			reason.clone(),
			duration,
			evidence_hash,
		));

		// Get the discipline_id from events
		let discipline_id = match System::events().last().unwrap().event {
			RuntimeEvent::AmbassadorGovernance(
				Event::DisciplinaryActionRegistered { discipline_id, .. }
			) => discipline_id,
			_ => panic!("Expected DisciplinaryActionRegistered event"),
		};

		// Prepare resolution parameters
		let resolution_summary = BoundedVec::try_from(b"Issue resolved through mediation. Details at: ipfs://QmResolutionEvidence456".to_vec()).unwrap();
		let resolution_evidence_hash = Some(sp_core::blake2_256("QmResolutionEvidence456".as_bytes()).into());

		// Resolve the disciplinary action
		assert_ok!(AmbassadorGovernance::<Runtime>::resolve_disciplinary_action(
			RuntimeOrigin::signed(resolver.clone()),
			discipline_id,
			resolution_summary.clone(),
			resolution_evidence_hash,
		));

		// Verify the resolution event was emitted
		let last_event = System::events().last().unwrap().event.clone();
		match last_event {
			RuntimeEvent::AmbassadorGovernance(
				Event::DisciplinaryActionResolved {
					discipline_id: resolved_id,
					subject: resolved_subject,
					resolution_summary: resolved_summary,
					evidence_hash: resolved_evidence_hash
				}
			) => {
				assert_eq!(resolved_id, discipline_id);
				assert_eq!(resolved_subject, subject);
				assert_eq!(resolved_summary, resolution_summary);
				assert_eq!(resolved_evidence_hash, resolution_evidence_hash);
			},
			_ => panic!("Expected DisciplinaryActionResolved event"),
		}

		// Verify the disciplinary action is now inactive
		let updated_discipline = Disciplines::<Runtime>::get(discipline_id).unwrap();
		assert_eq!(updated_discipline.active, false);
	});
}

#[test]
fn resolve_disciplinary_action_fails_for_nonexistent_action() {
	new_test_ext().execute_with(|| {
		// Setup
		let resolver = 4; // An account ID with sufficient rank in MockRankChecker

		// Assign sufficient rank
		assign_ambassador_rank(resolver, 4);

		// Create a random discipline_id that doesn't exist
		let nonexistent_discipline_id = H256::random();

		// Prepare resolution parameters
		let resolution_summary: BoundedVec<u8, <Runtime as crate::Config>::MaxResolutionLength> = BoundedVec::try_from(b"Issue resolved through mediation. Details at: ipfs://QmResolutionEvidence456".to_vec()).unwrap();
		let resolution_evidence_hash = Some(sp_core::blake2_256("QmResolutionEvidence456".as_bytes()).into());

		// Attempt to resolve a nonexistent disciplinary action should fail
		assert_noop!(
			AmbassadorGovernance::<Runtime>::resolve_disciplinary_action(
				RuntimeOrigin::signed(resolver.clone()),
				nonexistent_discipline_id,
				resolution_summary.clone(),
				resolution_evidence_hash
			),
			Error::<Runtime>::DisciplinaryActionNotFound
		);
	});
}

#[test]
fn resolve_disciplinary_action_fails_for_already_resolved_action() {
	new_test_ext().execute_with(|| {
		// Setup
		let issuer = 4; // An account ID with sufficient rank in MockRankChecker
		let resolver = 4; // An account ID with sufficient rank in MockRankChecker
		let subject = 2; // Any rank

		// Assign ranks
		assign_ambassador_rank(issuer, 4); // Sufficient for issuing
		assign_ambassador_rank(resolver, 4);

		// Register disciplinary action
		let level = DisciplineLevel::Formal;
		let reason: BoundedVec<u8, <Runtime as crate::Config>::MaxJustificationLength> = BoundedVec::try_from(b"Violation of code of conduct. Details at: ipfs://QmDisciplinaryEvidence123".to_vec()).unwrap();
		let duration = Some(100u64);
		let evidence_hash = Some(sp_core::blake2_256("QmDisciplinaryEvidence123".as_bytes()).into());

		assert_ok!(AmbassadorGovernance::<Runtime>::register_disciplinary_action(
			RuntimeOrigin::signed(issuer),
			subject,
			level.clone(),
			reason.clone(),
			duration,
			evidence_hash,
		));

		// Get the discipline_id from events
		let discipline_id = match System::events().last().unwrap().event {
			RuntimeEvent::AmbassadorGovernance(
				Event::DisciplinaryActionRegistered { discipline_id, .. }
			) => discipline_id,
			_ => panic!("Expected DisciplinaryActionRegistered event"),
		};

		// Resolve the disciplinary action
		let resolution_summary = BoundedVec::try_from(b"Issue resolved through mediation. Details at: ipfs://QmResolutionEvidence456".to_vec()).unwrap();
		let resolution_evidence_hash = Some(sp_core::blake2_256("QmResolutionEvidence456".as_bytes()).into());

		assert_ok!(AmbassadorGovernance::<Runtime>::resolve_disciplinary_action(
			RuntimeOrigin::signed(resolver.clone()),
			discipline_id,
			resolution_summary.clone(),
			resolution_evidence_hash,
		));

		// Attempt to resolve the same disciplinary action again should fail
		assert_noop!(
			AmbassadorGovernance::<Runtime>::resolve_disciplinary_action(
				RuntimeOrigin::signed(resolver.clone()),
				discipline_id,
				resolution_summary.clone(),
				resolution_evidence_hash
			),
			Error::<Runtime>::DisciplinaryActionAlreadyResolved
		);
	});
}

#[test]
fn resolve_disciplinary_action_fails_with_insufficient_rank() {
	new_test_ext().execute_with(|| {
		// Setup
		let issuer = 4; // An account ID with sufficient rank in MockRankChecker
		let low_rank_resolver = 1; // An account ID with only rank 1 (insufficient) from MockRankChecker
		let subject = 2; // Any account ID rank

		// Assign ranks
		assign_ambassador_rank(issuer, 4); // Sufficient for issuing
		assign_ambassador_rank(low_rank_resolver, 1); // Insufficient for resolving

		// Register disciplinary action
		let level = DisciplineLevel::Formal;
		let reason: BoundedVec<u8, <Runtime as crate::Config>::MaxJustificationLength> = BoundedVec::try_from(b"Violation of code of conduct. Details at: ipfs://QmDisciplinaryEvidence123".to_vec()).unwrap();
		let duration = Some(100u64);
		let evidence_hash = Some(sp_core::blake2_256("QmDisciplinaryEvidence123".as_bytes()).into());

		assert_ok!(AmbassadorGovernance::<Runtime>::register_disciplinary_action(
			RuntimeOrigin::signed(issuer),
			subject,
			level.clone(),
			reason.clone(),
			duration,
			evidence_hash,
		));

		// Get the discipline_id from events
		let discipline_id = match System::events().last().unwrap().event {
			RuntimeEvent::AmbassadorGovernance(
				Event::DisciplinaryActionRegistered { discipline_id, .. }
			) => discipline_id,
			_ => panic!("Expected DisciplinaryActionRegistered event"),
		};

		// Prepare resolution parameters
		let resolution_summary = BoundedVec::try_from(b"Issue resolved through mediation. Details at: ipfs://QmResolutionEvidence456".to_vec()).unwrap();
		let resolution_evidence_hash = Some(sp_core::blake2_256("QmResolutionEvidence456".as_bytes()).into());

		// Attempt to resolve with insufficient rank should fail
		assert_noop!(
			AmbassadorGovernance::<Runtime>::resolve_disciplinary_action(
				RuntimeOrigin::signed(low_rank_resolver.clone()),
				discipline_id,
				resolution_summary.clone(),
				resolution_evidence_hash
			),
			Error::<Runtime>::InsufficientRank
		);
	});
}

#[test]
fn form_appeal_committee_fails_with_conflict_of_interest() {
	new_test_ext().execute_with(|| {
		// Setup
		let appellant = 2;
		let committee_member = 3;
		let committee_organizer = 4;

		// Assign ranks (identities are automatically verified for accounts 1-10)
		assign_ambassador_rank(appellant, 2);
		assign_ambassador_rank(committee_member, 3);
		assign_ambassador_rank(committee_organizer, 4);

		// Step 1: Submit an appeal
		let original_decision = BoundedVec::try_from(b"Original decision that is being appealed. Evidence at: ipfs://QmOriginalDecision123".to_vec()).unwrap();
		let justification = BoundedVec::try_from(b"Appeal justification with evidence. Evidence at: ipfs://QmAppealEvidence456".to_vec()).unwrap();

		assert_ok!(AmbassadorGovernance::<Runtime>::submit_appeal(
			RuntimeOrigin::signed(appellant),
			original_decision.clone(),
			justification.clone(),
			None
		));

		// Get the appeal_id from events
		let appeal_id = match System::events().last().unwrap().event {
			RuntimeEvent::AmbassadorGovernance(
				Event::AppealSubmitted { appeal_id, .. }
			) => appeal_id,
			_ => panic!("Expected AppealSubmitted event"),
		};

		// Step 2: Register a conflict of interest related to the appeal
		let description = BoundedVec::try_from(
			b"Professional relationship with appellant. Evidence at: ipfs://QmConflictOfInterest123".to_vec()
		).unwrap();

		let relates_to = format!("{:?}", appeal_id);
		let relates_to_bounded = BoundedVec::try_from(relates_to.as_bytes().to_vec()).unwrap();
		let conflict_evidence_hash = sp_core::blake2_256("QmConflictOfInterest123".as_bytes()).into();

		assert_ok!(AmbassadorGovernance::<Runtime>::set_conflict_of_interest(
			RuntimeOrigin::signed(committee_member),
			ConflictType::ProfessionalCritical,
			description.clone(),
			Some(relates_to_bounded.clone()),
			None, // start_block (defaults to current)
			None, // end_block
			Some(conflict_evidence_hash),
			None, // nonce
		));

		// Verify the conflict of interest was registered correctly
		let nonce = match System::events().last().unwrap().event {
			RuntimeEvent::AmbassadorGovernance(
				Event::ConflictOfInterestSet { nonce, .. }
			) => nonce,
			_ => panic!("Expected ConflictOfInterestSet event"),
		};

		// Verify the conflict of interest details in storage
		let conflict = Conflicts::<Runtime>::get((committee_member, ConflictType::ProfessionalCritical, nonce.unwrap_or(0))).unwrap();
		assert_eq!(conflict.member, committee_member);
		assert_eq!(conflict.conflict_type, ConflictType::ProfessionalCritical);
		assert_eq!(conflict.description, description);
		assert_eq!(conflict.relates_to, Some(relates_to_bounded));
		assert_eq!(conflict.evidence_hash, Some(conflict_evidence_hash));

		// Step 3: Attempt to form an appeal committee with the conflicted member
		let members = vec![
				committee_member, // Member with a conflict of interest
				5, // Use direct account IDs
				6,
		];
		let bounded_members = BoundedVec::<_, _>::try_from(members).unwrap();

		assert_noop!(
			AmbassadorGovernance::<Runtime>::form_appeal_committee(
				RuntimeOrigin::signed(committee_organizer),
				appeal_id,
				bounded_members
			),
			Error::<Runtime>::ConflictOfInterestDetected
		);
	});
}

#[test]
fn form_appeal_committee_works_with_non_critical_conflict() {
	new_test_ext().execute_with(|| {
		// Setup
		let appellant = 2;
		let committee_member = 3;
		let committee_organizer = 4;

		// Assign ranks (identities are automatically verified for accounts 1-10)
		assign_ambassador_rank(appellant, 2);
		assign_ambassador_rank(committee_member, 3);
		assign_ambassador_rank(committee_organizer, 4);

		// Step 1: Submit an appeal
		let original_decision = BoundedVec::try_from(b"Original decision that is being appealed. Evidence at: ipfs://QmOriginalDecision123".to_vec()).unwrap();
		let justification = create_justification(100);

		assert_ok!(AmbassadorGovernance::<Runtime>::submit_appeal(
			RuntimeOrigin::signed(appellant),
			original_decision.clone(),
			justification.clone(),
			None
		));

		// Get the appeal_id from events
		let appeal_id = match System::events().last().unwrap().event {
			RuntimeEvent::AmbassadorGovernance(
				Event::AppealSubmitted { appeal_id, .. }
			) => appeal_id,
			_ => panic!("Expected AppealSubmitted event"),
		};

		// Step 2: Register a NON-CRITICAL conflict of interest related to the appeal
		let description = BoundedVec::try_from(
			b"Personal acquaintance with appellant. Evidence at: ipfs://QmConflictOfInterest456".to_vec()
		).unwrap();

		let relates_to = format!("{:?}", appeal_id);
		let relates_to_bounded = BoundedVec::try_from(relates_to.as_bytes().to_vec()).unwrap();
		let conflict_evidence_hash = sp_core::blake2_256("QmConflictOfInterest456".as_bytes()).into();

		assert_ok!(AmbassadorGovernance::<Runtime>::set_conflict_of_interest(
			RuntimeOrigin::signed(committee_member),
			ConflictType::PersonalNonCritical, // Non-critical conflict
			description.clone(),
			Some(relates_to_bounded.clone()),
			None, // start_block (defaults to current)
			None, // end_block
			Some(conflict_evidence_hash),
			None, // nonce
		));

		// Step 3: Form an appeal committee with the member who has a non-critical conflict
		let members_with_ranks: Vec<(u64, u32)> = vec![
			(committee_member, 3), // Member with a NON-CRITICAL conflict of interest
			(account("member2", 3, 3), 3),
			(account("member3", 3, 3), 3),
		];

		// Set up identities for other members
		setup_identity_for_account(members_with_ranks[1].0.clone());
		setup_identity_for_account(members_with_ranks[2].0.clone());

		// Assign ranks to other members
		assign_ambassador_rank(members_with_ranks[1].0.clone(), 3);
		assign_ambassador_rank(members_with_ranks[2].0.clone(), 3);

		// Extract just the account IDs for the committee
		let members: Vec<u64> = members_with_ranks.iter().map(|(account_id, _)| account_id.clone()).collect();
		let bounded_members = BoundedVec::<_, _>::try_from(members).unwrap();

		// Committee formation should succeed but emit a warning event
		assert_ok!(
			AmbassadorGovernance::<Runtime>::form_appeal_committee(
				RuntimeOrigin::signed(committee_organizer),
				appeal_id,
				bounded_members.clone()
			)
		);

		// Verify the committee was formed
		assert!(AppealCommittees::<Runtime>::contains_key(appeal_id));

		// Verify the warning event was emitted
		let warning_event_emitted = System::events().iter().any(|record| {
			matches!(
				record.event,
				RuntimeEvent::AmbassadorGovernance(
					Event::NonCriticalAppealConflictWarning { .. }
				)
			)
		});

		assert!(warning_event_emitted, "NonCriticalAppealConflictWarning event should have been emitted");
	});
}
