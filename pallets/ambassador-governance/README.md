# Ambassador Fellowship Governance Pallet

## Usage

### Running Tests

To run tests with println output visible:

```bash
cargo test -p pallet-ambassador-governance -- --nocapture
```

### Running Benchmarks

To run benchmarks for this pallet, first ensure the pallet is included in the runtime's `define_benchmarks!` macro.

```bash
# Install frame-omni-bencher
cargo install frame-omni-bencher --locked

# Download official template file that defines how weight information should be formatted
curl https://raw.githubusercontent.com/paritytech/polkadot-sdk/master/substrate/.maintain/frame-weight-template.hbs \
--output ./frame-weight-template.hbs

# Run from project root
cargo check --features runtime-benchmarks -p pallet-ambassador-governance
cargo check --features runtime-benchmarks -p pallet-ranked-collective-ambassador
cargo check --features runtime-benchmarks -p pallet-core-fellowship-ambassador

# Run build with feature flag
cargo build --release --features runtime-benchmarks

# Run the benchmarking tool to measure extrinsic weights
# Choose a specific extrinsic
RUST_LOG=debug,trie_cache=warn frame-omni-bencher v1 benchmark pallet \
--runtime ./target/release/wbuild/collectives-polkadot-runtime/collectives_polkadot_runtime.wasm \
--pallet pallet_ambassador_governance \
--extrinsic "*" \
--template ./frame-weight-template.hbs \
--output ./pallets/ambassador-governance/src/weights.rs > ./log.txt 2>&1
```

### Building

To build the pallet:

```bash
cargo build --release -p pallet-ambassador-governance
```

### Linting

```sh
cargo fmt -p pallet-ambassador-governance --check
cargo fmt -p pallet-ranked-collective-ambassador --check
cargo fmt -p pallet-core-fellowship-ambassador --check

# switch to nightly to lint
rustup default nightly
cargo fmt -p pallet-ambassador-governance
cargo fmt -p pallet-ranked-collective-ambassador
cargo fmt -p pallet-core-fellowship-ambassador
rustup default stable
```

## Security and Design Features

### Rank-Based Access Control

The Ambassador Governance pallet implements explicit rank-based access control for privileged operations to ensure that only accounts with sufficient rank can perform certain governance actions:

| Operation | Runtime Parameter | Description |
|-----------|------------------|-------------|
| Activate Emergency Protocol | `MinRankToActivateEmergencyProtocol` | Initiates emergency response procedures |
| Form Emergency Committee | `MinRankToFormEmergencyCommittee` | Creates committee to oversee emergency response |
| Form Appeal Committee | `MinRankToFormAppealCommittee` | Creates committee to review appeals |
| Submit Appeal | `MinRankToSubmitAppeal` | Submits appeal against a decision |
| Establish Integration | `MinRankToEstablishIntegration` | Creates formal integration with another collective |
| Register Service Provider | `MinRankForProviderRegistry` | Registers professional service provider |
| Create Service Referral | `MinRankForReferral` | Creates referral to a professional service provider |

These rank requirements are fully configurable through the pallet's Config trait parameters such that this flexibility allows different networks to implement governance policies appropriate to their specific needs and organizational structure.

### Evidence Handling Pattern

A crucial aspect of this pallet's design is its evidence handling pattern that ensures proper governance operation and auditability:

1. **On-chain Storage**: Only evidence hashes (H256 values) are stored on-chain
2. **Off-chain Storage**: Actual evidence is stored off-chain in external storage systems
3. **Reference Linking**: Human-readable parameters in extrinsics (like `resolution_summary`, `justification`, and `description`) must include references to where the off-chain evidence is stored
4. **Storage Efficiency**: Maintains transparency and auditability while keeping blockchain storage requirements minimal

It creates a verifiable link between on-chain actions and their supporting evidence when calling extrinsics like `resolve_emergency`, `submit_appeal`, or `establish_integration`, as users should include the location of off-chain evidence in the text parameters.

