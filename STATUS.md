# STATUS

Read this file first in every session. It is the short state of the work. Durable rules: CLAUDE.md, MODEL_ROUTING.md, BUD-AI-KAPSAMLI-DIREKTIF.md. Rewritten 2026-10-08 (owner asked for a simpler file).

Maintenance rule: the main session rewrites this file at the end of every round. Write facts with commit SHAs, paths and gate names. Never write exploit details of unfixed security findings here (MODEL_ROUTING 12.10).

## 1. Goal, branch, working rules

- Goal: Budlum ready for testnet, then mainnet. Every part is wired to a production call site, tested, and has no idle code.
- Branch of the latest session: `ccr-9d3ed79c-9wpla0` (cut from `claude/zkvm-bud-completion-84r6jc` at 8bb1962, about 60 commits on top). PR #1 tracks the older branch name. Do not open a PR unless the owner asks.
- Working rules (owner): the main session writes no code; coder agents do. At most 3 agents in parallel. Local checks only: `cargo check`, fmt, clippy, filtered tests (zkVM: `cd budzero && cargo test -p bud-proof --lib`, about 7 minutes). CI is read at the end of a round, not between steps.
- Every R3 change gets a fresh Opus verification by a new architect call. A forgery test must be red first, and each rule gets one mutation check. Scout (Haiku) output is a lead, never evidence.
- Questions to the owner: plain words, A/B/C/D choices, one recommended (R). Ask only about hard architecture, not about economics in this phase.
- The shell may sit inside `budzero/`; run git from `/home/user/budlum`. protoc may be missing: `find /tmp -name protoc` and set `PROTOC`.

## 2. What exists now

- B.U.D. storage writes in block (ADIM 1 to 5, done): `TransactionType::Storage` tag 47 with RegisterManifest, DeclareOperatorClass, DeclareSelfHostPolicy, OpenDeal (refused on mainnet). Code: `src/domain/storage_tx.rs`, `deal_open.rs`. Still out of block: challenge open and answer, audit wiring, payouts, maintenance, RPC mutation paths.
- Coding audit state (4a, 3dcbe16, 574a6fa, a56d04a): `StoredCodingAudit`, outcomes Passed, Failed, Missed, Void (no cooldown when the deal ended or the manifest is gone), one open audit per deal, finalized audits pruned by an epoch queue, queue root encoding is injective. UNWIRED: open, finalize and sweep have no production caller until steps 4b and 4c.
- Block entropy (5f2d909): `AccountState.current_block_entropy`, not in the root. No consumer yet. Global header time comes from the chain tip (d903aa9).
- Repository is Rust and BudL only (ADIM 8, done). Exceptions: Solidity verifier, budscan Firefox layer (paused).
- BudZKVM soundness work (all in `budzero/bud-proof`): 0x1F closed (990fdcd); Store through r0, canonical immediates, Eq witness (R1: 2babb74, 0fc7f90, 99ba45b); register writes only for writing opcodes (R2: a59afac, R2b: 311bf03); register table prefix and strict (index, time) order (R3: ed5624e); register init flag tied to blocks (R3b: dcb2efb); memory table active flag, prefix, inverse witness, init binding (R4a-1: 78212be, test f7618dd). TRACE_WIDTH 787, PROOF_FORMAT_VERSION still 1 (bump in R8). `verify_merkle_enabled` stays false.
- K1 (Priority Zero, QR video): empty content end to end (1c3bd7f, 834607e, ede96dd, ff1d03f); no silent zlib fallback (170661e); tolerant PNG reader (c0ad763; grey value from the first channel, check luma in K1-06); `verify_qr_video` with expected commitments (4db95e8); independent verifier decoder (5e64e53); reveal checks stream id (48a8be4); gateway rehashes ContentId on all stored paths (362e716); bud-node ContentId equals the core definition (1bac039).

## 3. Verification state

- Opus PASS: E4-a, 3a, B1, 4a-fix2 (conditional), R2, R3, R3b and R4a-1 (conditional, test added), K1-01 series, K1-04 (library only), K1-05. Not Opus verified and not required by plan: R1, K1-02, K1-03, K1-08a, K1-08b, K1-09.
- K1-04 and K1-05 do not close directive 1.3.5 and 1.3.4: the only production call compares the encode output with itself; the reader point is UNWIRED; the QR symbol layer is shared by both decoders; no persistent verification record; no resource budget before decode; no fuzz target.
- Local tests of every listed step passed (see the commit messages and agent reports). Full `cargo test` was never run. CI was last read at run 12 (head 8bb1962): red on Budlum Core (pq-ml-dsa feature step and Clippy), Typos, Repo Lint, docker-smoke (Trivy), Dependency Review; Gates, StorageProvider, BNS, Fork-Choice and Coverage were cancelled at the 6 hour limit. Nothing newer was read.
- The no-idle-code gate is red on three old items: `verify_canonical_program` (plonky3_prover.rs), `open_deal_consent_digest` and `StorageDealOpen` (storage_tx.rs).

