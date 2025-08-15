use std::time::Instant;
use anyhow::Result;
use parity_scale_codec::Encode;
use sha3::{Digest, Keccak256};
use subxt::ext::sp_core::{sr25519, H256, Pair};
use subxt::utils::{AccountId32, MultiAddress};
use subxt::{
  tx::Payload,
  OnlineClient, PolkadotConfig,
};
use subxt::tx::PairSigner;
use zombienet_sdk_tests::{
  environment::get_spawn_fn,
  small_network,
};

// Identity types
#[derive(Clone, Debug, Encode)]
pub enum Data {
  None,
  Raw(Vec<u8>),
  BlakeTwo256([u8; 32]),
  Sha256([u8; 32]),
  Keccak256([u8; 32]),
  ShaThree256([u8; 32]),
}

#[derive(Clone, Debug, Encode)]
pub struct IdentityInfo {
  pub display: Option<Data>,
  pub legal: Option<Data>,
  pub web: Option<Data>,
  pub riot: Option<Data>,
  pub email: Option<Data>,
  pub pgp_fingerprint: Option<[u8; 20]>,
  pub image: Option<Data>,
  pub twitter: Option<Data>,
  pub additional: Vec<(Data, Data)>,
}

#[derive(Clone, Debug, Encode)]
pub enum Judgement {
  Unknown,
  FeePaid(u128),
  Reasonable,
  KnownGood,
  OutOfDate,
  LowQuality,
  Erroneous,
}

// Constants for ambassador ranks
const RANK_AMBASSADOR: u16 = 1;
const RANK_SENIOR_AMBASSADOR: u16 = 2;
const RANK_LEAD_AMBASSADOR: u16 = 3;
const RANK_AMBASSADOR_ADVISOR: u16 = 4;
const RANK_AMBASSADOR_COORDINATOR: u16 = 5;

// Test account seeds
const ALICE: &str = "//Alice";
const BOB: &str = "//Bob";
const CHARLIE: &str = "//Charlie";
const DAVE: &str = "//Dave";
const EVE: &str = "//Eve";

// Helper functions
fn calculate_hash<T: AsRef<[u8]>>(data: T) -> H256 {
  let mut hasher = Keccak256::new();
  hasher.update(data.as_ref());
  let result = hasher.finalize();
  H256::from_slice(&result)
}

async fn setup_identity_for_account(
  client: &OnlineClient<PolkadotConfig>,
  signer: &PairSigner<PolkadotConfig, sr25519::Pair>,
  name: &str
) -> Result<(), anyhow::Error> {
  // Create identity info
  let identity_info = IdentityInfo {
    display: Some(Data::Raw(name.as_bytes().to_vec())),
    legal: Some(Data::Raw(format!("{} Legal", name).as_bytes().to_vec())),
    web: Some(Data::Raw(format!("https://{}.example.com", name.to_lowercase()).as_bytes().to_vec())),
    riot: Some(Data::Raw(format!("@{}:matrix.org", name.to_lowercase()).as_bytes().to_vec())),
    email: Some(Data::Raw(format!("{}@example.com", name.to_lowercase()).as_bytes().to_vec())),
    pgp_fingerprint: None,
    image: None,
    twitter: Some(Data::Raw(format!("@{}", name.to_lowercase()).as_bytes().to_vec())),
    additional: Vec::new(),
  };

  // Set identity
  let set_identity_tx = Payload::new(
    "Identity",
    "set_identity",
    identity_info
  );

  let result = client
    .tx()
    .sign_and_submit_then_watch(&set_identity_tx, signer, Default::default())
    .await?
    .wait_for_finalized_success()
    .await?;

  log::info!("Identity set for {}: {:?}", name, result.extrinsic_hash());

  // Provide judgment
  let registrar_account = PairSigner::new(sr25519::Pair::from_string_with_seed("//Alice", None).unwrap());
  let provide_judgment_tx = Payload::new(
    "Identity",
    "provide_judgment",
    (
      0u32, // Registrar index
      MultiAddress::Id(signer.account_id().clone()),
      Judgement::Reasonable
    )
  );

  client
    .tx()
    .sign_and_submit_then_watch(&provide_judgment_tx, &registrar_account, Default::default())
    .await?
    .wait_for_finalized_success()
    .await?;

  log::info!("Identity set up for {}", name);
  Ok(())
}

async fn assign_ambassador_rank(
  client: &OnlineClient<PolkadotConfig>,
  alice_signer: &PairSigner<PolkadotConfig, sr25519::Pair>,
  account_id: &AccountId32,
  rank: u16
) -> Result<(), anyhow::Error> {
  log::info!("Assigning rank {} to {:?}", rank, account_id);

  // First induct into the collective
  let induct_tx = Payload::new(
    "RankedCollective",
    "induct",
    account_id.clone()
  );

  client
    .tx()
    .sign_and_submit_then_watch(&induct_tx, alice_signer, Default::default())
    .await?
    .wait_for_finalized_success()
    .await?;

  log::info!("Account inducted into collective");

  // Then promote to the desired rank
  for r in 2..=rank {
    let promote_tx = Payload::new(
      "RankedCollective",
      "promote_member",
      (
        account_id.clone(),
        r
      )
    );

    client
      .tx()
      .sign_and_submit_then_watch(&promote_tx, alice_signer, Default::default())
      .await?
      .wait_for_finalized_success()
      .await?;

    log::info!("Account promoted to rank {}", r);
  }

  log::info!("Successfully assigned rank {} to {:?}", rank, account_id);
  Ok(())
}

