//! Validator stake bounds tests: verify minimum stake, mobile stake, and
//! class-specific constraints.

use crate::core::chain_config::Network;
use crate::validators::ValidatorClass;

#[test]
fn testnet_min_stake_is_positive() {
    let min = Network::Testnet.min_stake_full();
    assert!(min > 0, "testnet min stake must be positive");
}

#[test]
fn mainnet_min_stake_exceeds_testnet() {
    let mainnet = Network::Mainnet.min_stake_full();
    let testnet = Network::Testnet.min_stake_full();
    assert!(
        mainnet >= testnet,
        "mainnet min stake {} must be >= testnet {}",
        mainnet,
        testnet
    );
}

#[test]
fn mobile_min_stake_less_than_full() {
    let full = Network::Testnet.min_stake_full();
    let mobile = Network::Testnet.min_stake_mobile();
    assert!(
        mobile <= full,
        "mobile stake {} must be <= full stake {}",
        mobile,
        full
    );
}

#[test]
fn epoch_length_is_positive() {
    for network in [Network::Mainnet, Network::Testnet, Network::Devnet] {
        assert!(
            network.epoch_len() > 0,
            "epoch length must be positive on {network}"
        );
    }
}

#[test]
fn validator_class_variants_exist() {
    let _full = ValidatorClass::Full;
    let _mobile = ValidatorClass::Mobile;
    let _opp = ValidatorClass::MobileOpportunistic;
}

#[test]
fn validator_class_debug() {
    let full = format!("{:?}", ValidatorClass::Full);
    let mobile = format!("{:?}", ValidatorClass::Mobile);
    let opp = format!("{:?}", ValidatorClass::MobileOpportunistic);
    assert!(full.contains("Full"));
    assert!(mobile.contains("Mobile"));
    assert!(opp.contains("Opportunistic"));
}

#[test]
fn slot_ms_is_positive_for_all_networks() {
    for network in [Network::Mainnet, Network::Testnet, Network::Devnet] {
        assert!(
            network.slot_ms() > 0,
            "slot_ms must be positive on {network}"
        );
    }
}

#[test]
fn testnet_slot_ms_is_3000() {
    assert_eq!(Network::Testnet.slot_ms(), 3000);
}

#[test]
fn mainnet_slot_ms_is_6000() {
    assert_eq!(Network::Mainnet.slot_ms(), 6000);
}