## 4. Work queues

### 4.1 Stopped mid-work
- zkVM R4a-2 (memory table order and 32 bit address range, design B: table ids 1 memory, 2 stack, 3 storage on the bus; 32 bit address inside an id; VM memory at most 2^32 bytes; key = id * 2^32 + address; width about 854). The last `wip:` commit (fde76dc) holds partial edits. `cargo check` passes, tests were never run. Next: `cd budzero && cargo test -p bud-proof --lib memory`, finish per the plan (red forgery tests first, one mutation per rule, fresh Opus verification), or revert that commit. Stop and ask if the compiler emits a negative static storage slot (codegen.rs about 243 and 673).

### 4.2 zkVM queue after R4a-2
R4b (address bounds, alignment, stack floor, resolved dynamic storage slot; dynamic SRead and SWrite may be unprovable today). R5 (initial image lookup instead of fixed fold constants; consider moving it earlier). R6 (computed trace length, exit code, gas bound; ask the owner before `final_state_root` stops being reported as proof-verified). R7 (Poseidon chain for `event_digest`). R8 (PROOF_FORMAT_VERSION 2, ARCHITECTURE table, gates; cleanup of stale VerifyInference comments and dead prover code). Then A1 to A4 (VerifyMerkle, BudL builtin, HashMem, storage proofs end to end), B3, AIR audit of remaining opcodes. Single coder-deep at a time, Opus verification for each.

### 4.3 Chain replay roots (five steps, ready)
Cause (verified): the live path and five replay entries (startup replay, validate_candidate_chain, try_reorg, rebuild_state, get_state_snapshot and is_valid) build state roots with copied logic. Decision: option A, one shared function pair used by all entries (end-of-block state, start-of-replay state); do not move end-of-block hooks to the start of a block. Steps (coder-deep, Opus each, sequential in `src/chain/blockchain.rs`): 1 settlement root from the prefix chain in all replay entries; 2 PoA execution-domain stamp right after genesis state in every replay entry; 3 end-of-block hook clears the unfreeze queue in replay too; 4 AI pruning moves into that hook; 5 try_reorg must not overwrite bridge state from disk. Each needs a red test first, for example a 1005 block chain that must still pass `is_valid` and a restart.

### 4.4 Audit of the whole repository
- Opus finder scan of `src/chain` is done (19 findings, 3 critical). Details are NOT in the repository. They are in a private file of the old session and will be lost, see 5 (Q6). If the file is gone, run the finder again. Classes: blocks-outside slashing from invalid votes and gossiped proofs; live versus replay roots (4.3); finality state not restored after restart; short fork segments never resolved; evidence not verified outside the PoS engine; RPC unlock without refund; non-atomic reorg persistence; no root comparison in startup replay; every block writes all accounts; genesis can exceed the fixed supply.
- Next finder scans (Opus xhigh, one module group each): consensus and crypto; settlement, cross_domain, registry; network/node.rs; tokenomics and execution; bud-proof; rpc and storage; wallet-core, account_abstraction, ai_inference.
- Haiku sweep: done on 2026-10-08 for groups 1 to 9 (12 checks each). Result: low yield. Three leads read, all refuted (wallet-core change amount is guarded; bud-vm jump target is bounds checked in step; bud f64 digests have only test callers). Haiku group sweeps are dropped (MODEL_ROUTING section 1). Remaining leads are classes already known: pub fns with no non-test caller (idle code, wire or remove) and unwrap_or_default hiding errors (kept out of focused steps, 12.9).

### 4.5 B.U.D. in-block queue
3b OpenChallenge, 3c AnswerChallenge, 3d missed-challenge finalize and 3e protocol challenge issuance wait for the proof check (Q5). 4b (open, finalize, sweep audits in `apply_block_effects` at epoch start; order: finalize, expire and prune and slash, open, sweep; constant retention; constant open limit; consensus entropy) and 4c (`StorageTx::AnswerCodingAudit`; responder is the signer; epoch and time from the block; remove the old `ChainCommand::AnswerCodingAudit`) wait for Q1 (delay closing deals that have an open audit) and for Q5. Also open: `ChainCommand::StoragePrune` mutates state outside a block (chain_actor.rs about 3443). ADIM 6: remove the RPC mutation paths, then open storage on mainnet.

### 4.6 K1 queue (QR video)
Done list in 2. Left: K1-07 segmentation above the single-video ceiling (new `qr_segment.rs`; stacked after K1-05); K1-06 transport simulation (resize, 8x8 quantization, noise, frame loss, grey re-encode) inside verify; resource budget before decode and fuzz targets for every untrusted parser (K1-10); cross-platform determinism goldens named `qr_determinism_*` (K1-12); Priority Zero test suite (K1-11). Reader wiring follows Q3. Consensus-surface notes with their own PR: DN-1 validator re-checks recipe and commitment bindings; DN-2 operator challenge as the third verification point. Reports go under `docs/bud/` in English. Reported deviations: validator does not check bytes (owner decision); QR symbol layer shared by both decoders; sealed content frames are random (Q4 B).