## Runtime Integration Importance

The Ambassador Governance pallet is a critical component for the Ambassador Fellowship's runtime implementation. Proper integration of this pallet provides several key benefits:

- **Specialized Governance Framework**: Delivers essential governance mechanisms specifically designed for the Ambassador Fellowship's unique needs, including emergency response protocols, appeal processes, and cross-collective integration.

- **Transparent Decision-Making**: Verifiable evidence handling pattern where evidence hashes (H256 values) are stored on-chain while actual evidence is stored off-chain, creating a transparent audit trail for all governance decisions.

- **Operational Autonomy**: Enables the Ambassador Fellowship to function with appropriate governance autonomy while maintaining connections to the broader Polkadot ecosystem.

- **Scalable Governance**: Provides a foundation for governance that can scale with the growth of the Ambassador Fellowship and adapt to changing requirements.

- **Accountability Mechanisms**: Establishes clear processes for emergency responses, appeals, and professional service boundaries that ensure accountability at all levels of the fellowship.

### Evidence Handling Pattern

A crucial aspect of this pallet's design is its evidence handling pattern that ensures proper governance operation and auditability:

1. Only evidence hashes (H256 values) are stored on-chain
2. Actual evidence is stored off-chain in external storage systems
3. Human-readable parameters in extrinsics (like `resolution_summary`, `justification`, and `description`) should include references to where the off-chain evidence is stored
4. Approach that maintains transparency and auditability while keeping blockchain storage requirements minimal

### Deficiencies Without This Pallet

Without the Ambassador Governance pallet, the on-chain Ambassador Fellowship would face significant limitations:

- **Lack of Specialized Governance Mechanisms**: The core fellowship pallets alone provide only basic membership and rank management, but lack the specialized governance processes needed for emergency responses, appeals, and cross-collective integration.

- **Insufficient Evidence Handling**: No standardized way to maintain the critical link between on-chain decisions and off-chain evidence, undermining transparency and accountability.

- **Limited Dispute Resolution**: No formal appeal process for decisions, potentially leading to unresolved conflicts and governance paralysis.

- **Isolated Collective Operation**: No mechanisms for cross-collective integration, preventing the Ambassador Fellowship from effectively collaborating with other collectives in the ecosystem.

- **Inadequate Professional Services Boundaries**: Absence of structures to properly register and refer professional service providers, creating potential conflicts of interest and accountability gaps.

## Ambassador Fellowship Governance Diagrams

## Introduction

The Ambassador Governance pallet provides specialized governance mechanisms for the Ambassador Fellowship, enabling emergency response protocols, appeal processes, and cross-collective integration. It is intentionally decoupled from the core fellowship components (Ranked Collective and Core Fellowship pallets) to:

- Maintain separation of concerns, where the Ambassador Governance pallet handles specific governance scenarios that are unique to the Ambassador Fellowship, while the core fellowship pallets manage membership, ranks, and basic collective operations;
- Enable independent evolution, allowing each component to evolve independently without tight dependencies, allowing for more flexible governance adaptations;
- Support optional integration, where the Ambassador Governance pallet can reference the core fellowship components when necessary through configurable origins, but doesn't require them for basic operation.

It interacts with other components through Origin types that may be configured in the runtime to enforce rank requirements and handle authorization, evidence hashing for governance actions that are stored off-chain with only hashes stored on-chain and where descriptive parameters are included to provide human-readable references to those off-chain storage locations, and optional references to the core fellowship components when necessary whilst maintaining a loose coupling.

This architecture allows the Ambassador Fellowship to implement specialized governance mechanisms while leveraging the underlying membership and rank system provided by the core fellowship pallets.

## System Architecture Diagram

