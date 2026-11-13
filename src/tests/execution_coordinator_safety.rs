//! Execution coordinator safety tests: verify UEL types, BudZero size limits,
//! and coordinator construction.

use crate::execution::coordinator::{
    ExecutionCoordinator, StatefulBudZeroEe, UelReceipt, UelTx, MAX_BUDZERO_CODE_BYTES,
};

#[test]
fn max_budzero_code_is_256kb() {
    assert_eq!(MAX_BUDZERO_CODE_BYTES, 256 * 1024);
}

#[test]
fn uel_tx_type_exists() {
    let _ = std::mem::size_of::<UelTx>();
}

#[test]
fn uel_receipt_type_exists() {
    let _ = std::mem::size_of::<UelReceipt>();
}

#[test]
fn stateful_budzero_ee_constructs() {
    let ee = StatefulBudZeroEe::new();
    let _ = ee;
}

#[test]
fn execution_coordinator_constructs() {
    let coord = ExecutionCoordinator::new();
    let _ = coord.supervisor();
}
