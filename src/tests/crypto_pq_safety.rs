//! Post-quantum cryptography safety tests: ML-DSA key generation, signing,
//! verification, BLS keypair, and scheme consistency.

use crate::crypto::primitives::{
    BlsKeypair, CryptoError, PqKeyPair, PQ_SCHEME_ID,
};

#[test]
fn pq_scheme_id_is_ml_dsa_65() {
    assert!(
        PQ_SCHEME_ID.contains("ml-dsa") || PQ_SCHEME_ID.contains("dilithium"),
        "PQ scheme must be ML-DSA or Dilithium variant, got: {}",
        PQ_SCHEME_ID
    );
}

#[test]
fn pq_keypair_generates() {
    let kp = PqKeyPair::generate();
    assert!(!kp.public_key_bytes().is_empty());
}

#[test]
fn pq_keypair_sign_verify_round_trip() {
    let kp = PqKeyPair::generate();
    let ctx = b"BDLM_TEST";
    let msg = b"testnet audit message";
    let sig = kp.sign(ctx, msg).expect("sign");
    let result = PqKeyPair::verify(kp.public_key_bytes(), ctx, msg, &sig);
    assert!(result.is_ok(), "PQ signature verification failed");
}

#[test]
fn pq_keypair_rejects_wrong_message() {
    let kp = PqKeyPair::generate();
    let ctx = b"BDLM_TEST";
    let sig = kp.sign(ctx, b"correct").expect("sign");
    let result = PqKeyPair::verify(kp.public_key_bytes(), ctx, b"wrong", &sig);
    assert!(result.is_err(), "PQ should reject wrong message");
}

#[test]
fn pq_public_key_length_is_consistent() {
    let kp = PqKeyPair::generate();
    let expected_len = crate::crypto::primitives::pq_public_key_len();
    assert_eq!(kp.public_key_bytes().len(), expected_len);
}

#[test]
fn pq_validate_public_key_accepts_valid() {
    let kp = PqKeyPair::generate();
    assert!(PqKeyPair::validate_public_key(kp.public_key_bytes()).is_ok());
}

#[test]
fn pq_validate_public_key_rejects_garbage() {
    let garbage = vec![0xFFu8; 32];
    assert!(PqKeyPair::validate_public_key(&garbage).is_err());
}

#[test]
fn bls_keypair_generates() {
    let kp = BlsKeypair::generate().expect("BLS generate");
    let bytes = kp.to_bytes();
    assert!(!bytes.is_empty());
}

#[test]
fn bls_keypair_round_trips_through_bytes() {
    let kp = BlsKeypair::generate().expect("BLS generate");
    let bytes = kp.to_bytes();
    let restored = BlsKeypair::from_bytes(&bytes).expect("BLS from_bytes");
    assert_eq!(kp.to_bytes(), restored.to_bytes());
}

#[test]
fn bls_proof_of_possession_verifies() {
    let kp = BlsKeypair::generate().expect("BLS generate");
    let addr = crate::core::address::Address::from([7u8; 32]);
    let pop = kp.generate_pop(45261, &addr);
    assert!(!pop.is_empty());
    let ok = BlsKeypair::verify_pop(&kp.to_bytes(), 45261, &addr, &pop);
    assert!(ok.is_ok(), "BLS PoP verification failed");
}

#[test]
fn two_pq_keypairs_differ() {
    let kp1 = PqKeyPair::generate();
    let kp2 = PqKeyPair::generate();
    assert_ne!(kp1.public_key_bytes(), kp2.public_key_bytes());
}
