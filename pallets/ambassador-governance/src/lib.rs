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
	traits::{Hash, SaturatedConversion},
	RuntimeDebug,
};
use sp_std::prelude::*;
use sp_std::vec::Vec;

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
	/// Examples: Active exploits, significant fund loss, or severe reputation damage already occurring.
	Critical,
	/// High requires expedited approval
	///
	/// High severity emergencies represent significant threats that require
	/// expedited response within 1-2 days.
	///
	/// Examples: Vulnerabilities with high likelihood of exploitation or substantial impact if exploited.
	High,
	/// Medium follows standard approval process
	///
	/// Medium severity emergencies follow the standard approval process with
	/// response times of several days.
	///
	/// Examples: Important but non-critical issues that can be addressed through normal governance procedures.
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
	/// at a certain interval to coordinate on technical governance decisions affecting both collectives.
	JointGovernanceCouncil,
	/// Liaison System: Designated points of contact between collectives
	///
	/// Example: Certain number of Ambassador Fellows serving as liaisons to the Technical Fellowship,
	/// attending their meetings and facilitating regular communication channels.
	LiaisonSystem,
	/// Integrated Planning Cycles: Synchronized roadmaps and planning processes
	///
	/// Example: Certain number of Ambassador Fellows and Technical Fellows synchronizing quarterly planning
	/// with joint kickoff sessions and regular alignment check-ins.
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
	evidence: Option<H256>,
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
	evidence: Option<H256>,
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
pub struct IntegrationDetails<BlockNumber, BoundedString, BoundedVecT> {
	/// Integration mechanism
	mechanism: IntegrationMechanism,
	/// Collective being integrated with
	target_collective: TargetCollective,
	/// Integration description
	description: BoundedString,
	/// Participants from Ambassador Fellowship
	ambassador_participants: BoundedVecT,
	/// Participants from target collective
	target_participants: BoundedVecT,
	/// Block when integration was established
	established_at: BlockNumber,
	/// Integration agreement hash
	agreement_hash: Option<H256>,
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
	/// Voluntary exit from the fellowship
	VoluntaryExit,
	/// Promotion to a higher rank
	///
	/// Member advances to a higher rank within the Ambassador Fellowship hierarchy.
	Promotion,
	/// Demotion to a lower rank
	///
	/// Member moves to a lower rank within the Ambassador Fellowship hierarchy.
	Demotion,
	/// Role change within same rank
	///
	/// Member changes responsibilities while maintaining the same rank.
	RoleChange,
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
	/// Financial interest in outcome
	///
	/// Member has a financial stake in the outcome of a decision.
	Financial,
	/// Personal relationship with involved parties
	///
	/// Member has a personal relationship with individuals affected by a decision.
	Personal,
	/// Professional relationship with involved parties
	///
	/// Member has a professional relationship with individuals or organizations affected by a decision.
	Professional,
	/// Ideological bias
	///
	/// Member has strong ideological views that may affect their objectivity.
	Ideological,
	/// Other conflict type
	///
	/// Catch-all for conflicts that don't fit the predefined categories.
	Other,
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
	pub evidence: Option<H256>,
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
}

/// Conflict of interest registration
///
/// Stores information about declared conflicts of interest.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct ConflictRegistration<AccountId, BlockNumber, BoundedString> {
	/// Member declaring the conflict
	pub member: AccountId,
	/// Type of conflict
	pub conflict_type: ConflictType,
	/// Description of the conflict, must include the location
	/// where any off-chain evidence is stored for future reference
	pub description: BoundedString,
	/// Related matter or decision that can be an extrinsic hash, emergency ID, appeal ID, or any other
	/// on-chain identifier that this conflict relates to. If the conflict is general,
	/// this should contain a clear description of the scope of the conflict.
	pub related_matter: BoundedString,
	/// Block when conflict was registered
	pub registered_at: BlockNumber,
	/// Block when conflict expires (if applicable)
	pub expires_at: Option<BlockNumber>,
	/// Whether the conflict is still active
	pub active: bool,
}

/// On-chain remark
///
/// Stores standardized on-chain remarks with metadata.
/// Unlike System::remark, this provides a structured format with additional metadata.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct OnChainRemark<AccountId, BlockNumber, BoundedString> {
	/// Author of the remark
	pub author: AccountId,
	/// Category of the remark
	pub category: RemarkCategory,
	/// Unique identifier for the remark
	pub unique_id: BoundedString,
	/// Content of the remark must include the location where any off-chain evidence
	/// (referenced by the `related_hash`) is stored for future reference and auditability
	pub content: BoundedString,
	/// Block when remark was created
	pub created_at: BlockNumber,
	/// Related hash (if applicable) where actual evidence is stored off-chain and its location should be
	/// referenced in the `content` field
	pub related_hash: Option<H256>,
}

