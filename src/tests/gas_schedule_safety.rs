//! Gas schedule safety tests: verify that gas costs are ordered correctly
//! and bounded for all networks.

use crate::core::chain_config::Network;

#[test]
fn transfer_gas_is_21000_on_all_networks() {
    for network in [Network::Mainnet, Network::Testnet, Network::Devnet] {
        let gs = network.gas_schedule();
        assert_eq!(
            gs.transfer_gas, 21_000,
            "transfer gas on {network} must be 21000"
        );
    }
}

#[test]
fn stake_gas_exceeds_transfer_gas() {
    for network in [Network::Mainnet, Network::Testnet, Network::Devnet] {
        let gs = network.gas_schedule();
        assert!(
            gs.stake_gas > gs.transfer_gas,
            "stake_gas ({}) must exceed transfer_gas ({}) on {network}",
            gs.stake_gas,
            gs.transfer_gas
        );
    }
}

#[test]
fn vote_gas_exceeds_transfer_gas() {
    for network in [Network::Mainnet, Network::Testnet, Network::Devnet] {
        let gs = network.gas_schedule();
        assert!(
            gs.vote_gas > gs.transfer_gas,
            "vote_gas ({}) must exceed transfer_gas ({}) on {network}",
            gs.vote_gas,
            gs.transfer_gas
        );
    }
}

#[test]
fn contract_call_gas_exceeds_transfer_gas() {
    for network in [Network::Mainnet, Network::Testnet, Network::Devnet] {
        let gs = network.gas_schedule();
        assert!(
            gs.contract_call_gas > gs.transfer_gas,
            "contract_call_gas must exceed transfer_gas on {network}"
        );
    }
}

#[test]
fn base_fee_is_positive() {
    for network in [Network::Mainnet, Network::Testnet, Network::Devnet] {
        let gs = network.gas_schedule();
        assert!(gs.base_fee > 0, "base_fee must be positive on {network}");
    }
}

#[test]
fn gas_per_byte_is_positive() {
    for network in [Network::Mainnet, Network::Testnet, Network::Devnet] {
        let gs = network.gas_schedule();
        assert!(
            gs.gas_per_byte > 0,
            "gas_per_byte must be positive on {network}"
        );
    }
}

#[test]
fn gas_per_signature_is_positive() {
    for network in [Network::Mainnet, Network::Testnet, Network::Devnet] {
        let gs = network.gas_schedule();
        assert!(
            gs.gas_per_signature > 0,
            "gas_per_signature must be positive on {network}"
        );
    }
}
