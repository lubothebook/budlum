//! B.U.D. storage writes as signed, in-block transactions.
//!
//! The storage registry is hashed into the account state root
//! (`AccountState::calculate_state_root` folds `storage_registry.root()`), so
//! a write to it is a consensus write. Until this module every write reached
//! the registry through a local RPC command on one node, out of block, which
//! is why the chain actor refuses every storage command on mainnet: two
//! honest nodes would end up with different state roots.
//!
//! This family moves those writes into the block. Each variant is applied by
//! the executor's `Storage` arm, inside block execution, from a transaction
//! the sender signed. The first variant is manifest registration; deals,
//! challenges, coding audits and the operator class follow in their own
//! steps, so each one lands with its own tests.

use crate::core::account::AccountState;
use crate::core::address::Address;
use crate::storage::ContentManifest;
use serde::{Deserialize, Serialize};
use std::fmt;

/// One storage write. The sender is `tx.from`, already signed; no variant
/// carries an actor field a caller could fill with someone else's address.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageTx {
    /// Record a content manifest so deals, challenges and repair can index
    /// it. The sender must be the manifest's declared owner, which turns the
    /// owner field from a first-writer label into a signed claim.
    RegisterManifest { manifest: ContentManifest },
}

/// Why a storage transaction was refused. Every refusal leaves the registry
/// as it was.
///
/// WIRING: returned by `execute_storage_tx`; the executor's `Storage` arm
/// reads it as the `storage_tx_failed` message rather than by name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageTxError {
    /// The manifest failed `validate_untrusted`: its id does not derive from
    /// its contents, or its shard list, sizes or scheme disagree.
    InvalidManifest(String),
    /// The manifest names an owner other than the signer. A signer may only
    /// register content in its own name.
    OwnerIsNotSender { owner: Address, sender: Address },
    /// The manifest is already registered. Registration is first-writer-wins
    /// and a repeat would pay a fee for a write that changes nothing.
    AlreadyRegistered,
    /// Paid content may not be registered as plaintext: a plaintext content
    /// id is the hash of the bytes, and putting it on chain leaks the asset.
    PaidContentAsPlaintext(String),
    /// The registry refused the manifest's source regime or dictionary.
    Refused(String),
}

impl fmt::Display for StorageTxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidManifest(e) => write!(f, "invalid manifest: {e}"),
            Self::OwnerIsNotSender { owner, sender } => write!(
                f,
                "manifest owner {owner} is not the sender {sender}; a signer registers content \
                 only in its own name"
            ),
            Self::AlreadyRegistered => write!(f, "manifest is already registered"),
            Self::PaidContentAsPlaintext(e) => {
                write!(f, "refusing to register paid content as plaintext: {e}")
            }
            Self::Refused(e) => write!(f, "storage registry refused the manifest: {e}"),
        }
    }
}

impl std::error::Error for StorageTxError {}

/// Apply one storage transaction to `state` on behalf of `sender`.
///
/// Every check runs before the registry is written, so a refusal changes
/// nothing. The executor charges the fee and advances the nonce only when
/// this returns `Ok`.
///
/// # Errors
///
/// Returns the [`StorageTxError`] variant that names the failed check.
pub fn execute_storage_tx(
    state: &mut AccountState,
    sender: &Address,
    tx: &StorageTx,
) -> Result<(), StorageTxError> {
    match tx {
        StorageTx::RegisterManifest { manifest } => register_manifest(state, sender, manifest),
    }
}

fn register_manifest(
    state: &mut AccountState,
    sender: &Address,
    manifest: &ContentManifest,
) -> Result<(), StorageTxError> {
    // The id is re-derived from the contents. The signing preimage commits
    // the id rather than every shard, so this check is what binds the
    // shards, the scheme, the source, the edition, the dictionary and the
    // encryption claim to the signature.
    manifest
        .validate_untrusted()
        .map_err(StorageTxError::InvalidManifest)?;
    if manifest.owner != *sender {
        return Err(StorageTxError::OwnerIsNotSender {
            owner: manifest.owner,
            sender: *sender,
        });
    }
    if state
        .storage_registry
        .get_manifest(&manifest.manifest_id)
        .is_some()
    {
        return Err(StorageTxError::AlreadyRegistered);
    }
    if matches!(
        manifest.encryption,
        crate::storage::ContentEncryption::Plaintext
    ) {
        state
            .marketplace
            .check_content_may_be_public(&manifest.manifest_id)
            .map_err(StorageTxError::PaidContentAsPlaintext)?;
    }
    state
        .storage_registry
        .register_manifest_with_source(manifest)
        .map_err(|e| StorageTxError::Refused(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::{encode_object, ErasureScheme};

    fn owner() -> Address {
        Address::from([7u8; 32])
    }

    fn manifest_owned_by(who: Address) -> ContentManifest {
        // Distinct bytes per stripe: identical data shards share a content
        // id, and a code word that repeats one is refused.
        let bytes: Vec<u8> = (0..4096u32).map(|i| (i % 251) as u8).collect();
        let enc = encode_object(&bytes, ErasureScheme { k: 4, n: 6 }).expect("encode");
        let mut m = enc.to_manifest().expect("manifest");
        m.owner = who;
        m
    }

    #[test]
    fn the_owner_registers_its_manifest() {
        let mut state = AccountState::new();
        let m = manifest_owned_by(owner());
        execute_storage_tx(
            &mut state,
            &owner(),
            &StorageTx::RegisterManifest {
                manifest: m.clone(),
            },
        )
        .expect("the owner registers");
        assert_eq!(
            state.storage_registry.get_manifest(&m.manifest_id),
            Some(&m)
        );
    }

    #[test]
    fn a_signer_cannot_register_content_in_someone_elses_name() {
        let mut state = AccountState::new();
        let m = manifest_owned_by(owner());
        let stranger = Address::from([9u8; 32]);
        let err = execute_storage_tx(
            &mut state,
            &stranger,
            &StorageTx::RegisterManifest {
                manifest: m.clone(),
            },
        )
        .unwrap_err();
        assert!(matches!(err, StorageTxError::OwnerIsNotSender { .. }));
        assert!(state
            .storage_registry
            .get_manifest(&m.manifest_id)
            .is_none());
    }

    #[test]
    fn a_manifest_whose_id_does_not_derive_is_refused() {
        let mut state = AccountState::new();
        let mut m = manifest_owned_by(owner());
        m.manifest_id = crate::storage::ContentId::of(b"chosen by the caller");
        let err = execute_storage_tx(
            &mut state,
            &owner(),
            &StorageTx::RegisterManifest {
                manifest: m.clone(),
            },
        )
        .unwrap_err();
        assert!(matches!(err, StorageTxError::InvalidManifest(_)));
        assert!(state.storage_registry.is_empty());
    }

    #[test]
    fn a_second_registration_is_refused_and_changes_nothing() {
        let mut state = AccountState::new();
        let m = manifest_owned_by(owner());
        let tx = StorageTx::RegisterManifest {
            manifest: m.clone(),
        };
        execute_storage_tx(&mut state, &owner(), &tx).expect("first");
        let root = state.storage_registry.root();
        assert_eq!(
            execute_storage_tx(&mut state, &owner(), &tx).unwrap_err(),
            StorageTxError::AlreadyRegistered
        );
        assert_eq!(state.storage_registry.root(), root);
    }
}
