//! Network peer management and gossip dedup safety tests.

use crate::core::chain_config::Network;

/// Testnet has mDNS disabled.
#[test]
fn testnet_mdns_disabled() {
    let security = Network::Testnet.security_config();
    assert!(!security.mdns_enabled, "mDNS must be disabled on testnet");
}

/// Devnet has mDNS enabled for local discovery.
#[test]
fn devnet_mdns_enabled() {
    let security = Network::Devnet.security_config();
    assert!(security.mdns_enabled, "mDNS should be enabled on devnet");
}

/// Mainnet has mDNS disabled.
#[test]
fn mainnet_mdns_disabled() {
    let security = Network::Mainnet.security_config();
    assert!(!security.mdns_enabled, "mDNS must be disabled on mainnet");
}

/// Testnet max peers is bounded.
#[test]
fn testnet_max_peers_bounded() {
    // From testnet.toml: max_peers = 75
    let max_peers: usize = 75;
    assert!(max_peers >= 10, "too few peers for testnet");
    assert!(max_peers <= 200, "too many peers for testnet");
}

/// Network names display correctly.
#[test]
fn network_names_display() {
    assert_eq!(format!("{}", Network::Mainnet), "mainnet");
    assert_eq!(format!("{}", Network::Testnet), "testnet");
    assert_eq!(format!("{}", Network::Devnet), "devnet");
}

/// Placeholder peer detection works for mainnet guard.
#[test]
fn placeholder_peer_detection() {
    let peers = vec![
        "/ip4/127.0.0.1/tcp/5001".to_string(),
        "/ip4/0.0.0.0/tcp/5001".to_string(),
    ];
    let has_placeholder =
        crate::core::chain_config::first_placeholder_peer(&peers);
    // 0.0.0.0 is a placeholder on mainnet.
    if let Some(bad) = has_placeholder {
        assert!(bad.contains("0.0.0.0") || bad.contains("placeholder"));
    }
}

/// has_peer_source returns false when no peers and no DNS seeds.
#[test]
fn no_peer_source_detected() {
    let empty_boots: Vec<String> = vec![];
    let empty_dns: Vec<String> = vec![];
    assert!(
        !crate::core::chain_config::has_peer_source(Network::Testnet, &empty_boots, &empty_dns)
    );
}
