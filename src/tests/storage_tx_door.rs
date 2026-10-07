//! The executor's storage arm: manifest registration in block.
//!
//! The registry rules live in `src/domain/storage_tx.rs` and are unit-tested
//! there. This file pins what only the block-application path can see: the
//! frame refusal for a value-carrying storage tx, the fee and nonce
//! epilogue, that a refusal charges nothing, and that the write reaches the
//! state root, which is the reason it has to happen in a block at all.

use crate::core::account::AccountState;
use crate::core::address::Address;
use crate::core::transaction::{Transaction, TransactionType, DEFAULT_CHAIN_ID};
use crate::domain::storage_deal::OperatorClass;
use crate::domain::StorageTx;
use crate::execution::executor::Executor;
use crate::storage::{
    encode_object, ContentId, ContentManifest, ErasureScheme, MobileAvailabilityClass,
    MobileSelfContentPolicy, MobileSelfProfile,
};

fn addr(byte: u8) -> Address {
    Address::from([byte; 32])
}

fn manifest_owned_by(owner: Address) -> ContentManifest {
    let bytes: Vec<u8> = (0..4096u32).map(|i| (i % 253) as u8).collect();
    let mut m = encode_object(&bytes, ErasureScheme { k: 4, n: 6 })
        .expect("encode")
        .to_manifest()
        .expect("manifest");
    m.owner = owner;
    m
}

fn storage_tx(from: Address, amount: u64, body: StorageTx, nonce: u64) -> Transaction {
    Transaction::new_with_chain_id(
        from,
        Address::zero(),
        amount,
        1,
        nonce,
        vec![],
        DEFAULT_CHAIN_ID,
        TransactionType::Storage(body),
    )
}

fn apply(state: &mut AccountState, tx: Transaction) -> Result<(), String> {
    Executor::apply_transaction_checked(state, &tx)
        .map_err(|e| format!("{}: {}", e.code(), e.message()))
}

#[test]
fn a_manifest_registers_in_block_and_moves_the_state_root() {
    let alice = addr(1);
    let mut state = AccountState::new();
    state.add_balance(&alice, 1_000);
    let before = state.calculate_state_root();
    let m = manifest_owned_by(alice);

    apply(
        &mut state,
        storage_tx(
            alice,
            0,
            StorageTx::RegisterManifest {
                manifest: m.clone(),
            },
            0,
        ),
    )
    .expect("the owner registers its manifest");

    assert_eq!(
        state.storage_registry.get_manifest(&m.manifest_id),
        Some(&m)
    );
    assert_eq!(state.get_balance(&alice), 999, "the fee is charged");
    assert_eq!(state.get_nonce(&alice), 1, "the nonce advances");
    assert_ne!(
        state.calculate_state_root(),
        before,
        "a registry write that does not reach the root would let nodes disagree silently"
    );
}

#[test]
fn a_storage_tx_carrying_value_is_refused_before_anything_moves() {
    let alice = addr(1);
    let mut state = AccountState::new();
    state.add_balance(&alice, 1_000);
    let m = manifest_owned_by(alice);

    let err = apply(
        &mut state,
        storage_tx(
            alice,
            5,
            StorageTx::RegisterManifest {
                manifest: m.clone(),
            },
            0,
        ),
    )
    .unwrap_err();

    assert!(err.contains("storage_amount_must_be_zero"), "{err}");
    assert!(state
        .storage_registry
        .get_manifest(&m.manifest_id)
        .is_none());
    assert_eq!(state.get_balance(&alice), 1_000);
    assert_eq!(state.get_nonce(&alice), 0);
}

#[test]
fn a_refused_registration_charges_nothing() {
    let alice = addr(1);
    let mallory = addr(2);
    let mut state = AccountState::new();
    state.add_balance(&mallory, 1_000);
    let m = manifest_owned_by(alice);

    let err = apply(
        &mut state,
        storage_tx(
            mallory,
            0,
            StorageTx::RegisterManifest {
                manifest: m.clone(),
            },
            0,
        ),
    )
    .unwrap_err();

    assert!(err.contains("storage_tx_failed"), "{err}");
    assert!(state
        .storage_registry
        .get_manifest(&m.manifest_id)
        .is_none());
    assert_eq!(state.get_balance(&mallory), 1_000);
    assert_eq!(state.get_nonce(&mallory), 0);
}

#[test]
fn the_signature_commits_the_fields_the_manifest_id_leaves_out() {
    let alice = addr(1);
    let m = manifest_owned_by(alice);
    let hash_of = |manifest: ContentManifest| {
        storage_tx(alice, 0, StorageTx::RegisterManifest { manifest }, 0).calculate_hash()
    };
    let base = hash_of(m.clone());

    let mut other_owner = m.clone();
    other_owner.owner = addr(9);
    assert_ne!(hash_of(other_owner), base, "owner is outside the id");

    let mut other_len = m.clone();
    other_len.content_size = m.content_size.saturating_sub(1);
    assert_ne!(hash_of(other_len), base, "content_size is outside the id");
}

