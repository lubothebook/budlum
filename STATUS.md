# STATUS

Active work only. Durable decisions belong in memanto.

## Current goal

Complete B.U.D. and BudZKVM for mainnet. Branch `claude/zkvm-bud-completion-84r6jc`, PR #1 (draft).

## Owner decisions (2026-10-07)

- B1, storage writes: A. Move every B.U.D. storage write into signed in-block transactions (`TransactionType::Storage`). Mainnet gate is asked separately at the end.
- B2, coding audit failure cost: A. Record the failure and apply the 6 hour operator cooldown. No bond slash, because a column fault cannot yet be attributed to one operator.
- Z1, VerifyMerkle: A. The gate stays closed. Prepare the external review package.
- Z2, VerifyInference circuit: priority not set.
- D1 A: payer is tx.from; the tx carries the operator's ML-DSA consent (GrantAuthorization) over the deal digest.
- D3 A: in-block minimum operator bond is a protocol constant equal to today's default (1_000_000).
- D4 A: escrow is priced on held_bytes, the number the deal records.
- D2 (decided by Claude): the consent digest binds chain_id, payer and the payer's tx nonce, so a consent is single-use.
- D5 (decided by Claude): any payer may fund a deal, as today.
- D6 A: the executor refuses OpenDeal on mainnet until maintenance, payouts and challenges also run in block.
- F3 C: the in-block registration path refuses a Generated source for now. Stored and sealed recipes are allowed.
- Language rule (owner): the repo is Rust and BudL only.
  - 1B: the Solidity verifier stays (Ethereum runs it); it is bound to Rust-generated ABI and test vectors.
  - 2A: the budscan Firefox patch layer is replaced by a Rust-engine browser shell over the existing Rust core.
  - 3: inline shell in CI YAML moves into Rust xtask commands. Nix, Dockerfile, systemd and YAML stay as configuration formats.
- Standing rule: ask the owner only for hard architecture changes. Decide the rest, record it here.

## Steps

| ADIM | Scope | State |
|---|---|---|
| 1 | `StorageTx::RegisterManifest` | done, `9527a01`, Opus verification pending |
| 2 | `StorageTx::OpenDeal`, deterministic block time | planning |
| 3 | Open and answer a retrieval challenge in block | open |
| 4 | Stored coding audit with deadline, cooldown on failure | open |
| 5 | `DeclareOperatorClass` | open |
| 6 | Remove RPC mutation paths, mainnet gate (ask owner) | open |
| 7 | VerifyMerkle external review package | draft in scratchpad (not committed: open soundness leads, SECURITY.md process); finder running |
| 8 | Port Python and shell tooling to Rust (owner rule: Rust and BudL only) | coding |
| 9 | Move inline CI shell into Rust xtask | open, after ADIM 8 |
| 10 | Solidity verifier bound to Rust-generated ABI and vectors | open |
| 11 | budscan Rust-engine shell replacing the Firefox layer | planning |
| 12 | BudZKVM soundness fixes (details held privately per docs/SECURITY.md) | planning, local only |

## Open findings

- B2: `AnswerCodingAudit` accepts a caller-built `CodingAudit`, so an answerer can pick a column it computed honestly (`src/chain/chain_actor.rs`, AnswerCodingAudit arm). The maintenance sweep derives audits and discards them. Closed by ADIM 4.

## Deviations to report

- ADIM 1 removed one stale line from `.github/idle-code-baseline.txt`. The `no-idle-code` gate requires it because `register_manifest_with_source` gained a production caller. This is a tightening, but MODEL_ROUTING §12.6 forbids touching baseline files, so the owner should confirm.

## EFFORT LOG

(no finder-max calls)