```mermaid
flowchart TD
  subgraph "Ambassador Gov. System"
    direction TB
    RC[Ranked Collective Pallet]
    CF[Core Fellowship Pallet]
    
    RC <--> CF
    
    subgraph "Ranked Collective Components"
      direction TB
      RCM[Member Management]
      RCV[Vote Weight]
      RCT[Tally System]
      
      RCM --> RCV --> RCT
    end
    
    subgraph "Core Fellowship Components"
      direction TB
      CFP[Promotion/Demotion]
      CFE[Evidence Submission]
      CFA[Activity Tracking]
      
      CFP --> CFE --> CFA
    end
    
    RC --> RCM
    CF --> CFP
  end
  
  subgraph "Ambassador Gov. System"
    direction TB
    AG[Ambassador Gov. Pallet]
    
    subgraph "Emergency Response"
      direction TB
      ER[Emergency Protocol]
      EC[Emergency Committee]
      
      ER --> EC
    end
    
    subgraph "Appeal Process"
      direction TB
      AP[Appeal Submission]
      AC[Appeal Committee]
      AD[Appeal Decision]
      
      AP --> AC --> AD
    end
    
    subgraph "Cross-Collective Integration"
      direction TB
      CI[Integration Mechanisms]
    end
    
    AG --> ER
    AG --> AP
    AG --> CI
  end
  
  subgraph "Secretary Collective"
    direction TB
    SC[Secretary Collective Pallet]
    SS[Secretary Salary Pallet]
    
    SC --> SS
  end
  
  %% Dotted lines to show loose coupling between systems
  RC -.->|"Optional\nReferences"| AG
  CF -.->|"Optional\nReferences"| AG
  
  classDef main fill:#FF2670,stroke:#555555,stroke-width:2px
  classDef secondary fill:#07FFFF,stroke:#555555,stroke-width:1px
  classDef ambassador fill:#E4FF07,stroke:#555555,stroke-width:2px
  classDef ambassador_comp fill:#DCE2E9,stroke:#555555,stroke-width:1px
  classDef secretary fill:#AEB7CB,stroke:#555555,stroke-width:2px
  
  class RC,CF main
  class RCM,RCV,RCT,CFP,CFE,CFA secondary
  class AG ambassador
  class ER,EC,AP,AC,AD,CI ambassador_comp
  class SC,SS secretary
```

## Rank System Class Diagram

```mermaid
classDiagram
  class RankedCollective {
    +add_member(who: AccountId)
    +remove_member(who: AccountId)
    +promote_member(who: AccountId)
    +demote_member(who: AccountId)
    +ensure_member(who: AccountId): MemberRecord
    +rank_of(who: AccountId): Option~Rank~
  }

  class MemberRecord {
    +rank: Rank
    +new(rank: Rank): MemberRecord
  }

  class VoteWeight {
    <<interface>>
    +convert(rank: Rank): Votes
  }

  class AmbassadorVoteWeight {
    +convert(0): Votes = 0 // Rank 0 (Advocate): n/a
    +convert(1): Votes = 1 // Rank I (Member): 1 vote
    +convert(2): Votes = 3 // Rank II (Ambassador): 3 votes
    +convert(3): Votes = 6 // Rank III (Senior Ambassador): 6 votes
    +convert(4): Votes = 10 // Rank IV (Principal Ambassador): 10 votes
    +convert(5): Votes = 15 // Rank V (Regional Head): 15 votes
    +convert(6): Votes = 21 // Rank VI (Global Head): 21 votes
  }

  class Tally {
    +bare_ayes: MemberIndex
    +ayes: Votes
    +nays: Votes
    +from_parts(bare_ayes, ayes, nays): Self
  }

  class CoreFellowship {
    +set_params(params: ParamsType)
    +import_member()
    +induct(who: AccountId)
    +promote(who: AccountId, to_rank: Rank)
    +approve(who: AccountId)
    +submit_evidence(wish: Wish, evidence: Evidence)
    +set_active(is_active: bool)
    +bump(who: AccountId)
    +offboard(who: AccountId)
  }

  class MemberStatus {
    +is_active: bool
    +last_promotion: BlockNumber
    +last_proof: BlockNumber
  }

  class ParamsType {
    +demotion_period: Vec~BlockNumber~
    +min_promotion_period: Vec~BlockNumber~
    +offboard_timeout: BlockNumber
  }

  RankedCollective --> MemberRecord
  RankedCollective --> VoteWeight
  RankedCollective --> Tally
  VoteWeight <|-- Unit
  VoteWeight <|-- Linear
  VoteWeight <|-- Geometric
  CoreFellowship --> MemberStatus
  CoreFellowship --> ParamsType
  CoreFellowship --> RankedCollective
```