fn declare_policy_tx(from: Address, m: &ContentManifest, nonce: u64) -> Transaction {
    storage_tx(
        from,
        0,
        StorageTx::DeclareSelfHostPolicy {
            manifest_id: m.manifest_id,
            policy: self_host_policy(from, m),
            profile: self_host_profile(from),
        },
        nonce,
    )
}

fn self_host_policy(owner: Address, m: &ContentManifest) -> MobileSelfContentPolicy {
    MobileSelfContentPolicy {
        content_id: m.shards[0].shard_id,
        owner,
        critical: false,
        required_paid_replicas: 1,
        self_host_allowed: true,
    }
}

fn self_host_profile(owner: Address) -> MobileSelfProfile {
    MobileSelfProfile {
        owner,
        device_commitment: [3u8; 32],
        availability: MobileAvailabilityClass::Scheduled,
        max_storage_bytes: 1 << 20,
        metered_network_ok: false,
        battery_saver_aware: true,
        last_seen_block: 5,
    }
}

#[test]
fn an_operator_class_declaration_moves_the_state_root_and_charges_the_fee() {
    let alice = addr(1);
    let mut state = AccountState::new();
    state.add_balance(&alice, 1_000);
    let before = state.calculate_state_root();

    apply(
        &mut state,
        storage_tx(
            alice,
            0,
            StorageTx::DeclareOperatorClass {
                class: OperatorClass::Mobile,
            },
            0,
        ),
    )
    .expect("the operator declares its class");

    assert_eq!(
        state.storage_registry.operator_class(&alice),
        OperatorClass::Mobile
    );
    assert_eq!(state.get_balance(&alice), 999);
    assert_eq!(state.get_nonce(&alice), 1);
    assert_ne!(state.calculate_state_root(), before);
}

#[test]
fn a_refused_class_declaration_charges_nothing() {
    let alice = addr(1);
    let mut state = AccountState::new();
    state.add_balance(&alice, 1_000);

    let err = apply(
        &mut state,
        storage_tx(
            alice,
            0,
            StorageTx::DeclareOperatorClass {
                class: OperatorClass::AlwaysOn,
            },
            0,
        ),
    )
    .unwrap_err();

    assert!(err.contains("storage_tx_failed"), "{err}");
    assert_eq!(state.get_balance(&alice), 1_000);
    assert_eq!(state.get_nonce(&alice), 0);
}

#[test]
fn a_self_host_policy_declaration_moves_the_state_root_and_charges_the_fee() {
    let alice = addr(1);
    let mut state = AccountState::new();
    state.add_balance(&alice, 1_000);
    let m = manifest_owned_by(alice);
    apply(
        &mut state,
        storage_tx(
            alice,
            0,
            StorageTx::RegisterManifest {
                manifest: m.clone(),
            },
            0,
        ),
    )
    .expect("register");
    let before = state.calculate_state_root();

    apply(&mut state, declare_policy_tx(alice, &m, 1)).expect("the owner declares");

    assert!(state
        .storage_registry
        .self_host_policies
        .contains_key(&(m.manifest_id, m.shards[0].shard_id)));
    assert_eq!(state.get_balance(&alice), 998);
    assert_eq!(state.get_nonce(&alice), 2);
    assert_ne!(state.calculate_state_root(), before);
}

#[test]
fn a_refused_self_host_policy_charges_nothing() {
    let alice = addr(1);
    let mut state = AccountState::new();
    state.add_balance(&alice, 1_000);
    let m = manifest_owned_by(alice);

    let err = apply(&mut state, declare_policy_tx(alice, &m, 0)).unwrap_err();

    assert!(err.contains("storage_tx_failed"), "{err}");
    assert!(state.storage_registry.self_host_policies.is_empty());
    assert_eq!(state.get_balance(&alice), 1_000);
    assert_eq!(state.get_nonce(&alice), 0);
}

#[test]
fn the_signature_commits_every_policy_and_profile_field() {
    let alice = addr(1);
    let m = manifest_owned_by(alice);
    let hash_of =
        |manifest_id: ContentId, policy: MobileSelfContentPolicy, profile: MobileSelfProfile| {
            storage_tx(
                alice,
                0,
                StorageTx::DeclareSelfHostPolicy {
                    manifest_id,
                    policy,
                    profile,
                },
                0,
            )
            .calculate_hash()
        };
    let policy = self_host_policy(alice, &m);
    let profile = self_host_profile(alice);
    let base = hash_of(m.manifest_id, policy.clone(), profile.clone());

    let with_policy = |f: &dyn Fn(&mut MobileSelfContentPolicy)| {
        let mut p = policy.clone();
        f(&mut p);
        hash_of(m.manifest_id, p, profile.clone())
    };
    let with_profile = |f: &dyn Fn(&mut MobileSelfProfile)| {
        let mut p = profile.clone();
        f(&mut p);
        hash_of(m.manifest_id, policy.clone(), p)
    };

    assert_ne!(
        hash_of(ContentId([1u8; 32]), policy.clone(), profile.clone()),
        base
    );
    assert_ne!(with_policy(&|p| p.content_id = ContentId([2u8; 32])), base);
    assert_ne!(with_policy(&|p| p.owner = addr(9)), base);
    assert_ne!(with_policy(&|p| p.critical = true), base);
    assert_ne!(with_policy(&|p| p.required_paid_replicas = 2), base);
    assert_ne!(with_policy(&|p| p.self_host_allowed = false), base);
    assert_ne!(with_profile(&|p| p.owner = addr(9)), base);
    assert_ne!(with_profile(&|p| p.device_commitment = [4u8; 32]), base);
    assert_ne!(
        with_profile(&|p| p.availability = MobileAvailabilityClass::Opportunistic),
        base
    );
    assert_ne!(
        with_profile(&|p| p.availability = MobileAvailabilityClass::AlwaysOnReplica),
        base
    );
    assert_ne!(with_profile(&|p| p.max_storage_bytes = 2), base);
    assert_ne!(with_profile(&|p| p.metered_network_ok = true), base);
    assert_ne!(with_profile(&|p| p.battery_saver_aware = false), base);
    assert_ne!(with_profile(&|p| p.last_seen_block = 6), base);
}

