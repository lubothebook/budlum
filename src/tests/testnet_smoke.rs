//! Testnet smoke tests: verify that the testnet genesis configuration is valid,
//! that testnet-specific features are correctly gated, and that the node can
//! initialize with testnet parameters.

use crate::chain::genesis::GenesisConfig;
use crate::core::chain_config::Network;

/// The testnet chain ID is 45261.
#[test]
fn testnet_chain_id_is_45261() {
    assert_eq!(Network::Testnet.chain_id().value(), 45261);
}

/// The devnet chain ID is 1337.
#[test]
fn devnet_chain_id_is_1337() {
    assert_eq!(Network::Devnet.chain_id().value(), 1337);
}

/// Testnet genesis can be constructed from the network defaults.
#[test]
fn testnet_genesis_config_builds() {
    let config = GenesisConfig::for_network(Network::Testnet);
    assert_eq!(config.chain_id, 45261);
    assert_eq!(
        config.pq_scheme.as_deref(),
        Some("ml-dsa-65"),
        "testnet pq_scheme must be ml-dsa-65"
    );
}

/// Devnet genesis can be constructed from the network defaults.
#[test]
fn devnet_genesis_config_builds() {
    let config = GenesisConfig::for_network(Network::Devnet);
    assert_eq!(config.chain_id, 1337);
}

/// Genesis round-trips through JSON serialization.
#[test]
fn genesis_json_roundtrip() {
    let config = GenesisConfig::for_network(Network::Testnet);
    let json = serde_json::to_string_pretty(&config).expect("serialize");
    let parsed: GenesisConfig = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(parsed.chain_id, config.chain_id);
    assert_eq!(parsed.base_fee, config.base_fee);
}

/// The gas schedule has sane defaults for testnet.
#[test]
fn testnet_gas_schedule_is_sane() {
    let config = GenesisConfig::for_network(Network::Testnet);
    assert!(config.gas_schedule.base_fee > 0);
    assert!(config.gas_schedule.transfer_gas > 0);
    assert!(config.gas_schedule.stake_gas > config.gas_schedule.transfer_gas);
}

/// Testnet tokenomics total supply is 100_000_000 BUD (in smallest unit).
#[test]
fn testnet_total_supply_is_correct() {
    let config = GenesisConfig::for_network(Network::Testnet);
    let t = config
        .bud_tokenomics
        .as_ref()
        .expect("testnet must have tokenomics");
    let total = t.community + t.liquidity + t.ecosystem + t.team + t.burn_reserve;
    assert_eq!(total, 100_000_000_000_000u64, "total supply must be 100M BUD");
}

/// Slot duration on testnet is 3 seconds.
#[test]
fn testnet_slot_duration() {
    assert_eq!(Network::Testnet.slot_ms(), 3000);
}

/// Epoch length on testnet is 50 slots.
#[test]
fn testnet_epoch_length() {
    let params = Network::Testnet.consensus_params();
    assert_eq!(params.epoch_len, 50);
}

/// Default port for testnet is 5001.
#[test]
fn testnet_default_port() {
    assert_eq!(Network::Testnet.default_port(), 5001);
}

/// Network::from_chain_id round-trips for all known networks.
#[test]
fn chain_id_round_trip() {
    for network in [Network::Mainnet, Network::Testnet, Network::Devnet] {
        let chain_id = network.chain_id().value();
        let recovered = Network::from_chain_id(chain_id);
        assert_eq!(recovered, Some(network), "round-trip failed for {network}");
    }
}

/// An unknown chain ID produces None from from_chain_id.
#[test]
fn unknown_chain_id_returns_none() {
    assert_eq!(Network::from_chain_id(99999), None);
}
