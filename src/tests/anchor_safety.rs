//! Settlement anchor safety tests.

use crate::settlement::anchor_ledger;

#[test]
fn anchor_ledger_module_compiles() {
    let _ = std::mem::size_of::<anchor_ledger::AnchorKeyLedger>();
}