#[test]
fn the_signature_commits_the_declared_operator_class() {
    let alice = addr(1);
    let hash_of =
        |class| storage_tx(alice, 0, StorageTx::DeclareOperatorClass { class }, 0).calculate_hash();
    assert_ne!(
        hash_of(OperatorClass::AlwaysOn),
        hash_of(OperatorClass::Mobile)
    );
}

fn register_hash(from: Address, manifest: ContentManifest) -> String {
    storage_tx(from, 0, StorageTx::RegisterManifest { manifest }, 0).calculate_hash()
}

#[test]
fn the_registration_signature_commits_the_whole_manifest() {
    let alice = addr(1);
    let m = manifest_owned_by(alice);
    let base = register_hash(alice, m.clone());
    let changed = |f: &dyn Fn(&mut ContentManifest)| {
        let mut c = m.clone();
        f(&mut c);
        register_hash(alice, c)
    };

    assert_ne!(
        changed(&|c| c.shards[0].shard_id = ContentId([7u8; 32])),
        base
    );
    assert_ne!(changed(&|c| c.shards[1].index += 1), base);
    assert_ne!(changed(&|c| c.shards[1].size += 1), base);
    assert_ne!(
        changed(&|c| c.shards[0].kind = crate::storage::ShardKind::Parity),
        base
    );
    assert_ne!(changed(&|c| c.erasure.k += 1), base);
    assert_ne!(changed(&|c| c.erasure.n += 1), base);
    assert_ne!(
        changed(&|c| c.dictionary_id = Some(ContentId([5u8; 32]))),
        base
    );
    assert_ne!(
        changed(
            &|c| c.encryption = crate::storage::ContentEncryption::ClientSide(
                crate::storage::ContentCipher::Aes256Gcm
            )
        ),
        base
    );
    assert_ne!(
        changed(&|c| c.edition = crate::storage::BudStorageEdition::Three),
        base
    );
    assert_ne!(
        changed(&|c| c.source = crate::storage::ContentSource::Generated(generated_spec())),
        base
    );
    assert_ne!(changed(&|c| c.shard_count += 1), base);
}

fn generated_spec() -> crate::storage::generated::GeneratedSpec {
    crate::storage::generated::GeneratedSpec {
        generator: crate::storage::generated::GeneratorId::Avatar,
        seed: [1u8; 32],
        output_len: 64,
        step_budget: 10,
    }
}

#[test]
fn two_splits_of_source_and_dictionary_that_share_an_id_sign_differently() {
    let alice = addr(1);
    let spec = generated_spec();
    let digest = ContentId(crate::storage::generated::generated_spec_digest(&spec));
    // The id of the first comes from the real derivation.
    let a = manifest_owned_by(alice).with_source(crate::storage::ContentSource::Generated(spec));
    let mut b = a.clone();
    b.source = crate::storage::ContentSource::Stored;
    b.dictionary_id = Some(digest);
    // The id preimage appends the source bytes (tag 1 then the digest) and
    // the dictionary bytes (1 then the id) without separating them, and the
    // classic edition adds none. The second manifest therefore verifies
    // under the first one's id.
    b.verify_id().expect("the real derivation collides");
    assert_eq!(a.manifest_id, b.manifest_id);
    assert_ne!(register_hash(alice, a), register_hash(alice, b));
}

#[test]
fn a_generated_registration_is_refused_and_charges_nothing() {
    let alice = addr(1);
    let mut state = AccountState::new();
    state.add_balance(&alice, 1_000);
    let m = manifest_owned_by(alice)
        .with_source(crate::storage::ContentSource::Generated(generated_spec()));

    let err = apply(
        &mut state,
        storage_tx(
            alice,
            0,
            StorageTx::RegisterManifest {
                manifest: m.clone(),
            },
            0,
        ),
    )
    .unwrap_err();

    assert!(err.contains("storage_tx_failed"), "{err}");
    assert!(err.contains("not priced"), "{err}");
    assert!(state
        .storage_registry
        .get_manifest(&m.manifest_id)
        .is_none());
    assert_eq!(state.get_balance(&alice), 1_000);
    assert_eq!(state.get_nonce(&alice), 0);
}
