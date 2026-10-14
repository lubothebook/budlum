# PR9 follow-up

## Scope and baseline

The owner requested a separate pull request and no subagents. All work uses
`arena/1c9a05d6-budlum`. PR9 remains open and is not merged or closed.
The starting point is PR9 head `4a2132fa60681c8a38f1d6b44a1db227994e779c`.
The full PR9 history is included so that its fixes are not lost.

The source of findings remains `docs/bulgular/BULGU_DEFTERI.md`,
`docs/audit/FINDINGS.md`, and `notes/Pxx-scan.md`. At intake the ledger had
1,099 findings: 787 open, 218 awaiting CI, 87 awaiting decisions, and seven
marked false positive. No finding is closed just to reduce that count.
Protocol, economic, trust-root, and deployment decisions in the PR9 comment
are not assumed approved. Mainnet containment flags remain closed.

## CI evidence at intake

Run `38428202021` tested the PR9 head. The API reports:

- Secret Scan passed (job `115405636367`). This is the missing green evidence
  for the gitleaks configuration fix in `b35dd86` (BLG-0001).
- Core quality failed in the pedantic/nursery ratchet, not Format.
  Format, Clippy, and cargo doc passed (job `115405635789`).
- Repo Lint failed in Rust gates (job `115405636166`).
- Domain Tag Inventory failed (job `115405636519`). The source inventory
  lacks `BDLM_RELAYER_RESULT_MSG_V1`, added by `dbc72d6` for BLG-0288.
- Library tests exceeded 90 minutes and coverage exceeded 45 minutes.
  Several jobs that run the full library suite also hit their time limit.
  These are not successful test results.

Full log downloads are blocked by the sandbox network policy at the Actions
log host. Check-run annotations and job step results remain readable through
`gh api`. The follow-up emits escaped gate failures as Actions annotations,
without changing exit status, gate scope, or any baseline. The format surface
now runs only when the Format step itself fails, not for unrelated failures.
Gate unit tests are added to Repo Lint; previously the canaries ran but many
unit regression tests did not.

## Verification rules

Rust tools are not installed in this sandbox. Repository policy also assigns
Rust verification to GitHub CI. No local Rust test pass is claimed.
Follow-up fixes stay pending until their CI evidence is read. This document
is a continuation record, not a claim that all 1,099 findings are resolved.
