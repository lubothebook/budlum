# STATUS

Active work only. Durable decisions belong in memanto.

## Current goal

Complete B.U.D. and BudZKVM for mainnet. Branch `claude/zkvm-bud-completion-84r6jc`, PR #1 (draft).

## Owner decisions (2026-10-07)

- B1, storage writes: A. Move every B.U.D. storage write into signed in-block transactions (`TransactionType::Storage`). Mainnet gate is asked separately at the end.
- B2, coding audit failure cost: A. Record the failure and apply the 6 hour operator cooldown. No bond slash, because a column fault cannot yet be attributed to one operator.
- Z1, VerifyMerkle: A. The gate stays closed. Prepare the external review package.
- Z2, VerifyInference circuit: priority not set.

## Steps

| ADIM | Scope | State |
|---|---|---|
| 1 | `StorageTx::RegisterManifest` | done, `9527a01`, Opus verification pending |
| 2 | `StorageTx::OpenDeal`, deterministic block time | planning |
| 3 | Open and answer a retrieval challenge in block | open |
| 4 | Stored coding audit with deadline, cooldown on failure | open |
| 5 | `DeclareOperatorClass` | open |
| 6 | Remove RPC mutation paths, mainnet gate (ask owner) | open |
| 7 | VerifyMerkle external review package | discovery |

## Open findings

- B2: `AnswerCodingAudit` accepts a caller-built `CodingAudit`, so an answerer can pick a column it computed honestly (`src/chain/chain_actor.rs`, AnswerCodingAudit arm). The maintenance sweep derives audits and discards them. Closed by ADIM 4.

## Deviations to report

- ADIM 1 removed one stale line from `.github/idle-code-baseline.txt`. The `no-idle-code` gate requires it because `register_manifest_with_source` gained a production caller. This is a tightening, but MODEL_ROUTING §12.6 forbids touching baseline files, so the owner should confirm.

## EFFORT LOG

(no finder-max calls)
