//! One deal-open implementation for every path that opens a paid storage deal.
//!
//! The RPC path and the in-block path both escrow a client fee and lock an
//! operator bond. Keeping that in one function on `AccountState` means one
//! copy of the rules and one answer to "what does a refused open leave behind".

use crate::core::account::AccountState;
use crate::core::address::Address;
use crate::domain::storage_deal::{DealStatus, StorageEconomicsParams, StorageError};
use crate::domain::storage_params::StorageDomainParams;
use crate::domain::Hash32;
use crate::storage::generated::held_bytes;

/// What a deal-open asks for, apart from who pays and who is checked.
pub struct DealOpenTerms<'a> {
    pub domain_id: u32,
    pub manifest: &'a crate::storage::ContentManifest,
    pub shard_id: crate::storage::ContentId,
    pub operator: Address,
    pub replica_index: u8,
    pub start_epoch: u64,
    pub end_epoch: u64,
    pub economics: StorageEconomicsParams,
    pub merkle_proof: Option<Vec<u8>>,
    pub storage_root: Option<Hash32>,
}

/// Open a paid storage deal: every refusal is decided before any write.
///
/// The fee is priced on `held_bytes`, the same number `open_deal` records on
/// the deal, so the escrow and the recorded price cannot differ. Balance
/// checks only read. The registry open comes next and writes nothing when it
/// refuses. The debits come last and cannot fail after the checks above.
///
/// `payer_reserve` is a balance the payer must keep after paying. It is
/// required, not spent.
pub fn open_deal_escrowed(
    state: &mut AccountState,
    terms: &DealOpenTerms<'_>,
    payer: Address,
    now_unix_secs: u64,
    min_operator_bond: u64,
    payer_reserve: u64,
) -> Result<u64, String> {
    let operator = terms.operator;
    let economics = &terms.economics;
    let manifest = terms.manifest;
    let shard_id = terms.shard_id;

    // An operator that missed a challenge sits out six hours. Checked here
    // because this layer knows wall time; the registry works in epochs.
    if let Some(until) = state
        .storage_registry
        .operator_cooldown_until(&operator, now_unix_secs)
    {
        return Err(format!(
            "operator {operator} missed a challenge and cannot take                  storage work until unix {until} ({} seconds left)",
            until.saturating_sub(now_unix_secs)
        ));
    }

    let epochs = terms.end_epoch.saturating_sub(terms.start_epoch);
    if epochs == 0 {
        return Err("Deal duration must be > 0".into());
    }

    // The shard is looked up here rather than trusting a caller-supplied
    // size, so the escrow and the deal are computed from the same entry.
    let listed_bytes = u64::from(
        manifest
            .shard(&shard_id)
            .ok_or_else(|| {
                format!(
                    "shard {shard_id:?} is not part of manifest {:?}",
                    manifest.manifest_id
                )
            })?
            .size,
    );

    // Replay guard: a signed deal-open must not debit escrow and lock bond
    // twice for the same placement. An ACTIVE deal that already covers this
    // (manifest, shard, operator, replica, epoch range) refuses the open.
    let duplicate = state
        .storage_registry
        .deals_for_shard(&manifest.manifest_id, &shard_id)
        .iter()
        .any(|d| {
            d.status == DealStatus::Active
                && d.operator == operator
                && d.replica_index == terms.replica_index
                && d.deal_start_epoch == terms.start_epoch
                && d.deal_end_epoch == terms.end_epoch
        });
    if duplicate {
        return Err(format!(
            "an active deal already covers shard {shard_id:?} for operator {operator} at replica {} over epochs {}..{}; refusing a replayed open",
            terms.replica_index, terms.start_epoch, terms.end_epoch
        ));
    }

    // Price what the operator holds, which is what `open_deal` records.
    let held = held_bytes(&manifest.source, listed_bytes).ok_or_else(|| {
        format!(
            "open_deal failed: {:?}",
            StorageError::InvalidManifest {
                reason: String::from(
                    "held_bytes refused the source (hybrid prefix longer than listed size)",
                ),
            }
        )
    })?;
    let total_fee = economics.total_fee(held, epochs);
    let bond = economics.operator_bond;

    // Read-only balance checks. When one account is both payer and operator
    // it must cover the fee, the reserve and the bond together.
    let payer_needs = total_fee.checked_add(payer_reserve);
    if payer_needs.is_none_or(|need| state.get_balance(&payer) < need) {
        return Err(format!(
            "Insufficient payer balance for deal fee {total_fee}"
        ));
    }
    let operator_has = if payer == operator {
        state.get_balance(&operator).checked_sub(total_fee)
    } else {
        Some(state.get_balance(&operator))
    };
    if bond > 0 && operator_has.is_none_or(|have| have < bond) {
        return Err(format!("Insufficient operator balance for bond {bond}"));
    }

    let domain_params = StorageDomainParams {
        min_operator_bond,
        ..StorageDomainParams::default()
    };
    let deal_id = state
        .storage_registry
        .open_deal(
            terms.domain_id,
            manifest,
            shard_id,
            operator,
            terms.replica_index,
            terms.start_epoch,
            terms.end_epoch,
            economics.clone(),
            &domain_params,
            terms.merkle_proof.clone(),
            terms.storage_root,
        )
        .map_err(|e| format!("open_deal failed: {e:?}"))?;

    debit(state, &payer, total_fee)?;
    debit(state, &operator, bond)?;
    Ok(deal_id)
}

