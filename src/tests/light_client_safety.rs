//! Light client safety tests: trusted checkpoint construction, chain
//! verification, trust modes, and PQ scheme enforcement.

use crate::light_client::{LightClient, TrustMode, TrustedCheckpoint};

fn testnet_checkpoint() -> TrustedCheckpoint {
    TrustedCheckpoint {
        height: 0,
        block_hash: "0".repeat(64),
        set_hash: "0".repeat(64),
    }
}

#[test]
fn light_client_constructs_from_genesis() {
    let lc = LightClient::new(testnet_checkpoint());
    assert!(lc.is_ok());
}

#[test]
fn light_client_for_testnet_chain() {
    let lc = LightClient::for_chain(testnet_checkpoint(), 45261);
    assert!(lc.is_ok());
    let lc = lc.unwrap();
    assert_eq!(lc.trust_mode(), TrustMode::Classical);
}

#[test]
fn light_client_for_unknown_chain_still_works() {
    let lc = LightClient::for_chain(testnet_checkpoint(), 99999);
    assert!(lc.is_ok());
}

#[test]
fn trust_mode_describe_returns_nonempty() {
    for mode in [TrustMode::Classical, TrustMode::Hybrid, TrustMode::PostQuantum] {
        assert!(!mode.describe().is_empty());
    }
}

#[test]
fn trusted_checkpoint_fields() {
    let cp = testnet_checkpoint();
    assert_eq!(cp.height, 0);
    assert_eq!(cp.block_hash.len(), 64);
}

#[test]
fn light_client_trusted_accessor() {
    let lc = LightClient::new(testnet_checkpoint()).unwrap();
    let trusted = lc.trusted();
    assert_eq!(trusted.height, 0);
}

#[test]
fn light_client_next_checkpoint_at_genesis() {
    let lc = LightClient::new(testnet_checkpoint()).unwrap();
    let next = lc.next_checkpoint_height(10, 100);
    assert!(next.is_some());
}
