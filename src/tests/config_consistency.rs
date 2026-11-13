//! Configuration consistency tests: verify that testnet.toml, genesis files,
//! and code constants are aligned.

#[test]
fn testnet_toml_has_correct_chain_id() {
    let content = include_str!("../../config/testnet.toml");
    assert!(content.contains("45261"), "testnet.toml must reference chain ID 45261");
}

#[test]
fn testnet_toml_has_rpc_section() {
    let content = include_str!("../../config/testnet.toml");
    assert!(content.contains("[rpc]"), "testnet.toml must have [rpc] section");
}

#[test]
fn testnet_toml_has_p2p_section() {
    let content = include_str!("../../config/testnet.toml");
    assert!(
        content.contains("[p2p]") || content.contains("[network]"),
        "testnet.toml must have [p2p] or [network] section"
    );
}

#[test]
fn testnet_genesis_json_is_valid() {
    let content = include_str!("../../config/testnet-genesis.json");
    let parsed: serde_json::Value = serde_json::from_str(content).expect("valid JSON");
    assert!(parsed.is_object());
}

#[test]
fn devnet_genesis_json_is_valid() {
    let content = include_str!("../../config/devnet-genesis.json");
    let parsed: serde_json::Value = serde_json::from_str(content).expect("valid JSON");
    assert!(parsed.is_object());
}

#[test]
fn testnet_faucet_toml_is_valid() {
    let content = include_str!("../../config/testnet-faucet.toml");
    let parsed: toml::Value = content.parse().expect("valid TOML");
    assert!(parsed.get("faucet").is_some());
}

#[test]
fn testnet_bootstrap_peers_toml_is_valid() {
    let content = include_str!("../../config/testnet-bootstrap-peers.toml");
    let parsed: toml::Value = content.parse().expect("valid TOML");
    assert!(parsed.get("limits").is_some());
}