### 4.7 Mainnet blocker classes (no detail)
PoS validator exits at start (PKCS#11 VRF backend missing); three of four mainnet domains cannot finalize; consensus state written outside blocks (external roots, message registry, bridge state, stake bonding, proof fee cut, storage maintenance); global header sealed only by an operator RPC and the received header is not validated; snapshot manifest signing has no production caller (remote snapshot sync is refused, safe); mainnet genesis file is still a template; PQ anchor and cold wallet gated off; domain fork choice unused; bridge relayer, egress privacy, identity resolver, account abstraction registry unwired; validator reward pool unwired (economic, parked); VerifyMerkle and VerifyInference closed; privacy closed; AI inference structural path open for non-proof models.

### 4.8 CI repair list (end of a round, one pass with coders)
Read the failing steps first. Budlum Core: feature step "pq-ml-dsa solo" and Clippy. Typos: two words flagged in this file, and one truncated test input in xtask/tools/src/json.rs about 342. docker-smoke: Trivy fixable HIGH or CRITICAL. Dependency Review: fails in about 1 second, check the repository dependency graph setting first. No-idle-code: wire or remove the three items in 3. Five jobs cancelled at 6 hours: find the hang, do not raise timeouts. Watch the coverage job (45 minute limit) against the 7 minute zkVM lib run. Then rewrite this file.

## 5. Owner decisions on record

Earlier: B1 A all B.U.D. storage writes are signed in-block transactions. B2 A a failed coding audit applies the 6 hour cooldown, no slash. Z1 A VerifyMerkle gate stays closed. D1 to D6 (operator consent in the transaction, consent digest binds chain id, payer and nonce, protocol minimum bond, escrow on `held_bytes`, any payer may fund, OpenDeal refused on mainnet until payouts and challenges run in block). F3 C generated sources refused in block for now. Rust and BudL only. budscan paused. K1 A, A, A, A (explicit empty marker; segmentation above the ceiling; PNG-sequence carrier with simulated transport; validator checks recipe and commitment bindings, bytes are checked by client and reader). BUD 3.0: content is held only as a recipe; every content has a recipe from the existing QR video system; no generated versus organic split; do not judge 3.0 by existing systems. BUD 2.0 target 0.016 USD per TB per month, measured per content mix. Tools: cloudflare security-audit skill allowed at project scope and kept out of the repository (install per session, first run on `src/registry/` in an owner-opened Opus xhigh session); autoharness not approved. Security channel 1 C; baseline edits approved. No economic decisions in this phase. Everything coded must be wired to a production call site.

Answers of 2026-10-08:
- Q1 A: a deletion (NFT burn or prune) waits until an open audit of that deal ends. New step before 4b.
- Q2 A: bridge messages and bridge state changes become signed in-block transactions (new transaction types; architect plan first). Until then the bridge/message state stays a mainnet blocker.
- Q3 A: the reader app or wallet checks a video against the manifest on the chain (build the call site in wallet-core; architect plan first). Defaults kept: optional plain-text check for key holders of sealed content; every check goes to an append-only local log.
- Q4 B: sealed content keeps random nonces; directive 1.3.6 (same content, same frames) does not apply to private content. Record as a deviation in the K1 report.
- Q5 B: build the proof check first (zkVM queue, storage proofs end to end), then the keeper checks in blocks. Interpretation: this also holds 3b to 3e and the audit wiring 4b and 4c; the owner may relax it for 4b and 4c. Penalty stays the deposit only (B2 A).
- Q6 A: unfixed security findings go to the private channel in docs/SECURITY.md (private GitHub security advisory). The src/chain finder report has not been filed yet: file it first, then close findings by commit.

## 6. Next step (new session)

1. Read this file. Run the environment check from MODEL_ROUTING.md section 10; compare `git log` and `git status` with section 2 and 4.1.
2. Decide on the wip commit (4.1). Then run the chain replay steps 1 to 5 (4.3) one by one with coder-deep and a fresh Opus verification each.
3. File the src/chain findings through the private channel (Q6) if not done; run the other finder scans (4.4) one module group at a time without waiting for the owner.
4. Ask the architect for plans: bridge and message state as in-block transactions (Q2), reader wiring in wallet-core (Q3), delete-waits-for-audit (Q1). Continue K1 (4.6) and the zkVM queue (4.2) in parallel when files differ.
5. At the end of the round read CI and fix with coders (4.8); rewrite this file.

## EFFORT LOG

(no finder-max calls)
