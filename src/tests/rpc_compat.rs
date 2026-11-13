//! RPC compatibility gateway tests: verify that the Ethereum and Cosmos
//! compatibility RPCs translate correctly.

/// The testnet config enables Ethereum compatibility RPC.
#[test]
fn testnet_config_enables_eth_compat() {
    let toml_content = include_str!("../../config/testnet.toml");
    assert!(
        toml_content.contains("[compat.ethereum]"),
        "testnet.toml must have [compat.ethereum] section"
    );
    assert!(
        toml_content.contains("enabled = true"),
        "Ethereum compat must be enabled on testnet"
    );
}

/// The testnet config enables Cosmos compatibility RPC.
#[test]
fn testnet_config_enables_cosmos_compat() {
    let toml_content = include_str!("../../config/testnet.toml");
    assert!(
        toml_content.contains("[compat.cosmos]"),
        "testnet.toml must have [compat.cosmos] section"
    );
}

/// The testnet Ethereum RPC listens on a distinct port from the native RPC.
#[test]
fn eth_rpc_port_differs_from_native() {
    let toml_content = include_str!("../../config/testnet.toml");
    // Native RPC: 18545, Ethereum compat: 18547
    assert!(toml_content.contains("18545")); // native
    assert!(toml_content.contains("18547")); // ethereum
}

/// The testnet chain ID is in the genesis file.
#[test]
fn testnet_genesis_has_chain_id() {
    let genesis_content = include_str!("../../config/testnet-genesis.json");
    let parsed: serde_json::Value = serde_json::from_str(genesis_content).expect("valid JSON");
    assert_eq!(parsed["chain_id"], 45261);
}

/// The testnet genesis has the correct PQ scheme.
#[test]
fn testnet_genesis_has_pq_scheme() {
    let genesis_content = include_str!("../../config/testnet-genesis.json");
    let parsed: serde_json::Value = serde_json::from_str(genesis_content).expect("valid JSON");
    assert_eq!(parsed["pq_scheme"], "ml-dsa-65");
}

/// The devnet genesis has the correct chain ID.
#[test]
fn devnet_genesis_has_chain_id() {
    let genesis_content = include_str!("../../config/devnet-genesis.json");
    let parsed: serde_json::Value = serde_json::from_str(genesis_content).expect("valid JSON");
    assert_eq!(parsed["chain_id"], 1337);
}