/// Professional service type
///
/// Defines the types of professional services that are outside the scope of Ambassador Fellowship duties.
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
/// Stores information about registered professional service providers.
/// Providers must have a verified identity through the identity pallet.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct ServiceProviderDetails<AccountId, BlockNumber, BoundedString> {
	/// Provider account with verified identity
	provider_account: AccountId,
	/// Provider name (specific to this service offering, may differ from identity name)
	provider_name: BoundedString,
	/// Service-specific contact information (should not duplicate information already in on-chain identity)
	contact_info: BoundedString,
	/// Service types offered
	service_types: BoundedVec<ProfessionalServiceType, ConstU32<10>>,
	/// Registrant (who registered this provider)
	registrant: AccountId,
	/// Registration date
	registered_at: BlockNumber,
}

/// Professional service referral
///
/// Stores information about referrals to professional service providers.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct ServiceReferral<AccountId, BlockNumber, BoundedString> {
	/// Referrer (Ambassador who made the referral)
	referrer: AccountId,
	/// Service provider being referred to
	provider_id: BoundedString,
	/// Service type being referred
	service_type: ProfessionalServiceType,
	/// Referral description
	description: BoundedString,
	/// Referral date
	referred_at: BlockNumber,
	/// Whether the referrer is receiving compensation for this referral (transparency requirement)
	compensation_disclosed: bool,
	/// Description of compensation received by referrer for this referral (if any)
	compensation_details: Option<BoundedString>,
}

