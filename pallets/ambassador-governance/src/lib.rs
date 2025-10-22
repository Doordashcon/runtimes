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

//! # Ambassador Fellowship Governance Pallet
//!
//! This pallet implements governance mechanisms for the Ambassador Fellowship including:
//! - Emergency Response Procedures
//! - Appeal and Remediation Process
//! - Cross-Collective Integration
//! - Progressive Discipline Approach
//! - Role Transition Process
//! - Transparency Safeguards
//! - On-Chain Remarks
//! - Governance Health Monitoring
//!
//! These mechanisms align with proposed updates to the Ambassador Fellowship Manifesto
//! and support the Polkadot On-chain Readiness Assessment Framework for Collectives.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	pallet_prelude::*,
	traits::{EnsureOrigin, Get, ReservableCurrency},
	BoundedVec,
};
use frame_system::{ensure_signed, pallet_prelude::*};
use pallet_ranked_collective_ambassador::Rank;
use scale_info::TypeInfo;
use sp_core::H256;
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, Hash, SaturatedConversion},
	RuntimeDebug,
};
use sp_std::prelude::*;

/// Trait for verifying identity
pub trait IdentityVerifier<AccountId> {
	/// Returns true if the account has a verified identity
	fn has_identity(who: &AccountId) -> bool;
}

/// Trait for checking if an account has a minimum rank
pub trait RankChecker<AccountId> {
	/// Returns true if the account has at least the specified minimum rank
	fn has_minimum_rank(who: &AccountId, min_rank: u16) -> bool;
}

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

pub use pallet::*;
pub use weights::WeightInfo;

/// Type alias for the Currency::Balance type

/// Emergency types that can trigger the emergency response protocol
///
/// Defines the various types of emergencies that can be declared
/// by certain authorized origins to initiate the emergency response procedure
///
/// Each emergency type may require different response strategies and committee
/// compositions based on the nature of the threat.
#[derive(
	Copy,
	Clone,
	Encode,
	Decode,
	PartialEq,
	Eq,
	RuntimeDebug,
	TypeInfo,
	MaxEncodedLen,
	frame_support::__private::codec::DecodeWithMemTracking,
)]
pub enum EmergencyType {
	/// Critical security vulnerability that poses immediate risk to network or user assets
	///
	/// Examples: Exploitable code vulnerabilities, cryptographic weaknesses,
	/// discovered attack vectors that could compromise system integrity.
	SecurityVulnerability,
	/// Governance attack attempting to manipulate collective decision-making
	///
	/// Examples: Vote buying, sybil attacks, coordinated actions to
	/// subvert governance mechanisms for malicious purposes.
	GovernanceAttack,
	/// Technical failure affecting system functionality
	///
	/// Examples: Critical bugs, network partitions, consensus failures,
	/// other technical issues that impair normal operations.
	TechnicalFailure,
	/// Reputation threat that could damage collective credibility
	///
	/// Examples: Public relations crises, misinformation campaigns,
	/// or actions by members that could harm the collective's standing.
	ReputationThreat,
	/// Other emergency type not covered by predefined categories
	///
	/// Catch-all variant that allows for flexibility in addressing unforeseen
	/// emergency situations that don't fit into the standard categories.
	Other,
}

/// Emergency severity levels that determine response timeframes and resource allocation
///
/// Defines the severity levels for emergency situations, which influence
/// the urgency and scale of the response required. Each level corresponds to different
/// response timeframes and resource allocation requirements.
#[derive(
	Copy,
	Clone,
	Encode,
	Decode,
	PartialEq,
	Eq,
	RuntimeDebug,
	TypeInfo,
	MaxEncodedLen,
	frame_support::__private::codec::DecodeWithMemTracking,
)]
pub enum EmergencySeverity {
	/// Critical requires immediate action
	///
	/// Critical emergencies pose an existential threat to the collective or network
	/// and require immediate response within hours.
	///
	/// Examples: Active exploits, significant fund loss, or severe reputation damage already
	/// occurring.
	Critical,
	/// High requires expedited approval
	///
	/// High severity emergencies represent significant threats that require
	/// expedited response within 1-2 days.
	///
	/// Examples: Vulnerabilities with high likelihood of exploitation or substantial impact if
	/// exploited.
	High,
	/// Medium follows standard approval process
	///
	/// Medium severity emergencies follow the standard approval process with
	/// response times of several days.
	///
	/// Examples: Important but non-critical issues that can be addressed through normal governance
	/// procedures.
	Medium,
}

/// Role in emergency committee
#[derive(
	Encode,
	Decode,
	Clone,
	PartialEq,
	Eq,
	RuntimeDebug,
	TypeInfo,
	MaxEncodedLen,
	frame_support::__private::codec::DecodeWithMemTracking,
)]
pub enum EmergencyRole {
	/// Technical lead responsible for affected system
	TechnicalLead,
	/// Governance representative
	GovernanceRepresentative,
	/// Independent wellbeing/ethics expert
	IndependentExpert,
}

/// Appeal status
#[derive(
	Encode,
	Decode,
	Clone,
	PartialEq,
	Eq,
	RuntimeDebug,
	TypeInfo,
	MaxEncodedLen,
	frame_support::__private::codec::DecodeWithMemTracking,
)]
pub enum AppealStatus {
	/// Appeal submitted and awaiting review
	Submitted,
	/// Appeal under review by committee
	UnderReview,
	/// Appeal decision made
	Decided,
	/// Appeal closed
	Closed,
}

/// Appeal decision
#[derive(
	Encode,
	Decode,
	Clone,
	PartialEq,
	Eq,
	RuntimeDebug,
	TypeInfo,
	MaxEncodedLen,
	frame_support::__private::codec::DecodeWithMemTracking,
)]
pub enum AppealDecision {
	/// Original decision upheld
	Upheld,
	/// Original decision modified
	Modified,
	/// Original decision rejected
	Rejected,
}

/// Cross-collective integration mechanism
#[derive(
	Encode,
	Decode,
	Clone,
	PartialEq,
	Eq,
	RuntimeDebug,
	TypeInfo,
	MaxEncodedLen,
	frame_support::__private::codec::DecodeWithMemTracking,
)]
pub enum IntegrationMechanism {
	/// Joint Governance Council: Formal council with members from both collectives
	///
	/// Example: Council with certain number of Ambassador Fellows and Technical Fellows that meets
	/// at a certain interval to coordinate on technical governance decisions affecting both
	/// collectives.
	JointGovernanceCouncil,
	/// Liaison System: Designated points of contact between collectives
	///
	/// Example: Certain number of Ambassador Fellows serving as liaisons to the Technical
	/// Fellowship, attending their meetings and facilitating regular communication channels.
	LiaisonSystem,
	/// Integrated Planning Cycles: Synchronized roadmaps and planning processes
	///
	/// Example: Certain number of Ambassador Fellows and Technical Fellows synchronizing quarterly
	/// planning with joint kickoff sessions and regular alignment check-ins.
	IntegratedPlanningCycles,
}

/// Target collective for cross-collective integration
///
/// Various collectives that can be integrated with the Ambassador Fellowship.
/// Provides type-safe representation of known collectives while allowing for custom collectives
/// through the `Other` variant.
#[derive(
	Encode,
	Decode,
	Clone,
	PartialEq,
	Eq,
	RuntimeDebug,
	TypeInfo,
	MaxEncodedLen,
	frame_support::__private::codec::DecodeWithMemTracking,
)]
pub enum TargetCollective {
	/// Technical Fellowship collective
	///
	/// Technical Fellowship is responsible for the technical governance of
	/// the network, including protocol upgrades, technical standards, and
	/// implementation quality assurance.
	TechnicalFellowship,
	/// Secretary collective
	///
	/// Secretary collective handles administrative functions, documentation,
	/// and coordination activities across the governance system.
	SecretaryCollective,
	/// Alliance collective
	///
	/// Alliance collective focuses on ecosystem growth, partnerships,
	/// and community engagement to expand the network's adoption and utility.
	AllianceCollective,
	/// Other collective with custom name
	///
	/// Catch-all variant that allows for integration with collectives not explicitly
	/// defined in this enum. The bounded vector stores the name of the custom
	/// collective, limited to 32 bytes for efficient on-chain storage.
	Other(BoundedVec<u8, ConstU32<32>>),
}

/// Emergency details
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct EmergencyDetails<AccountId, BlockNumber, BoundedString> {
	/// Emergency type
	emergency_type: EmergencyType,
	/// Severity level
	severity: EmergencySeverity,
	/// Initiator of the emergency response
	initiator: AccountId,
	/// Justification for emergency declaration
	justification: BoundedString,
	/// Block when emergency was declared
	declared_at: BlockNumber,
	/// Block when emergency was resolved (if resolved)
	resolved_at: Option<BlockNumber>,
	/// Evidence hash
	evidence_hash: Option<H256>,
	/// Abuse detected after investigation or not
	abuse_detected: bool,
}

/// Appeal details
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct AppealDetails<AccountId, BlockNumber, BoundedString> {
	/// Appellant account
	appellant: AccountId,
	/// Original decision being appealed
	original_decision: BoundedString,
	/// Appeal justification
	justification: BoundedString,
	/// Block when appeal was submitted
	submitted_at: BlockNumber,
	/// Status of appeal currently
	status: AppealStatus,
	/// Decision on the appeal (if decided)
	decision: Option<AppealDecision>,
	/// Block when decision was made (if decided)
	decided_at: Option<BlockNumber>,
	/// Evidence hash
	evidence_hash: Option<H256>,
}

/// Cross-collective integration details
///
/// Stores information about established integrations between the Ambassador
/// Fellowship and other collectives.
#[derive(
	Encode,
	Decode,
	Clone,
	PartialEq,
	Eq,
	RuntimeDebug,
	TypeInfo,
	MaxEncodedLen,
	frame_support::__private::codec::DecodeWithMemTracking,
)]
pub struct IntegrationDetails<BlockNumber, BoundedString, BoundedVecS, BoundedVecT> {
	/// Integration mechanism
	mechanism: IntegrationMechanism,
	/// Collective being integrated with
	target_collective: TargetCollective,
	/// Integration description
	description: BoundedString,
	/// Participants from Ambassador Fellowship
	ambassador_participants: BoundedVecS,
	/// Participants from target collective
	target_participants: BoundedVecT,
	/// Block when integration was established
	established_at: BlockNumber,
	/// Integration evidence hash
	evidence_hash: Option<H256>,
}

