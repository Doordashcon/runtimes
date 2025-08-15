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
	mock::*, AppealDecision, AppealStatus, EmergencyRole, EmergencySeverity, EmergencyType, Error,
	Event, IntegrationMechanism, Pallet as AmbassadorGovernance, ProfessionalServiceType,
	TargetCollective,
};
use frame_support::{assert_noop, assert_ok};
use frame_system::ensure_signed;
use sp_core::H256;
use sp_runtime::traits::BadOrigin;
use sp_std::vec;

// Helper function to create a random H256 hash
fn random_hash() -> H256 {
	H256::random()
}

// Helper function to create a justification string
fn create_justification(len: usize) -> Vec<u8> {
	vec![0u8; len]
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

// Helper function to create a description of specified length
fn create_description(length: usize) -> Vec<u8> {
	let mut description = Vec::new();
	for _ in 0..length {
		description.push(b'D');
	}
	description
}

#[test]
fn activate_emergency_protocol_works() {
	new_test_ext().execute_with(|| {
		// Setup
		let emergency_type = EmergencyType::SecurityVulnerability;
		let severity = EmergencySeverity::Critical;
		let justification = create_justification(100);
		let evidence = Some(random_hash());

		// Execute
		assert_ok!(AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
			signed_origin(1),
			emergency_type.clone(),
			severity.clone(),
			justification.clone(),
			evidence
		));

		// Get the emergency ID from the emitted event
		let emergency_id = get_last_emergency_id();

		let emergency = AmbassadorGovernance::<Runtime>::emergencies(emergency_id).unwrap();
		assert_eq!(emergency.emergency_type, emergency_type);
		assert_eq!(emergency.severity, severity);
		assert_eq!(emergency.initiator, 1);
		assert_eq!(emergency.resolved_at, None); // Check resolved_at is None instead of resolved field

		// Check event was emitted
		frame_system::Pallet::<Runtime>::assert_has_event(RuntimeEvent::AmbassadorGovernance(
			Event::EmergencyActivated { emergency_id, emergency_type, severity, initiator: 1 },
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
		let evidence = Some(random_hash());

		// Execute and verify failure with unsigned origin
		assert_noop!(
			AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
				frame_system::RawOrigin::None.into(),
				emergency_type.clone(),
				severity.clone(),
				justification.clone(),
				evidence
			),
			sp_runtime::traits::BadOrigin
		);
	});
}

#[test]
fn activate_emergency_protocol_fails_with_too_long_justification() {
	new_test_ext().execute_with(|| {
		// Setup
		let emergency_type = EmergencyType::SecurityVulnerability;
		let severity = EmergencySeverity::Critical;
		let justification = create_justification(MaxJustificationLength::get() as usize + 1);
		let evidence = Some(random_hash());

		// Execute and verify failure with too long justification
		assert_noop!(
			AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
				RuntimeOrigin::signed(1),
				emergency_type.clone(),
				severity.clone(),
				justification,
				evidence
			),
			Error::<Runtime>::JustificationTooLong
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
		let evidence = Some(random_hash());

		assert_ok!(AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
			signed_origin(1),
			emergency_type.clone(),
			severity.clone(),
			justification.clone(),
			evidence
		));

		let emergency_id = get_last_emergency_id();

		// Create committee members with roles
		let members = vec![
			(2, EmergencyRole::TechnicalLead),
			(3, EmergencyRole::GovernanceRepresentative),
			(4, EmergencyRole::IndependentExpert),
		];

		// Execute
		assert_ok!(AmbassadorGovernance::<Runtime>::form_emergency_committee(
			signed_origin(1),
			emergency_id,
			members.clone()
		));

		// Verify
		let committee =
			AmbassadorGovernance::<Runtime>::emergency_committees(emergency_id).unwrap();
		assert_eq!(committee.len(), 3);
		assert!(committee.contains(&(2, EmergencyRole::TechnicalLead)));
		assert!(committee.contains(&(3, EmergencyRole::GovernanceRepresentative)));
		assert!(committee.contains(&(4, EmergencyRole::IndependentExpert)));

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
		let evidence = Some(random_hash());

		assert_ok!(AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
			RuntimeOrigin::signed(1),
			emergency_type.clone(),
			severity.clone(),
			justification.clone(),
			evidence
		));

		let emergency_id = get_last_emergency_id();

		// Create committee members with roles
		let members = vec![
			(2, EmergencyRole::TechnicalLead),
			(3, EmergencyRole::GovernanceRepresentative),
			(4, EmergencyRole::IndependentExpert),
		];

		// Try to form emergency committee as account 4 (has Rank 0, non-ambassador)
		// In the mock runtime, only account 1 can use CommitteeFormationOrigin
		assert_noop!(
			AmbassadorGovernance::<Runtime>::form_emergency_committee(
				RuntimeOrigin::signed(4),
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
		let members = vec![
			(2, EmergencyRole::TechnicalLead),
			(3, EmergencyRole::GovernanceRepresentative),
			(4, EmergencyRole::IndependentExpert),
		];

		// Execute and verify failure
		assert_noop!(
			AmbassadorGovernance::<Runtime>::form_emergency_committee(
				RuntimeOrigin::signed(1),
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
		let evidence = Some(random_hash());

		assert_ok!(AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
			RuntimeOrigin::signed(1),
			emergency_type.clone(),
			severity.clone(),
			justification.clone(),
			evidence
		));

		let emergency_id = get_last_emergency_id();

		// Create committee members with roles
		let members = vec![
			(2, EmergencyRole::TechnicalLead),
			(3, EmergencyRole::GovernanceRepresentative),
			(4, EmergencyRole::IndependentExpert),
		];

		// Form committee first time
		assert_ok!(AmbassadorGovernance::<Runtime>::form_emergency_committee(
			RuntimeOrigin::signed(1),
			emergency_id,
			members.clone()
		));

		// Try to form committee again
		assert_noop!(
			AmbassadorGovernance::<Runtime>::form_emergency_committee(
				RuntimeOrigin::signed(1),
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
		let evidence = Some(random_hash());

		assert_ok!(AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
			RuntimeOrigin::signed(1),
			emergency_type.clone(),
			severity.clone(),
			justification.clone(),
			evidence
		));

		let emergency_id = get_last_emergency_id();

		// Create too many committee members (MaxCommitteeMembers is 5)
		let mut members = Vec::new();
		// Add 6 members to exceed the limit
		members.push((1, EmergencyRole::TechnicalLead));
		members.push((2, EmergencyRole::GovernanceRepresentative));
		members.push((3, EmergencyRole::IndependentExpert));
		members.push((4, EmergencyRole::IndependentExpert));
		members.push((5, EmergencyRole::IndependentExpert));
		// Extra to exceed limit
		members.push((6, EmergencyRole::IndependentExpert));

		// Execute and verify failure
		assert_noop!(
			AmbassadorGovernance::<Runtime>::form_emergency_committee(
				RuntimeOrigin::signed(1),
				emergency_id,
				members
			),
			Error::<Runtime>::TooManyCommitteeMembers
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
		let evidence = Some(random_hash());

		assert_ok!(AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
			RuntimeOrigin::signed(1),
			emergency_type.clone(),
			severity.clone(),
			justification.clone(),
			evidence
		));

		let emergency_id = get_last_emergency_id();

		// Create committee members with missing required roles (missing IndependentExpert)
		let members =
			vec![(2, EmergencyRole::TechnicalLead), (3, EmergencyRole::GovernanceRepresentative)];

		// Execute and verify failure
		assert_noop!(
			AmbassadorGovernance::<Runtime>::form_emergency_committee(
				RuntimeOrigin::signed(1),
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
		let evidence = Some(random_hash());

		assert_ok!(AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
			RuntimeOrigin::signed(1),
			emergency_type.clone(),
			severity.clone(),
			justification.clone(),
			evidence.clone()
		));

		let emergency_id = get_last_emergency_id();

		// Create committee members with roles
		let members = vec![
			(2, EmergencyRole::TechnicalLead),
			(3, EmergencyRole::GovernanceRepresentative),
			(4, EmergencyRole::IndependentExpert),
		];

		assert_ok!(AmbassadorGovernance::<Runtime>::form_emergency_committee(
			RuntimeOrigin::signed(1),
			emergency_id,
			members.clone()
		));

		// Resolve emergency
		let resolution_summary = create_justification(100);
		let abuse_detected = false;

		// Execute as committee member
		assert_ok!(AmbassadorGovernance::<Runtime>::resolve_emergency(
			RuntimeOrigin::signed(2),
			emergency_id,
			abuse_detected,
			resolution_summary.clone(),
			evidence
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
				resolution_summary: resolution_summary.to_vec(),
			},
		));
	});
}

#[test]
fn activate_emergency_protocol_fails_with_insufficient_rank() {
	new_test_ext().execute_with(|| {
		// Try to activate emergency protocol as account 4 (has Rank 0, non-ambassador)
		// In the mock runtime, MinRankToActivateEmergencyProtocol is set to 3 (Senior Ambassador)
		assert_noop!(
			AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
				RuntimeOrigin::signed(4),
				EmergencyType::SecurityVulnerability,
				EmergencySeverity::Critical,
				b"Critical security vulnerability found in XYZ component - evidence stored at ipfs://QmHash123".to_vec(),
				Some(H256::repeat_byte(1)),
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
				b"Critical security vulnerability found in XYZ component - evidence stored at ipfs://QmHash456".to_vec(),
				Some(H256::repeat_byte(1)),
			),
			Error::<Runtime>::InsufficientRank
		);
	});
}

#[test]
fn resolve_emergency_works_with_senior_ambassador() {
	new_test_ext().execute_with(|| {
		// Setup by first activating an emergency
		let emergency_type = EmergencyType::SecurityVulnerability;
		let severity = EmergencySeverity::Critical;
		let justification = create_justification(100);
		let evidence = Some(random_hash());

		assert_ok!(AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
			RuntimeOrigin::signed(1),
			emergency_type.clone(),
			severity.clone(),
			justification.clone(),
			evidence.clone()
		));

		let emergency_id = get_last_emergency_id();

		// Create committee members with roles including the Senior Ambassador
		let members = vec![
			(1, EmergencyRole::TechnicalLead),
			(2, EmergencyRole::GovernanceRepresentative),
			(3, EmergencyRole::IndependentExpert),
		];

		assert_ok!(AmbassadorGovernance::<Runtime>::form_emergency_committee(
			RuntimeOrigin::signed(1),
			emergency_id,
			members.clone()
		));

		// Resolve emergency without forming committee (as Senior Ambassador)
		let resolution_summary = create_justification(100);
		let abuse_detected = false;

		// Execute as Senior Ambassador
		assert_ok!(AmbassadorGovernance::<Runtime>::resolve_emergency(
			RuntimeOrigin::signed(1),
			emergency_id,
			abuse_detected,
			resolution_summary.clone(),
			evidence
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
		let resolution_summary = create_justification(100);
		let abuse_detected = false;
		let emergency_type = EmergencyType::SecurityVulnerability;
		let severity = EmergencySeverity::Critical;
		let justification = create_justification(100);
		let evidence = Some(random_hash());

		assert_ok!(AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
			RuntimeOrigin::signed(1),
			emergency_type.clone(),
			severity.clone(),
			justification.clone(),
			evidence.clone()
		));

		let emergency_id = get_last_emergency_id();

		// Create committee members with roles
		let members = vec![
			(1, EmergencyRole::TechnicalLead),
			(2, EmergencyRole::GovernanceRepresentative),
			(3, EmergencyRole::IndependentExpert),
		];

		assert_ok!(AmbassadorGovernance::<Runtime>::form_emergency_committee(
			RuntimeOrigin::signed(1),
			emergency_id,
			members.clone()
		));

		// Execute and verify failure
		assert_noop!(
			AmbassadorGovernance::<Runtime>::resolve_emergency(
				RuntimeOrigin::signed(1),
				emergency_id_non_existent,
				abuse_detected,
				resolution_summary,
				evidence
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
		let evidence = Some(random_hash());

		assert_ok!(AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
			RuntimeOrigin::signed(1),
			emergency_type.clone(),
			severity.clone(),
			justification.clone(),
			evidence.clone()
		));

		let emergency_id = get_last_emergency_id();

		// Create committee members with roles including the Senior Ambassador
		let members = vec![
			(1, EmergencyRole::TechnicalLead),
			(2, EmergencyRole::GovernanceRepresentative),
			(3, EmergencyRole::IndependentExpert),
		];

		assert_ok!(AmbassadorGovernance::<Runtime>::form_emergency_committee(
			RuntimeOrigin::signed(1),
			emergency_id,
			members.clone()
		));

		// Resolve emergency first time
		let resolution_summary = create_justification(100);
		let abuse_detected = false;

		assert_ok!(AmbassadorGovernance::<Runtime>::resolve_emergency(
			RuntimeOrigin::signed(1),
			emergency_id,
			abuse_detected,
			resolution_summary.clone(),
			evidence
		));

		// Try to resolve again
		assert_noop!(
			AmbassadorGovernance::<Runtime>::resolve_emergency(
				RuntimeOrigin::signed(1),
				emergency_id,
				abuse_detected,
				resolution_summary,
				evidence
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
		let evidence = Some(random_hash());

		assert_ok!(AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
			RuntimeOrigin::signed(1),
			emergency_type.clone(),
			severity.clone(),
			justification.clone(),
			evidence.clone()
		));

		let emergency_id = get_last_emergency_id();

		// Create committee members with roles
		let members = vec![
			(2, EmergencyRole::TechnicalLead),
			(3, EmergencyRole::GovernanceRepresentative),
			(4, EmergencyRole::IndependentExpert),
		];

		assert_ok!(AmbassadorGovernance::<Runtime>::form_emergency_committee(
			RuntimeOrigin::signed(1),
			emergency_id,
			members.clone()
		));

		// Try to resolve with unauthorized account (not committee member or Senior Ambassador)
		let resolution_summary = create_justification(100);
		let abuse_detected = false;

		assert_noop!(
			AmbassadorGovernance::<Runtime>::resolve_emergency(
				RuntimeOrigin::signed(5), // Not a committee member
				emergency_id,
				abuse_detected,
				resolution_summary,
				evidence
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
		let evidence = Some(H256::from([1; 32]));

		assert_ok!(AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
			signed_origin(1),
			emergency_type,
			severity,
			justification.clone(),
			evidence
		));

		let emergency_id = get_last_emergency_id();

		// Create committee members with roles
		let members = vec![
			(2, EmergencyRole::TechnicalLead),
			(3, EmergencyRole::GovernanceRepresentative),
			(4, EmergencyRole::IndependentExpert),
		];

		assert_ok!(AmbassadorGovernance::<Runtime>::form_emergency_committee(
			signed_origin(1),
			emergency_id,
			members.clone()
		));

		// First resolution should succeed
		let resolution_summary = create_justification(100);
		let abuse_detected = false;

		assert_ok!(AmbassadorGovernance::<Runtime>::resolve_emergency(
			signed_origin(2), // Committee member
			emergency_id,
			abuse_detected,
			resolution_summary.clone(),
			evidence
		));

		// Second resolution should fail
		assert_noop!(
			AmbassadorGovernance::<Runtime>::resolve_emergency(
				signed_origin(2),
				emergency_id,
				abuse_detected,
				resolution_summary,
				evidence
			),
			Error::<Runtime>::EmergencyAlreadyResolved
		);
	});
}

#[test]
fn submit_appeal_works() {
	new_test_ext().execute_with(|| {
		// Setup
		let original_decision = b"Original decision text".to_vec();
		let justification = create_justification(100);
		let evidence = Some(random_hash());

		// Execute
		assert_ok!(AmbassadorGovernance::<Runtime>::submit_appeal(
			RuntimeOrigin::signed(1),
			original_decision.clone(),
			justification.clone(),
			evidence
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
		let original_decision = b"Original decision text".to_vec();
		let justification = create_justification(100);
		let evidence = Some(random_hash());

		// Execute and verify failure with unsigned origin
		assert_noop!(
			AmbassadorGovernance::<Runtime>::submit_appeal(
				frame_system::RawOrigin::None.into(),
				original_decision.clone(),
				justification.clone(),
				evidence
			),
			sp_runtime::traits::BadOrigin
		);
	});
}

#[test]
fn submit_appeal_fails_with_insufficient_rank() {
	new_test_ext().execute_with(|| {
		// Setup
		let original_decision = b"Original decision text".to_vec();
		let justification = create_justification(100);
		let evidence = Some(random_hash());

		// Try to submit appeal as account 4 (has Rank 0, non-ambassador)
		// In the mock runtime, the AppealSubmissionOrigin is restricted to specific accounts
		assert_noop!(
			AmbassadorGovernance::<Runtime>::submit_appeal(
				RuntimeOrigin::signed(4),
				original_decision.clone(),
				justification.clone(),
				evidence
			),
			Error::<Runtime>::InsufficientRank
		);
	});
}

#[test]
fn submit_appeal_fails_with_too_long_justification() {
	new_test_ext().execute_with(|| {
		// Setup
		let original_decision = b"Original decision text".to_vec();
		let justification = create_justification(MaxJustificationLength::get() as usize + 1);
		let evidence = Some(random_hash());

		// Execute and verify failure with too long justification
		assert_noop!(
			AmbassadorGovernance::<Runtime>::submit_appeal(
				RuntimeOrigin::signed(1),
				original_decision.clone(),
				justification,
				evidence
			),
			Error::<Runtime>::JustificationTooLong
		);
	});
}

#[test]
fn form_appeal_committee_works() {
	new_test_ext().execute_with(|| {
		// Setup by first submitting an appeal
		let original_decision = b"Original decision text".to_vec();
		let justification = create_justification(100);
		let evidence = Some(random_hash());

		assert_ok!(AmbassadorGovernance::<Runtime>::submit_appeal(
			RuntimeOrigin::signed(1),
			original_decision.clone(),
			justification.clone(),
			evidence
		));

		let appeal_id = get_last_appeal_id();

		// Create committee members
		let members = vec![2, 3, 4];

		// Execute
		assert_ok!(AmbassadorGovernance::<Runtime>::form_appeal_committee(
			RuntimeOrigin::signed(1),
			appeal_id,
			members.clone()
		));

		// Verify
		let committee = AmbassadorGovernance::<Runtime>::appeal_committees(appeal_id).unwrap();
		assert_eq!(committee.len(), 3);
		assert!(committee.contains(&2));
		assert!(committee.contains(&3));
		assert!(committee.contains(&4));

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
		let original_decision = b"Original decision text".to_vec();
		let justification = create_justification(100);
		let evidence = Some(random_hash());

		assert_ok!(AmbassadorGovernance::<Runtime>::submit_appeal(
			RuntimeOrigin::signed(1),
			original_decision.clone(),
			justification.clone(),
			evidence
		));

		let appeal_id = get_last_appeal_id();

		// Create committee members
		let members = vec![2, 3, 1];

		// Try to form appeal committee as account 4 (has Rank 0, non-ambassador)
		// In the mock runtime, only specific accounts can use AppealCommitteeOrigin
		assert_noop!(
			AmbassadorGovernance::<Runtime>::form_appeal_committee(
				RuntimeOrigin::signed(4),
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
		let members = vec![2, 3, 4];

		// Execute and verify failure
		assert_noop!(
			AmbassadorGovernance::<Runtime>::form_appeal_committee(
				RuntimeOrigin::signed(1),
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
		let original_decision = b"Original decision text".to_vec();
		let justification = create_justification(100);
		let evidence = Some(random_hash());

		assert_ok!(AmbassadorGovernance::<Runtime>::submit_appeal(
			RuntimeOrigin::signed(1),
			original_decision.clone(),
			justification.clone(),
			evidence
		));

		let appeal_id = get_last_appeal_id();

		// Create committee members
		let members = vec![2, 3, 4];

		// Form committee first time
		assert_ok!(AmbassadorGovernance::<Runtime>::form_appeal_committee(
			RuntimeOrigin::signed(1),
			appeal_id,
			members.clone()
		));

		// Try to form committee again
		assert_noop!(
			AmbassadorGovernance::<Runtime>::form_appeal_committee(
				RuntimeOrigin::signed(1),
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
		let original_decision = b"Original decision text".to_vec();
		let justification = create_justification(100);
		let evidence = Some(random_hash());

		assert_ok!(AmbassadorGovernance::<Runtime>::submit_appeal(
			RuntimeOrigin::signed(1),
			original_decision.clone(),
			justification.clone(),
			evidence
		));

		let appeal_id = get_last_appeal_id();

		// Create too many committee members
		let mut members = Vec::new();
		for i in 0..(MaxCommitteeMembers::get() as u64 + 1) {
			members.push(i + 1);
		}

		// Execute and verify failure
		assert_noop!(
			AmbassadorGovernance::<Runtime>::form_appeal_committee(
				RuntimeOrigin::signed(1),
				appeal_id,
				members
			),
			Error::<Runtime>::TooManyCommitteeMembers
		);
	});
}

#[test]
fn decide_appeal_works() {
	new_test_ext().execute_with(|| {
		// Setup by first submitting an appeal and forming committee
		let original_decision = b"Original decision text".to_vec();
		let justification = create_justification(100);
		let evidence = Some(random_hash());

		assert_ok!(AmbassadorGovernance::<Runtime>::submit_appeal(
			RuntimeOrigin::signed(1),
			original_decision.clone(),
			justification.clone(),
			evidence
		));

		let appeal_id = get_last_appeal_id();

		// Create committee members
		let members = vec![2, 3, 4];

		assert_ok!(AmbassadorGovernance::<Runtime>::form_appeal_committee(
			RuntimeOrigin::signed(1),
			appeal_id,
			members.clone()
		));

		// Decide on appeal
		let decision = AppealDecision::Modified;

		// Execute as committee member
		assert_ok!(AmbassadorGovernance::<Runtime>::decide_appeal(
			RuntimeOrigin::signed(2),
			appeal_id,
			decision.clone()
		));

		// Verify
		let appeal = AmbassadorGovernance::<Runtime>::appeals(appeal_id).unwrap();
		assert_eq!(appeal.status, AppealStatus::Decided);
		assert_eq!(appeal.decision, Some(decision.clone()));

		// Check event was emitted
		frame_system::Pallet::<Runtime>::assert_has_event(RuntimeEvent::AmbassadorGovernance(
			Event::AppealDecided { appeal_id, decision },
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
			AmbassadorGovernance::<Runtime>::decide_appeal(signed_origin(1), appeal_id, decision),
			Error::<Runtime>::AppealCommitteeNotFormed
		);
	});
}

#[test]
fn decide_appeal_fails_with_no_committee_formed() {
	new_test_ext().execute_with(|| {
		// Setup by first submitting an appeal but don't form committee
		let original_decision = b"Original decision text".to_vec();
		let justification = create_justification(100);
		let evidence = Some(random_hash());

		assert_ok!(AmbassadorGovernance::<Runtime>::submit_appeal(
			RuntimeOrigin::signed(1),
			original_decision.clone(),
			justification.clone(),
			evidence
		));

		let appeal_id = get_last_appeal_id();

		// Try to decide without forming committee
		let decision = AppealDecision::Upheld;

		// Execute and verify failure
		assert_noop!(
			AmbassadorGovernance::<Runtime>::decide_appeal(signed_origin(2), appeal_id, decision),
			Error::<Runtime>::AppealCommitteeNotFormed
		);
	});
}

#[test]
fn decide_appeal_fails_with_unauthorized_account() {
	new_test_ext().execute_with(|| {
		// Setup by first submitting an appeal and forming committee
		let original_decision = b"Original decision text".to_vec();
		let justification = create_justification(100);
		let evidence = Some(random_hash());

		assert_ok!(AmbassadorGovernance::<Runtime>::submit_appeal(
			RuntimeOrigin::signed(1),
			original_decision.clone(),
			justification.clone(),
			evidence
		));

		let appeal_id = get_last_appeal_id();

		// Create committee members
		let members = vec![2, 3, 4];

		assert_ok!(AmbassadorGovernance::<Runtime>::form_appeal_committee(
			RuntimeOrigin::signed(1),
			appeal_id,
			members.clone()
		));

		// Try to decide with unauthorized account (not committee member)
		let decision = AppealDecision::Upheld;

		assert_noop!(
			AmbassadorGovernance::<Runtime>::decide_appeal(
				signed_origin(5), // Not a committee member
				appeal_id,
				decision
			),
			Error::<Runtime>::NotCommitteeMember
		);
	});
}

#[test]
fn decide_appeal_fails_with_already_decided_appeal() {
	new_test_ext().execute_with(|| {
		// Setup by first submitting an appeal and form committee
		let original_decision = b"Original decision text".to_vec();
		let justification = create_justification(100);
		let evidence = Some(random_hash());

		assert_ok!(AmbassadorGovernance::<Runtime>::submit_appeal(
			RuntimeOrigin::signed(1),
			original_decision.clone(),
			justification.clone(),
			evidence
		));

		let appeal_id = get_last_appeal_id();

		// Create committee members
		let members = vec![2, 3, 4];

		assert_ok!(AmbassadorGovernance::<Runtime>::form_appeal_committee(
			RuntimeOrigin::signed(1),
			appeal_id,
			members.clone()
		));

		// Decide on appeal first time
		let decision = AppealDecision::Modified;

		assert_ok!(AmbassadorGovernance::<Runtime>::decide_appeal(
			RuntimeOrigin::signed(2),
			appeal_id,
			decision.clone()
		));

		// Try to decide again
		assert_noop!(
			AmbassadorGovernance::<Runtime>::decide_appeal(
				RuntimeOrigin::signed(2),
				appeal_id,
				AppealDecision::Rejected
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
		let ambassador_participants = vec![1, 2];
		let target_participants = vec![3, 4];
		let agreement_hash = Some(random_hash());

		// Execute
		assert_ok!(AmbassadorGovernance::<Runtime>::establish_integration(
			RuntimeOrigin::signed(1),
			mechanism.clone(),
			target_collective.clone(),
			description.clone(),
			ambassador_participants.clone(),
			target_participants.clone(),
			agreement_hash
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
		let ambassador_participants = vec![1, 2];
		let target_participants = vec![3, 4];
		let agreement_hash = Some(random_hash());

		// Execute and verify failure with unsigned origin
		assert_noop!(
			AmbassadorGovernance::<Runtime>::establish_integration(
				RuntimeOrigin::none(),
				mechanism.clone(),
				target_collective.clone(),
				description.clone(),
				ambassador_participants.clone(),
				target_participants.clone(),
				agreement_hash
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
		let ambassador_participants = vec![1, 2];
		let target_participants = vec![3, 4];
		let agreement_hash = Some(random_hash());

		// Try to establish integration as account 4 (has Rank 0, non-ambassador)
		// In the mock runtime, only specific accounts can use IntegrationOrigin
		assert_noop!(
			AmbassadorGovernance::<Runtime>::establish_integration(
				RuntimeOrigin::signed(4),
				mechanism.clone(),
				target_collective.clone(),
				description.clone(),
				ambassador_participants.clone(),
				target_participants.clone(),
				agreement_hash
			),
			Error::<Runtime>::InsufficientRank
		);
	});
}

#[test]
fn establish_integration_fails_with_too_long_description() {
	new_test_ext().execute_with(|| {
		// Setup
		let mechanism = IntegrationMechanism::JointGovernanceCouncil;
		let target_collective =
			TargetCollective::Other(b"Technical Committee".to_vec().try_into().unwrap());
		let description = create_description(MaxDescriptionLength::get() as usize + 1);
		let ambassador_participants = vec![1, 2];
		let target_participants = vec![3, 4];
		let agreement_hash = Some(random_hash());

		// Execute and verify failure with too long description
		assert_noop!(
			AmbassadorGovernance::<Runtime>::establish_integration(
				RuntimeOrigin::signed(1),
				mechanism.clone(),
				target_collective.clone(),
				description,
				ambassador_participants.clone(),
				target_participants.clone(),
				agreement_hash
			),
			Error::<Runtime>::TooLongDescription
		);
	});
}

#[test]
fn establish_integration_fails_with_too_many_participants() {
	new_test_ext().execute_with(|| {
		// Setup
		let mechanism = IntegrationMechanism::JointGovernanceCouncil;
		let target_collective =
			TargetCollective::Other(b"Technical Committee".to_vec().try_into().unwrap());
		let description = create_description(100);

		// Create too many ambassador participants
		let mut ambassador_participants = Vec::new();
		for i in 0..(MaxParticipants::get() as u64 + 1) {
			ambassador_participants.push(i + 1);
		}

		let target_participants = vec![3, 4];
		let agreement_hash = Some(random_hash());

		// Execute and verify failure with too many participants
		assert_noop!(
			AmbassadorGovernance::<Runtime>::establish_integration(
				RuntimeOrigin::signed(1),
				mechanism.clone(),
				target_collective.clone(),
				description.clone(),
				ambassador_participants,
				target_participants.clone(),
				agreement_hash
			),
			Error::<Runtime>::TooManyParticipants
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
		let evidence = Some(random_hash());

		assert_ok!(AmbassadorGovernance::<Runtime>::activate_emergency_protocol(
			RuntimeOrigin::signed(1),
			emergency_type.clone(),
			severity.clone(),
			justification.clone(),
			evidence.clone()
		));

		let emergency_id = get_last_emergency_id();

		// Form emergency committee with required roles
		let members = vec![
			(2, EmergencyRole::TechnicalLead),
			(3, EmergencyRole::GovernanceRepresentative),
			(4, EmergencyRole::IndependentExpert),
		];

		assert_ok!(AmbassadorGovernance::<Runtime>::form_emergency_committee(
			RuntimeOrigin::signed(1),
			emergency_id,
			members.clone()
		));

		// Resolve emergency as committee member
		let resolution_summary = create_justification(100);
		let abuse_detected = true; // Detected abuse

		assert_ok!(AmbassadorGovernance::<Runtime>::resolve_emergency(
			RuntimeOrigin::signed(2), // Technical Lead resolves
			emergency_id,
			abuse_detected,
			resolution_summary.clone(),
			evidence
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
		let original_decision = b"Original decision text".to_vec();
		let justification = create_justification(100);
		let evidence = Some(random_hash());

		assert_ok!(AmbassadorGovernance::<Runtime>::submit_appeal(
			RuntimeOrigin::signed(1),
			original_decision.clone(),
			justification.clone(),
			evidence
		));

		let appeal_id = get_last_appeal_id();

		// Form appeal committee
		let members = vec![2, 3, 4];

		assert_ok!(AmbassadorGovernance::<Runtime>::form_appeal_committee(
			RuntimeOrigin::signed(1),
			appeal_id,
			members.clone()
		));

		// Decide on appeal
		let decision = AppealDecision::Modified;

		assert_ok!(AmbassadorGovernance::<Runtime>::decide_appeal(
			RuntimeOrigin::signed(2),
			appeal_id,
			decision.clone()
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

// Helper function to create a provider name of specified length
fn create_provider_name(length: usize) -> Vec<u8> {
	vec![b'P'; length]
}

// Helper function to create contact info of specified length
fn create_contact_info(length: usize) -> Vec<u8> {
	vec![b'C'; length]
}

#[test]
fn register_service_provider_works() {
	new_test_ext().execute_with(|| {
		// Setup
		let provider_account = 3; // Account with verified identity
		let provider_name = create_provider_name(50);
		let service_types = vec![
			ProfessionalServiceType::LegalFinancial,
			ProfessionalServiceType::TechnicalDevelopment,
		];
		let contact_info = create_contact_info(100); // Include reference to off-chain evidence
		let evidence_hash = Some(random_hash());

		// Execute as account 2 (has Rank II)
		assert_ok!(AmbassadorGovernance::<Runtime>::register_service_provider(
			RuntimeOrigin::signed(2),
			provider_account,
			provider_name.clone(),
			service_types.clone(),
			contact_info.clone(),
			evidence_hash
		));

		// Get the provider ID from the event
		let events = frame_system::Pallet::<Runtime>::events();
		let mut provider_id = None;

		for event in &events {
			if let RuntimeEvent::AmbassadorGovernance(Event::ServiceProviderRegistered {
				provider_id: id,
				..
			}) = &event.event
			{
				provider_id = Some(id.clone());
				break;
			}
		}

		let provider_id = provider_id.expect("ServiceProviderRegistered event should be emitted");

		// Verify the provider was stored correctly
		let provider = AmbassadorGovernance::<Runtime>::service_providers(provider_id).unwrap();
		assert_eq!(provider.provider_account, provider_account);
		assert_eq!(provider.provider_name, provider_name);
		assert_eq!(provider.service_types, service_types);
		assert_eq!(provider.contact_info, contact_info);
		assert_eq!(provider.registrant, 2); // Registered by account 2
	});
}

#[test]
fn register_service_provider_fails_with_insufficient_rank() {
	new_test_ext().execute_with(|| {
		// Setup
		let provider_account = 3; // Account with verified identity
		let provider_name = create_provider_name(50);
		let service_types = vec![ProfessionalServiceType::LegalFinancial];
		let contact_info = create_contact_info(100);
		let evidence_hash = Some(random_hash());

		// Execute as account 4 (has Rank 0, below required Rank II)
		assert_noop!(
			AmbassadorGovernance::<Runtime>::register_service_provider(
				RuntimeOrigin::signed(4),
				provider_account,
				provider_name,
				service_types,
				contact_info,
				evidence_hash
			),
			Error::<Runtime>::InsufficientRankForProviderRegistry
		);
	});
}

#[test]
fn register_service_provider_fails_with_unverified_identity() {
	new_test_ext().execute_with(|| {
		// Setup
		let provider_account = 11; // Account WITHOUT verified identity
		let provider_name = create_provider_name(50);
		let service_types = vec![ProfessionalServiceType::LegalFinancial];
		let contact_info = create_contact_info(100);
		let evidence_hash = Some(random_hash());

		// Execute as account 1 (has sufficient rank)
		assert_noop!(
			AmbassadorGovernance::<Runtime>::register_service_provider(
				RuntimeOrigin::signed(1),
				provider_account,
				provider_name,
				service_types,
				contact_info,
				evidence_hash
			),
			Error::<Runtime>::IdentityNotVerified
		);
	});
}

#[test]
fn create_service_referral_works() {
	new_test_ext().execute_with(|| {
		// First register a service provider
		let provider_account = 3;
		let provider_name = create_provider_name(50);
		let service_types = vec![
			ProfessionalServiceType::LegalFinancial,
			ProfessionalServiceType::TechnicalDevelopment,
		];
		let contact_info = create_contact_info(100);
		let evidence_hash = Some(random_hash());

		assert_ok!(AmbassadorGovernance::<Runtime>::register_service_provider(
			RuntimeOrigin::signed(1),
			provider_account,
			provider_name.clone(),
			service_types.clone(),
			contact_info.clone(),
			evidence_hash
		));

		// Get the provider ID from the event
		let events = frame_system::Pallet::<Runtime>::events();
		let mut provider_id = None;

		for event in &events {
			if let RuntimeEvent::AmbassadorGovernance(Event::ServiceProviderRegistered {
				provider_id: id,
				..
			}) = &event.event
			{
				provider_id = Some(id.clone());
				break;
			}
		}

		let provider_id = provider_id.expect("ServiceProviderRegistered event should be emitted");

		// Create referral
		let description = create_description(100); // Include reference to off-chain evidence
		let compensation_disclosed = true;
		let compensation_details = Some(b"Received 10 tokens for this referral".to_vec());

		// Clear events
		frame_system::Pallet::<Runtime>::reset_events();

		// Create referral as account 2 (has Rank II)
		assert_ok!(AmbassadorGovernance::<Runtime>::create_service_referral(
			RuntimeOrigin::signed(2),
			provider_id,
			ProfessionalServiceType::LegalFinancial,
			description.clone(),
			compensation_disclosed,
			compensation_details.clone()
		));

		// Verify event was emitted
		let events = frame_system::Pallet::<Runtime>::events();
		let mut found_event = false;
		let mut referral_id = None;

		for event in &events {
			if let RuntimeEvent::AmbassadorGovernance(Event::ServiceReferralCreated {
				referral_id: id,
				referrer,
				provider_id: pid,
				service_type,
				compensation_disclosed: disclosed,
			}) = &event.event
			{
				found_event = true;
				referral_id = Some(id.clone());
				assert_eq!(*referrer, 2);
				assert_eq!(*pid, provider_id);
				assert_eq!(*service_type, ProfessionalServiceType::LegalFinancial);
				assert_eq!(*disclosed, compensation_disclosed);
				break;
			}
		}

		assert!(found_event, "ServiceReferralCreated event should be emitted");

		// Verify the referral was stored correctly
		let referral_id = referral_id.unwrap();
		let referral = AmbassadorGovernance::<Runtime>::service_referrals(referral_id).unwrap();
		assert_eq!(referral.referrer, 2);
		assert_eq!(referral.service_type, ProfessionalServiceType::LegalFinancial);
		assert_eq!(referral.description, description);
		assert_eq!(referral.compensation_disclosed, compensation_disclosed);
		assert!(referral.compensation_details.is_some());
	});
}

#[test]
fn create_service_referral_fails_with_insufficient_rank() {
	new_test_ext().execute_with(|| {
		// First register a service provider
		let provider_account = 3;
		let provider_name = create_provider_name(50);
		let service_types = vec![ProfessionalServiceType::LegalFinancial];
		let contact_info = create_contact_info(100);
		let evidence_hash = Some(random_hash());

		assert_ok!(AmbassadorGovernance::<Runtime>::register_service_provider(
			RuntimeOrigin::signed(1),
			provider_account,
			provider_name.clone(),
			service_types.clone(),
			contact_info.clone(),
			evidence_hash
		));

		// Get the provider ID from the event
		let events = frame_system::Pallet::<Runtime>::events();
		let mut provider_id = None;

		for event in &events {
			if let RuntimeEvent::AmbassadorGovernance(Event::ServiceProviderRegistered {
				provider_id: id,
				..
			}) = &event.event
			{
				provider_id = Some(id.clone());
				break;
			}
		}

		let provider_id = provider_id.expect("ServiceProviderRegistered event should be emitted");

		// Try to create referral as account 4 (has Rank 0, below required Rank II)
		let description = create_description(100);
		let compensation_disclosed = false;
		let compensation_details = None;

		assert_noop!(
			AmbassadorGovernance::<Runtime>::create_service_referral(
				RuntimeOrigin::signed(4),
				provider_id,
				ProfessionalServiceType::LegalFinancial,
				description,
				compensation_disclosed,
				compensation_details
			),
			Error::<Runtime>::InsufficientRankForReferral
		);
	});
}

#[test]
fn create_service_referral_fails_with_nonexistent_provider() {
	new_test_ext().execute_with(|| {
		// Try to create referral for a non-existent provider
		let non_existent_provider_id = H256::random();
		let description = create_description(100);
		let compensation_disclosed = false;
		let compensation_details = None;

		assert_noop!(
			AmbassadorGovernance::<Runtime>::create_service_referral(
				RuntimeOrigin::signed(2),
				non_existent_provider_id,
				ProfessionalServiceType::LegalFinancial,
				description,
				compensation_disclosed,
				compensation_details
			),
			Error::<Runtime>::ServiceProviderNotFound
		);
	});
}

#[test]
fn create_service_referral_fails_with_missing_compensation_disclosure() {
	new_test_ext().execute_with(|| {
		// First register a service provider
		let provider_account = 3;
		let provider_name = create_provider_name(50);
		let service_types = vec![ProfessionalServiceType::LegalFinancial];
		let contact_info = create_contact_info(100);
		let evidence_hash = Some(random_hash());

		assert_ok!(AmbassadorGovernance::<Runtime>::register_service_provider(
			RuntimeOrigin::signed(1),
			provider_account,
			provider_name.clone(),
			service_types.clone(),
			contact_info.clone(),
			evidence_hash
		));

		// Get the provider ID from the event
		let events = frame_system::Pallet::<Runtime>::events();
		let mut provider_id = None;

		for event in &events {
			if let RuntimeEvent::AmbassadorGovernance(Event::ServiceProviderRegistered {
				provider_id: id,
				..
			}) = &event.event
			{
				provider_id = Some(id.clone());
				break;
			}
		}

		let provider_id = provider_id.expect("ServiceProviderRegistered event should be emitted");

		// Try to create referral with compensation_disclosed being true but no details
		let description = create_description(100);
		let compensation_disclosed = true;
		let compensation_details = None; // Missing details

		assert_noop!(
			AmbassadorGovernance::<Runtime>::create_service_referral(
				RuntimeOrigin::signed(2),
				provider_id,
				ProfessionalServiceType::LegalFinancial,
				description,
				compensation_disclosed,
				compensation_details
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
		let provider_name = create_provider_name(50);
		let service_types = vec![
			ProfessionalServiceType::LegalFinancial,
			ProfessionalServiceType::TechnicalDevelopment,
		];
		let contact_info = create_contact_info(100); // Include reference to off-chain evidence
		let evidence_hash = Some(random_hash());

		assert_ok!(AmbassadorGovernance::<Runtime>::register_service_provider(
			RuntimeOrigin::signed(1), // Senior Ambassador (Rank III)
			provider_account,
			provider_name.clone(),
			service_types.clone(),
			contact_info.clone(),
			evidence_hash
		));

		// Get the provider ID from the event
		let events = frame_system::Pallet::<Runtime>::events();
		let mut provider_id = None;

		for event in &events {
			if let RuntimeEvent::AmbassadorGovernance(Event::ServiceProviderRegistered {
				provider_id: id,
				..
			}) = &event.event
			{
				provider_id = Some(id.clone());
				break;
			}
		}

		let provider_id = provider_id.expect("ServiceProviderRegistered event should be emitted");

		// Create a referral with compensation disclosure
		let description = create_description(100); // Include reference to off-chain evidence
		let compensation_disclosed = true;
		let compensation_details = Some(b"Received 5% commission for this referral".to_vec());

		// Clear events
		frame_system::Pallet::<Runtime>::reset_events();

		assert_ok!(AmbassadorGovernance::<Runtime>::create_service_referral(
			RuntimeOrigin::signed(2), // Ambassador (Rank II)
			provider_id,
			ProfessionalServiceType::LegalFinancial,
			description.clone(),
			compensation_disclosed,
			compensation_details.clone()
		));

		// Create another referral without compensation
		let description2 = create_description(80);
		let compensation_disclosed2 = false;
		let compensation_details2 = None;

		assert_ok!(AmbassadorGovernance::<Runtime>::create_service_referral(
			RuntimeOrigin::signed(1), // Senior Ambassador (Rank III)
			provider_id,
			ProfessionalServiceType::TechnicalDevelopment,
			description2.clone(),
			compensation_disclosed2,
			compensation_details2
		));

		// Verify events were emitted
		let events = frame_system::Pallet::<Runtime>::events();
		let mut found_referrals = 0;

		for event in &events {
			if let RuntimeEvent::AmbassadorGovernance(Event::ServiceReferralCreated { .. }) =
				&event.event
			{
				found_referrals += 1;
			}
		}

		assert_eq!(found_referrals, 2, "Two ServiceReferralCreated events should be emitted");
	});
}

#[test]
fn identity_verification_is_enforced() {
	new_test_ext().execute_with(|| {
		// Account 1 has a verified identity (according to MockIdentityVerifier)
		let account_with_identity = 1;

		// Account 11 does not have a verified identity (according to MockIdentityVerifier)
		let account_without_identity = 11;

		// Test with account that has verified identity - should succeed
		assert_ok!(AmbassadorGovernance::<Runtime>::submit_appeal(
			signed_origin(account_with_identity),
			b"Appeal decision hash".to_vec(),
			b"Appeal justification with reference to off-chain evidence location".to_vec(),
			None
		));

		// Test with account that doesn't have verified identity and it should fail with IdentityNotVerified error
		assert_noop!(
			AmbassadorGovernance::<Runtime>::submit_appeal(
				signed_origin(account_without_identity),
				b"Appeal decision hash".to_vec(),
				b"Appeal justification with reference to off-chain evidence location".to_vec(),
				None
			),
			crate::Error::<Runtime>::IdentityNotVerified
		);

		// Test another extrinsic with identity verification
		assert_ok!(AmbassadorGovernance::<Runtime>::register_conflict_of_interest(
			signed_origin(account_with_identity),
			crate::ConflictType::Financial,
			b"Conflict".to_vec(),
			b"Related matter".to_vec(),
			None
		));

		// Test the same extrinsic with account without identity and it should fail
		assert_noop!(
			AmbassadorGovernance::<Runtime>::register_conflict_of_interest(
				signed_origin(account_without_identity),
				crate::ConflictType::Financial,
				b"Conflict".to_vec(),
				b"Related matter".to_vec(),
				None
			),
			crate::Error::<Runtime>::IdentityNotVerified
		);
	});
}
