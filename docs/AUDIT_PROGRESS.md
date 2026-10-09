# Audit progress log

This file is the running state of the audit in `docs/AUDIT_PLAN.md`. It is committed after every step so that a new session loses nothing. It holds status only. It holds no finding detail, no severity and no file or line of an unfixed security issue (MODEL_ROUTING 12.10). Full detail lives in the owner's private report.

Base commit of the audit: 25ede0b. Date of the first session: 2026-10-09.

| Part | Scan by Sonnet | Raw findings | Verified by Opus | Note |
|---|---|---|---|---|
| P62 | done | 6 | no | invariant scan; Z6, Z12 and Z3 coverage items; nothing about secrets, unsafe, float |
| P01 | done | 27 | 4 of 27 (all 4 confirmed) | next: F05 to F27 |
| P02 | done | 21 | no | |
| P03 | done | 25 | no | |
| P04 | done | 25 | no | |
| P05 | running when this log was written | n/a | no | |
| P06 to P61 | not started | n/a | no | follow the wave order in AUDIT_PLAN.md |

Verification in flight when this log was written: P01 F05 to F08 (Opus call, result not received).

Rules for the next session:

1. Do not rescan a part marked done unless the owner says so. Verify its raw findings in batches of at most 8 (at most 4 for the most serious ones) with a new architect call each.
2. Ask the owner where the private report is. If it is lost, rescan P01 to P04 and P62 (about 5 waves of cost).
3. Update this file and commit after every scan result and every verification result.

Public-text items found by P62 that need no secrecy (the owner may fix them as normal work): the banned term in public docs and in the `bud` CLI help text; shell and script blocks inside two workflow files; six `allow` attributes outside the area the suppression gate scans; two `ignore` attributes in `crates/bpqs` tests that CI never runs.
