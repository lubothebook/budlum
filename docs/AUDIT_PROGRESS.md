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
| P28 | done (2026-10-11), detail notes/P28-scan.md | 19 (9 Medium, 9 Low, 1 Info) | no | F01 to F04 emit binding and metering, F08 sealed discount, F16 replayable grants |
| P29 | done (2026-10-11), detail notes/P29-scan.md | 13 (3 Medium, 8 Low, 2 Info) | no | F01 receiver skips payload commitment check, F02 4 byte frame digest, F04 repair margin constant |
| P30 | done (2026-10-11), detail notes/P30-scan.md | 14 (1 High, 5 Medium, 7 Low, 1 Info) | no | F01 view grants outside blocks in storage root, F02 recipeCommitment offline guess, F03 reveal RPC trusts caller body |
| P31 | done (2026-10-11), detail notes/P31-scan.md | 13 (2 Medium, 6 Low, 5 Info) | no | F01 register_model id squat, F02 leaf omits execution_dims and modalities |
| P33 | done (2026-10-11), detail notes/P33-scan.md | 19 (7 Medium, 11 Low, 1 Info) | no | F01 Bridge::start unwired, F07 PromptWallet negative rate mints funds, F04 served name check weak |
| P34 | done (2026-10-11), detail notes/P34-scan.md | 18 (10 Medium, 7 Low, 1 Info) | no | F01 to F04 redact.rs masking gaps, F05 ai-knowledge unwired, F07 output lock digest gaps |
| P32 | done (2026-10-11), detail notes/P32-scan.md | 18 (3 High, 6 Medium, 8 Low, 1 Info) | no | F01 prune_expired loses max_fee escrow, F02 STARK uses prover public inputs (latent), F03 junk envelope passes require_proof |
| P36 | done (2026-10-11), detail notes/P36-scan.md | 19 (2 High, 6 Medium, 8 Low, 3 Info) | no | F01 AiModelRegister fields missing from proto, F02 vote and blob tokens never refill, F08 peer table fill |
| P35 | done (2026-10-11), detail notes/P35-scan.md | 26 (4 High, 10 Medium, 10 Low, 3 Info) | no | F01 match scrutinee register overwritten, F02 let aliasing, F03 nested struct heap overlap, F04 mapping slot 32 bit |
| P37 | done (2026-10-11), detail notes/P37-scan.md | 36 (16 Medium, 18 Low, 2 Info) | no | F01 bud-core has no consumer in the node, F20 4 GiB chunk allocation, F02 v1 parity is XOR constant, F22 das_root duplicate leaf |
| P38 | done (2026-10-11), detail notes/P38-scan.md | 31 (10 Medium, 18 Low, 3 Info) | no | F01 open_shard_pack loses pack on truncated tail, F02 19 of 26 modules unwired, F06 30 of 30 price claim unsupported |
| P40 | done (2026-10-11), detail notes/P40-scan.md | 29 (9 Medium, 17 Low, 3 Info) | no | F01 zk witness hashes only lengths, F02 any prover stdout counts as ProvenExternally, F03 timeseries decode reserves 1.6 GB |
| P39 | done (2026-10-11), detail notes/P39-scan.md | 29 (17 Medium, 11 Low, 1 Info) | no | F04 QR place_data loses bits, F01 to F03 qrvideo decoder no integrity, F08 scrub domain tag mismatch, F05 PoR tag ignores nonce |
| P41 | done (2026-10-11), detail notes/P41-scan.md | 20 (8 Medium, 10 Low, 2 Info) | no | F01 bud tx send fixed devnet chain id, F02 relayer scan RPC arg mismatch, F12 relay full signs without activation state |
| P42 | done (2026-10-11), detail notes/P42-scan.md | 26 (2 High, 7 Medium, 16 Low, 1 Info) | no | F01 vault reaches_walk unbounded recursion, F02 Pollen guard checks content_id but gateway serves storage_root, F03 EVM submit always retries |
| P43 | done (2026-10-11), detail notes/P43-scan.md (crates/budscan/Cargo.lock not readable) | 23 (5 Medium, 16 Low, 2 Info) | no | F01 budscan checks wrong identity, F02 and F03 Pollen gate and purchase unwired, F04 DataAsset ownership not proven |
| P45 | done (2026-10-11), detail notes/P45-scan.md | 17 (8 Medium, 7 Low, 2 Info) | no | F01 Pollen digests lack chain_id, F02 negatives pass for any reason, F04 vacuous chaos tests |
| P44 | done (2026-10-11), detail notes/P44-scan.md | 20 (8 Medium, 8 Low, 4 Info) | no | see notes/P44-scan.md F01 to F06 |
| P47 | done (2026-10-11), detail notes/P47-scan.md | 20 (4 Medium, 11 Low, 5 Info) | no | F01 load_test zeroes overlay roots, F02 audit answer not bound to column, F03 PoA self-nominated set pinned, F04 mainnet stake floor untested |
| P46 | done (2026-10-11), detail notes/P46-scan.md | 15 (3 Medium, 8 Low, 4 Info) | no | F01 future timestamp test cannot reach drift gate, F02 epoch isolation test cannot fail, F03 BFT leader tests weak |
| P48 | done (2026-10-11), detail notes/P48-scan.md | 20 (4 Medium, 10 Low, 6 Info) | no | F01 qcblob quorum tests never reach quorum check, F03 privacy_mainnet_disabled untested, F04 relayer slash with fake provenance |
| P49 | done (2026-10-11), detail notes/P49-scan.md | 17 (1 High, 5 Medium, 6 Low, 5 Info) | no | F01 settlement_prod.rs cfg(false) compiles out about 45 tests, F05 test pins manifest_id collision, F02 supply cap proptest cannot fail |
| P50 | done (2026-10-11), detail notes/P50-scan.md | 24 (11 Medium, 11 Low, 2 Info) | no | F01 air_selectors gate does not check the assert, F02 and F03 ci_workflow_guards substring match, F05 cfg(test) item hides next function |
| P52 | done (2026-10-11), detail notes/P52-scan.md (Cargo.lock not readable) | 23 (9 Medium, 13 Low, 1 Info) | no | F03 module-coverage baseline file missing so gate always skips, F02 wallet-core root wrong, F01 minting gate skips one-line fns |
| P51 | done (2026-10-11), detail notes/P51-scan.md | 22 (10 Medium, 11 Low, 1 Info) | no | F01 forgery_tests any earlier test satisfies gate, F03 slash-caller scan cut at first cfg(test), F06 coverage reads first file |
| P32 on | not started | n/a | no | rolling start (CLAUDE.md 4A.10) |

Verification calls done: 6 (V06 for P02 F01 to F07, V05 for P03 seven findings, V04 for P01 F09 to F16, V01 for P01 F01 to F04, V02 for P01 F05 to F08, V03 for P04 F01 to F04). No agent was running when this log was written.

Rules for the next session:

1. Do not rescan a part marked done. Verify its raw findings in batches of at most 8 (at most 4 for the most serious ones) with a new architect call each. Order: P04 F01 to F04, P01 F09 on, P03 F20, P02 F03, then the rest.
2. Scan P05 next, then follow the waves.
3. Ask the owner the two open decisions (proposer liveness, checkpoint and reorg rule; STATUS.md:102) before writing handoffs.
4. Fix the try_reorg suffix bug (P01-F05b) first: it is plain wiring and blocks every reorg test.
5. Update this file and docs/audit/FINDINGS.md and commit after every result.
