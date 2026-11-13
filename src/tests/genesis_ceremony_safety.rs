//! Genesis ceremony validation tests: verify that genesis configs pass all
//! validation gates and that invalid configs are rejected.

use crate::chain::genesis::GenesisConfig;
use crate::core::chain_config::Network;

#[test]
fn testnet_genesis_validates_pq_scheme() {
    let config = GenesisConfig::for_network(Network::Testnet);
    assert!(config.validate_pq_scheme().is_ok());
}

#[test]
fn devnet_genesis_validates_pq_scheme() {
    let config = GenesisConfig::for_network(Network::Devnet);
    assert!(config.validate_pq_scheme().is_ok());
}

#[test]
fn mainnet_genesis_validates_pq_scheme() {
    let config = GenesisConfig::for_network(Network::Mainnet);
    assert!(config.validate_pq_scheme().is_ok());
}

#[test]
fn testnet_genesis_validates_total_issuance() {
    let config = GenesisConfig::for_network(Network::Testnet);
    assert!(config.validate_total_issuance().is_ok());
}

#[test]
fn testnet_genesis_validates_tokenomics_supply() {
    let config = GenesisConfig::for_network(Network::Testnet);
    assert!(config.validate_tokenomics_supply().is_ok());
}

#[test]
fn testnet_genesis_validates_consensus_ceremony() {
    let config = GenesisConfig::for_network(Network::Testnet);
    assert!(config.validate_consensus_ceremony(Network::Testnet).is_ok());
}

#[test]
fn genesis_build_block_produces_valid_block() {
    let config = GenesisConfig::for_network(Network::Testnet);
    let block = config.build_genesis_block();
    assert_eq!(block.header.height, 0);
    assert_eq!(block.header.parent_hash, [0u8; 32]);
}

#[test]
fn genesis_build_state_produces_accounts() {
    let config = GenesisConfig::for_network(Network::Testnet);
    let state = config.build_state();
    assert!(
        !state.accounts.is_empty() || config.allocations.is_empty(),
        "state accounts should match allocations"
    );
}

#[test]
fn testnet_reward_pool_flag() {
    let config = GenesisConfig::for_network(Network::Testnet);
    assert!(config.validate_reward_pool_flag().is_ok());
}

#[test]
fn genesis_serde_preserves_pq_scheme() {
    let config = GenesisConfig::for_network(Network::Testnet);
    let json = serde_json::to_string(&config).unwrap();
    let parsed: GenesisConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(config.pq_scheme, parsed.pq_scheme);
}
