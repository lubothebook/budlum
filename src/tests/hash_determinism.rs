//! Hash function determinism and domain separation tests.

use crate::core::hash::{hash_fields_bytes, Hash32};

#[test]
fn hash_of_same_input_is_identical() {
    let h1 = hash_fields_bytes(&[b"BDLM_TEST_V1", b"data"]);
    let h2 = hash_fields_bytes(&[b"BDLM_TEST_V1", b"data"]);
    assert_eq!(h1, h2);
}

#[test]
fn hash_of_different_input_differs() {
    let h1 = hash_fields_bytes(&[b"BDLM_TEST_V1", b"data1"]);
    let h2 = hash_fields_bytes(&[b"BDLM_TEST_V1", b"data2"]);
    assert_ne!(h1, h2);
}

#[test]
fn hash_domain_separation_works() {
    let h1 = hash_fields_bytes(&[b"BDLM_BLOCK_V1", b"data"]);
    let h2 = hash_fields_bytes(&[b"BDLM_TX_V1", b"data"]);
    assert_ne!(h1, h2);
}

#[test]
fn hash_field_count_matters() {
    let h1 = hash_fields_bytes(&[b"a", b"b"]);
    let h2 = hash_fields_bytes(&[b"ab"]);
    assert_ne!(h1, h2, "field count must affect the hash (G-5)");
}

#[test]
fn hash_output_is_32_bytes() {
    let h = hash_fields_bytes(&[b"test"]);
    assert_eq!(h.len(), 32);
}

#[test]
fn hash_of_empty_fields() {
    let h = hash_fields_bytes(&[]);
    assert_eq!(h.len(), 32);
}

#[test]
fn hash_is_not_all_zeros() {
    let h = hash_fields_bytes(&[b"BDLM_V1", b"non-trivial"]);
    assert_ne!(h, [0u8; 32]);
}

#[test]
fn hash_is_not_all_ones() {
    let h = hash_fields_bytes(&[b"BDLM_V1", b"non-trivial"]);
    assert_ne!(h, [0xFFu8; 32]);
}