## Onboarding Sequence Diagram

```mermaid
sequenceDiagram
  participant A as Applicant
  participant CF as Core Fellowship
  participant RC as Ranked Collective
  
  A->>CF: induct(who)
  Note over CF: Requires InductOrigin
  CF->>RC: add_member(who, rank=0)
  CF->>A: Candidate status granted
  
  A->>CF: submit_evidence(wish=Promotion, evidence)
  Note over A: Submits evidence of efforts
  
  A->>CF: promote(who, to_rank=1)
  Note over CF: Requires PromoteOrigin
  CF->>RC: promote_member(who)
  CF->>A: Membership granted (Rank 1)
```

## Promotion Flow Diagram

```mermaid
flowchart TD
  Start[Member at current rank] --> Evidence[Submit evidence]
  Evidence --> Promote{Promotion request?}
  
  Promote -->|Yes| MinPeriod{Min promotion period elapsed?}
  MinPeriod -->|No| Wait[Wait for minimum period]
  MinPeriod -->|Yes| PromoteVote[PromoteOrigin votes]
  PromoteVote --> PromoteDecision{Approved?}
  PromoteDecision -->|Yes| PromoteMember[Promote to next rank]
  PromoteDecision -->|No| Reject[Promotion rejected]
  
  Promote -->|No| Approve[Request approval]
  Approve --> ApproveVote[ApproveOrigin votes]
  ApproveVote --> ApproveDecision{Approved?}
  ApproveDecision -->|Yes| ResetDemotion[Reset demotion timer]
  ApproveDecision -->|No| DemotionTimer{Demotion period elapsed?}
  
  DemotionTimer -->|Yes| Bump[Anyone can bump]
  DemotionTimer -->|No| Continue[Continue at current rank]
  Bump --> Demote[Demote by one rank]
  
  PromoteMember --> NewRank[Member at new rank]
  Reject --> Start
  ResetDemotion --> Start
  Continue --> Start
  Demote --> LowerRank[Member at lower rank]
  LowerRank --> Start
  
  Wait --> Start
```

## Voting Weight System

```mermaid
flowchart LR
  subgraph "Vote Weight Schemes"
    Manifesto["Manifesto: Official vote weights from Ambassador Fellowship Manifesto"]
  end
  
  subgraph "Official Vote Weights (Ambassador Fellowship Manifesto)"
    R0["Rank 0 (Advocate): n/a"]
    R1["Rank I (Member): 1 vote"]
    R2["Rank II (Ambassador): 3 votes"]
    R3["Rank III (Senior Ambassador): 6 votes"]
    R4["Rank IV (Principal Ambassador): 10 votes"]
    R5["Rank V (Regional Head): 15 votes"]
    R6["Rank VI (Global Head): 21 votes"]
  end
  
  Manifesto --> R0
  Manifesto --> R1
  Manifesto --> R2
  Manifesto --> R3
  Manifesto --> R4
  Manifesto --> R5
  Manifesto --> R6
```

## Rank System Implementation