/// Progressive discipline level
///
/// Defines the levels of progressive discipline that can be applied to members
/// who violate collective rules or fail to meet obligations.
#[derive(
	Encode,
	Decode,
	Clone,
	PartialEq,
	Eq,
	RuntimeDebug,
	TypeInfo,
	MaxEncodedLen,
	frame_support::__private::codec::DecodeWithMemTracking,
)]
pub enum DisciplineLevel {
	/// Informal warning with no on-chain record
	///
	/// First step in progressive discipline, typically for minor first-time violations.
	/// No permanent record is maintained on-chain, but the warning is logged in events.
	Informal,
	/// Formal warning with on-chain record
	///
	/// Second step for repeated minor violations or moderate first-time violations.
	/// Creates a permanent on-chain record of the warning.
	Formal,
	/// Probation with restricted privileges
	///
	/// Third step for serious violations or continued pattern of minor violations.
	/// Member remains active but with restricted privileges for a specified period.
	Probation,
	/// Suspension of membership privileges
	///
	/// Fourth step for severe violations or failure to improve during probation.
	/// Temporarily removes all membership privileges for a specified period.
	Suspension,
	/// Removal from collective
	///
	/// Final step for the most severe violations or continued failure to meet obligations.
	/// Permanently removes member from the collective.
	Removal,
}

/// Rank transition type
///
/// Defines the types of rank transitions that can occur within the Ambassador Fellowship.
#[derive(
	Encode,
	Decode,
	Clone,
	PartialEq,
	Eq,
	RuntimeDebug,
	TypeInfo,
	MaxEncodedLen,
	frame_support::__private::codec::DecodeWithMemTracking,
)]
pub enum TransitionType {
	/// Demotion to a lower rank
	///
	/// Member moves to a lower rank within the Ambassador Fellowship hierarchy.
	Demotion,
	/// Promotion to a higher rank
	///
	/// Member advances to a higher rank within the Ambassador Fellowship hierarchy.
	Promotion,
	/// Removal from collective
	///
	/// Final step for the most severe violations or continued failure to meet obligations.
	/// Permanently removes member from the collective.
	Removal,
	/// Role change within same rank
	///
	/// Member changes responsibilities while maintaining the same rank.
	RoleChange,
	/// Resignation
	Resignation,
	/// Retirement
	Retirement,
	/// Temporary absence
	///
	/// Member takes a temporary leave of absence with intent to return.
	TemporaryAbsence,
}

/// Conflict of interest type
///
/// Defines the types of conflicts of interest that can be registered.
#[derive(
	Encode,
	Decode,
	Clone,
	PartialEq,
	Eq,
	RuntimeDebug,
	TypeInfo,
	MaxEncodedLen,
	frame_support::__private::codec::DecodeWithMemTracking,
)]
pub enum ConflictType {
	/// Financial interest in outcome (Critical)
	///
	/// Member has a financial stake in the outcome of a decision.
	/// This is considered a critical conflict for committee participation.
	FinancialCritical,
	/// Professional relationship with involved parties (Critical)
	///
	/// Member has a professional relationship with individuals or organizations affected by a
	/// decision. This is considered a critical conflict for committee participation.
	ProfessionalCritical,
	/// Personal relationship with involved parties (Non-critical)
	///
	/// Member has a personal relationship with individuals affected by a decision.
	/// This is considered a non-critical conflict for committee participation.
	PersonalNonCritical,
	/// Ideological bias (Non-critical)
	///
	/// Member has strong ideological views that may affect their objectivity.
	/// This is considered a non-critical conflict for committee participation.
	IdeologicalNonCritical,
	/// Other conflict type (Non-critical)
	///
	/// Catch-all for conflicts that don't fit the predefined categories.
	/// This is considered a non-critical conflict for committee participation.
	OtherNonCritical,
}

/// Helper trait to determine if a conflict type is critical
pub trait IsConflictCritical {
	/// Returns true if the conflict type is considered critical for committee participation
	fn is_critical(&self) -> bool;
}

impl IsConflictCritical for ConflictType {
	fn is_critical(&self) -> bool {
		match self {
			ConflictType::FinancialCritical | ConflictType::ProfessionalCritical => true,
			_ => false,
		}
	}
}

/// On-chain remark category
///
/// Standardized categories for on-chain remarks as specified in the framework.
#[derive(
	Encode,
	Decode,
	Clone,
	PartialEq,
	Eq,
	RuntimeDebug,
	TypeInfo,
	MaxEncodedLen,
	frame_support::__private::codec::DecodeWithMemTracking,
)]
pub enum RemarkCategory {
	/// Governance action
	///
	/// Remarks related to governance decisions and actions.
	Governance,
	/// Technical update
	///
	/// Remarks related to technical changes or updates.
	Technical,
	/// Administrative notice
	///
	/// Remarks related to administrative matters.
	Administrative,
	/// Community announcement
	///
	/// Remarks intended for the broader community.
	Community,
	/// Emergency notification
	///
	/// Urgent remarks related to emergency situations.
	Emergency,
	/// Professional Services
	///
	/// Remarks related to professional services boundaries and referrals.
	ProfessionalServices,
}

/// Discipline details
///
/// Details of a disciplinary action
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct DisciplineDetails<AccountId, BlockNumber, BoundedString> {
	/// Subject of the disciplinary action
	pub subject: AccountId,
	/// Issuer of the disciplinary action
	pub issuer: AccountId,
	/// Level of discipline
	pub level: DisciplineLevel,
	/// Reason for the disciplinary action, must include the location
	/// where any off-chain evidence (referenced by the evidence hash) is stored
	pub reason: BoundedString,
	/// Block when the disciplinary action was issued
	pub issued_at: BlockNumber,
	/// Duration of the disciplinary action (if applicable)
	pub duration: Option<BlockNumber>,
	/// Optional hash of evidence supporting the disciplinary action
	/// where the actual evidence is stored off-chain, and its location should be
	/// referenced in the reason field
	pub evidence_hash: Option<H256>,
	/// Whether the disciplinary action is active
	pub active: bool,
}

/// Rank transition details
///
/// Stores information about rank transitions within the Ambassador Fellowship.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct TransitionDetails<AccountId, BlockNumber, BoundedString> {
	/// Member undergoing transition
	pub member: AccountId,
	/// Type of transition
	pub transition_type: TransitionType,
	/// Previous rank in the Ambassador Fellowship
	pub previous_rank: Rank,
	/// New rank in the Ambassador Fellowship
	pub new_rank: Rank,
	/// Justification for the transition, must include the location
	/// where any off-chain evidence or documentation is stored for future reference
	pub justification: BoundedString,
	/// Block when transition was initiated
	pub initiated_at: BlockNumber,
	/// Block when transition will be completed
	pub effective_at: BlockNumber,
	/// Successor account (if applicable)
	pub successor: Option<AccountId>,
	/// Knowledge transfer status
	pub knowledge_transfer_complete: bool,
	/// Optional hash of evidence supporting the transition
	/// where the actual evidence is stored off-chain, and its location should be
	/// referenced in the justification field
	pub evidence_hash: Option<H256>,
}

/// Conflict of interest registration
///
/// Stores information about declared conflicts of interest.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct ConflictRegistration<
	AccountId,
	ConflictType,
	BlockNumber,
	BoundedDescription,
	BoundedRelatesTo,
> {
	/// Member declaring the conflict
	pub member: AccountId,
	/// Type of conflict
	pub conflict_type: ConflictType,
	/// Description of the conflict, must include the location
	/// where any off-chain evidence is stored for future reference
	pub description: BoundedDescription,
	/// Related matter or decision that can be an extrinsic hash, emergency ID, appeal ID, or any
	/// other on-chain identifier that this conflict relates to. If the conflict is general,
	/// this should contain a clear description of the scope of the conflict.
	pub relates_to: Option<BoundedRelatesTo>,
	/// Block when conflict was registered
	pub start_block: Option<BlockNumber>,
	/// Block when conflict expires (if applicable)
	pub end_block: Option<BlockNumber>,
	/// Optional hash of evidence supporting the conflict declaration
	/// where the actual evidence is stored off-chain, and its location should be
	/// referenced in the description field
	pub evidence_hash: Option<H256>,
	/// Last block when conflict was updated
	pub last_updated: BlockNumber,
}

/// On-chain remark
///
/// Stores standardized on-chain remarks with metadata.
/// Unlike System::remark, this provides a structured format with additional metadata.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct OnChainRemark<BlockNumber, ContentBound> {
	/// Content of the remark must include the location where any off-chain evidence
	/// (referenced by the `evidence_hash`) is stored for future reference and auditability
	pub content: ContentBound,
	/// Evidence hash (if applicable) where actual evidence is stored off-chain and its location
	/// should be referenced in the `content` field
	pub evidence_hash: Option<H256>,
	/// Block when remark was last updated
	pub last_updated: BlockNumber,
}

/// Professional service type
///
/// Defines the types of professional services that are outside the scope of Ambassador Fellowship
/// duties.
#[derive(
	Encode,
	Decode,
	Clone,
	PartialEq,
	Eq,
	RuntimeDebug,
	TypeInfo,
	MaxEncodedLen,
	frame_support::__private::codec::DecodeWithMemTracking,
)]
pub enum ProfessionalServiceType {
	/// Business consulting and strategic advisory
	BusinessConsulting,
	/// Technical implementation and development
	TechnicalDevelopment,
	/// Legal, financial, or tax advice
	LegalFinancial,
	/// Marketing and design services
	MarketingDesign,
	/// Extended mentorship or coaching programs
	ExtendedMentorship,
	/// Other professional services
	Other,
}

/// Professional service provider details
///
/// Stores minimal information about registered professional service providers.
/// Providers must have a verified identity through the identity pallet.
/// Most provider information is stored in their on-chain identity.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct ServiceProviderDetails<
	AccountId,
	BlockNumber,
	ServiceTypesBoundedVec,
	EvidenceInfoBoundedString,
> {
	/// Provider account with verified identity
	provider_account: AccountId,
	/// Service types offered
	service_types: ServiceTypesBoundedVec,
	/// Evidence information (references to off-chain evidence supporting the provider's
	/// credentials)
	evidence_info: EvidenceInfoBoundedString,
	/// Optional hash of evidence supporting the provider's credentials
	/// where the actual evidence is stored off-chain, and its location should be
	/// referenced in the evidence_info field
	evidence_hash: Option<H256>,
	/// Last updated block
	last_updated: BlockNumber,
}

/// Professional service referral
///
/// Stores information about referrals to professional service providers.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct ServiceReferral<
	AccountId,
	BlockNumber,
	ServiceTypesBoundedVec,
	DescriptionBoundedString,
	CompensationDetailsBoundedString,
	H256,
