# STATUS

Read this file at the start of every session. It is the short state of the work. Rules: CLAUDE.md and MODEL_ROUTING.md. Map of the repo: docs/AGENT_MAP.md. Rewritten 2026-10-09 (smaller; old round logs are in git history).

Maintenance: the main session rewrites this file at the end of every round. Facts with commit SHAs, paths and gate names. No exploit steps of unfixed findings (details live in docs/audit/FINDINGS.md during testnet, CLAUDE.md Z14).

## 1. Goal and working rules

- Goal: Budlum ready for testnet, then mainnet. Every part wired to a production call site, tested, no idle code.
- Open PR: #3, branch `claude/branch-change-pra-check-j01l81`. Audit work of the latest session is on it (head 6007bd3 plus the directive rewrite). Do not open another PR unless the owner asks. Merge approval is the owner's.
- The main session writes no code; coder agents do. At most 3 agents in parallel.
- CI checks (fmt, clippy, test, typos, deny, audit, gates) run on GitHub only. Nobody runs them locally. Read results with the GitHub tools (CLAUDE.md section 4).
- Every R3 code change gets a fresh Opus verification (new architect call). A forgery test is red first.
- Shell may sit inside `budzero/`; run git from `/home/user/budlum`.

## 2. What exists now

- B.U.D. storage writes in block (ADIM 1 to 5, done): `TransactionType::Storage` tag 47 with RegisterManifest, DeclareOperatorClass, DeclareSelfHostPolicy, OpenDeal (refused on mainnet). Code: `src/domain/storage_tx.rs`, `deal_open.rs`. Still out of block: challenge open and answer, audit wiring, payouts, maintenance, RPC mutation paths.
- Coding audit state (3dcbe16, 574a6fa, a56d04a): `StoredCodingAudit` with outcomes Passed, Failed, Missed, Void; one open audit per deal. UNWIRED: open, finalize and sweep have no production caller until steps 4b and 4c.
- Block entropy (5f2d909): `AccountState.current_block_entropy`, not in the root, no consumer yet.
- BudZKVM soundness work in `budzero/bud-proof`: 0x1F closed; R1, R2, R2b, R3, R3b, R4a-1 done and Opus checked; R4a-2 is 992d639 (Opus PASS conditional, mutation checks NOT run). TRACE_WIDTH 787 before R4a-2, PROOF_FORMAT_VERSION still 1 (bump in R8). `verify_merkle_enabled` stays false.
- K1 (QR video, priority zero): empty content, no silent zlib fallback, tolerant PNG reader, `verify_qr_video`, independent decoder, reveal checks stream id, gateway rehashes ContentId. K1-04 and K1-05 do not close directive 1.3.5 and 1.3.4 (see 4.4).
- Repo is Rust and BudL only. Exceptions: Solidity verifier, budscan Firefox layer (paused).
- Chain replay roots step 1: e39f37c (shared end of block function). NOT Opus verified, tests not run. Steps 2 to 5 wait.
- Small fixes on PR #3, not Opus verified: dcf748a RPC operator Host and Origin check, 2b88bb3 repair_index fails on gaps, c5459d4 AI execution dims bound (Opus PASS conditional).

## 3. CI state (PR #3)

Last read: before the audit commits. Red: dead-public-api-is-ratcheted (the line `src/chain/chain_actor.rs:answer_coding_audit` must be removed from `.github/dead-pub-api-baseline.txt`; the tool layer blocked the edit, owner must do it), Dependency Review (turn on Dependency graph in repository settings), no-idle-code (3 items: `verify_canonical_program` in plonky3_prover.rs about 1973-1999, `MAX_MEMORY_BYTES`, `decode_qr_video`), clippy `large_enum_variant` at core/transaction.rs:262, domain/storage_tx.rs:33, network/protocol.rs:35 (Box the variants, no allow). Read the real CI result first in the next session.

## 4. Work queues

### 4.1 Code audit (main work now)
Method and prompts: `docs/AUDIT_PLAN.md`. State: `docs/AUDIT_PROGRESS.md`. Findings: `docs/audit/FINDINGS.md`. Next, in order:
1. Verify P05 F01 (possible High: supply, burn debits nothing) then F02 to F04. The earlier verify call was stopped by the usage limit, no result.
2. Scan P06 (stopped, no result), then P07, P60 (wave D3), then follow the waves.
3. Verify remaining Critical, High and Medium raw findings in batches of at most 8: P01 F17 on, P02 F08 on, P03 rest, P04 F05 on, P05 F05 on. Low and Info stay raw (AUDIT_PLAN section 8).
4. Trace the open sub-findings S1 to S4 of P04.