```mermaid
flowchart TB
  subgraph "Ranked Collective Pallet"
    Members["Members Storage<br/>Map: AccountId to MemberRecord"]
    MemberCount["MemberCount Storage<br/>Map: Rank to MemberIndex"]
    IdToIndex["IdToIndex Storage<br/>Map: (Rank, AccountId) to MemberIndex"]
    IndexToId["IndexToId Storage<br/>Map: (Rank, MemberIndex) to AccountId"]
  end
  
  subgraph "Core Fellowship Pallet"
    Member["Member Storage<br/>Map: AccountId to MemberStatus"]
    MemberEvidence["MemberEvidence Storage<br/>Map: AccountId to (Wish, Evidence)"]
    Params["Params Storage<br/>ParamsType"]
  end
  
  subgraph "Operations"
    Add["Add Member"]
    Remove["Remove Member"]
    Promote["Promote Member"]
    Demote["Demote Member"]
    SubmitEvidence["Submit Evidence"]
    Approve["Approve Member"]
    Bump["Bump Member"]
    SetActive["Set Active Status"]
  end
  
  Add --> Members
  Add --> MemberCount
  Add --> IdToIndex
  Add --> IndexToId
  
  Remove --> Members
  Remove --> MemberCount
  Remove --> IdToIndex
  Remove --> IndexToId
  
  Promote --> Members
  Promote --> MemberCount
  Promote --> IdToIndex
  Promote --> IndexToId
  
  Demote --> Members
  Demote --> MemberCount
  Demote --> IdToIndex
  Demote --> IndexToId
  
  SubmitEvidence --> MemberEvidence
  Approve --> Member
  Bump --> Members
  Bump --> Member
  SetActive --> Member
```

## Ambassador Fellowship Vertical Structure

```mermaid
flowchart TB
  R0[Candidates<br/>Rank 0]
  R1[Members<br/>Rank 1]
  R2[Rank 2]
  R3[Rank 3]
  R4[Rank 4]
  R5[Rank 5]
  
  R5 --> R4
  R4 --> R3
  R3 --> R2
  R2 --> R1
  R1 --> R0
  
  classDef candidate fill:#DCE2E9,stroke:#555555,stroke-width:1px
  classDef member fill:#07FFFF,stroke:#555555,stroke-width:1px
  classDef higher fill:#FF2670,stroke:#555555,stroke-width:2px
  
  class R0 candidate
  class R1 member
  class R2,R3,R4,R5 higher
```

## Decision Making System

```mermaid
graph TD
  subgraph "Origins"
    direction TB
    IO[InductOrigin]
    PO[PromoteOrigin]
    AO[ApproveOrigin]
    SO[SetParamsOrigin]
  end
  
  subgraph "Actions"
    direction TB
    Induct[Induct Candidate]
    Promote[Promote Member]
    Approve[Approve Member]
    SetParams[Set Parameters]
    Bump[Bump - Demotion]
    Offboard[Offboard Candidate]
  end
  
  subgraph "Parameters"
    direction TB
    DP[Demotion Period]
    MPP[Min Promotion Period]
    OT[Offboard Timeout]
  end
  
  IO --> Induct
  PO --> Promote
  AO --> Approve
  SO --> SetParams
  
  DP --> Bump
  MPP --> Promote
  OT --> Offboard
```

## Ambassador Fellowship Governance Class Diagram

