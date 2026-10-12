use crate::core::address::Address;
use crate::domain::Hash32;
use crate::storage::assignment::{assign_object, AssignmentError, ShardCandidate};
use crate::storage::erasure::{encode_object, ErasureError, MAX_TOTAL_SHARDS};
use crate::storage::manifest::{ContentEncryption, ContentManifest, ErasureScheme, ShardRef};
use std::collections::BTreeSet;

/// Upper bound on all shard bytes returned by one prepared upload.
pub const MAX_PREPARED_STORAGE_BYTES: usize = 4 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageUploadContext {
    /// Chain whose validator set determines placement.
    pub chain_id: u64,
    /// Epoch at the chain tip when this context was read.
    pub epoch: u64,
    /// Domain-separated placement entropy derived from the chain tip.
    pub entropy: Hash32,
    /// Active validators eligible to hold the upload's shards.
    pub candidates: Vec<ShardCandidate>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedShard {
    pub reference: ShardRef,
    pub holder: Address,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedStorageUpload {
    pub manifest: ContentManifest,
    pub shards: Vec<PreparedShard>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageUploadError {
    EmptyData,
    InvalidScheme(String),
    EncodedSizeExceeded { size: usize, limit: usize },
    InsufficientHolders { needed: usize, available: usize },
    Erasure(ErasureError),
    Manifest(String),
    Assignment(AssignmentError),
    InconsistentEncoding,
}

impl std::fmt::Display for StorageUploadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyData => write!(f, "storage upload data must not be empty"),
            Self::InvalidScheme(reason) => write!(f, "invalid storage erasure scheme: {reason}"),
            Self::EncodedSizeExceeded { size, limit } => write!(
                f,
                "encoded storage upload is {size} bytes, exceeding the {limit} byte limit"
            ),
            Self::InsufficientHolders { needed, available } => write!(
                f,
                "storage placement needs {needed} distinct staked holders, found {available}"
            ),
            Self::Erasure(error) => write!(f, "storage encoding failed: {error}"),
            Self::Manifest(reason) => write!(f, "storage manifest construction failed: {reason}"),
            Self::Assignment(error) => write!(f, "storage placement failed: {error}"),
            Self::InconsistentEncoding => {
                write!(f, "storage encoder returned a code word inconsistent with its manifest")
            }
        }
    }
}

impl std::error::Error for StorageUploadError {}

impl From<ErasureError> for StorageUploadError {
    fn from(error: ErasureError) -> Self {
        Self::Erasure(error)
    }
}

impl From<AssignmentError> for StorageUploadError {
    fn from(error: AssignmentError) -> Self {
        Self::Assignment(error)
    }
}