> {
	/// Referrer (Ambassador who made the referral)
	referrer: AccountId,
	/// Provider being referred to (account with verified identity)
	provider_account: AccountId,
	/// Type of service being referred
	service_types: ServiceTypesBoundedVec,
	/// Description of the referral
	description: DescriptionBoundedString,
	/// Whether the referrer disclosed receiving compensation for this referral (transparency
	/// requirement)
	compensation_disclosed: bool,
	/// Description of compensation received by referrer for this referral (if any)
	compensation_details: Option<CompensationDetailsBoundedString>,
	/// Optional hash of evidence supporting the referral
	evidence_hash: Option<H256>,
	/// When the referral was last updated
	last_updated: BlockNumber,
}

/// Governance health metrics
///
/// Stores metrics for monitoring governance health.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct GovernanceHealthMetrics<BlockNumber, EvidenceInfoBound> {
	/// Participation rate (0-100%)
	/// Percentage of eligible members who participated in governance activities
	pub participation_rate: u8,
	/// Vote concentration index (0-100%)
	/// Measures how concentrated voting power is among participants, where
	/// lower values indicate more equal distribution of votes and
	/// higher values indicate votes are concentrated among fewer participants
	pub vote_concentration: u8,
	/// Average response time taken to respond to governance actions in blocks
	pub avg_response_time: BlockNumber,
	/// Start period
	/// Block number at which the metrics period started
	pub start_block: BlockNumber,
	/// End period
	/// Block number at which the metrics period ended
	pub end_block: BlockNumber,
	/// Evidence information, must include the location
	/// where any off-chain evidence is stored for future reference and auditability
	pub evidence_info: EvidenceInfoBound,
	/// Optional evidence hash
	pub evidence_hash: Option<H256>,
	/// Last updated block
	pub last_updated: BlockNumber,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Overarching event type
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Currency mechanism
		type Currency: ReservableCurrency<Self::AccountId>;

		/// Block number type
		type BlockNumber: Parameter + Member + Copy + Default + MaxEncodedLen + TypeInfo + AtLeast32BitUnsigned + PartialOrd + sp_runtime::traits::Bounded +
    From<u32> + From<u64> + From<<<<Self as frame_system::Config>::Block as sp_runtime::traits::Block>::Header as sp_runtime::traits::Header>::Number>;

		/// Origin authorized to activate emergency protocol
		type EmergencyOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Origin authorized to form emergency committees
		type CommitteeFormationOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Origin authorized to resolve emergencies
		type EmergencyResolutionOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Origin authorized to submit appeals
		type AppealSubmissionOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Origin authorized to form appeal committees
		type AppealCommitteeOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Origin for deciding on appeals (appeal committee only)
		type AppealDecisionOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Origin for establishing cross-collective integration
		type IntegrationOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Maximum size for justification strings
		#[pallet::constant]
		type MaxJustificationLength: Get<u32>;

		/// Maximum size for description strings
		#[pallet::constant]
		type MaxDescriptionLength: Get<u32>;

		/// Maximum size for resolution strings
		#[pallet::constant]
		type MaxResolutionLength: Get<u32>;

		/// Maximum size for remark content strings
		#[pallet::constant]
		type MaxRemarkContentLength: Get<u32>;

		/// Maximum number of appeal committee members
		#[pallet::constant]
		type MaxAppealCommitteeMembers: Get<u32>;

		/// Maximum number of emergency committee members
		#[pallet::constant]
		type MaxEmergencyCommitteeMembers: Get<u32>;

		/// Maximum number of source collective (Ambassador Fellowship) participants in integration
		#[pallet::constant]
		type MaxSourceParticipants: Get<u32>;

		/// Maximum number of target collective participants in integration
		#[pallet::constant]
		type MaxTargetParticipants: Get<u32>;

		/// Maximum number of service types
		#[pallet::constant]
		type MaxServiceTypes: Get<u32>;

		/// Maximum size for compensation details strings
		#[pallet::constant]
		type MaxCompensationDetailsLength: Get<u32>;

		/// Maximum size for evidence information strings
		#[pallet::constant]
		type MaxEvidenceInfoLength: Get<u32>;

		/// Weight information for extrinsics in this pallet
		type WeightInfo: weights::WeightInfo;

		/// Identity verification mechanism that uses
		/// identity registrar for verifying account identities
		type IdentityRegistrar: IdentityVerifier<Self::AccountId>;

		/// Rank checker for verifying ambassador ranks
		type RankChecker: RankChecker<Self::AccountId>;

		/// Minimum rank required to register a service provider
		#[pallet::constant]
		type MinRankForProviderRegistry: Get<u16>;

		/// Minimum rank required to create a service referral
		#[pallet::constant]
		type MinRankForReferral: Get<u16>;

		/// Minimum rank required to activate emergency protocol
		#[pallet::constant]
		type MinRankToActivateEmergencyProtocol: Get<u16>;

		/// Minimum rank required to form emergency committee
		#[pallet::constant]
		type MinRankToFormEmergencyCommittee: Get<u16>;

		/// Minimum rank required to submit appeal
		#[pallet::constant]
		type MinRankToSubmitAppeal: Get<u16>;

		/// Minimum rank required to form appeal committee
		#[pallet::constant]
		type MinRankToFormAppealCommittee: Get<u16>;

		/// Minimum rank required to establish integration
		#[pallet::constant]
		type MinRankToEstablishIntegration: Get<u16>;

		/// Minimum rank accountable for participation metric monitoring
		#[pallet::constant]
		type MinRankForParticipationMetricMonitoring: Get<u16>;

		/// Minimum rank accountable for parameter adjustment triggers
		#[pallet::constant]
		type MinRankForParameterAdjustmentTriggers: Get<u16>;

		/// Minimum rank accountable for governance parameter registry
		#[pallet::constant]
		type MinRankForGovernanceParameterRegistry: Get<u16>;

		/// Minimum rank accountable for emergency classification
		#[pallet::constant]
		type MinRankForEmergencyClassification: Get<u16>;

		/// Minimum rank accountable for emergency response authority
		#[pallet::constant]
		type MinRankForEmergencyResponseAuthority: Get<u16>;

		/// Minimum rank accountable for governance participation requirements
		#[pallet::constant]
		type MinRankForGovernanceParticipationRequirements: Get<u16>;

		/// Minimum rank accountable for decide appeal
		#[pallet::constant]
		type MinRankForDecideAppeal: Get<u16>;

		/// Minimum rank accountable for disciplinary action enforcement
		#[pallet::constant]
		type MinRankForDisciplinaryActionEnforcement: Get<u16>;

		/// Minimum rank accountable for transparency and fairness safeguards
		#[pallet::constant]
		type MinRankToSetConflictOfInterest: Get<u16>;

		/// Minimum rank accountable for cross-collective coordination mechanisms
		#[pallet::constant]
		type MinRankForCoordinationMechanisms: Get<u16>;

		/// Minimum rank accountable for joint decision-making procedures
		#[pallet::constant]
		type MinRankForJointDecisionMaking: Get<u16>;

		/// Minimum rank accountable for knowledge and resource sharing
		#[pallet::constant]
		type MinRankForKnowledgeSharing: Get<u16>;

		/// Minimum rank accountable for collective boundary management
		#[pallet::constant]
		type MinRankForBoundaryManagement: Get<u16>;

		/// Minimum rank accountable for role fulfillment contingency
		#[pallet::constant]
		type MinRankForRoleFulfillmentContingency: Get<u16>;

		/// Minimum rank required to set remarks
		#[pallet::constant]
		type MinRankToSetRemark: Get<u16>;

		/// Maximum number of conflicts of interest to check per committee formation
		/// This limits the computational complexity of conflict of interest checks
		#[pallet::constant]
		type MaxConflictOfInterestChecks: Get<u32>;
	}

	/// Emergency responses that have been declared
	#[pallet::storage]
	#[pallet::getter(fn emergencies)]
	pub type Emergencies<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::Hash, // Emergency ID
		EmergencyDetails<T::AccountId, T::BlockNumber, BoundedVec<u8, T::MaxJustificationLength>>,
	>;

	/// Emergency committee members for each emergency
	#[pallet::storage]
	#[pallet::getter(fn emergency_committees)]
	pub type EmergencyCommittees<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::Hash, // Emergency ID
		BoundedVec<(T::AccountId, EmergencyRole), T::MaxEmergencyCommitteeMembers>,
	>;

	/// Appeals that have been submitted
	#[pallet::storage]
	#[pallet::getter(fn appeals)]
	pub type Appeals<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::Hash, // Appeal ID
		AppealDetails<T::AccountId, T::BlockNumber, BoundedVec<u8, T::MaxJustificationLength>>,
	>;

	/// Appeal committee members for each appeal
	#[pallet::storage]
	#[pallet::getter(fn appeal_committees)]
	pub type AppealCommittees<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::Hash, // Appeal ID
		BoundedVec<T::AccountId, T::MaxAppealCommitteeMembers>,
	>;

	/// Cross-collective integrations that have been established
	#[pallet::storage]
	#[pallet::getter(fn integrations)]
	pub type Integrations<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::Hash, // Integration ID
		IntegrationDetails<
			T::BlockNumber,
			BoundedVec<u8, T::MaxDescriptionLength>,
			BoundedVec<T::AccountId, T::MaxSourceParticipants>,
			BoundedVec<T::AccountId, T::MaxTargetParticipants>,
		>,
	>;

	/// Disciplinary actions
	#[pallet::storage]
	#[pallet::getter(fn disciplines)]
	pub type Disciplines<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::Hash, // Discipline ID
		DisciplineDetails<T::AccountId, T::BlockNumber, BoundedVec<u8, T::MaxJustificationLength>>,
	>;

	/// Role transitions
	#[pallet::storage]
	#[pallet::getter(fn transitions)]
	pub type Transitions<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::Hash, // Transition ID
		TransitionDetails<T::AccountId, T::BlockNumber, BoundedVec<u8, T::MaxJustificationLength>>,
	>;

	/// Conflict of interest registrations
	#[pallet::storage]
	#[pallet::getter(fn conflicts)]
	pub type Conflicts<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		(T::AccountId, ConflictType, u32), // Composite key: (member, conflict_type, nonce)
		ConflictRegistration<
			T::AccountId,
			ConflictType,
			T::BlockNumber,
			BoundedVec<u8, T::MaxDescriptionLength>,
			BoundedVec<u8, T::MaxDescriptionLength>,
		>,
	>;

	// Conflict of interest nonce tracker
	#[pallet::storage]
	#[pallet::getter(fn conflict_nonces)]
	pub type ConflictNonces<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		(T::AccountId, ConflictType), // Key for tracking nonces
		u32,
		ValueQuery,
	>;

	/// On-chain remarks
	#[pallet::storage]
	#[pallet::getter(fn remarks)]
	pub type Remarks<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		(T::AccountId, RemarkCategory, u32), // (author, category, nonce)
		OnChainRemark<T::BlockNumber, BoundedVec<u8, T::MaxRemarkContentLength>>,
		OptionQuery,
	>;

	/// Stores the next nonce for a given author and category pair
	#[pallet::storage]
	#[pallet::getter(fn remark_nonces)]
	pub type RemarkNonces<T: Config> =
		StorageMap<_, Blake2_128Concat, (T::AccountId, RemarkCategory), u32, ValueQuery>;

	/// Professional service providers registry
	///
	/// Maps from provider account to details about the service provider.
	/// The account must have a verified identity through the identity pallet.
	/// Most provider information is stored in their on-chain identity.
	#[pallet::storage]
	#[pallet::getter(fn service_providers)]
	pub type ServiceProviders<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId, // Provider account ID (must have verified identity)
		ServiceProviderDetails<
			T::AccountId,
			T::BlockNumber,
			BoundedVec<ProfessionalServiceType, T::MaxServiceTypes>,
			BoundedVec<u8, T::MaxEvidenceInfoLength>,
		>,
		OptionQuery,
	>;

	/// Professional service referrals
	#[pallet::storage]
	#[pallet::getter(fn service_referrals)]
	pub type ServiceReferrals<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::Hash, // Referral ID
		ServiceReferral<
			T::AccountId,
			T::BlockNumber,
			BoundedVec<ProfessionalServiceType, T::MaxServiceTypes>,
			BoundedVec<u8, T::MaxDescriptionLength>,
			BoundedVec<u8, T::MaxCompensationDetailsLength>,
			H256,
		>,
	>;

	/// Governance health metrics
	#[pallet::storage]
	#[pallet::getter(fn governance_health)]
	pub type GovernanceHealth<T: Config> = StorageValue<
		_,
		GovernanceHealthMetrics<T::BlockNumber, BoundedVec<u8, T::MaxEvidenceInfoLength>>,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Emergency protocol activated
		EmergencyActivated {
			emergency_id: T::Hash,
			emergency_type: EmergencyType,
			severity: EmergencySeverity,
			initiator: T::AccountId,
		},
		/// Emergency committee formed
		EmergencyCommitteeFormed {
			emergency_id: T::Hash,
			members: BoundedVec<(T::AccountId, EmergencyRole), T::MaxEmergencyCommitteeMembers>,
		},
		/// Emergency resolved
		EmergencyResolved {
			emergency_id: T::Hash,
			abuse_detected: bool,
			resolution_summary: BoundedVec<u8, T::MaxResolutionLength>,
		},
		/// Appeal submitted
		AppealSubmitted {
			appeal_id: T::Hash,
			appellant: T::AccountId,
			original_decision: BoundedVec<u8, T::MaxJustificationLength>,
		},
		/// Appeal committee formed
		AppealCommitteeFormed {
			appeal_id: T::Hash,
			members: BoundedVec<T::AccountId, T::MaxAppealCommitteeMembers>,
		},
		/// Appeal decided
		AppealDecided {
			appeal_id: T::Hash,
			decider: T::AccountId,
			decision: AppealDecision,
			justification: BoundedVec<u8, T::MaxJustificationLength>,
			evidence_hash: Option<H256>,
		},
		/// Cross-collective integration established
		///
		/// Emitted when a new cross-collective integration is established between the
		/// Ambassador Fellowship and another collective. Event that signals the successful
		/// creation of a formal relationship between collectives and can be used by off-chain
		/// systems to track and display active integrations.
		IntegrationEstablished {
			/// Unique identifier for the integration, derived from the integration details
			integration_id: T::Hash,
			/// Integration mechanism type that was established (e.g. JointGovernanceCouncil,
			/// LiaisonSystem, or IntegratedPlanningCycles)
			mechanism: IntegrationMechanism,
			/// Collective being integrated with
			target_collective: TargetCollective,
			/// Description of the integration
			description: BoundedVec<u8, T::MaxDescriptionLength>,
			/// Ambassador participants in the integration
			ambassador_participants: BoundedVec<T::AccountId, T::MaxSourceParticipants>,
			/// Target collective participants in the integration
			target_participants: BoundedVec<T::AccountId, T::MaxTargetParticipants>,
			/// Optional hash of evidence supporting the integration
			evidence_hash: Option<H256>,
			/// Block number when the integration was last updated
			last_updated: T::BlockNumber,
		},
		/// Disciplinary action registered
		DisciplinaryActionRegistered {
			discipline_id: T::Hash,
			subject: T::AccountId,
			level: DisciplineLevel,
			reason: BoundedVec<u8, T::MaxJustificationLength>,
		},
		/// Disciplinary action resolved
		DisciplinaryActionResolved {
			discipline_id: T::Hash,
			subject: T::AccountId,
			resolution_summary: BoundedVec<u8, T::MaxResolutionLength>,
			evidence_hash: Option<H256>,
		},
		/// Rank transition recorded
		RankTransitionRegistered {
			transition_id: T::Hash,
			member: T::AccountId,
			transition_type: TransitionType,
			previous_rank: Rank,
			new_rank: Rank,
			effective_at: T::BlockNumber,
			evidence_hash: Option<H256>,
		},
		/// Conflict of interest registered
		ConflictOfInterestSet {
			member: T::AccountId,
			conflict_type: ConflictType,
			description: BoundedVec<u8, T::MaxDescriptionLength>,
			relates_to: Option<BoundedVec<u8, T::MaxDescriptionLength>>,
			start_block: Option<T::BlockNumber>,
			end_block: Option<T::BlockNumber>,
			evidence_hash: Option<H256>,
			nonce: Option<u32>,
			last_updated: T::BlockNumber,
		},
		/// Critical conflict of interest detected during committee formation
		CriticalEmergencyConflictOfInterestDetected {
			emergency_id: T::Hash,
			member: T::AccountId,
			conflict_type: ConflictType,
		},
		/// Non-critical conflict of interest warning during committee formation
		NonCriticalEmergencyConflictOfInterestWarning {
			emergency_id: T::Hash,
			member: T::AccountId,
			conflict_type: ConflictType,
		},
		/// Critical conflict of interest detected during appeal committee formation
		CriticalAppealConflictDetected {
			appeal_id: T::Hash,
			member: T::AccountId,
			conflict_type: ConflictType,
		},
		/// Non-critical conflict of interest warning during appeal committee formation
		NonCriticalAppealConflictWarning {
			appeal_id: T::Hash,
			member: T::AccountId,
			conflict_type: ConflictType,
		},
		/// Critical conflict of interest detected during integration establishment
		CriticalIntegrationConflictDetected {
			integration_id: T::Hash,
			member: T::AccountId,
			conflict_type: ConflictType,
		},
		/// Non-critical conflict of interest warning during integration establishment
		NonCriticalIntegrationConflictWarning {
			integration_id: T::Hash,
			member: T::AccountId,
			conflict_type: ConflictType,
		},
		/// On-chain remark created or updated
		RemarkSet {
			author: T::AccountId,
			category: RemarkCategory,
			content: BoundedVec<u8, T::MaxRemarkContentLength>,
			evidence_hash: Option<H256>,
			nonce: u32,
			last_updated: T::BlockNumber,
		},
		/// Governance health metrics updated
		GovernanceHealthMetricsSet {
			participation_rate: u8,
			vote_concentration: u8,
			avg_response_time: T::BlockNumber,
			start_block: T::BlockNumber,
			end_block: T::BlockNumber,
			evidence_info: BoundedVec<u8, T::MaxEvidenceInfoLength>,
			evidence_hash: Option<H256>,
			last_updated: T::BlockNumber,
		},
		/// Professional service provider set
		ServiceProviderSet {
			/// Account of the provider (must have verified identity)
			provider_account: T::AccountId,
			/// Types of services offered
			service_types: BoundedVec<ProfessionalServiceType, T::MaxServiceTypes>,
			/// Evidence information supporting the provider's credentials
			evidence_info: BoundedVec<u8, T::MaxEvidenceInfoLength>,
			/// Optional hash of evidence supporting the provider's credentials
			evidence_hash: Option<H256>,
			/// Last updated block
			last_updated: T::BlockNumber,
		},
		/// Professional service referral created
		ServiceReferralSet {
			/// Unique identifier for the referral
			referral_id: T::Hash,
			/// Account that made the referral
			referrer: T::AccountId,
			/// Provider being referred to (account with verified identity)
			provider_account: T::AccountId,
			/// Types of service being referred
			service_types: BoundedVec<ProfessionalServiceType, T::MaxServiceTypes>,
			/// Description of the referral
			description: BoundedVec<u8, T::MaxDescriptionLength>,
			/// Whether the referrer disclosed receiving compensation for this referral
			/// (transparency requirement)
			compensation_disclosed: bool,
			/// Optional details of compensation if disclosed
			compensation_details: Option<BoundedVec<u8, T::MaxCompensationDetailsLength>>,
			/// Optional hash of evidence supporting the referral
			evidence_hash: Option<H256>,
			/// Last updated block
			last_updated: T::BlockNumber,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Emergency not found
		EmergencyNotFound,
		/// Emergency already resolved
		EmergencyAlreadyResolved,
		/// Emergency committee already formed
		CommitteeAlreadyFormed,
		/// Emergency committee not formed
		CommitteeNotFormed,
		/// Invalid block period
		InvalidBlockPeriod,
		/// Invalid committee composition
		InvalidCommitteeComposition,
		/// Not authorized origin
		NotAuthorizedOrigin,
		/// Not a committee member
		NotCommitteeMember,
		/// Too many committee members
		TooManyCommitteeMembers,
		/// Appeal not found
		AppealNotFound,
		/// Appeal already decided
		AppealAlreadyDecided,
		/// Appeal committee already formed
		AppealCommitteeAlreadyFormed,
		/// Appeal committee not formed
		AppealCommitteeNotFormed,
		/// Integration already exists
		IntegrationAlreadyExists,
		/// Integration not found
		IntegrationNotFound,
		/// Missing required role
		MissingRequiredRole,
		/// Conflict of interest not found
		ConflictOfInterestNotFound,
		/// Conflict of interest detected
		ConflictOfInterestDetected,
		/// Too many conflict of interest checks
		TooManyConflictOfInterestChecks,
		/// Remark not found
		RemarkNotFound,
		/// Service provider not found
		ServiceProviderNotFound,
		/// Service provider already registered
		ServiceProviderAlreadyRegistered,
		/// Identity not verified
		IdentityNotVerified,
		/// Insufficient rank for the operation
		InsufficientRank,
		/// Missing compensation disclosure
		MissingCompensationDisclosure,
		/// Disciplinary action not found
		DisciplinaryActionNotFound,
		/// Disciplinary action already resolved
		DisciplinaryActionAlreadyResolved,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Activate emergency protocol
		///
		/// Allows authorized users to declare an emergency and initiate
		/// the emergency response procedure.
		///
		/// Parameters:
		/// - `emergency_type`: Emergency type being declared
		/// - `severity`: Severity level of the emergency
		/// - `justification`: Justification for declaring the emergency, should include the
		///   location where any off-chain evidence is stored for future reference
		/// - `evidence_hash`: Optional hash of evidence supporting the emergency declaration
		///
		/// Emits `EmergencyActivated` event when successful.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::activate_emergency_protocol())]
		pub fn activate_emergency_protocol(
			origin: OriginFor<T>,
			emergency_type: EmergencyType,
			severity: EmergencySeverity,
			justification: BoundedVec<u8, T::MaxJustificationLength>,
			evidence_hash: Option<H256>,
		) -> DispatchResult {
			T::EmergencyOrigin::ensure_origin(origin.clone())?;
			let initiator = ensure_signed(origin)?;

			// Check that the caller has a verified identity
			ensure!(
				T::IdentityRegistrar::has_identity(&initiator),
				Error::<T>::IdentityNotVerified
			);

			ensure!(
				T::RankChecker::has_minimum_rank(
					&initiator,
					T::MinRankToActivateEmergencyProtocol::get()
				),
				Error::<T>::InsufficientRank
			);

			let emergency_details = EmergencyDetails {
				emergency_type: emergency_type.clone(),
				severity: severity.clone(),
				initiator: initiator.clone(),
				justification,
				declared_at: frame_system::Pallet::<T>::block_number().saturated_into(),
				resolved_at: None,
				evidence_hash,
				abuse_detected: false,
			};

			let emergency_id = T::Hashing::hash_of(&emergency_details);
			Emergencies::<T>::insert(emergency_id, emergency_details);

			Self::deposit_event(Event::EmergencyActivated {
				emergency_id,
				emergency_type,
				severity,
				initiator,
			});

			Ok(())
		}

		/// Form emergency committee
		///
		/// Allows authorized user to form an emergency committee
		/// to oversee the emergency response.
		///
		/// Parameters:
		/// - `emergency_id`: ID of the emergency
		/// - `members`: List of committee members and their roles
		///
		/// Emits `EmergencyCommitteeFormed` event when successful.
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::form_emergency_committee())]
		pub fn form_emergency_committee(
			origin: OriginFor<T>,
			emergency_id: T::Hash,
			members: BoundedVec<(T::AccountId, EmergencyRole), T::MaxEmergencyCommitteeMembers>,
		) -> DispatchResult {
			// Check origin using CommitteeFormationOrigin filter
			T::CommitteeFormationOrigin::ensure_origin(origin.clone())?;
			let who = ensure_signed(origin)?;

			// Check that the caller has a verified identity
			ensure!(T::IdentityRegistrar::has_identity(&who), Error::<T>::IdentityNotVerified);

			// Check that the caller has the minimum required rank
			ensure!(
				T::RankChecker::has_minimum_rank(&who, T::MinRankToFormEmergencyCommittee::get()),
				Error::<T>::InsufficientRank
			);

			ensure!(Emergencies::<T>::contains_key(emergency_id), Error::<T>::EmergencyNotFound);
			ensure!(
				!EmergencyCommittees::<T>::contains_key(emergency_id),
				Error::<T>::CommitteeAlreadyFormed
			);

			// Check for conflicts of interest for each proposed committee member
			let mut conflict_check_count: u32 = 0;
			let max_conflict_checks = T::MaxConflictOfInterestChecks::get();

			for (member, _) in &members {
				// Check if this member has conflicts of interest related to this emergency
				let emergency_id_str = format!("{:?}", emergency_id);

				for ((account_id, conflict_type, _), conflict) in Conflicts::<T>::iter() {
					// Increment the counter and check if we've exceeded the maximum
					conflict_check_count += 1;
					ensure!(
						conflict_check_count <= max_conflict_checks,
						Error::<T>::TooManyConflictOfInterestChecks
					);

					// Check if this conflict of interest belongs to a committee member and relates
					// to this emergency
					if account_id == *member &&
						conflict.relates_to.as_ref().map_or(false, |relates_to| {
							relates_to.as_slice() == emergency_id_str.as_bytes()
						}) {
						// Use the IsConflictCritical trait to determine if this is a critical
						// conflict
						if conflict_type.is_critical() {
							// Critical conflict detected that prevents committee formation
							Self::deposit_event(
								Event::CriticalEmergencyConflictOfInterestDetected {
									emergency_id,
									member: account_id.clone(),
									conflict_type: conflict_type.clone(),
								},
							);
							return Err(Error::<T>::ConflictOfInterestDetected.into());
						} else {
							// Non-critical conflict detected that emits warning but allows
							// committee formation
							Self::deposit_event(
								Event::NonCriticalEmergencyConflictOfInterestWarning {
									emergency_id,
									member: account_id.clone(),
									conflict_type: conflict_type.clone(),
								},
							);
						}
					}
				}
			}

			// Check committee composition requirements
			let mut has_technical_lead = false;
			let mut has_governance_rep = false;
			let mut has_independent_expert = false;

			for (_, role) in &members {
				match role {
					EmergencyRole::TechnicalLead => has_technical_lead = true,
					EmergencyRole::GovernanceRepresentative => has_governance_rep = true,
					EmergencyRole::IndependentExpert => has_independent_expert = true,
				}
			}

			ensure!(has_technical_lead, Error::<T>::MissingRequiredRole);
			ensure!(has_governance_rep, Error::<T>::MissingRequiredRole);
			ensure!(has_independent_expert, Error::<T>::MissingRequiredRole);

			EmergencyCommittees::<T>::insert(emergency_id, members.clone());

			Self::deposit_event(Event::EmergencyCommitteeFormed { emergency_id, members });

			Ok(())
		}

		/// Resolve emergency
		///
		/// Allows an authorized user or emergency committee member to resolve an emergency.
		///
		/// Parameters:
		/// - `emergency_id`: ID of the emergency
		/// - `abuse_detected`: Abuse detected during the emergency response or not
		/// - `resolution_summary`: Summary of the resolution actions, should include the location
		///   where any off-chain evidence is stored for future reference
		/// - `evidence_hash`: Optional hash of evidence supporting the resolution
		///
		/// Emits `EmergencyResolved` event when successful.
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::resolve_emergency())]
		pub fn resolve_emergency(
			origin: OriginFor<T>,
			emergency_id: T::Hash,
			abuse_detected: bool,
			resolution_summary: BoundedVec<u8, T::MaxResolutionLength>,
			evidence_hash: Option<H256>,
		) -> DispatchResult {
			let who = ensure_signed(origin.clone())?;

			// Check that the caller has a verified identity
			ensure!(T::IdentityRegistrar::has_identity(&who), Error::<T>::IdentityNotVerified);

			// Ensure the caller has at least the minimum rank required for emergency response
			// authority
			ensure!(
				T::RankChecker::has_minimum_rank(
					&who,
					T::MinRankForEmergencyResponseAuthority::get()
				),
				Error::<T>::InsufficientRank
			);

			Emergencies::<T>::try_mutate(emergency_id, |maybe_emergency| -> DispatchResult {
				let emergency = maybe_emergency.as_mut().ok_or(Error::<T>::EmergencyNotFound)?;
				ensure!(emergency.resolved_at.is_none(), Error::<T>::EmergencyAlreadyResolved);

				emergency.resolved_at =
					Some(frame_system::Pallet::<T>::block_number().saturated_into());
				emergency.abuse_detected = abuse_detected;
				emergency.evidence_hash = evidence_hash;

				Ok(())
			})?;

			// Check if origin is either EmergencyResolutionOrigin or a committee member
			let is_authorized_origin = T::EmergencyResolutionOrigin::ensure_origin(origin).is_ok();
			let is_committee_member =
				if let Some(committee) = EmergencyCommittees::<T>::get(emergency_id) {
					committee.iter().any(|(member, _)| member == &who)
				} else {
					false
				};

			ensure!(is_authorized_origin, Error::<T>::NotAuthorizedOrigin);
			ensure!(is_committee_member, Error::<T>::NotCommitteeMember);

			Self::deposit_event(Event::EmergencyResolved {
				emergency_id,
				abuse_detected,
				resolution_summary,
			});

			Ok(())
		}

		/// Register disciplinary action
		///
		/// Allows authorized users to register taking disciplinary action against a member.
		/// This extrinsic only documents the disciplinary action for governance transparency and
		/// accountability.
		///
		/// Parameters:
		/// - `subject`: Account ID of the member to be disciplined
		/// - `level`: Level of discipline to be applied
		/// - `reason`: Reason for the disciplinary action, must include the location where any
		///   off-chain evidence (referenced by the `evidence_hash`) is stored
		/// - `duration`: Duration of the disciplinary action (if applicable)
		/// - `evidence_hash`: Optional hash of evidence supporting the disciplinary action. The
		///   actual evidence is stored off-chain, and its location should be referenced in the
		///   reason field
		///
		/// Emits `DisciplinaryActionRegistered` event when successful.
		#[pallet::call_index(7)]
		#[pallet::weight(T::WeightInfo::register_disciplinary_action())]
		pub fn register_disciplinary_action(
			origin: OriginFor<T>,
			subject: T::AccountId,
			level: DisciplineLevel,
			reason: BoundedVec<u8, T::MaxJustificationLength>,
			duration: Option<T::BlockNumber>,
			evidence_hash: Option<H256>,
		) -> DispatchResult {
			T::EmergencyOrigin::ensure_origin(origin.clone())?;
			let issuer = ensure_signed(origin.clone())?;

			// Check that the caller has a verified identity
			ensure!(T::IdentityRegistrar::has_identity(&issuer), Error::<T>::IdentityNotVerified);

			ensure!(
				T::RankChecker::has_minimum_rank(
					&issuer,
					T::MinRankForDisciplinaryActionEnforcement::get()
				),
				Error::<T>::InsufficientRank
			);

			let discipline_details = DisciplineDetails {
				subject: subject.clone(),
				issuer,
				level: level.clone(),
				reason: reason.clone(),
				issued_at: frame_system::Pallet::<T>::block_number().saturated_into(),
				duration,
				evidence_hash,
				active: true,
			};

			let discipline_id = T::Hashing::hash_of(&discipline_details);

			Disciplines::<T>::insert(discipline_id, discipline_details);

			Self::deposit_event(Event::DisciplinaryActionRegistered {
				discipline_id,
				subject,
				level,
				reason,
			});

			Ok(())
		}

		/// Resolve disciplinary action
		///
		/// Allows authorized users to resolve a disciplinary action, setting its active status to
		/// false. This follows the Progressive Enforcement pattern from the Ambassador Fellowship
		/// Manifesto, providing a remediation path for correction.
		///
		/// Parameters:
		/// - `discipline_id`: ID of the disciplinary action to resolve
		/// - `resolution_summary`: Summary of the resolution, should include the location where any
		///   off-chain evidence is stored for future reference
		/// - `evidence_hash`: Optional hash of evidence supporting the resolution
		///
		/// Emits `DisciplinaryActionResolved` event when successful.
		#[pallet::call_index(8)]
		#[pallet::weight(T::WeightInfo::resolve_disciplinary_action())]
		pub fn resolve_disciplinary_action(
			origin: OriginFor<T>,
			discipline_id: T::Hash,
			resolution_summary: BoundedVec<u8, T::MaxResolutionLength>,
			evidence_hash: Option<H256>,
		) -> DispatchResult {
			T::EmergencyOrigin::ensure_origin(origin.clone())?;
			let resolver = ensure_signed(origin.clone())?;

			// Check that the caller has a verified identity
			ensure!(T::IdentityRegistrar::has_identity(&resolver), Error::<T>::IdentityNotVerified);

			// Ensure the caller has sufficient rank
			ensure!(
				T::RankChecker::has_minimum_rank(
					&resolver,
					T::MinRankForDisciplinaryActionEnforcement::get()
				),
				Error::<T>::InsufficientRank
			);

			// Ensure the disciplinary action exists
			ensure!(
				Disciplines::<T>::contains_key(discipline_id),
				Error::<T>::DisciplinaryActionNotFound
			);

			// Update the disciplinary action to set active to false
			Disciplines::<T>::try_mutate(discipline_id, |maybe_discipline| -> DispatchResult {
				let discipline =
					maybe_discipline.as_mut().ok_or(Error::<T>::DisciplinaryActionNotFound)?;

				// Ensure the disciplinary action is currently active
				ensure!(discipline.active, Error::<T>::DisciplinaryActionAlreadyResolved);

				// Set the disciplinary action as inactive
				discipline.active = false;

				Ok(())
			})?;

			// Get the subject from the disciplinary action
			let subject = Disciplines::<T>::get(discipline_id)
				.map(|d| d.subject)
				.ok_or(Error::<T>::DisciplinaryActionNotFound)?;

			// Emit event
			Self::deposit_event(Event::DisciplinaryActionResolved {
				discipline_id,
				subject,
				resolution_summary,
				evidence_hash,
			});

			Ok(())
		}

		/// Submit appeal
		///
		/// Allows any Ambassador to submit an appeal against a decision.
		///
		/// Parameters:
		/// - `original_decision`: Description of the original decision being appealed
		/// - `justification`: Justification for the appeal, should include the location where any
		///   off-chain evidence is stored for future reference
		/// - `evidence_hash`: Optional hash of evidence supporting the appeal
		///
		/// Emits `AppealSubmitted` event when successful.
		#[pallet::call_index(9)]
		#[pallet::weight(T::WeightInfo::submit_appeal())]
		pub fn submit_appeal(
			origin: OriginFor<T>,
			original_decision: BoundedVec<u8, T::MaxJustificationLength>,
			justification: BoundedVec<u8, T::MaxJustificationLength>,
			evidence_hash: Option<H256>,
		) -> DispatchResult {
			T::AppealSubmissionOrigin::ensure_origin(origin.clone())?;
			let appellant = ensure_signed(origin)?;

			// Check that the appellant has a verified identity
			ensure!(
				T::IdentityRegistrar::has_identity(&appellant),
				Error::<T>::IdentityNotVerified
			);

			// Check that the appellant has the minimum required rank
			ensure!(
				T::RankChecker::has_minimum_rank(&appellant, T::MinRankToSubmitAppeal::get()),
				Error::<T>::InsufficientRank
			);

			let appeal_details = AppealDetails {
				appellant: appellant.clone(),
				original_decision: original_decision.clone(),
				justification,
				submitted_at: frame_system::Pallet::<T>::block_number().saturated_into(),
				status: AppealStatus::Submitted,
				decision: None,
				decided_at: None,
				evidence_hash,
			};

			let appeal_id = T::Hashing::hash_of(&appeal_details);

			Appeals::<T>::insert(appeal_id, appeal_details);

			Self::deposit_event(Event::AppealSubmitted { appeal_id, appellant, original_decision });

			Ok(())
		}

		/// Form appeal committee
		///
		/// Allows an authorized user to form an appeal committee to review an appeal.
		///
		/// Parameters:
		/// - `appeal_id`: ID of the appeal
		/// - `members`: List of committee members
		///
		/// Emits `AppealCommitteeFormed` event when successful.
		#[pallet::call_index(10)]
		#[pallet::weight(T::WeightInfo::form_appeal_committee())]
		pub fn form_appeal_committee(
			origin: OriginFor<T>,
			appeal_id: T::Hash,
			members: BoundedVec<T::AccountId, T::MaxAppealCommitteeMembers>,
		) -> DispatchResult {
			T::AppealCommitteeOrigin::ensure_origin(origin.clone())?;
			let who = ensure_signed(origin)?;

			// Check that the caller has a verified identity
			ensure!(T::IdentityRegistrar::has_identity(&who), Error::<T>::IdentityNotVerified);

			// Check that the caller has the minimum required rank
			ensure!(
				T::RankChecker::has_minimum_rank(&who, T::MinRankToFormAppealCommittee::get()),
				Error::<T>::InsufficientRank
			);

			ensure!(Appeals::<T>::contains_key(appeal_id), Error::<T>::AppealNotFound);
			ensure!(
				!AppealCommittees::<T>::contains_key(appeal_id),
				Error::<T>::AppealCommitteeAlreadyFormed
			);

			// Check for conflicts of interest for each proposed committee member
			let mut conflict_check_count: u32 = 0;
			let max_conflict_checks = T::MaxConflictOfInterestChecks::get();

			for member in &members {
				// Check if this member has conflicts of interest related to this appeal
				let appeal_id_str = format!("{:?}", appeal_id);

				for ((account_id, conflict_type, _), conflict) in Conflicts::<T>::iter() {
					// Increment the counter and check if we've exceeded the maximum
					conflict_check_count += 1;
					ensure!(
						conflict_check_count <= max_conflict_checks,
						Error::<T>::TooManyConflictOfInterestChecks
					);

					// Check if this conflict belongs to a committee member and relates to this
					// appeal
					if account_id == *member &&
						conflict.relates_to.as_ref().map_or(false, |relates_to| {
							relates_to.as_slice() == appeal_id_str.as_bytes()
						}) {
						// Determine if this is a critical conflict that prevents committee
						// participation
						if conflict_type.is_critical() {
							// Critical conflict detected that prevents committee formation
							Self::deposit_event(Event::CriticalAppealConflictDetected {
								appeal_id,
								member: account_id.clone(),
								conflict_type: conflict_type.clone(),
							});
							return Err(Error::<T>::ConflictOfInterestDetected.into());
						} else {
							// Non-critical conflict so emit warning but allow committee formation
							Self::deposit_event(Event::NonCriticalAppealConflictWarning {
								appeal_id,
								member: account_id.clone(),
								conflict_type: conflict_type.clone(),
							});
						}
					}
				}
			}

			AppealCommittees::<T>::insert(appeal_id, members.clone());

			// Update appeal status
			Appeals::<T>::try_mutate(appeal_id, |maybe_appeal| -> DispatchResult {
				let appeal = maybe_appeal.as_mut().ok_or(Error::<T>::AppealNotFound)?;
				appeal.status = AppealStatus::UnderReview;
				Ok(())
			})?;

			Self::deposit_event(Event::AppealCommitteeFormed { appeal_id, members });

			Ok(())
		}

		/// Decide on appeal
		///
		/// Allows an appeal committee member to decide on an appeal.
		///
		/// Parameters:
		/// - `appeal_id`: ID of the appeal
		/// - `decision`: Decision on the appeal
		/// - `justification`: Justification for the decision (includes reference to off-chain
		///   evidence)
		/// - `evidence_hash`: Hash of evidence supporting the decision
		///
		/// Emits `AppealDecided` event when successful.
		#[pallet::call_index(11)]
		#[pallet::weight(T::WeightInfo::decide_appeal())]
		pub fn decide_appeal(
			origin: OriginFor<T>,
			appeal_id: T::Hash,
			decision: AppealDecision,
			justification: BoundedVec<u8, T::MaxJustificationLength>,
			evidence_hash: Option<H256>,
		) -> DispatchResult {
			let who = ensure_signed(origin.clone())?;

			// Check that the caller has a verified identity
			ensure!(T::IdentityRegistrar::has_identity(&who), Error::<T>::IdentityNotVerified);

			// Check that the caller has at least the minimum rank required for decide appeal
			ensure!(
				T::RankChecker::has_minimum_rank(&who, T::MinRankForDecideAppeal::get()),
				Error::<T>::InsufficientRank
			);
			// Check if origin is a committee member
			let committee = AppealCommittees::<T>::get(appeal_id)
				.ok_or(Error::<T>::AppealCommitteeNotFormed)?;
			ensure!(committee.contains(&who), Error::<T>::NotCommitteeMember);

			Appeals::<T>::try_mutate(appeal_id, |maybe_appeal| -> DispatchResult {
				let appeal = maybe_appeal.as_mut().ok_or(Error::<T>::AppealNotFound)?;
				ensure!(appeal.decision.is_none(), Error::<T>::AppealAlreadyDecided);

				appeal.status = AppealStatus::Decided;
				appeal.decision = Some(decision.clone());
				appeal.decided_at =
					Some(frame_system::Pallet::<T>::block_number().saturated_into());
				appeal.justification = justification.clone();
				appeal.evidence_hash = evidence_hash;

				Ok(())
			})?;

			Self::deposit_event(Event::AppealDecided {
				appeal_id,
				decider: who,
				decision,
				justification,
				evidence_hash,
			});

			Ok(())
		}

		/// Establish integration with another collective
		///
		/// Establishes formal integration between the Ambassador Fellowship and another collective.
		///
		/// Parameters:
		/// - `mechanism`: Integration mechanism defining how the collectives will work together.
		///   Determines the governance structure and operational model of the integration.
		/// - `target_collective`: Target collective to integrate with
		/// - `description`: Description of the integration purpose, objectives, and scope. Should
		///   include the location where any off-chain agreement is stored for future reference.
		/// - `ambassador_participants`: List of Ambassador Fellowship participants who will be
		///   involved in the integration activities and governance.
		/// - `target_participants`: List of participants from the target collective who will be
		///   collaborating with the Ambassador Fellowship.
		/// - `evidence_hash`: Optional evidence hash of the formal integration agreement document.
		///   Creates an on-chain reference to the off-chain legal or governance document when
		///   provided. Storage location of this document should be specified in the `description`
		///   parameter.
		///
		/// Emits `IntegrationEstablished` event when successful.
		///
		/// # Examples
		///
		/// - Joint Working Group: Create a working group between Ambassador Fellowship and
		///   Technical Fellowship
		/// - Liaison System: Establish liaisons between Ambassador Fellowship and Secretary
		///   Collective
		/// - Integrated Planning: Create joint planning cycles with other collectives
		/// - Knowledge Sharing: Establish knowledge sharing between Ambassador Fellowship and other
		///   collectives
		#[pallet::call_index(12)]
		#[pallet::weight(T::WeightInfo::establish_integration())]
		pub fn establish_integration(
			origin: OriginFor<T>,
			mechanism: IntegrationMechanism,
			target_collective: TargetCollective,
			description: BoundedVec<u8, T::MaxDescriptionLength>,
			ambassador_participants: BoundedVec<T::AccountId, T::MaxSourceParticipants>,
			target_participants: BoundedVec<T::AccountId, T::MaxTargetParticipants>,
			evidence_hash: Option<H256>,
		) -> DispatchResult {
			T::IntegrationOrigin::ensure_origin(origin.clone())?;
			let who = ensure_signed(origin)?;

			// Check that the caller has a verified identity
			ensure!(T::IdentityRegistrar::has_identity(&who), Error::<T>::IdentityNotVerified);

			// Check that the caller has the minimum required rank
			ensure!(
				T::RankChecker::has_minimum_rank(&who, T::MinRankToEstablishIntegration::get()),
				Error::<T>::InsufficientRank
			);

			// Create integration details first to get the ID for conflict checking
			let integration_details = IntegrationDetails {
				mechanism: mechanism.clone(),
				target_collective: target_collective.clone(),
				description: description.clone(),
				ambassador_participants: ambassador_participants.clone(),
				target_participants: target_participants.clone(),
				established_at: frame_system::Pallet::<T>::block_number().saturated_into(),
				evidence_hash,
			};

			let integration_id = T::Hashing::hash_of(&integration_details);

			// Ensure the integration doesn't already exist
			ensure!(
				!Integrations::<T>::contains_key(integration_id),
				Error::<T>::IntegrationAlreadyExists
			);

			// Check for conflicts of interest for each ambassador participant
			let mut conflict_check_count: u32 = 0;
			let max_conflict_checks = T::MaxConflictOfInterestChecks::get();

			for participant in &ambassador_participants {
				// Check if this participant has conflicts of interest related to this integration
				let integration_id_str = format!("{:?}", integration_id);

				for ((account_id, conflict_type, _), conflict) in Conflicts::<T>::iter() {
					// Increment the counter and check if we've exceeded the maximum
					conflict_check_count += 1;
					ensure!(
						conflict_check_count <= max_conflict_checks,
						Error::<T>::TooManyConflictOfInterestChecks
					);

					// Check if this conflict belongs to a participant and relates to this
					// integration
					if account_id == *participant &&
						conflict.relates_to.as_ref().map_or(false, |relates_to| {
							relates_to.as_slice() == integration_id_str.as_bytes()
						}) {
						// Use the IsConflictCritical trait to determine if this is a critical
						// conflict
						if conflict_type.is_critical() {
							// Critical conflict detected that prevents integration establishment
							Self::deposit_event(Event::CriticalIntegrationConflictDetected {
								integration_id,
								member: account_id.clone(),
								conflict_type: conflict_type.clone(),
							});
							return Err(Error::<T>::ConflictOfInterestDetected.into());
						} else {
							// Non-critical conflict detected that emits warning but allows
							// integration
							Self::deposit_event(Event::NonCriticalIntegrationConflictWarning {
								integration_id,
								member: account_id.clone(),
								conflict_type: conflict_type.clone(),
							});
						}
					}
				}
			}

			// Store the integration details
			Integrations::<T>::insert(integration_id, integration_details);

			Self::deposit_event(Event::IntegrationEstablished {
				integration_id,
				mechanism,
				target_collective,
				description,
				ambassador_participants,
				target_participants: target_participants.clone(),
				evidence_hash,
				last_updated: frame_system::Pallet::<T>::block_number().saturated_into(),
			});

			Ok(())
		}

		/// Record rank transition
		///
		/// Allows authorized users to record the intent for a rank transition for a member, but
		/// does NOT actually execute the rank change. This extrinsic only documents the
		/// transition for governance transparency and accountability.
		///
		/// IMPORTANT: This function only records the transition details on-chain. The actual rank
		/// change must be executed separately using the appropriate mechanism:
		/// - For standard promotions and demotions, use the `promote_member` and `demote_member`
		///   extrinsics from the ranked-collective-ambassador pallet
		/// - For complex transitions recorded with this extrinsic, a manual process must be
		///   followed to execute the actual rank change once the `effective_at` block is reached
		///
		/// Parameters:
		/// - `member`: Account ID of the member undergoing transition
		/// - `transition_type`: Type of transition
		/// - `previous_rank`: Previous rank of the member
		/// - `new_rank`: New rank of the member
		/// - `justification`: Justification for the transition, must include the location where any
		///   off-chain evidence or documentation is stored for future reference
		/// - `effective_at`: Block when the transition will be completed
		/// - `successor`: Successor account (if applicable)
		/// - `evidence_hash`: Optional hash of the evidence supporting this transition. The actual
		///   evidence is stored off-chain, and its location should be referenced in the
		///   `justification` field
		///
		/// Emits `RankTransitionRegistered` event when successful.
		#[pallet::call_index(13)]
		#[pallet::weight(T::WeightInfo::register_rank_transition())]
		pub fn register_rank_transition(
			origin: OriginFor<T>,
			member: T::AccountId,
			transition_type: TransitionType,
			previous_rank: Rank,
			new_rank: Rank,
			justification: BoundedVec<u8, T::MaxJustificationLength>,
			effective_at: T::BlockNumber,
			successor: Option<T::AccountId>,
			evidence_hash: Option<H256>,
		) -> DispatchResult {
			T::EmergencyOrigin::ensure_origin(origin.clone())?;
			let initiator = ensure_signed(origin.clone())?;

			// Check that the caller has a verified identity
			ensure!(
				T::IdentityRegistrar::has_identity(&initiator),
				Error::<T>::IdentityNotVerified
			);

			// Ensure the caller has at least the minimum rank required for role fulfillment
			// contingency
			ensure!(
				T::RankChecker::has_minimum_rank(
					&initiator,
					T::MinRankForRoleFulfillmentContingency::get()
				),
				Error::<T>::InsufficientRank
			);

			let transition_details = TransitionDetails {
				member: member.clone(),
				transition_type: transition_type.clone(),
				previous_rank,
				new_rank,
				justification,
				initiated_at: frame_system::Pallet::<T>::block_number().saturated_into(),
				effective_at,
				successor,
				knowledge_transfer_complete: false,
				evidence_hash,
			};

			let transition_id = T::Hashing::hash_of(&transition_details);

			Transitions::<T>::insert(transition_id, transition_details);

			Self::deposit_event(Event::RankTransitionRegistered {
				transition_id,
				member,
				transition_type,
				previous_rank,
				new_rank,
				effective_at,
				evidence_hash,
			});

			Ok(())
		}

		/// Set conflict of interest
		///
		/// Allows members to set or update conflicts of interest.
		///
		/// Parameters:
		/// - `conflict_type`: Type of conflict
		/// - `description`: Description of the conflict, should include the location where any
		///   off-chain evidence is stored for future reference
		/// - `relates_to`: Optionally the related matter or decision
		/// - `start_block`: Optionally the block when conflict starts otherwise the current block
		///   used
		/// - `end_block`: Optionally the block when conflict expires (if applicable)
		/// - `evidence_hash`: Optional hash of evidence supporting the conflict declaration
		/// - `nonce`: Optionally the nonce for tracking updates
		///
		/// Emits `ConflictOfInterestSet` event when successful.
		#[pallet::call_index(14)]
		#[pallet::weight(T::WeightInfo::set_conflict_of_interest())]
		pub fn set_conflict_of_interest(
			origin: OriginFor<T>,
			conflict_type: ConflictType,
			description: BoundedVec<u8, T::MaxDescriptionLength>,
			relates_to: Option<BoundedVec<u8, T::MaxDescriptionLength>>,
			start_block: Option<T::BlockNumber>,
			end_block: Option<T::BlockNumber>,
			evidence_hash: Option<H256>,
			nonce: Option<u32>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Check that the caller has a verified identity
			ensure!(T::IdentityRegistrar::has_identity(&who), Error::<T>::IdentityNotVerified);

			// Ensure the caller has at least the minimum rank required for transparency mechanisms
			ensure!(
				T::RankChecker::has_minimum_rank(&who, T::MinRankToSetConflictOfInterest::get()),
				Error::<T>::InsufficientRank
			);

			// Get the current block number
			let current_block = frame_system::Pallet::<T>::block_number().saturated_into();

			// Determine the start block (default to current block if not provided)
			let start_block = start_block.unwrap_or(current_block);

			// Handle nonce for new or existing conflict
			let nonce_to_use = if let Some(specific_nonce) = nonce {
				// Verify the conflict exists if updating
				ensure!(
					Conflicts::<T>::contains_key((
						who.clone(),
						conflict_type.clone(),
						specific_nonce
					)),
					Error::<T>::ConflictOfInterestNotFound
				);
				specific_nonce
			} else {
				// Create a new conflict with the next nonce
				let author_type_key = (who.clone(), conflict_type.clone());
				let next_nonce = ConflictNonces::<T>::get(author_type_key.clone());
				ConflictNonces::<T>::insert(author_type_key, next_nonce + 1);
				next_nonce
			};

			// Create the conflict registration
			let conflict_registration = ConflictRegistration {
				member: who.clone(),
				conflict_type: conflict_type.clone(),
				description: description.clone(),
				relates_to: relates_to.clone(),
				start_block: Some(start_block),
				end_block,
				evidence_hash,
				last_updated: current_block,
			};

			// Store using composite key
			Conflicts::<T>::insert(
				(who.clone(), conflict_type.clone(), nonce_to_use),
				conflict_registration,
			);

			Self::deposit_event(Event::ConflictOfInterestSet {
				member: who,
				conflict_type,
				description,
				relates_to,
				start_block: Some(start_block),
				end_block,
				evidence_hash,
				nonce: Some(nonce_to_use),
				last_updated: current_block,
			});

			Ok(())
		}

		/// Create or update an on-chain remark
		///
		/// Allows authorized users to create or update on-chain remarks for governance
		/// transparency. If a remark with the same author, category, and optionally provided
		/// nonce already exists, it will be updated.
		///
		/// Parameters:
		/// - `category`: Category of the remark
		/// - `content`: Content of the remark, must include the location where any off-chain
		///   evidence (referenced by the `evidence_hash`) is stored for future reference and
		///   auditability
		/// - `evidence_hash`: Optional hash of evidence supporting the remark. The actual evidence
		///   is stored off-chain, and its location should be referenced in the `content` field
		/// - `nonce`: Optional to update a specific nonce
		///
		/// Emits `RemarkSet` event when successful.
		#[pallet::call_index(15)]
		#[pallet::weight(T::WeightInfo::set_remark())]
		pub fn set_remark(
			origin: OriginFor<T>,
			category: RemarkCategory,
			content: BoundedVec<u8, T::MaxRemarkContentLength>,
			evidence_hash: Option<H256>,
			nonce: Option<u32>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Check that the caller has a verified identity
			ensure!(T::IdentityRegistrar::has_identity(&who), Error::<T>::IdentityNotVerified);

			// Check that the caller has the minimum required rank
			ensure!(
				T::RankChecker::has_minimum_rank(&who, T::MinRankToSetRemark::get()),
				Error::<T>::InsufficientRank
			);

			// Get the current block number
			let current_block = frame_system::Pallet::<T>::block_number().saturated_into();

			// Get the key for the author/category pair
			let author_category_key = (who.clone(), category.clone());

			// Determine which nonce to use
			let nonce_to_update = if let Some(specific_nonce) = nonce {
				// Check if the specified nonce exists
				ensure!(
					Remarks::<T>::contains_key((who.clone(), category.clone(), specific_nonce)),
					Error::<T>::RemarkNotFound
				);
				specific_nonce
			} else {
				// Get the next nonce for this author/category pair
				let next_nonce = RemarkNonces::<T>::get(author_category_key.clone());
				// Increment the nonce for future remarks
				RemarkNonces::<T>::insert(author_category_key, next_nonce + 1);
				next_nonce
			};

			// Create or update the remark
			let on_chain_remark = OnChainRemark {
				content: content.clone(),
				evidence_hash,
				last_updated: current_block,
			};

			Remarks::<T>::insert((who.clone(), category.clone(), nonce_to_update), on_chain_remark);

			Self::deposit_event(Event::RemarkSet {
				author: who,
				category,
				content,
				evidence_hash,
				nonce: nonce_to_update,
				last_updated: current_block,
			});

			Ok(())
		}

		/// Set governance health metrics
		///
		/// Allows authorized users to set governance health metrics.
		///
		/// Parameters:
		/// - `participation_rate`: Participation rate (0-100%) percentage of eligible members who
		///   participated in governance activities
		/// - `vote_concentration`: Vote concentration index (0-100%) measures how concentrated
		///   voting power is among participants. Lower values indicate more equal distribution of
		///   votes, while higher values indicate votes are concentrated among fewer participants
		/// - `avg_response_time`: Average response time in blocks is the average time taken to
		///   respond to governance actions
		/// - `start_block`: Starting block number of the period these metrics cover
		/// - `end_block`: Ending block number of the period these metrics cover
		/// - `evidence_info`: Evidence information, must include the location where any off-chain
		///   evidence is stored for future reference and auditability
		/// - `evidence_hash`: Optional hash of the evidence supporting the reported metrics. The
		///   actual evidence is stored off-chain, and its location should be referenced in the
		///   evidence_info field
		///
		/// Emits `GovernanceHealthMetricsSet` event when successful.
		#[pallet::call_index(16)]
		#[pallet::weight(T::WeightInfo::set_governance_health_metrics())]
		pub fn set_governance_health_metrics(
			origin: OriginFor<T>,
			participation_rate: u8,
			vote_concentration: u8,
			avg_response_time: T::BlockNumber,
			start_block: T::BlockNumber,
			end_block: T::BlockNumber,
			evidence_info: BoundedVec<u8, T::MaxEvidenceInfoLength>,
			evidence_hash: Option<H256>,
		) -> DispatchResult {
			T::EmergencyOrigin::ensure_origin(origin.clone())?;
			let who = ensure_signed(origin.clone())?;

			// Check that the caller has a verified identity
			ensure!(T::IdentityRegistrar::has_identity(&who), Error::<T>::IdentityNotVerified);
			ensure!(end_block > start_block, Error::<T>::InvalidBlockPeriod);

			// Get the current block number
			let current_block = frame_system::Pallet::<T>::block_number().saturated_into();

			GovernanceHealth::<T>::put(GovernanceHealthMetrics {
				participation_rate,
				vote_concentration,
				avg_response_time,
				start_block,
				end_block,
				evidence_info: evidence_info.clone(),
				evidence_hash,
				last_updated: current_block,
			});

			Self::deposit_event(Event::GovernanceHealthMetricsSet {
				participation_rate,
				vote_concentration,
				avg_response_time,
				start_block,
				end_block,
				evidence_info,
				evidence_hash,
				last_updated: current_block,
			});

			Ok(())
		}

		/// Set service provider
		///
		/// Allows authorized users to register a new service provider or update an existing one.
		///
		/// Parameters:
		/// - `provider_account`: Account of the provider (must have verified identity)
		/// - `service_types`: Types of services offered
		/// - `evidence_info`: Evidence information supporting the provider's credentials, must
		///   include the location where any off-chain evidence is stored for future reference and
		///   auditability
		/// - `evidence_hash`: Optional hash of evidence supporting the provider's credentials,
		///   where the actual evidence is stored off-chain, and its location should be referenced
		///   in the evidence_info field
		///
		/// Emits `ServiceProviderSet` event when successful.
		#[pallet::call_index(17)]
		#[pallet::weight(T::WeightInfo::set_service_provider())]
		pub fn set_service_provider(
			origin: OriginFor<T>,
			provider_account: T::AccountId,
			service_types: BoundedVec<ProfessionalServiceType, T::MaxServiceTypes>,
			evidence_info: BoundedVec<u8, T::MaxEvidenceInfoLength>,
			evidence_hash: Option<H256>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Check that the caller has a verified identity
			ensure!(T::IdentityRegistrar::has_identity(&who), Error::<T>::IdentityNotVerified);

			// Ensure the caller has at least the minimum rank required to register a service
			// provider
			ensure!(
				T::RankChecker::has_minimum_rank(&who, T::MinRankForProviderRegistry::get()),
				Error::<T>::InsufficientRank
			);

			// Ensure the provider account has a verified identity
			ensure!(
				T::IdentityRegistrar::has_identity(&provider_account),
				Error::<T>::IdentityNotVerified
			);

			// Get the current block number
			let current_block = frame_system::Pallet::<T>::block_number().saturated_into();

			// Create or update the provider details
			let provider_details = ServiceProviderDetails {
				provider_account: provider_account.clone(),
				service_types: service_types.clone(),
				evidence_info: evidence_info.clone(),
				evidence_hash,
				last_updated: current_block,
			};

			// Set or update the provider
			ServiceProviders::<T>::insert(&provider_account, provider_details);

			Self::deposit_event(Event::ServiceProviderSet {
				provider_account,
				service_types,
				evidence_info,
				evidence_hash,
				last_updated: current_block,
			});

			Ok(())
		}

		/// Set professional service referral
		///
		/// Allows ambassadors to create referrals to registered professional service providers.
		///
		/// Parameters:
		/// - `provider_account`: Account of the service provider (must be registered)
		/// - `service_types`: Types of professional services being referred
		/// - `description`: Description of the referral
		/// - `compensation_disclosed`: Whether the referrer disclosed receiving compensation for
		///   this referral (transparency requirement)
		/// - `compensation_details`: Details of compensation if disclosed
		/// - `evidence_hash`: Optional hash of evidence supporting the referral
		///
		/// Emits `ServiceReferralSet` event when successful.
		#[pallet::call_index(18)]
		#[pallet::weight(T::WeightInfo::set_service_referral())]
		pub fn set_service_referral(
			origin: OriginFor<T>,
			provider_account: T::AccountId,
			service_types: BoundedVec<ProfessionalServiceType, T::MaxServiceTypes>,
			description: BoundedVec<u8, T::MaxDescriptionLength>,
			compensation_disclosed: bool,
			compensation_details: Option<BoundedVec<u8, T::MaxCompensationDetailsLength>>,
			evidence_hash: Option<H256>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Check that the caller has a verified identity
			ensure!(T::IdentityRegistrar::has_identity(&who), Error::<T>::IdentityNotVerified);

			// Ensure the caller has at least the minimum rank required to create a referral
			ensure!(
				T::RankChecker::has_minimum_rank(&who, T::MinRankForReferral::get()),
				Error::<T>::InsufficientRank
			);

			// Ensure the provider exists
			ensure!(
				ServiceProviders::<T>::contains_key(&provider_account),
				Error::<T>::ServiceProviderNotFound
			);

			// If compensation is received, ensure details are provided
			if compensation_disclosed {
				ensure!(compensation_details.is_some(), Error::<T>::MissingCompensationDisclosure);
			}

			// Get the current block number
			let current_block = frame_system::Pallet::<T>::block_number().saturated_into();

			let referral = ServiceReferral {
				referrer: who.clone(),
				provider_account: provider_account.clone(),
				service_types: service_types.clone(),
				description: description.clone(),
				compensation_disclosed,
				compensation_details: compensation_details.clone(),
				evidence_hash,
				last_updated: current_block,
			};

			// Store the referral
			let referral_id = T::Hashing::hash_of(&referral);
			ServiceReferrals::<T>::insert(referral_id, referral);

			Self::deposit_event(Event::ServiceReferralSet {
				referral_id,
				referrer: who,
				provider_account,
				service_types,
				description: description.clone(),
				compensation_disclosed,
				compensation_details: compensation_details.clone(),
				evidence_hash,
				last_updated: current_block,
			});

			Ok(())
		}
	}
}