```mermaid
classDiagram
  class AmbassadorGovernance {
    +activate_emergency_protocol(emergency_type, severity, justification, evidence)
    +form_emergency_committee(emergency_id, members)
    +resolve_emergency(emergency_id, abuse_detected, resolution_summary, evidence)
    +submit_appeal(original_decision, justification, evidence)
    +form_appeal_committee(appeal_id, members)
    +decide_appeal(appeal_id, decision)
    +establish_integration(mechanism, target_collective, description, participants, agreement_hash)
  }

  class EmergencyDetails {
    +emergency_type: EmergencyType
    +severity: EmergencySeverity
    +justification: BoundedString
    +reporter: AccountId
    +created_at: BlockNumber
    +resolved_at: Option~BlockNumber~
    +resolution_summary: Option~BoundedString~
    +abuse_detected: Option~bool~
    +evidence: Option~Hash~
  }

  class AppealDetails {
    +appellant: AccountId
    +original_decision: BoundedString
    +justification: BoundedString
    +submitted_at: BlockNumber
    +status: AppealStatus
    +decision: Option~AppealDecision~
    +decided_at: Option~BlockNumber~
    +evidence: Option~Hash~
  }

  class IntegrationDetails {
    +mechanism: IntegrationMechanism
    +target_collective: BoundedString
    +description: BoundedString
    +ambassador_participants: BoundedVec~AccountId~
    +target_participants: BoundedVec~AccountId~
    +established_at: BlockNumber
    +agreement_hash: Option~Hash~
  }

  class EmergencyType {
    <<enumeration>>
    SecurityVulnerability
    GovernanceAttack
    TechnicalFailure
    ReputationThreat
    Other
  }

  class EmergencySeverity {
    <<enumeration>>
    Critical
    High
    Medium
  }

  class AppealStatus {
    <<enumeration>>
    Submitted
    UnderReview
    Decided
    Closed
  }

  class AppealDecision {
    <<enumeration>>
    Upheld
    Modified
    Rejected
  }

  class IntegrationMechanism {
    <<enumeration>>
    JointGovernanceCouncil
    LiaisonSystem
    IntegratedPlanningCycles
  }

  AmbassadorGovernance --> EmergencyDetails
  AmbassadorGovernance --> AppealDetails
  AmbassadorGovernance --> IntegrationDetails
  EmergencyDetails --> EmergencyType
  EmergencyDetails --> EmergencySeverity
  AppealDetails --> AppealStatus
  AppealDetails --> AppealDecision
  IntegrationDetails --> IntegrationMechanism
```

## Professional Services Boundaries

The Ambassador Fellowship Governance pallet includes a Professional Services Boundaries feature that enables the registration and referral of professional service providers. This feature ensures that only verified service providers can be registered and that referrals are created by qualified ambassadors.

### Key Components

- **Identity Verification**: Service providers must have verified identities through the Identity pallet
- **Rank-Based Access Control**: Only ambassadors with sufficient rank can register providers or create referrals
- **Evidence Hashing**: In accordance with the evidence handling pattern of the pallet, evidence hashes are stored on-chain while actual evidence is stored off-chain
- **Transparent Referrals**: Service referrals include compensation disclosure for transparency

### Class Diagram

```mermaid
classDiagram
  class AmbassadorGovernance {
    +register_service_provider(provider_account, provider_name, service_types, contact_info, evidence_hash)
    +create_service_referral(provider_id, service_type, description, compensation_disclosed, compensation_details)
    +MinRankForProviderRegistry: u16
    +MinRankForReferral: u16
  }

  class ServiceProviderDetails {
    +provider_account: AccountId
    +provider_name: BoundedString
    +service_types: Vec~ProfessionalServiceType~
    +contact_info: BoundedString
    +registrant: AccountId
    +registered_at: BlockNumber
  }

  class ServiceReferral {
    +provider_id: Hash
    +service_type: ProfessionalServiceType
    +description: BoundedString
    +compensation_disclosed: bool
    +compensation_details: Option~BoundedString~
    +referrer: AccountId
    +created_at: BlockNumber
  }

  class ProfessionalServiceType {
    <<enumeration>>
    Legal
    Financial
    Technical
    Marketing
    Other
  }

  class IdentityVerifier {
    <<interface>>
    +has_identity(who: AccountId): bool
  }

  class RankChecker {
    <<interface>>
    +has_minimum_rank(who: AccountId, min_rank: u16): bool
  }

  AmbassadorGovernance --> ServiceProviderDetails
  AmbassadorGovernance --> ServiceReferral
  ServiceProviderDetails --> ProfessionalServiceType
  ServiceReferral --> ProfessionalServiceType
  AmbassadorGovernance ..> IdentityVerifier: uses
  AmbassadorGovernance ..> RankChecker: uses
```

### Sequence Diagram

