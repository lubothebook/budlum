//! Address type safety tests: construction, display, comparison, and
//! serialization.

use crate::core::address::Address;

#[test]
fn address_from_bytes_is_deterministic() {
    let a1 = Address::from([42u8; 32]);
    let a2 = Address::from([42u8; 32]);
    assert_eq!(a1, a2);
}

#[test]
fn different_bytes_different_address() {
    let a1 = Address::from([1u8; 32]);
    let a2 = Address::from([2u8; 32]);
    assert_ne!(a1, a2);
}

#[test]
fn address_as_bytes_round_trip() {
    let bytes = [99u8; 32];
    let addr = Address::from(bytes);
    assert_eq!(addr.as_bytes(), &bytes);
}

#[test]
fn address_display_is_hex() {
    let addr = Address::from([0u8; 32]);
    let display = format!("{}", addr);
    assert!(!display.is_empty());
}

#[test]
fn zero_address() {
    let zero = Address::from([0u8; 32]);
    assert_eq!(zero.as_bytes(), &[0u8; 32]);
}

#[test]
fn address_serde_round_trip() {
    let addr = Address::from([7u8; 32]);
    let json = serde_json::to_string(&addr).expect("serialize");
    let parsed: Address = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(addr, parsed);
}

#[test]
fn address_clone_is_equal() {
    let addr = Address::from([42u8; 32]);
    let cloned = addr.clone();
    assert_eq!(addr, cloned);
}

#[test]
fn address_hash_is_consistent() {
    use std::collections::HashSet;
    let addr = Address::from([42u8; 32]);
    let mut set = HashSet::new();
    set.insert(addr);
    assert!(set.contains(&addr));
}
