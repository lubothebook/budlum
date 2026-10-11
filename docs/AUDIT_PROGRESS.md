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
| P05 | done (session 2, coder-deep, all 6078 lines read) | 22 | 12 of 22 (V07: F01 to F04; V08: F05 to F12 confirmed, new F06a High) | next: F13 on are Low and stay raw; F01 needs a fresh architect check and an owner decision |
| P06 | done (2026-10-11) | 16 (2 High, 9 Medium, 4 Low, 1 Info) | 8 of 16 (V09: F01 to F08 confirmed, F04 partly, F05 raised to High; new F01a, F01b, F04a) | next: F09 on; check whether external_roots feeds the state root |
| P07 | done (2026-10-11) | 21 (3 High, 8 Medium, 8 Low, 2 Info) | 8 of 21 (V10: F01, F02, F04, F07 confirmed; F03, F05, F06, F08 partly) | next: F09 on (Medium first) |
| P60 | done (2026-10-11) | 9 (1 Critical, 1 High, 5 Medium, 1 Low, 1 Info) | 2 of 9 (V11: F01 and F02 confirmed, both latent: contract not deployed) | next: F03 to F07 |
| P08 | done (2026-10-11) | 13 (3 High, 3 Medium, 5 Low, 2 Info) | 6 of 13 (V12: F01 to F06 confirmed) | F07 on stay raw (Low and Info) |
| P09 | done (2026-10-11) | 11 (1 High, 4 Medium, 6 Low) | 5 of 11 (V13: F01 High confirmed, cheap trigger about 43k BUD; F03 confirmed; F02, F04, F05 partly and lowered) | F06 on stay raw |
| P10 | done (2026-10-11) | 11 (+F12 High vote griefing) | 5 of 11 + F12 (V14: F01, F02, F04, F05, F12 confirmed; F03 partly, not reached because F01 keeps finalized_height at 0) | F06 on stay raw; trace F12 report path to the slash |
| P11 | done (2026-10-11) | 12 (1 Critical, 3 Medium, 8 Low) | F01 (main session), F02 to F04 (V16: F02, F03 confirmed, F04 partly lowered) | F05 on stay raw |
| P12 | done (2026-10-11) | 7 (1 High, 2 Medium, 4 Low) | 3 of 7 (V15: F01 to F03 confirmed, latent) | F04 on stay raw |
| P15 | done (2026-10-11) | 5 (+F06 DoS lead) | F01 CONFIRMED Critical (V17), F06 confirmed Medium/High | F02 to F05 stay raw |
| P14 | done (2026-10-11) | 8 (2 Medium, 5 Low, 1 Info; all test quality) | 2 of 8 (V19: F01 confirmed, F02 lowered to Low) | F03 on stay raw |
| P13 | done (2026-10-11) | 10 (1 High, 5 Medium, 3 Low, 1 Info) | 6 of 10 (V18: F01 High confirmed and reachable via AiAttachExecutionProof, new F01a; F02, F03 lowered to Low; F04 confirmed. V19: F05, F06 confirmed) | F07 on stay raw |
| P16 | done (2026-10-11) | 23 (5 Medium, 16 Low, 2 Info) | 5 of 23 (V20: F10, F11 confirmed and reached in production via the twin, new F11a; F02 confirmed latent; F01, F08 lowered to Low) | F03 on stay raw |
| P17 | done (2026-10-11) | 19 (1 High, 10 Medium, 6 Low, 2 Info) | no | F01 first (64 bit commitment), then F08, F09 |
| P18 | done (2026-10-11) | 14 (3 High, 4 Medium, 7 Low) + Info | no | F01 first (unauthorised StateUpdate nonce), F02, F03 |
| P58, P59 | done (2026-10-11) | P58 12 (5 Medium, 6 Low, 1 Info), P59 9 (4 Medium, 4 Low, 1 Info) (Part 22) | no | no Z3 softening found; R1 level |
| P19 | done (2026-10-11) | 12 (1 High, 2 Medium, 7 Low, 2 Info) | no | F01 slash after unstake, F03 mint after burn |
| P20 | done (2026-10-11) | 13 (5 Medium, 8 Low) + 9 Info | no | F01 governance slot exhaustion, F06 V5 equivocation |
| P61 | done (2026-10-11) | 12 (2 High, 3 Medium, 4 Low, 3 Info) | no | F01 archive.toml chain id, F02 systemd key var |
| P22 | done (2026-10-11) | 11 (4 Medium, 4 Low, 3 Info) | no | F01 RPC defaults after parse, F02 no fail closed on mainnet |
| P21 | done (2026-10-11) | 8 (4 Medium, 4 Low) + Info | no | F04 storage RPC mutation paths, F01 no rate limit |
| P23 | done (2026-10-11) | 17 (1 High, 9 Medium, 7 Low) + Info | no | F01 bond and fee lost on prune, F08 huge epochs lock challenges |
| P24 | done (2026-10-11) | 8 (5 Medium, 2 Low, 1 Info) | no | F05 maintenance outside blocks (Known), F01 ledger brick |
| P25 | done (2026-10-11) | 11 (1 High, 5 Medium, 4 Low, 1 Info) | no | F01 PoW finality too cheap on bridge enabled domain 1 |
| P26 | done (2026-10-11) | 13 (5 Medium, 8 Low) + Info | no | F01 prune without snapshot, F02 delete_block leaves indexes |
| P27 | done (2026-10-11), detail notes/P27-scan.md | 16 (4 Medium, 9 Low, 3 Info) | no | F01 living_threshold decay, F03 to F05 unwired parts |
| P28 to P30 | scan running | n/a | no | P28, P29, P30 in flight |
| P31 on | not started | n/a | no | rolling start (CLAUDE.md 4A.10) |

Verification calls done: 6 (V06 for P02 F01 to F07, V05 for P03 seven findings, V04 for P01 F09 to F16, V01 for P01 F01 to F04, V02 for P01 F05 to F08, V03 for P04 F01 to F04). No agent was running when this log was written.

Rules for the next session:

1. Do not rescan a part marked done. Verify its raw findings in batches of at most 8 (at most 4 for the most serious ones) with a new architect call each. Order: P04 F01 to F04, P01 F09 on, P03 F20, P02 F03, then the rest.
2. Scan P05 next, then follow the waves.
3. Ask the owner the two open decisions (proposer liveness, checkpoint and reorg rule; STATUS.md:102) before writing handoffs.
4. Fix the try_reorg suffix bug (P01-F05b) first: it is plain wiring and blocks every reorg test.
5. Update this file and docs/audit/FINDINGS.md and commit after every result.