/// Subtract `amount` from an account balance. The checks in the caller
/// make the failure unreachable, but it is reported rather than wrapped.
fn debit(state: &mut AccountState, who: &Address, amount: u64) -> Result<(), String> {
    if amount == 0 {
        return Ok(());
    }
    let account = state.get_or_create(who);
    account.balance = account
        .balance
        .checked_sub(amount)
        .ok_or_else(|| format!("balance underflow debiting {amount} from {who}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::storage_deal::FEE_RATE_SCALE;
    use crate::storage::ContentManifest;

    const BOND: u64 = 1_000_000;

    fn proof() -> Vec<u8> {
        let envelope = bud_proof::ProofEnvelope {
            proof_format_version: 1,
            backend: "test-backend".to_string(),
            p3_version: "0.6".to_string(),
            fri_params_id: "test-fri".to_string(),
            public_inputs_hash: [0x42u8; 32],
            proof_bytes: vec![0xABu8; 96],
            degree_bits: 8,
        };
        bincode::serialize(&envelope).unwrap()
    }

    fn manifest() -> ContentManifest {
        ContentManifest::from_bytes_sliced(b"deal open escrow payload", 8).unwrap()
    }

    /// The listed size of the first shard, which for a stored manifest is
    /// also what the deal holds.
    fn shard_bytes(manifest: &ContentManifest) -> u64 {
        u64::from(manifest.shards[0].size)
    }

    fn terms(manifest: &ContentManifest, operator: Address) -> DealOpenTerms<'_> {
        DealOpenTerms {
            domain_id: 42,
            manifest,
            shard_id: manifest.shards[0].shard_id,
            operator,
            replica_index: 0,
            start_epoch: 0,
            end_epoch: 10,
            economics: StorageEconomicsParams {
                operator_bond: BOND,
                // Priced so that one epoch costs 10 base units.
                fee_per_byte_epoch: 10 * (FEE_RATE_SCALE as u64) / shard_bytes(manifest),
            },
            merkle_proof: Some(proof()),
            storage_root: Some([0x42u8; 32]),
        }
    }

    fn fee_of(t: &DealOpenTerms<'_>) -> u64 {
        t.economics
            .total_fee(shard_bytes(t.manifest), t.end_epoch - t.start_epoch)
    }

    fn operator() -> Address {
        Address::from([11u8; 32])
    }

    fn payer() -> Address {
        Address::from([12u8; 32])
    }

    fn funded(payer_balance: u64, operator_balance: u64) -> AccountState {
        let mut state = AccountState::new();
        state.add_balance(&payer(), payer_balance);
        state.add_balance(&operator(), operator_balance);
        state
    }

    /// Everything a refused open must leave alone.
    fn snapshot(state: &AccountState) -> (u64, u64, [u8; 32]) {
        (
            state.get_balance(&payer()),
            state.get_balance(&operator()),
            state.storage_registry.root(),
        )
    }

    fn assert_refused_untouched(
        mut state: AccountState,
        t: &DealOpenTerms<'_>,
        payer_address: Address,
        now: u64,
        reserve: u64,
        expected: &str,
    ) {
        let before = snapshot(&state);
        let err = open_deal_escrowed(&mut state, t, payer_address, now, BOND, reserve)
            .expect_err("the open must be refused");
        assert!(err.contains(expected), "got {err:?}, wanted {expected:?}");
        assert_eq!(snapshot(&state), before, "a refusal must not write");
    }

    #[test]
    fn a_cooled_down_operator_is_refused_without_a_write() {
        let manifest = manifest();
        let t = terms(&manifest, operator());
        let mut state = funded(5_000_000, 5_000_000);
        state
            .storage_registry
            .begin_operator_cooldown(operator(), 100);
        assert_refused_untouched(state, &t, payer(), 200, 0, "missed a challenge");
    }

    #[test]
    fn a_zero_length_deal_is_refused_without_a_write() {
        let manifest = manifest();
        let mut t = terms(&manifest, operator());
        t.end_epoch = t.start_epoch;
        assert_refused_untouched(
            funded(5_000_000, 5_000_000),
            &t,
            payer(),
            0,
            0,
            "Deal duration must be > 0",
        );
    }

    #[test]
    fn an_unknown_shard_is_refused_without_a_write() {
        let manifest = manifest();
        let mut t = terms(&manifest, operator());
        t.shard_id = crate::storage::ContentId([0xEEu8; 32]);
        assert_refused_untouched(
            funded(5_000_000, 5_000_000),
            &t,
            payer(),
            0,
            0,
            "is not part of manifest",
        );
    }

    #[test]
    fn a_replayed_open_is_refused_without_a_second_write() {
        let manifest = manifest();
        let t = terms(&manifest, operator());
        let mut state = funded(5_000_000, 5_000_000);
        open_deal_escrowed(&mut state, &t, payer(), 0, BOND, 0).unwrap();
        assert_refused_untouched(state, &t, payer(), 0, 0, "refusing a replayed open");
    }

    #[test]
    fn a_short_payer_is_refused_without_a_write() {
        let manifest = manifest();
        let t = terms(&manifest, operator());
        let fee = fee_of(&t);
        assert_refused_untouched(
            funded(fee - 1, 5_000_000),
            &t,
            payer(),
            0,
            0,
            "Insufficient payer balance",
        );
    }

    #[test]
    fn a_short_operator_is_refused_without_a_write() {
        let manifest = manifest();
        let t = terms(&manifest, operator());
        assert_refused_untouched(
            funded(5_000_000, BOND - 1),
            &t,
            payer(),
            0,
            0,
            "Insufficient operator balance",
        );
    }

    #[test]
    fn a_registry_refusal_is_reported_and_writes_nothing() {
        let manifest = manifest();
        let mut t = terms(&manifest, operator());
        t.merkle_proof = None;
        assert_refused_untouched(
            funded(5_000_000, 5_000_000),
            &t,
            payer(),
            0,
            0,
            "open_deal failed",
        );
    }

    #[test]
    fn a_bond_below_the_minimum_is_refused_without_a_write() {
        let manifest = manifest();
        let mut t = terms(&manifest, operator());
        t.economics.operator_bond = BOND - 1;
        assert_refused_untouched(
            funded(5_000_000, 5_000_000),
            &t,
            payer(),
            0,
            0,
            "open_deal failed",
        );
    }

    #[test]
    fn payer_as_operator_must_cover_fee_and_bond_together() {
        let manifest = manifest();
        let t = terms(&manifest, operator());
        let fee = fee_of(&t);
        assert!(fee > 0);

        let mut short = AccountState::new();
        short.add_balance(&operator(), fee + BOND - 1);
        let before = (
            short.get_balance(&operator()),
            short.storage_registry.root(),
        );
        let err = open_deal_escrowed(&mut short, &t, operator(), 0, BOND, 0)
            .expect_err("one unit short must be refused");
        assert!(err.contains("Insufficient operator balance"), "got {err:?}");
        assert_eq!(
            (
                short.get_balance(&operator()),
                short.storage_registry.root()
            ),
            before
        );

        let mut exact = AccountState::new();
        exact.add_balance(&operator(), fee + BOND);
        open_deal_escrowed(&mut exact, &t, operator(), 0, BOND, 0).unwrap();
        assert_eq!(exact.get_balance(&operator()), 0);
    }

    #[test]
    fn the_payer_reserve_is_required_but_not_spent() {
        let manifest = manifest();
        let t = terms(&manifest, operator());
        let fee = fee_of(&t);
        let reserve = 7;

        assert_refused_untouched(
            funded(fee + reserve - 1, 5_000_000),
            &t,
            payer(),
            0,
            reserve,
            "Insufficient payer balance",
        );

        let mut state = funded(fee + reserve, 5_000_000);
        open_deal_escrowed(&mut state, &t, payer(), 0, BOND, reserve).unwrap();
        assert_eq!(state.get_balance(&payer()), reserve);
        assert_eq!(state.get_balance(&operator()), 5_000_000 - BOND);
    }

    #[test]
    fn the_escrow_equals_the_fee_the_deal_records() {
        let manifest = manifest();
        let t = terms(&manifest, operator());
        let mut state = funded(5_000_000, 5_000_000);
        let deal_id = open_deal_escrowed(&mut state, &t, payer(), 0, BOND, 0).unwrap();
        let deal = state.storage_registry.get_deal(deal_id).unwrap();
        assert_eq!(deal.status, DealStatus::Active);
        let epochs = t.end_epoch - t.start_epoch;
        assert!(deal.total_fee(epochs) > 0);
        assert_eq!(
            state.get_balance(&payer()),
            5_000_000 - deal.total_fee(epochs)
        );
        assert_eq!(state.get_balance(&operator()), 5_000_000 - BOND);
    }
}
