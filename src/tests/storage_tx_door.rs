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
use crate::domain::StorageTx;
use crate::execution::executor::Executor;
use crate::storage::{encode_object, ContentManifest, ErasureScheme};

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
