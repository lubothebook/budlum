# Audit progress log

This file is the running state of the audit in `docs/AUDIT_PLAN.md`. Finding detail is in `docs/audit/FINDINGS.md` (testnet phase, rule Z14 in CLAUDE.md). Commit this file after every scan result and every verification result.

Base commit of the audit: 25ede0b. Date of the first session: 2026-10-09. The clone was shallow (50 commits).

| Part | Scan by Sonnet | Raw findings | Verified by Opus | Note |
|---|---|---|---|---|
| P62 | done | 6 | no | invariant scan |
| P01 | done | 27 (+1 new root cause) | 16 of 27 (V04: F09 to F16, six confirmed, F09 partly, all Medium) | next: F17 to F27 |
| P02 | done | 21 | 7 of 21 (V06: F01 to F07; F04 partly, rest confirmed; severities lowered for F02, F05, F06, F07) | next: F08 to F21 |
| P03 | done | 25 | 7 of 25 (V05: F01, F02, F05, F11, F12, F13, F20, all confirmed, most latent) | next: rest |
| P04 | done | 25 | 4 of 25 (V03): F01 Critical confirmed, F02 and F03 High confirmed, F04 lowered to Medium (partly) | next: F05 to F25; trace S1 to S4 |
| P05 | rescan running (coder-deep, started in session 2) | n/a | no | |
| P06 to P61 | not started | n/a | no | follow the wave order in AUDIT_PLAN.md |

Verification calls done: 6 (V06 for P02 F01 to F07, V05 for P03 seven findings, V04 for P01 F09 to F16, V01 for P01 F01 to F04, V02 for P01 F05 to F08, V03 for P04 F01 to F04). No agent was running when this log was written.

Rules for the next session:

1. Do not rescan a part marked done. Verify its raw findings in batches of at most 8 (at most 4 for the most serious ones) with a new architect call each. Order: P04 F01 to F04, P01 F09 on, P03 F20, P02 F03, then the rest.
2. Scan P05 next, then follow the waves.
3. Ask the owner the two open decisions (proposer liveness, checkpoint and reorg rule; STATUS.md:102) before writing handoffs.
4. Fix the try_reorg suffix bug (P01-F05b) first: it is plain wiring and blocks every reorg test.
5. Update this file and docs/audit/FINDINGS.md and commit after every result.
