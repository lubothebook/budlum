//! Secp256k1 recovery safety tests for Ethereum compatibility.

use crate::crypto::secp256k1_recover::eip191_digest;

#[test]
fn eip191_digest_is_deterministic() {
    let hash = [42u8; 32];
    let d1 = eip191_digest(&hash);
    let d2 = eip191_digest(&hash);
    assert_eq!(d1, d2);
}

#[test]
fn eip191_digest_is_32_bytes() {
    let hash = [0u8; 32];
    let digest = eip191_digest(&hash);
    assert_eq!(digest.len(), 32);
}

#[test]
fn eip191_different_input_different_digest() {
    let d1 = eip191_digest(&[1u8; 32]);
    let d2 = eip191_digest(&[2u8; 32]);
    assert_ne!(d1, d2);
}
