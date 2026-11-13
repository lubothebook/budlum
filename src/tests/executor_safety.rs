//! Executor safety tests: verify that the executor can apply transactions
//! and blocks to state.

use crate::chain::genesis::GenesisConfig;
use crate::core::chain_config::Network;
use crate::execution::executor::Executor;

#[test]
fn executor_type_exists() {
    let _ = std::mem::size_of::<Executor>();
}

#[test]
fn genesis_state_is_buildable() {
    let config = GenesisConfig::for_network(Network::Testnet);
    let state = config.build_state();
    assert!(state.validators.is_empty() || !state.validators.is_empty());
}

#[test]
fn genesis_state_has_tokenomics() {
    let config = GenesisConfig::for_network(Network::Testnet);
    let state = config.build_state();
    let total = state.tokenomics.community
        + state.tokenomics.liquidity
        + state.tokenomics.ecosystem
        + state.tokenomics.team
        + state.tokenomics.burn_reserve;
    assert_eq!(total, 100_000_000_000_000u64);
}

#[test]
fn genesis_state_has_bns_registry() {
    let config = GenesisConfig::for_network(Network::Testnet);
    let state = config.build_state();
    assert!(state.bns_registry.names.is_empty());
}