/// Encode an object, build its manifest, and assign one distinct staked holder
/// to each shard. `encryption` records whether the supplied bytes were encrypted
/// before this call; this function does not handle keys or encrypt the input.
/// The returned order is the code-word order used by the manifest and
/// reconstruction path.
///
/// # Errors
///
/// Refuses invalid or oversized code words, manifests that do not validate,
/// and candidate sets too small to place every shard on a distinct address.
pub fn prepare_storage_upload(
    data: &[u8],
    scheme: ErasureScheme,
    owner: Address,
    encryption: ContentEncryption,
    entropy: &Hash32,
    candidates: &[ShardCandidate],
) -> Result<PreparedStorageUpload, StorageUploadError> {
    if data.is_empty() {
        return Err(StorageUploadError::EmptyData);
    }
    scheme
        .validate()
        .map_err(StorageUploadError::InvalidScheme)?;
    let total_shards = usize::try_from(scheme.n)
        .map_err(|_| StorageUploadError::InvalidScheme("shard count does not fit usize".into()))?;
    if total_shards > MAX_TOTAL_SHARDS {
        return Err(StorageUploadError::InvalidScheme(format!(
            "shard count {total_shards} exceeds {MAX_TOTAL_SHARDS}"
        )));
    }
    let data_shards = usize::try_from(scheme.k).map_err(|_| {
        StorageUploadError::InvalidScheme("data shard count does not fit usize".into())
    })?;
    let stripe_len = data.len().div_ceil(data_shards);
    let encoded_size = stripe_len
        .checked_mul(total_shards)
        .ok_or(StorageUploadError::EncodedSizeExceeded {
            size: usize::MAX,
            limit: MAX_PREPARED_STORAGE_BYTES,
        })?;
    if encoded_size > MAX_PREPARED_STORAGE_BYTES {
        return Err(StorageUploadError::EncodedSizeExceeded {
            size: encoded_size,
            limit: MAX_PREPARED_STORAGE_BYTES,
        });
    }

    let distinct_holders: BTreeSet<Address> = candidates
        .iter()
        .filter(|candidate| candidate.stake > 0)
        .map(|candidate| candidate.address)
        .collect();
    if distinct_holders.len() < total_shards {
        return Err(StorageUploadError::InsufficientHolders {
            needed: total_shards,
            available: distinct_holders.len(),
        });
    }

    let encoded = encode_object(data, scheme)?;
    let manifest = encoded
        .to_manifest()
        .map_err(StorageUploadError::Manifest)?
        .with_encryption(encryption)
        .with_owner(owner);
    manifest
        .validate_untrusted()
        .map_err(StorageUploadError::Manifest)?;

    let shard_ids: Vec<_> = manifest.shards.iter().map(|shard| shard.shard_id).collect();
    let holders = assign_object(&shard_ids, entropy, candidates)?;
    if holders.len() != encoded.shards.len()
        || holders.iter().copied().collect::<BTreeSet<_>>().len() != holders.len()
    {
        return Err(StorageUploadError::InconsistentEncoding);
    }

    let mut references = manifest.shards.iter().cloned().collect::<Vec<_>>();
    references.sort_by_key(|shard| shard.index);
    let shards = encoded
        .shards
        .into_iter()
        .zip(references)
        .zip(holders)
        .map(|((bytes, reference), holder)| PreparedShard {
            reference,
            holder,
            bytes,
        })
        .collect();

    Ok(PreparedStorageUpload { manifest, shards })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::{reconstruct_object, verify_object_encoding};

    fn candidates(n: usize) -> Vec<ShardCandidate> {
        (0..n)
            .map(|i| ShardCandidate {
                address: Address([u8::try_from(i + 1).expect("test candidate index"); 32]),
                stake: 1_000 + i as u64,
            })
            .collect()
    }

    fn data() -> Vec<u8> {
        (0..128).map(|i| (i * 37 + 11) as u8).collect()
    }

    #[test]
    fn upload_preparation_connects_encoding_manifest_and_placement() {
        let source = data();
        let prepared = prepare_storage_upload(
            &source,
            ErasureScheme { k: 4, n: 6 },
            Address([9; 32]),
            ContentEncryption::Plaintext,
            &[7; 32],
            &candidates(6),
        )
        .expect("the upload plan is complete");

        assert_eq!(prepared.manifest.erasure, ErasureScheme { k: 4, n: 6 });
        assert_eq!(prepared.manifest.owner, Address([9; 32]));
        assert_eq!(prepared.shards.len(), 6);
        assert_eq!(
            prepared
                .shards
                .iter()
                .map(|shard| shard.holder)
                .collect::<BTreeSet<_>>()
                .len(),
            6
        );
        verify_object_encoding(&source, &prepared.manifest)
            .expect("the returned manifest matches the produced code word");

        let survivors: Vec<Option<Vec<u8>>> = prepared
            .shards
            .iter()
            .enumerate()
            .map(|(index, shard)| (index != 1 && index != 4).then(|| shard.bytes.clone()))
            .collect();
        let rebuilt = reconstruct_object(&prepared.manifest, &survivors)
            .expect("any four of the six prepared shards reconstruct");
        assert_eq!(rebuilt, source);
    }

    #[test]
    fn upload_records_client_encryption_in_the_manifest() {
        let encryption =
            ContentEncryption::ClientSide(crate::storage::ContentCipher::XChaCha20Poly1305);
        let prepared = prepare_storage_upload(
            &[0xA5; 64],
            ErasureScheme { k: 1, n: 1 },
            Address([9; 32]),
            encryption,
            &[7; 32],
            &candidates(1),
        )
        .expect("already-encrypted input can be described in the manifest");

        assert_eq!(prepared.manifest.encryption, encryption);
        assert!(prepared.manifest.is_client_encrypted());
    }

    #[test]
    fn upload_refuses_a_candidate_set_that_cannot_keep_shards_distinct() {
        let err = prepare_storage_upload(
            &data(),
            ErasureScheme { k: 4, n: 6 },
            Address([9; 32]),
            ContentEncryption::Plaintext,
            &[7; 32],
            &candidates(5),
        )
        .expect_err("the sixth shard must not be co-located by fallback");
        assert_eq!(
            err,
            StorageUploadError::InsufficientHolders {
                needed: 6,
                available: 5,
            }
        );
    }

    #[test]
    fn upload_size_is_bounded_before_encoding() {
        let large = vec![1u8; MAX_PREPARED_STORAGE_BYTES + 1];
        let err = prepare_storage_upload(
            &large,
            ErasureScheme { k: 1, n: 1 },
            Address([9; 32]),
            ContentEncryption::Plaintext,
            &[7; 32],
            &candidates(1),
        )
        .expect_err("oversized code words refuse before allocation");
        assert!(matches!(err, StorageUploadError::EncodedSizeExceeded { .. }));
    }

    #[test]
    fn upload_refuses_duplicate_coded_shards() {
        let err = prepare_storage_upload(
            &[0u8; 64],
            ErasureScheme { k: 2, n: 3 },
            Address([9; 32]),
            ContentEncryption::Plaintext,
            &[7; 32],
            &candidates(3),
        )
        .expect_err("a repeated content id is not independent redundancy");
        assert!(matches!(err, StorageUploadError::Manifest(_)));
    }
}