async fn assign_technical_collective_membership(
  client: &OnlineClient<PolkadotConfig>,
  alice_signer: &PairSigner<PolkadotConfig, sr25519::Pair>,
  account_id: &AccountId32
) -> Result<(), anyhow::Error> {
  log::info!("Adding {:?} to technical collective", account_id);

  let add_member_tx = Payload::new(
    "TechnicalCommittee",
    "add_member",
    account_id.clone()
  );

  client
    .tx()
    .sign_and_submit_then_watch(&add_member_tx, alice_signer, Default::default())
    .await?
    .wait_for_finalized_success()
    .await?;

  log::info!("Successfully added {:?} to technical collective", account_id);
  Ok(())
}

#[tokio::test]
async fn test_ambassador_governance() -> Result<(), anyhow::Error> {
  let spawn = get_spawn_fn();
  let start = Instant::now();

  // Start a small network with default configuration
  let config = small_network().map_err(|e| anyhow::anyhow!("{:?}", e))?;
  let network = spawn(config).await?;
  let alice = network.get_node("alice")?;
  let client = alice.wait_client().await?;

  log::info!("⏱️  Network started in {:?}", start.elapsed());

  // Create signers for test accounts
  let alice_signer = PairSigner::new(sr25519::Pair::from_string_with_seed(ALICE, None).unwrap());
  let bob_signer = PairSigner::new(sr25519::Pair::from_string_with_seed(BOB, None).unwrap());
  let charlie_signer = PairSigner::new(sr25519::Pair::from_string_with_seed(CHARLIE, None).unwrap());

  // Setup identities for test accounts
  log::info!("Setting up identities for test accounts");
  setup_identity_for_account(&client, &alice_signer, "Alice").await?;
  setup_identity_for_account(&client, &bob_signer, "Bob").await?;
  setup_identity_for_account(&client, &charlie_signer, "Charlie").await?;

  // Assign ambassador ranks
  log::info!("Assigning ambassador ranks");
  assign_ambassador_rank(&client, &alice_signer, alice_signer.account_id(), RANK_AMBASSADOR_COORDINATOR).await?;
  assign_ambassador_rank(&client, &alice_signer, bob_signer.account_id(), RANK_SENIOR_AMBASSADOR).await?;
  assign_ambassador_rank(&client, &alice_signer, charlie_signer.account_id(), RANK_AMBASSADOR).await?;

  // Assign technical collective membership
  log::info!("Adding Alice to technical collective");
  assign_technical_collective_membership(&client, &alice_signer, alice_signer.account_id()).await?;

  // Test emergency protocol activation
  log::info!("Testing emergency protocol activation");

  // Create evidence for emergency protocol
  let emergency_description = "Critical security vulnerability in ambassador portal (evidence: ipfs://QmHash1)";
  let emergency_evidence = "Detailed technical analysis of the vulnerability and potential impact";
  let emergency_evidence_hash = calculate_hash(emergency_evidence);

  // Activate emergency protocol
  let activate_emergency_tx = Payload::new(
    "AmbassadorGovernance",
    "activate_emergency_protocol",
    (
      emergency_description,
      emergency_evidence_hash
    )
  );

  client
    .tx()
    .sign_and_submit_then_watch(&activate_emergency_tx, &alice_signer, Default::default())
    .await?
    .wait_for_finalized_success()
    .await?;

  log::info!("Emergency protocol activated");

  // Form emergency committee
  let committee_description = "Emergency committee to address security vulnerability (evidence: ipfs://QmHash2)";
  let committee_evidence = "Committee formation criteria and selection process";
  let committee_evidence_hash = calculate_hash(committee_evidence);
  let committee_members = vec![
    alice_signer.account_id().clone(),
    bob_signer.account_id().clone()
  ];

  let form_committee_tx = Payload::new(
    "AmbassadorGovernance",
    "form_emergency_committee",
    (
      committee_description,
      committee_evidence_hash,
      committee_members
    )
  );

  client
    .tx()
    .sign_and_submit_then_watch(&form_committee_tx, &alice_signer, Default::default())
    .await?
    .wait_for_finalized_success()
    .await?;

  log::info!("Emergency committee formed");

  // Resolve emergency
  let resolution_summary = "Security vulnerability patched and verified (evidence: ipfs://QmHash3)";
  let resolution_evidence = "Technical details of the patch and security audit results";
  let resolution_evidence_hash = calculate_hash(resolution_evidence);

  let resolve_emergency_tx = Payload::new(
    "AmbassadorGovernance",
    "resolve_emergency",
    (
      resolution_summary,
      resolution_evidence_hash
    )
  );

  client
    .tx()
    .sign_and_submit_then_watch(&resolve_emergency_tx, &alice_signer, Default::default())
    .await?
    .wait_for_finalized_success()
    .await?;

  log::info!("Emergency resolved");

  log::info!("All tests completed successfully");
  Ok(())
}
