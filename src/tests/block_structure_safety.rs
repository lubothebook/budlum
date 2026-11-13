//! Block structure safety tests: verify block fields, hash determinism,
//! and genesis block properties.

use crate::chain::genesis::GenesisConfig;
use crate::core::chain_config::Network;

#[test]
fn genesis_block_has_index_zero() {
    let config = GenesisConfig::for_network(Network::Testnet);
    let block = config.build_genesis_block();
    assert_eq!(block.index, 0);
}

#[test]
fn genesis_block_has_empty_previous_hash() {
    let config = GenesisConfig::for_network(Network::Testnet);
    let block = config.build_genesis_block();
    assert!(
        block.previous_hash.is_empty()
            || block.previous_hash.chars().all(|c| c == '0'),
        "genesis previous_hash must be empty or all zeros"
    );
}

#[test]
fn genesis_block_hash_is_deterministic() {
    let config = GenesisConfig::for_network(Network::Testnet);
    let b1 = config.build_genesis_block();
    let b2 = config.build_genesis_block();
    assert_eq!(b1.hash, b2.hash);
}

#[test]
fn genesis_block_hash_is_not_empty() {
    let config = GenesisConfig::for_network(Network::Testnet);
    let block = config.build_genesis_block();
    assert!(!block.hash.is_empty());
}

#[test]
fn genesis_block_has_correct_chain_id() {
    let config = GenesisConfig::for_network(Network::Testnet);
    let block = config.build_genesis_block();
    assert_eq!(block.chain_id, 45261);
}

#[test]
fn genesis_block_has_no_transactions() {
    let config = GenesisConfig::for_network(Network::Testnet);
    let block = config.build_genesis_block();
    assert!(block.transactions.is_empty());
}

#[test]
fn devnet_genesis_block_has_correct_chain_id() {
    let config = GenesisConfig::for_network(Network::Devnet);
    let block = config.build_genesis_block();
    assert_eq!(block.chain_id, 1337);
}

#[test]
fn genesis_block_epoch_is_zero() {
    let config = GenesisConfig::for_network(Network::Testnet);
    let block = config.build_genesis_block();
    assert_eq!(block.epoch, 0);
}

#[test]
fn genesis_block_slot_is_zero() {
    let config = GenesisConfig::for_network(Network::Testnet);
    let block = config.build_genesis_block();
    assert_eq!(block.slot, 0);
}
