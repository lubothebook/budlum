//! B.U.D. 3.0 content addressing tests: verify that content IDs are
//! deterministic, that the same content always produces the same ID, and that
//! different content produces different IDs.

use crate::core::hash::hash_fields_bytes;

/// Hash of empty content is deterministic.
#[test]
fn empty_content_hash_is_deterministic() {
    let h1 = hash_fields_bytes(&[b"BDLM_CONTENT_ID_V1", b""]);
    let h2 = hash_fields_bytes(&[b"BDLM_CONTENT_ID_V1", b""]);
    assert_eq!(h1, h2);
}

/// Hash of non-empty content is deterministic.
#[test]
fn nonempty_content_hash_is_deterministic() {
    let data = b"hello budlum";
    let h1 = hash_fields_bytes(&[b"BDLM_CONTENT_ID_V1", data]);
    let h2 = hash_fields_bytes(&[b"BDLM_CONTENT_ID_V1", data]);
    assert_eq!(h1, h2);
}

/// Different content produces different hashes.
#[test]
fn different_content_different_hash() {
    let h1 = hash_fields_bytes(&[b"BDLM_CONTENT_ID_V1", b"content A"]);
    let h2 = hash_fields_bytes(&[b"BDLM_CONTENT_ID_V1", b"content B"]);
    assert_ne!(h1, h2);
}

/// Domain tag is part of the hash input (domain separation).
#[test]
fn domain_tag_affects_hash() {
    let data = b"same data";
    let h1 = hash_fields_bytes(&[b"BDLM_CONTENT_ID_V1", data]);
    let h2 = hash_fields_bytes(&[b"BDLM_BLOCK_HASH_V1", data]);
    assert_ne!(h1, h2, "different domain tags must produce different hashes");
}

/// G-5 invariant: hash inputs are length-prefixed. The hash_fields_bytes
/// function prepends the field length, so [b"ab", b"cd"] != [b"abcd"].
#[test]
fn length_prefix_prevents_concatenation_collision() {
    let h1 = hash_fields_bytes(&[b"ab", b"cd"]);
    let h2 = hash_fields_bytes(&[b"abcd"]);
    assert_ne!(h1, h2, "G-5 violation: concatenation collision");
}

/// Three-field hash differs from two-field hash with same total bytes.
#[test]
fn field_count_affects_hash() {
    let h1 = hash_fields_bytes(&[b"a", b"b", b"c"]);
    let h2 = hash_fields_bytes(&[b"a", b"bc"]);
    assert_ne!(h1, h2);
}

/// Hash output is 32 bytes (SHA3-256).
#[test]
fn hash_output_is_32_bytes() {
    let h = hash_fields_bytes(&[b"test"]);
    assert_eq!(h.len(), 32);
}