/// Governance health metrics
///
/// Stores metrics for monitoring governance health.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct GovernanceHealthMetrics<BlockNumber> {
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
		type BlockNumber: Parameter + Member + Copy + Default + MaxEncodedLen + TypeInfo + From<u32>;

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

		/// Maximum number of committee members
		#[pallet::constant]
		type MaxCommitteeMembers: Get<u32>;

		/// Maximum number of participants in integration
		#[pallet::constant]
		type MaxParticipants: Get<u32>;

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
		BoundedVec<(T::AccountId, EmergencyRole), T::MaxCommitteeMembers>,
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
		BoundedVec<T::AccountId, T::MaxCommitteeMembers>,
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
			BoundedVec<T::AccountId, T::MaxParticipants>,
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
		T::Hash, // Conflict ID
		ConflictRegistration<
			T::AccountId,
			T::BlockNumber,
			BoundedVec<u8, T::MaxJustificationLength>,
		>,
	>;

	/// On-chain remarks
	#[pallet::storage]
	#[pallet::getter(fn remarks)]
	pub type Remarks<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::Hash, // Remark ID
		OnChainRemark<T::AccountId, T::BlockNumber, BoundedVec<u8, T::MaxJustificationLength>>,
	>;

	/// Professional service providers registry
	#[pallet::storage]
	#[pallet::getter(fn service_providers)]
	pub type ServiceProviders<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::Hash, // Provider ID
		ServiceProviderDetails<
			T::AccountId,
			T::BlockNumber,
			BoundedVec<u8, T::MaxDescriptionLength>,
		>,
	>;

	/// Professional service referrals
	#[pallet::storage]
	#[pallet::getter(fn service_referrals)]
	pub type ServiceReferrals<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::Hash, // Referral ID
		ServiceReferral<T::AccountId, T::BlockNumber, BoundedVec<u8, T::MaxDescriptionLength>>,
	>;

	/// Governance health metrics
	#[pallet::storage]
	#[pallet::getter(fn governance_health)]
	pub type GovernanceHealth<T: Config> = StorageValue<_, GovernanceHealthMetrics<T::BlockNumber>>;

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
			members: Vec<(T::AccountId, EmergencyRole)>,
		},
		/// Emergency resolved
		EmergencyResolved {
			emergency_id: T::Hash,
			abuse_detected: bool,
			resolution_summary: Vec<u8>,
		},
		/// Appeal submitted
		AppealSubmitted { appeal_id: T::Hash, appellant: T::AccountId, original_decision: Vec<u8> },
		/// Appeal committee formed
		AppealCommitteeFormed { appeal_id: T::Hash, members: Vec<T::AccountId> },
		/// Appeal decided
		AppealDecided { appeal_id: T::Hash, decision: AppealDecision },
		/// Cross-collective integration established
		///
		/// Emitted when a new cross-collective integration is established between the
		/// Ambassador Fellowship and another collective. Event that signals the successful
		/// creation of a formal relationship between collectives and can be used by off-chain
		/// systems to track and display active integrations.
		IntegrationEstablished {
			/// Unique identifier for the integration, derived from the integration details
			integration_id: T::Hash,
			/// Integration mechanism type that was established (e.g. JointGovernanceCouncil, LiaisonSystem,
			/// or IntegratedPlanningCycles)
			mechanism: IntegrationMechanism,
			/// Collective being integrated with
			target_collective: TargetCollective,
		},
		/// Disciplinary action taken
		DisciplinaryActionTaken {
			discipline_id: T::Hash,
			subject: T::AccountId,
			level: DisciplineLevel,
			reason: Vec<u8>,
		},
		/// Rank transition initiated
		RankTransitionInitiated {
			transition_id: T::Hash,
			member: T::AccountId,
			transition_type: TransitionType,
			previous_rank: Rank,
			new_rank: Rank,
		},
		/// Conflict of interest registered
		ConflictOfInterestRegistered {
			conflict_id: T::Hash,
			member: T::AccountId,
			conflict_type: ConflictType,
			description: Vec<u8>,
		},
		/// On-chain remark created
		OnChainRemarkCreated {
			remark_id: T::Hash,
			author: T::AccountId,
			category: RemarkCategory,
			unique_id: Vec<u8>,
		},
		/// Governance health metrics updated
		GovernanceHealthUpdated {
			participation_rate: u8,
			vote_concentration: u8,
			avg_response_time: T::BlockNumber,
		},
		/// Professional service provider registered
		ServiceProviderRegistered {
			/// Unique identifier for the service provider
			provider_id: T::Hash,
			/// Account of the provider (must have verified identity)
			provider_account: T::AccountId,
			/// Account that registered the provider
			registrant: T::AccountId,
			/// Service-specific name of the provider (may differ from identity name)
			provider_name: Vec<u8>,
			/// Types of services offered
			service_types: Vec<ProfessionalServiceType>,
		},
		/// Professional service referral created
		ServiceReferralCreated {
			/// Unique identifier for the referral
			referral_id: T::Hash,
			/// Account that made the referral
			referrer: T::AccountId,
			/// Provider being referred to
			provider_id: T::Hash,
			/// Type of service being referred
			service_type: ProfessionalServiceType,
			/// Whether the referrer disclosed receiving compensation for this referral
			compensation_disclosed: bool,
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
		/// Invalid committee composition
		InvalidCommitteeComposition,
		/// Not authorized origin
		NotAuthorizedOrigin,
		/// Not a committee member
		NotCommitteeMember,
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
		/// Too many committee members
		TooManyCommitteeMembers,
		/// Description is too long
		TooLongDescription,
		/// Justification is too long
		JustificationTooLong,
		/// Too many participants
		TooManyParticipants,
		/// Missing required role
		MissingRequiredRole,
		/// Service provider not found
		ServiceProviderNotFound,
		/// Service provider already registered
		ServiceProviderAlreadyRegistered,
		/// Identity not verified
		IdentityNotVerified,
		/// Insufficient rank for the operation
		InsufficientRank,
		/// Insufficient rank to register service provider
		InsufficientRankForProviderRegistry,
		/// Insufficient rank to create service referral
		InsufficientRankForReferral,
		/// Missing compensation disclosure
		MissingCompensationDisclosure,
		/// Too many service types
		TooManyServiceTypes,
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
		/// - `justification`: Justification for declaring the emergency, should include the location
		///   where any off-chain evidence is stored for future reference
		/// - `evidence`: Optional hash of evidence supporting the emergency declaration
		///
		/// Emits `EmergencyActivated` event when successful.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::activate_emergency_protocol())]
		pub fn activate_emergency_protocol(
			origin: OriginFor<T>,
			emergency_type: EmergencyType,
			severity: EmergencySeverity,
			justification: Vec<u8>,
			evidence: Option<H256>,
		) -> DispatchResult {
			T::EmergencyOrigin::ensure_origin(origin.clone())?;
			let initiator = ensure_signed(origin)?;

			ensure!(
				T::RankChecker::has_minimum_rank(
					&initiator,
					T::MinRankToActivateEmergencyProtocol::get()
				),
				Error::<T>::InsufficientRank
			);

			let bounded_justification: BoundedVec<_, _> =
				justification.try_into().map_err(|_| Error::<T>::JustificationTooLong)?;

			let emergency_details = EmergencyDetails {
				emergency_type: emergency_type.clone(),
				severity: severity.clone(),
				initiator: initiator.clone(),
				justification: bounded_justification,
				declared_at: <T as Config>::BlockNumber::from(
					frame_system::Pallet::<T>::block_number().saturated_into::<u32>(),
				),
				resolved_at: None,
				evidence,
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
			members: Vec<(T::AccountId, EmergencyRole)>,
		) -> DispatchResult {
			// Check origin using CommitteeFormationOrigin filter
			T::CommitteeFormationOrigin::ensure_origin(origin.clone())?;
			let who = ensure_signed(origin)?;

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

			let bounded_members: BoundedVec<_, _> =
				members.clone().try_into().map_err(|_| Error::<T>::TooManyCommitteeMembers)?;

			EmergencyCommittees::<T>::insert(emergency_id, bounded_members);

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
		/// - `evidence`: Optional hash of evidence supporting the resolution
		///
		/// Emits `EmergencyResolved` event when successful.
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::resolve_emergency())]
		pub fn resolve_emergency(
			origin: OriginFor<T>,
			emergency_id: T::Hash,
			abuse_detected: bool,
			resolution_summary: Vec<u8>,
			evidence: Option<H256>,
		) -> DispatchResult {
			let who = ensure_signed(origin.clone())?;

			Emergencies::<T>::try_mutate(emergency_id, |maybe_emergency| -> DispatchResult {
				let emergency = maybe_emergency.as_mut().ok_or(Error::<T>::EmergencyNotFound)?;
				ensure!(emergency.resolved_at.is_none(), Error::<T>::EmergencyAlreadyResolved);

				emergency.resolved_at = Some(<T as Config>::BlockNumber::from(
					frame_system::Pallet::<T>::block_number().saturated_into::<u32>(),
				));
				emergency.abuse_detected = abuse_detected;
				emergency.evidence = evidence;

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

		/// Submit appeal
		///
		/// Allows any Ambassador to submit an appeal against a decision.
		///
		/// Parameters:
		/// - `original_decision`: Description of the original decision being appealed
		/// - `justification`: Justification for the appeal, should include the location
		///   where any off-chain evidence is stored for future reference
		/// - `evidence`: Optional hash of evidence supporting the appeal
		///
		/// Emits `AppealSubmitted` event when successful.
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::submit_appeal())]
		pub fn submit_appeal(
			origin: OriginFor<T>,
			original_decision: Vec<u8>,
			justification: Vec<u8>,
			evidence: Option<H256>,
		) -> DispatchResult {
			T::AppealSubmissionOrigin::ensure_origin(origin.clone())?;
			let appellant = ensure_signed(origin)?;

			// Check that the appellant has the minimum required rank
			ensure!(
				T::RankChecker::has_minimum_rank(&appellant, T::MinRankToSubmitAppeal::get()),
				Error::<T>::InsufficientRank
			);

			let bounded_original_decision: BoundedVec<_, _> = original_decision
				.clone()
				.try_into()
				.map_err(|_| Error::<T>::TooManyCommitteeMembers)?;

			let bounded_justification: BoundedVec<_, _> =
				justification.try_into().map_err(|_| Error::<T>::JustificationTooLong)?;

			let appeal_details = AppealDetails {
				appellant: appellant.clone(),
				original_decision: bounded_original_decision,
				justification: bounded_justification,
				submitted_at: <T as Config>::BlockNumber::from(
					frame_system::Pallet::<T>::block_number().saturated_into::<u32>(),
				),
				status: AppealStatus::Submitted,
				decision: None,
				decided_at: None,
				evidence,
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
		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::form_appeal_committee())]
		pub fn form_appeal_committee(
			origin: OriginFor<T>,
			appeal_id: T::Hash,
			members: Vec<T::AccountId>,
		) -> DispatchResult {
			T::AppealCommitteeOrigin::ensure_origin(origin.clone())?;
			let who = ensure_signed(origin)?;

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

			let bounded_members: BoundedVec<_, _> =
				members.clone().try_into().map_err(|_| Error::<T>::TooManyCommitteeMembers)?;

			AppealCommittees::<T>::insert(appeal_id, bounded_members);

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
		///
		/// Emits `AppealDecided` event when successful.
		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::decide_appeal())]
		pub fn decide_appeal(
			origin: OriginFor<T>,
			appeal_id: T::Hash,
			decision: AppealDecision,
		) -> DispatchResult {
			let who = ensure_signed(origin.clone())?;

			// Check if origin is a committee member
			let committee = AppealCommittees::<T>::get(appeal_id)
				.ok_or(Error::<T>::AppealCommitteeNotFormed)?;
			ensure!(committee.contains(&who), Error::<T>::NotCommitteeMember);

			Appeals::<T>::try_mutate(appeal_id, |maybe_appeal| -> DispatchResult {
				let appeal = maybe_appeal.as_mut().ok_or(Error::<T>::AppealNotFound)?;
				ensure!(appeal.decision.is_none(), Error::<T>::AppealAlreadyDecided);

				appeal.status = AppealStatus::Decided;
				appeal.decision = Some(decision.clone());
				appeal.decided_at = Some(<T as Config>::BlockNumber::from(
					frame_system::Pallet::<T>::block_number().saturated_into::<u32>(),
				));

				Ok(())
			})?;

			Self::deposit_event(Event::AppealDecided { appeal_id, decision });

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
		/// - `description`: Description of the integration purpose, objectives, and scope.
		///   Should include the location where any off-chain agreement is stored for future reference.
		/// - `ambassador_participants`: List of Ambassador Fellowship participants who will be involved
		///   in the integration activities and governance.
		/// - `target_participants`: List of participants from the target collective who will be
		///   collaborating with the Ambassador Fellowship.
		/// - `agreement_hash`: Optional hash of the formal integration agreement document.
		///   Creates an on-chain reference to the off-chain legal or governance document when provided.
		///   Storage location of this document should be specified in the `description` parameter.
		///
		/// Emits `IntegrationEstablished` event when successful.
		///
		/// # Examples
		///
		/// - Joint Working Group: Create a working group between Ambassador Fellowship and Technical Fellowship
		/// - Liaison System: Establish liaisons between Ambassador Fellowship and Secretary Collective
		/// - Integrated Planning: Create joint planning cycles with other collectives
		#[pallet::call_index(6)]
		#[pallet::weight(T::WeightInfo::establish_integration())]
		pub fn establish_integration(
			origin: OriginFor<T>,
			mechanism: IntegrationMechanism,
			target_collective: TargetCollective,
			description: Vec<u8>,
			ambassador_participants: Vec<T::AccountId>,
			target_participants: Vec<T::AccountId>,
			agreement_hash: Option<H256>,
		) -> DispatchResult {
			T::IntegrationOrigin::ensure_origin(origin.clone())?;
			let who = ensure_signed(origin)?;

			// Check that the caller has the minimum required rank
			ensure!(
				T::RankChecker::has_minimum_rank(&who, T::MinRankToEstablishIntegration::get()),
				Error::<T>::InsufficientRank
			);

			let bounded_description: BoundedVec<_, _> =
				description.clone().try_into().map_err(|_| Error::<T>::TooLongDescription)?;

			let bounded_ambassador_participants: BoundedVec<_, _> = ambassador_participants
				.clone()
				.try_into()
				.map_err(|_| Error::<T>::TooManyParticipants)?;

			let bounded_target_participants: BoundedVec<_, _> = target_participants
				.clone()
				.try_into()
				.map_err(|_| Error::<T>::TooManyParticipants)?;

			let integration_details = IntegrationDetails {
				mechanism: mechanism.clone(),
				target_collective: target_collective.clone(),
				description: bounded_description,
				ambassador_participants: bounded_ambassador_participants,
				target_participants: bounded_target_participants,
				established_at: <T as Config>::BlockNumber::from(
					frame_system::Pallet::<T>::block_number().saturated_into::<u32>(),
				),
				agreement_hash,
			};

			let integration_id = T::Hashing::hash_of(&integration_details);

			ensure!(
				!Integrations::<T>::contains_key(integration_id),
				Error::<T>::IntegrationAlreadyExists
			);

			Integrations::<T>::insert(integration_id, integration_details);

			Self::deposit_event(Event::IntegrationEstablished {
				integration_id,
				mechanism,
				target_collective,
			});

			Ok(())
		}

		/// Initiate disciplinary action
		///
		/// Allows authorized users to take disciplinary action against a member.
		///
		/// Parameters:
		/// - `subject`: Account ID of the member to be disciplined
		/// - `level`: Level of discipline to be applied
		/// - `reason`: Reason for the disciplinary action, must include the location
		///   where any off-chain evidence (referenced by the `evidence_hash`) is stored
		/// - `duration`: Duration of the disciplinary action (if applicable)
		/// - `evidence`: Optional hash of evidence supporting the disciplinary action.
		///   The actual evidence is stored off-chain and its location should be
		///   referenced in the `reason` field
		///
		/// Emits `DisciplinaryActionTaken` event when successful.
		#[pallet::call_index(7)]
		#[pallet::weight(T::WeightInfo::initiate_disciplinary_action())]
		pub fn initiate_disciplinary_action(
			origin: OriginFor<T>,
			subject: T::AccountId,
			level: DisciplineLevel,
			reason: Vec<u8>,
			duration: Option<T::BlockNumber>,
			evidence: Option<H256>,
		) -> DispatchResult {
			T::EmergencyOrigin::ensure_origin(origin.clone())?;

			let bounded_reason: BoundedVec<_, _> =
				reason.clone().try_into().map_err(|_| Error::<T>::TooManyCommitteeMembers)?;

			let issuer = ensure_signed(origin.clone())?;
			let discipline_details = DisciplineDetails {
				subject: subject.clone(),
				issuer,
				level: level.clone(),
				reason: bounded_reason,
				issued_at: <T as Config>::BlockNumber::from(
					frame_system::Pallet::<T>::block_number().saturated_into::<u32>(),
				),
				duration,
				evidence,
				active: true,
			};

			let discipline_id = T::Hashing::hash_of(&discipline_details);

			Disciplines::<T>::insert(discipline_id, discipline_details);

			Self::deposit_event(Event::DisciplinaryActionTaken {
				discipline_id,
				subject,
				level,
				reason,
			});

			Ok(())
		}

		/// Initiate rank transition
		///
		/// Allows authorized users to initiate a rank transition for a member.
		/// Note: For standard promotions and demotions, consider using the `promote_member` and
		/// `demote_member` extrinsics from the ranked-collective-ambassador pallet instead
		/// since this extrinsic is intended for more complex transitions that require additional context.
		///
		/// Parameters:
		/// - `member`: Account ID of the member undergoing transition
		/// - `transition_type`: Type of transition
		/// - `previous_rank`: Previous rank of the member
		/// - `new_rank`: New rank of the member
		/// - `justification`: Justification for the transition
		/// - `effective_at`: Block when the transition will be completed
		/// - `successor`: Successor account (if applicable)
		///
		/// Emits `RankTransitionInitiated` event when successful.
		#[pallet::call_index(8)]
		#[pallet::weight(T::WeightInfo::initiate_rank_transition())]
		pub fn initiate_rank_transition(
			origin: OriginFor<T>,
			member: T::AccountId,
			transition_type: TransitionType,
			previous_rank: Rank,
			new_rank: Rank,
			justification: Vec<u8>,
			effective_at: T::BlockNumber,
			successor: Option<T::AccountId>,
		) -> DispatchResult {
			T::EmergencyOrigin::ensure_origin(origin.clone())?;

			let bounded_justification: BoundedVec<_, _> =
				justification.clone().try_into().map_err(|_| Error::<T>::JustificationTooLong)?;

			let transition_details = TransitionDetails {
				member: member.clone(),
				transition_type: transition_type.clone(),
				previous_rank,
				new_rank,
				justification: bounded_justification,
				initiated_at: <T as Config>::BlockNumber::from(
					frame_system::Pallet::<T>::block_number().saturated_into::<u32>(),
				),
				effective_at,
				successor,
				knowledge_transfer_complete: false,
			};

			let transition_id = T::Hashing::hash_of(&transition_details);

			Transitions::<T>::insert(transition_id, transition_details);

			Self::deposit_event(Event::RankTransitionInitiated {
				transition_id,
				member,
				transition_type,
				previous_rank,
				new_rank,
			});

			Ok(())
		}

		/// Register conflict of interest
		///
		/// Allows members to register conflicts of interest.
		///
		/// Parameters:
		/// - `conflict_type`: Type of conflict
		/// - `description`: Description of the conflict
		/// - `related_matter`: Related matter or decision
		/// - `expires_at`: Block when the conflict expires (if applicable)
		///
		/// Emits `ConflictOfInterestRegistered` event when successful.
		#[pallet::call_index(9)]
		#[pallet::weight(T::WeightInfo::register_conflict_of_interest())]
		pub fn register_conflict_of_interest(
			origin: OriginFor<T>,
			conflict_type: ConflictType,
			description: Vec<u8>,
			related_matter: Vec<u8>,
			expires_at: Option<T::BlockNumber>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let bounded_description: BoundedVec<_, _> = description
				.clone()
				.try_into()
				.map_err(|_| Error::<T>::TooManyCommitteeMembers)?;

			let bounded_related_matter: BoundedVec<_, _> = related_matter
				.clone()
				.try_into()
				.map_err(|_| Error::<T>::TooManyCommitteeMembers)?;

			let conflict_registration = ConflictRegistration {
				member: who.clone(),
				conflict_type: conflict_type.clone(),
				description: bounded_description,
				related_matter: bounded_related_matter,
				registered_at: <T as Config>::BlockNumber::from(
					frame_system::Pallet::<T>::block_number().saturated_into::<u32>(),
				),
				expires_at,
				active: true,
			};

			let conflict_id = T::Hashing::hash_of(&conflict_registration);

			Conflicts::<T>::insert(conflict_id, conflict_registration);

			Self::deposit_event(Event::ConflictOfInterestRegistered {
				conflict_id,
				member: who,
				conflict_type,
				description,
			});

			Ok(())
		}

		/// Create on-chain remark
		///
		/// Allows authorized users to create structured on-chain remarks with metadata.
		/// Unlike System::remark, this provides additional context and standardized format.
		///
		/// Parameters:
		/// - `category`: Category of the remark
		/// - `unique_id`: Unique identifier for the remark
		/// - `content`: Content of the remark, must include the location
		///   where any off-chain evidence (referenced by the `related_hash`) is stored
		///   for future reference and auditability
		/// - `related_hash`: Related hash (if applicable). The actual evidence is stored
		///   off-chain, and its location should be referenced in the `content` field
		///
		/// Emits `OnChainRemarkCreated` event when successful.
		#[pallet::call_index(10)]
		#[pallet::weight(T::WeightInfo::create_remark())]
		pub fn create_remark(
			origin: OriginFor<T>,
			category: RemarkCategory,
			unique_id: Vec<u8>,
			content: Vec<u8>,
			related_hash: Option<H256>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let bounded_unique_id: BoundedVec<_, _> =
				unique_id.clone().try_into().map_err(|_| Error::<T>::TooManyCommitteeMembers)?;

			let bounded_content: BoundedVec<_, _> =
				content.clone().try_into().map_err(|_| Error::<T>::TooManyCommitteeMembers)?;

			let on_chain_remark = OnChainRemark {
				author: who.clone(),
				category: category.clone(),
				unique_id: bounded_unique_id,
				content: bounded_content,
				created_at: <T as Config>::BlockNumber::from(
					frame_system::Pallet::<T>::block_number().saturated_into::<u32>(),
				),
				related_hash,
			};

			let remark_id = T::Hashing::hash_of(&on_chain_remark);

			Remarks::<T>::insert(remark_id, on_chain_remark);

			Self::deposit_event(Event::OnChainRemarkCreated {
				remark_id,
				author: who,
				category,
				unique_id,
			});

			Ok(())
		}

		/// Update governance health metrics
		///
		/// Allows authorized users to update governance health metrics.
		///
		/// Parameters:
		/// - `participation_rate`: Participation rate (0-100%) percentage of eligible
		///   members who participated in governance activities
		/// - `vote_concentration`: Vote concentration index (0-100%) measures how concentrated
		///   voting power is among participants. Lower values indicate more equal distribution
		///   of votes, while higher values indicate votes are concentrated among fewer participants
		/// - `avg_response_time`: Average response time in blocks is the average time taken
		///   to respond to governance actions
		///
		/// Emits `GovernanceHealthUpdated` event when successful.
		#[pallet::call_index(11)]
		#[pallet::weight(T::WeightInfo::update_governance_health_metrics())]
		pub fn update_governance_health_metrics(
			origin: OriginFor<T>,
			participation_rate: u8,
			vote_concentration: u8,
			avg_response_time: T::BlockNumber,
		) -> DispatchResult {
			T::EmergencyOrigin::ensure_origin(origin.clone())?;

			GovernanceHealth::<T>::put(GovernanceHealthMetrics {
				participation_rate,
				vote_concentration,
				avg_response_time,
				last_updated: <T as Config>::BlockNumber::from(
					frame_system::Pallet::<T>::block_number().saturated_into::<u32>(),
				),
			});

			Self::deposit_event(Event::GovernanceHealthUpdated {
				participation_rate,
				vote_concentration,
				avg_response_time,
			});

			Ok(())
		}

		/// Register professional service provider
		///
		/// Allows authorized members to register professional service providers in the registry
		/// using the referral framework mentioned in the Ambassador Fellowship Manifesto, and
		/// where the provider must have a verified identity through the identity pallet.
		///
		/// Parameters:
		/// - `provider_account`: Account of the service provider (must have verified identity)
		/// - `provider_name`: Name or identifier of the service provider
		/// - `service_types`: Types of professional services offered
		/// - `contact_info`: Contact information for the provider, should include the location
		///   where any off-chain evidence (referenced by the `evidence_hash`) is stored
		///   for future reference and auditability
		/// - `evidence_hash`: Optional hash of evidence supporting the provider's credentials
		///
		/// Emits `ServiceProviderRegistered` event when successful.
		#[pallet::call_index(12)]
		#[pallet::weight(T::WeightInfo::register_service_provider())]
		pub fn register_service_provider(
			origin: OriginFor<T>,
			provider_account: T::AccountId,
			provider_name: Vec<u8>,
			service_types: Vec<ProfessionalServiceType>,
			contact_info: Vec<u8>,
			_evidence_hash: Option<H256>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Ensure the caller has at least the minimum rank required to register a service provider
			ensure!(
				T::RankChecker::has_minimum_rank(&who, T::MinRankForProviderRegistry::get()),
				Error::<T>::InsufficientRankForProviderRegistry
			);

			// Ensure the provider account has a verified identity
			ensure!(
				T::IdentityRegistrar::has_identity(&provider_account),
				Error::<T>::IdentityNotVerified
			);

			let bounded_provider_name: BoundedVec<_, _> =
				provider_name.clone().try_into().map_err(|_| Error::<T>::JustificationTooLong)?;

			let bounded_contact_info: BoundedVec<_, _> =
				contact_info.clone().try_into().map_err(|_| Error::<T>::JustificationTooLong)?;

			let bounded_service_types: BoundedVec<ProfessionalServiceType, ConstU32<10>> =
				service_types.clone().try_into().map_err(|_| Error::<T>::TooManyServiceTypes)?;

			let provider_details = ServiceProviderDetails {
				provider_account: provider_account.clone(),
				provider_name: bounded_provider_name,
				service_types: bounded_service_types,
				contact_info: bounded_contact_info,
				registrant: who.clone(),
				registered_at: <T as Config>::BlockNumber::from(
					frame_system::Pallet::<T>::block_number().saturated_into::<u32>(),
				),
			};

			let provider_id = T::Hashing::hash_of(&provider_details);

			// Ensure this provider hasn't been registered before
			ensure!(
				!ServiceProviders::<T>::contains_key(provider_id),
				Error::<T>::ServiceProviderAlreadyRegistered
			);

			ServiceProviders::<T>::insert(provider_id, provider_details);

			Self::deposit_event(Event::ServiceProviderRegistered {
				provider_id,
				provider_account,
				registrant: who,
				provider_name,
				service_types,
			});

			Ok(())
		}

		/// Create professional service referral
		///
		/// Allows authorized members to create referrals to professional service providers
		/// using the referral framework mentioned in the Ambassador Fellowship Manifesto.
		///
		/// Parameters:
		/// - `provider_id`: ID of the service provider being referred
		/// - `service_type`: Type of service being referred
		/// - `description`: Description of the referral, should include the location
		///   where any off-chain evidence is stored for future reference and auditability
		/// - `compensation_disclosed`: If compensation is being received for the referral or not
		/// - `compensation_details`: Details of compensation if applicable
		///
		/// Emits `ServiceReferralCreated` event when successful.
		#[pallet::call_index(13)]
		#[pallet::weight(T::WeightInfo::create_service_referral())]
		pub fn create_service_referral(
			origin: OriginFor<T>,
			provider_id: T::Hash,
			service_type: ProfessionalServiceType,
			description: Vec<u8>,
			compensation_disclosed: bool,
			compensation_details: Option<Vec<u8>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Ensure the caller has at least the minimum rank required to create a referral
			ensure!(
				T::RankChecker::has_minimum_rank(&who, T::MinRankForReferral::get()),
				Error::<T>::InsufficientRankForReferral
			);

			// Ensure the provider exists
			let _provider = ServiceProviders::<T>::get(provider_id)
				.ok_or(Error::<T>::ServiceProviderNotFound)?;

			// If compensation is received, ensure details are provided
			if compensation_disclosed {
				ensure!(compensation_details.is_some(), Error::<T>::MissingCompensationDisclosure);
			}

			let bounded_description: BoundedVec<_, _> =
				description.clone().try_into().map_err(|_| Error::<T>::JustificationTooLong)?;

			let bounded_compensation: Option<BoundedVec<_, _>> =
				if let Some(comp) = compensation_details.clone() {
					Some(comp.try_into().map_err(|_| Error::<T>::JustificationTooLong)?)
				} else {
					None
				};

			// Convert provider_id to BoundedVec for storage
			let provider_id_bytes = provider_id.as_ref().to_vec();
			let bounded_provider_id: BoundedVec<_, _> =
				provider_id_bytes.try_into().map_err(|_| Error::<T>::JustificationTooLong)?;

			let referral = ServiceReferral {
				referrer: who.clone(),
				provider_id: bounded_provider_id,
				service_type: service_type.clone(),
				description: bounded_description,
				referred_at: <T as Config>::BlockNumber::from(
					frame_system::Pallet::<T>::block_number().saturated_into::<u32>(),
				),
				compensation_disclosed,
				compensation_details: bounded_compensation,
			};

			let referral_id = T::Hashing::hash_of(&referral);

			ServiceReferrals::<T>::insert(referral_id, referral);

			Self::deposit_event(Event::ServiceReferralCreated {
				referral_id,
				referrer: who,
				provider_id,
				service_type,
				compensation_disclosed,
			});

			Ok(())
		}
	}
}