```mermaid
sequenceDiagram
  participant A as Ambassador (with sufficient rank)
  participant AG as Ambassador Governance
  participant IP as Identity Pallet
  participant SP as Service Provider
  
  %% Service Provider Registration Flow
  A->>AG: register_service_provider(provider_account, ...)
  AG->>IP: has_identity(provider_account)
  IP-->>AG: true/false
  Note over AG: Verify ambassador has minimum required rank
  AG->>AG: Store provider details
  AG-->>A: ServiceProviderRegistered event
  
  %% Service Referral Flow
  A->>AG: create_service_referral(provider_id, ...)
  Note over AG: Verify ambassador has minimum required rank
  AG->>AG: Check provider exists
  AG->>AG: Validate compensation disclosure
  AG->>AG: Store referral details
  AG-->>A: ServiceReferralCreated event
```

## Ambassador Fellowship Governance Sequence Diagram

```mermaid
sequenceDiagram
  participant SA as Senior Ambassador
  participant AG as Ambassador Governance
  participant EC as Emergency Committee
  participant A as Ambassador
  participant AC as Appeal Committee
  participant PA as Principal Ambassador
  
  %% Emergency Response Flow
  SA->>AG: activate_emergency_protocol(type, severity)
  Note over AG: Emergency created
  SA->>AG: form_emergency_committee(emergency_id, members)
  AG->>EC: Committee formed
  EC->>AG: resolve_emergency(emergency_id, resolution)
  
  %% Appeal Process Flow (independent)
  A->>AG: submit_appeal(decision, justification)
  Note over AG: Appeal created
  SA->>AG: form_appeal_committee(appeal_id, members)
  AG->>AC: Committee formed
  AC->>AG: decide_appeal(appeal_id, decision)
  
  %% Cross-Collective Integration (independent)
  PA->>AG: establish_integration(mechanism, target, participants)
  Note over AG: Integration established
```

## Secretary Collective Diagram

```mermaid
classDiagram
  class SecretaryCollective {
    +add_member(who: AccountId)
    +remove_member(who: AccountId)
    +promote_member(who: AccountId)
    +demote_member(who: AccountId)
  }
  
  class SecretarySalary {
    +register_payment(who: AccountId)
    +get_salary(rank: Rank, who: AccountId): Balance
    +payout(who: AccountId)
  }
  
  class ProxyType {
    <<enumeration>>
    Any
    NonTransfer
    Governance
    Staking
    Fellowship
    Ambassador
    Secretary
  }
  
  SecretaryCollective --> SecretarySalary
  ProxyType --> SecretaryCollective: Secretary proxy type
```

## Decoupled Architecture Diagram

```mermaid
flowchart TB
  subgraph "Core Fellowship System"
    RC[Ranked Collective Pallet]
    CF[Core Fellowship Pallet]
    RC <--> CF
  end
  
  subgraph "Ambassador Fellowship Governance System"
    AG[Ambassador Fellowship Governance Pallet]
    
    subgraph "Governance Mechanisms"
      ER[Emergency Response]
      AP[Appeal Process]
      CI[Cross-Collective Integration]
    end
    
    AG --> ER
    AG --> AP
    AG --> CI
  end
  
  subgraph "Secretary System"
    SC[Secretary Collective]
    SS[Secretary Salary]
    SC --> SS
  end
  
  %% Loose coupling through origins and references
  RC -.->|"Optional<br/>References"| AG
  CF -.->|"Optional<br/>References"| AG
  
  %% Independent operation paths
  AG -->|"Independent<br/>Operation"| IndOp[Independent Governance Operations]
  SC -->|"Independent<br/>Operation"| SecOp[Secretary Operations]
  
  classDef core fill:#FF2670,stroke:#555555,stroke-width:2px
  classDef ambassador fill:#E4FF07,stroke:#555555,stroke-width:2px
  classDef mechanism fill:#DCE2E9,stroke:#555555,stroke-width:1px
  classDef independent fill:#07FFFF,stroke:#555555,stroke-width:1px
  classDef secretary fill:#AEB7CB,stroke:#555555,stroke-width:2px
  
  class RC,CF core
  class AG ambassador
  class ER,AP,CI mechanism
  class IndOp,SecOp independent
  class SC,SS secretary
```