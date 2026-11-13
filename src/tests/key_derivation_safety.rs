//! Key derivation safety tests.

use crate::crypto::key_derivation::RootSecret;

#[test]
fn root_secret_derives_from_entropy() {
    let entropy = [42u8; 32];
    let root = RootSecret::derive(&entropy, "");
    assert!(root.is_ok());
}

#[test]
fn different_entropy_different_root() {
    let r1 = RootSecret::derive(&[1u8; 32], "").unwrap();
    let r2 = RootSecret::derive(&[2u8; 32], "").unwrap();
    let a1 = r1.user_account(0, 0).unwrap();
    let a2 = r2.user_account(0, 0).unwrap();
    assert_ne!(a1.address, a2.address);
}

#[test]
fn different_index_different_account() {
    let root = RootSecret::derive(&[42u8; 32], "").unwrap();
    let a0 = root.user_account(0, 0).unwrap();
    let a1 = root.user_account(0, 1).unwrap();
    assert_ne!(a0.address, a1.address);
}

#[test]
fn different_account_path_different_address() {
    let root = RootSecret::derive(&[42u8; 32], "").unwrap();
    let a0 = root.user_account(0, 0).unwrap();
    let a1 = root.user_account(1, 0).unwrap();
    assert_ne!(a0.address, a1.address);
}
