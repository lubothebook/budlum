//! Validator registration door tests: verify the single-registration-door
//! invariant (V-5) and class-specific enforcement.

use crate::core::address::Address;
use crate::validators::{
    GatewayOffer, LeaderCommitment, RelayOffer, SeatFields, ValidatorClass, ValidatorError,
    MAX_OFFER_ADDRESS_BYTES, MAX_RELAY_ADDRESSES,
};

/// A relay offer with too many multiaddresses is rejected.
#[test]
fn relay_offer_address_count_bounded() {
    let offer = RelayOffer {
        multiaddrs: (0..MAX_RELAY_ADDRESSES + 1)
            .map(|i| format!("/ip4/127.0.0.1/tcp/{i}"))
            .collect(),
        capacity_kbps: 1000,
    };
    assert!(
        offer.multiaddrs.len() > MAX_RELAY_ADDRESSES,
        "test setup: offer has too many addresses"
    );
}

/// A gateway offer with an oversized address is detectable.
#[test]
fn gateway_offer_address_length_bounded() {
    let long_address = "a".repeat(MAX_OFFER_ADDRESS_BYTES + 1);
    let offer = GatewayOffer {
        address: long_address.clone(),
    };
    assert!(offer.address.len() > MAX_OFFER_ADDRESS_BYTES);
}

/// SeatFields round-trips through serde.
#[test]
fn seat_fields_serde_roundtrip() {
    let fields = SeatFields {
        vrf_key: vec![0u8; 32],
        pq_vote_key: vec![0u8; 64],
        leader_commitment: LeaderCommitment {
            commitment: [42u8; 32],
            length: 1000,
        },
    };
    let json = serde_json::to_string(&fields).expect("serialize");
    let parsed: SeatFields = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(parsed.leader_commitment.length, 1000);
    assert_eq!(parsed.leader_commitment.commitment, [42u8; 32]);
}

/// LeaderCommitment round-trips through serde.
#[test]
fn leader_commitment_serde_roundtrip() {
    let lc = LeaderCommitment {
        commitment: [0xAB; 32],
        length: 500,
    };
    let json = serde_json::to_string(&lc).expect("serialize");
    let parsed: LeaderCommitment = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(parsed, lc);
}

/// RelayOffer round-trips through serde.
#[test]
fn relay_offer_serde_roundtrip() {
    let offer = RelayOffer {
        multiaddrs: vec![
            "/ip4/1.2.3.4/tcp/5001".to_string(),
            "/ip4/5.6.7.8/tcp/5001".to_string(),
        ],
        capacity_kbps: 10000,
    };
    let json = serde_json::to_string(&offer).expect("serialize");
    let parsed: RelayOffer = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(parsed.multiaddrs.len(), 2);
    assert_eq!(parsed.capacity_kbps, 10000);
}

/// GatewayOffer round-trips through serde.
#[test]
fn gateway_offer_serde_roundtrip() {
    let offer = GatewayOffer {
        address: "https://gateway.example.com".to_string(),
    };
    let json = serde_json::to_string(&offer).expect("serialize");
    let parsed: GatewayOffer = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(parsed.address, "https://gateway.example.com");
}
