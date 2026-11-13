//! Tokenomics invariant tests: verify that the economic model constants are
//! consistent and that supply conservation holds.

use crate::chain::genesis::GenesisConfig;
use crate::core::chain_config::Network;

fn tokenomics(network: Network) -> crate::tokenomics::TokenomicsParams {
    GenesisConfig::for_network(network)
        .bud_tokenomics
        .expect("network must have tokenomics")
}

/// G-3 invariant: total supply is exactly 100_000_000 BUD.
#[test]
fn total_supply_is_100m_bud() {
    for network in [Network::Mainnet, Network::Testnet, Network::Devnet] {
        let t = tokenomics(network);
        let total = t.community + t.liquidity + t.ecosystem + t.team + t.burn_reserve;
        assert_eq!(
            total, 100_000_000_000_000u64,
            "G-3 violation: total supply for {network} is {total}, expected 100_000_000_000_000"
        );
    }
}

/// The burn reserve is 40% of total supply.
#[test]
fn burn_reserve_is_forty_percent() {
    let t = tokenomics(Network::Testnet);
    let total = 100_000_000_000_000u64;
    assert_eq!(t.burn_reserve, total * 40 / 100);
}

/// Team allocation is 20% of total supply.
#[test]
fn team_allocation_is_twenty_percent() {
    let t = tokenomics(Network::Testnet);
    let total = 100_000_000_000_000u64;
    assert_eq!(t.team, total * 20 / 100);
}

/// Ecosystem allocation is 20% of total supply.
#[test]
fn ecosystem_allocation_is_twenty_percent() {
    let t = tokenomics(Network::Testnet);
    let total = 100_000_000_000_000u64;
    assert_eq!(t.ecosystem, total * 20 / 100);
}

/// Community and liquidity are each 10%.
#[test]
fn community_and_liquidity_are_ten_percent_each() {
    let t = tokenomics(Network::Testnet);
    let total = 100_000_000_000_000u64;
    assert_eq!(t.community, total * 10 / 100);
    assert_eq!(t.liquidity, total * 10 / 100);
}

/// Epoch length and slot duration produce a consistent year length.
#[test]
fn epochs_per_year_is_consistent() {
    let t = tokenomics(Network::Testnet);
    let seconds_per_epoch = t.slot_duration_secs * t.epoch_length_slots;
    let seconds_per_year = 365u64 * 24 * 3600;
    let computed_epochs_per_year = seconds_per_year / seconds_per_epoch;
    let diff = if t.epochs_per_year > computed_epochs_per_year {
        t.epochs_per_year - computed_epochs_per_year
    } else {
        computed_epochs_per_year - t.epochs_per_year
    };
    assert!(
        diff <= computed_epochs_per_year / 20,
        "epochs_per_year {} is too far from computed {}",
        t.epochs_per_year,
        computed_epochs_per_year
    );
}

/// Team cliff is at least one year.
#[test]
fn team_cliff_is_at_least_one_year() {
    let t = tokenomics(Network::Testnet);
    assert!(
        t.team_cliff_epochs >= t.epochs_per_year,
        "team cliff {} is less than one year {}",
        t.team_cliff_epochs,
        t.epochs_per_year
    );
}

/// Team vesting is at least four years.
#[test]
fn team_vesting_is_at_least_four_years() {
    let t = tokenomics(Network::Testnet);
    assert!(
        t.team_vesting_epochs >= t.epochs_per_year * 4,
        "team vesting {} is less than four years {}",
        t.team_vesting_epochs,
        t.epochs_per_year * 4
    );
}

/// Transaction fee burn ratio is between 0% and 100%.
#[test]
fn tx_fee_burn_ratio_is_bounded() {
    let t = tokenomics(Network::Testnet);
    assert!(
        t.tx_fee_burn_ratio_fixed <= 1_000_000,
        "burn ratio {} exceeds 100%",
        t.tx_fee_burn_ratio_fixed
    );
}

/// Validator annual yield ratio is between 0% and 20%.
#[test]
fn validator_yield_is_bounded() {
    let t = tokenomics(Network::Testnet);
    assert!(
        t.validator_annual_yield_ratio_fixed <= 200_000,
        "validator yield {} exceeds 20%",
        t.validator_annual_yield_ratio_fixed
    );
}
