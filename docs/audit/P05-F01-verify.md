# P05-F01 verification (Part 7, F01)

Role: finder. Method: reading only. No cargo or CI tool was run. Commit base: 10831a0.

## Verdict

CONFIRMED (accounting defect, certain by reading). Reachability correction: the cycle is latent in the current production build, because no production path creates a bridge lock. Severity stays High for the logic. It becomes live when a lock entry point is wired.

## Claim under test

A lock, mint, burn, unlock cycle grows total balances by one transfer amount, because burn debits nothing and unlock credits the owner.

## Evidence, leg by leg (A = transfer amount, f = relayer fee from split_bridge_fee)

1. Lock, debit -A. src/chain/blockchain.rs:2236-2241 checks the owner balance. 2242-2255 calls bridge_state.lock. 2257-2258 does `owner_account.balance.saturating_sub(amount)`. Nothing is credited to an escrow account. The amount leaves the balances.
2. Lock opens mint headroom of A. src/core/account.rs:2063-2068: total_bud_committed counts liquid balances, stake, unbonding and role bonds. It does not count bridge-locked amounts. So the lock lowers committed supply by A.
3. Mint, credit +A of new supply. All three mint paths use try_mint_balance (cap checked, account.rs:1949-1952, 1966-1976):
   - src/execution/executor.rs:1171 (headroom), 1191 (recipient, A - f), 1200 (tx.from, f).
   - src/chain/blockchain.rs:2139 (headroom), 2154 (recipient, A - f), 2157 (relayer, f).
   - src/chain/blockchain.rs:3311 (headroom), 3323 (recipient), 3326 (relayer). This is submit_relay_proof (fn at 3229).
   No mint path debits an escrow or reverses the lock debit. After lock and mint, committed supply is back to the start value. The owner lost A, the recipient and relayer hold A.
4. Burn, 0. src/cross_domain/bridge.rs:529-585 only reads the transfer, builds the message and sets status Burned at 581-583. BridgeState has no AccountState access. src/chain/blockchain.rs:2322-2350 calls burn_with_event, persists, registers the message. No balance operation. The recipient keeps A - f, the relayer keeps f.
5. Unlock. Three paths, with different balance effects:
   - src/execution/executor.rs:1205-1276 (RelayerResult tx, MessageKind::BridgeBurn): 1240 bridge_state.unlock, 1265 try_add_balance(owner, A - f), 1272 try_add_balance(tx.from, f). Credit +A.
   - src/chain/blockchain.rs:3329-3385 (submit_relay_proof, BridgeBurn branch): 3356 unlock, 3381 try_add_balance(owner, A - f), 3384 try_add_balance(relayer, f). Credit +A. RPC is operator gated (src/rpc/server.rs:2858).
   - src/chain/blockchain.rs:2362-2450 (unlock_bridge_transfer_from_verified_event): 2443 unlock, then persist only. Credit 0.
   The unlock precondition is status Burned on the target domain (bridge.rs:602-610), so every unlock follows a burn.

## Net supply change per full cycle

- Unlock through executor.rs:1205 or blockchain.rs:3329: -A + A + 0 + A = +A. The owner gets A back (minus f), the recipient still holds the minted A - f, the relayer got f twice. This matches F01, and the change is exactly A, not "about" A.
- Unlock through blockchain.rs:2362: -A + A + 0 + 0 = 0. Supply holds, but the owner never gets the locked value back and the recipient keeps the minted value after its burn. The burn is a burn in name only.
- The tx fee of the relayer result tx (executor.rs:1283) is the normal fee flow and is not part of A.

## Cap effect

Unlock uses try_add_balance (account.rs:1907-1917), which checks only u64 overflow, not the cap. Mint checks headroom, but the lock debit created that headroom (point 2). So the overshoot above BUD_TOTAL_SUPPLY is bounded by the sum of transfers in flight at the same time. Below the cap, each cycle uses up headroom, which includes headroom created by the burn mechanism, so the cycle cancels burned supply.

## Corrections and additions to F01

- Reachability. Blockchain::lock_bridge_transfer has no production caller. The RPC entry was removed (src/rpc/api.rs:303-313). ChainHandle::lock_bridge_transfer (src/chain/chain_actor.rs:2417) has no caller. Both are in .github/idle-code-baseline.txt:1191 and 1217. The only transfer insert is bridge.rs:412 inside BridgeState::lock. Its other caller is bridge_relayer.rs:145, which is unwired (F10). So mint and unlock in production find no transfer row today. "Operator gated (server.rs:2770)" covers only the burn leg. Not checked: whether a snapshot or storage import can bring in bridge rows.
- Second inflating unlock path. F01 names only executor.rs:1265. submit_relay_proof at blockchain.rs:3381-3384 has the same +A credit.
- Path disagreement. Two unlock paths credit A and one credits nothing. The same logical unlock gives different balances and a different state root depending on the entry point.

## Fix direction (not a patch)

- Choose one ledger model and pair every leg. Lock and mint both act on the same AccountState, so a conserving model is escrow: lock moves A from owner to a bridge escrow that total_bud_committed counts; mint of a locally locked transfer moves from escrow and does not create supply; burn moves the recipient's amount back to escrow (exact amount, balance checked before the bridge state moves); unlock moves escrow to owner.
- If the model is "mint creates supply", then burn must destroy it: debit the recipient in burn_bridge_transfer_with_event with exact burn semantics, refuse when short, and do it before bridge_state.burn_with_event and before submit_cross_domain_message (see F11 for order).
- Use one shared function for the three unlock paths so that their balance effects cannot differ.
- Count bridge-locked or escrowed value in total_bud_committed so a lock does not open false mint headroom.

## Missing tests

- src/tests/bridge_lifecycle.rs:80-169 is named lock_mint_burn_unlock, but it never mints with success, never burns and never unlocks (111-168). It has no balance or supply assertion.
- Needed: a full cycle test through each unlock entry (executor RelayerResult with an anchored external root, submit_relay_proof, verified-event path) that asserts total_bud_committed() before equals after, plus owner and recipient balances.
- Needed: a cross-path test that the same cycle through each entry gives the same balances.
- Needed: a property test over amount, bridge_relayer_fee_ppm and bridge_relayer_min_fee for the conservation invariant.
- Needed: a near-cap test where unlock must not push committed supply above BUD_TOTAL_SUPPLY.
- After a fix: a burn by a recipient with a short balance must be refused with no state change.