### 4.2 Fix steps ready after owner answers (section 5)
- P04-F01 (Critical): invalid vote counter must not change state outside a block. Direction needs an owner decision (counter out of consensus state, or evidence carried in blocks).
- P03-F20: bound the three burn and yield ratios to at most `FIXED_POINT_SCALE` in `validate_tokenomics_supply`. Needs approval (protocol parameter, MODEL_ROUTING section 8). Test first.
- P01-F05b: try_reorg suffix bug, plain wiring, blocks every reorg test. Fix first.
- Chain replay roots steps 2 to 5 (after an Opus check of e39f37c): PoA execution-domain stamp in every replay entry; end-of-block hook clears the unfreeze queue in replay; AI pruning into that hook; try_reorg must not overwrite bridge state from disk.
- Q1 step: a deletion waits until an open audit of that deal ends (architect handoff must be written again).

### 4.3 zkVM queue
R4a-2b (fix stale doc in adapter.rs lines 93-109; add test that a memory Load at 2^32 cannot alias stack slot 0). R4b (address bounds, alignment, stack floor, dynamic storage slot). R5 (initial image lookup). R6 (computed trace length, exit code, gas bound; ask the owner before `final_state_root` stops being reported as proof-verified). R7 (Poseidon chain for `event_digest`). R8 (PROOF_FORMAT_VERSION 2, ARCHITECTURE table, gates, cleanup). Then A1 to A4, B3, AIR audit of the remaining opcodes. One coder-deep at a time, Opus check for each.

### 4.4 B.U.D. and K1
Waiting for the proof check (Q5): 3b OpenChallenge, 3c AnswerChallenge, 3d missed-challenge finalize, 3e issuance, 4b audit wiring in `apply_block_effects`, 4c `StorageTx::AnswerCodingAudit`. Open: `ChainCommand::StoragePrune` mutates state outside a block (chain_actor.rs about 3443). ADIM 6: remove RPC mutation paths, then open storage on mainnet.
K1 left: K1-07 segmentation above the single-video ceiling; K1-06 transport simulation; resource budget before decode and fuzz targets (K1-10); determinism goldens (K1-12); test suite (K1-11); reader wiring (Q3). Gaps of K1-04 and K1-05: the production call compares the encode output with itself, reader point unwired, QR symbol layer shared by both decoders, no persistent verification record.

### 4.5 Mainnet blocker classes (no detail)
PoS validator exits at start (PKCS#11 VRF backend missing); three of four mainnet domains cannot finalize; consensus state written outside blocks (external roots, message registry, bridge state, stake bonding, proof fee cut, storage maintenance); global header sealed only by an operator RPC and the received header is not validated; snapshot manifest signing has no production caller; mainnet genesis file is a template; PQ anchor and cold wallet gated off; domain fork choice unused; bridge relayer, egress privacy, identity resolver, account abstraction registry unwired; validator reward pool unwired (economic, parked); VerifyMerkle and VerifyInference closed; AI inference structural path open for non-proof models.

## 5. Owner decisions

Pending (ask in one message, plain words, A/B/C, one recommended):
1. Proposer liveness design for PoS and PoA (P01 F01, F02).
2. PoS checkpoint and reorg rule (P01 F04).
3. P04-F01 direction (see 4.2).
4. P03-F20 ratio bound approval.
5. `MODEL_ROUTING` section 4 step 4 now says Opus checks only R3 diffs; raw Low and Info audit findings stay unverified (done by owner approval 2026-10-09).

On record: B1 A all B.U.D. storage writes are signed in-block transactions. B2 A failed coding audit applies the 6 hour cooldown, no slash. Z1 A VerifyMerkle gate stays closed. D1 to D6 (operator consent in the transaction, consent digest binds chain id, payer and nonce, protocol minimum bond, escrow on `held_bytes`, any payer may fund, OpenDeal refused on mainnet until payouts and challenges run in block). F3 C generated sources refused in block. Rust and BudL only. budscan paused. K1 A A A A. BUD 3.0: content held only as a recipe. BUD 2.0 target 0.016 USD per TB per month. Security channel: private GitHub advisory (docs/SECURITY.md), but testnet findings stay in `docs/audit/` (Z14). No economic decisions in this phase. Everything coded must be wired to a production call site.
Answers 2026-10-08: Q1 A deletion waits for an open audit. Q2 A bridge messages and state become signed in-block transactions. Q3 A reader or wallet checks a video against the on-chain manifest. Q4 B sealed content keeps random nonces. Q5 B build the proof check first, then keeper checks. Q6 A unfixed findings go to the private channel.

## 6. Next session

1. Environment check (MODEL_ROUTING section 10). Read this file and docs/AGENT_MAP.md. Compare with `git log`.
2. Read PR #3 CI with the GitHub tools; fix red jobs with coders (section 3).
3. Audit work (4.1). Ask the pending decisions (section 5) in one message.
4. At the end of the round rewrite this file.

## EFFORT LOG

(no finder-max calls)
