//! Account model invariant tests: verify balance arithmetic, fee bounds,
//! unbonding epochs, and unit conversions.

use crate::core::account::{
    Account, BudEnvUnit, ATTOS_PER_ATOM, ATTOS_PER_BUD, GENESIS_BALANCE, MAX_BASE_FEE,
    MAX_BLOCK_REWARD, MIN_TX_FEE, UNBONDING_EPOCHS,
};
use crate::core::address::Address;

#[test]
fn min_tx_fee_is_one() {
    assert_eq!(MIN_TX_FEE, 1);
}

#[test]
fn max_base_fee_is_bounded() {
    assert!(MAX_BASE_FEE <= 10_000_000);
}

#[test]
fn unbonding_epochs_is_seven() {
    assert_eq!(UNBONDING_EPOCHS, 7);
}

#[test]
fn genesis_balance_is_one_billion() {
    assert_eq!(GENESIS_BALANCE, 1_000_000_000);
}

#[test]
fn attos_per_bud_is_1e18() {
    assert_eq!(ATTOS_PER_BUD, 1_000_000_000_000_000_000u128);
}

#[test]
fn attos_per_atom_is_1e12() {
    assert_eq!(ATTOS_PER_ATOM, 1_000_000_000_000u64);
}

#[test]
fn new_account_has_zero_balance() {
    let addr = Address::from([1u8; 32]);
    let acc = Account::new(addr);
    assert_eq!(acc.balance, 0);
    assert_eq!(acc.nonce, 0);
}

#[test]
fn account_with_balance() {
    let addr = Address::from([1u8; 32]);
    let acc = Account::with_balance(addr, 1000);
    assert_eq!(acc.balance, 1000);
}

#[test]
fn bud_env_unit_quantum_attos() {
    let evm = BudEnvUnit::Wei;
    assert_eq!(evm.quantum_attos(), 1);
    let cosmos = BudEnvUnit::Atom;
    assert_eq!(cosmos.quantum_attos(), ATTOS_PER_ATOM as u128);
}

#[test]
fn account_checked_total_attos_consistency() {
    let addr = Address::from([1u8; 32]);
    let mut acc = Account::with_balance(addr, 1000);
    let total = acc.checked_total_attos().unwrap();
    acc.set_from_total_attos(total).unwrap();
    assert_eq!(acc.balance, 1000);
}

#[test]
fn max_block_reward_is_bounded() {
    assert!(MAX_BLOCK_REWARD <= 100_000 * crate::tokenomics::BUD_UNIT);
}
