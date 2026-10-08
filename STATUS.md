# STATUS

Read this file first in every session. It is the technical state of the work: what exists, what was decided, what is in flight and what to do next. Durable rules live in CLAUDE.md, MODEL_ROUTING.md and BUD-AI-KAPSAMLI-DIREKTIF.md.

## Maintenance rule (owner)

The main session rewrites sections 1 to 6 of this file at the end of every work round and before any session ends. A new session must be able to continue from this file alone. Write facts with commit shas, file paths and gate names. Do not write exploit details of unfixed security findings here.

## 1. Goal and branch

- Goal: Budlum testnet ready, then mainnet. Every part works, is wired to a production call site and is tested. No dead or idle code.
- Branch of the latest session: `ccr-9d3ed79c-9wpla0` (cut from `claude/zkvm-bud-completion-84r6jc` at 8bb1962, about 55 commits on top). PR #1 against `main` tracks the older branch name; no PR exists for the new branch yet (do not open one unless the owner asks).
- Directives: CLAUDE.md (purpose, invariants, loop), MODEL_ROUTING.md (agents, effort, reading rules), BUD-AI-KAPSAMLI-DIREKTIF.md (B.U.D. 1.0, 2.0, 3.0, gates K0 to K6). Setup steps are in MODEL_ROUTING_KURULUM.md.
- Project settings: `.claude/settings.json` (model opusplan, read deny for target/ and Cargo.lock) and seven agents in `.claude/agents/` (scout Haiku; finder, architect, finder-max Opus; coder, coder-deep, coder-lite Sonnet).
- Behaviour principles (Karpathy, also in CLAUDE.md): think before coding and state assumptions; minimum code that solves the problem; surgical changes where every changed line traces to the request; define a verifiable goal and loop until it is verified.
- Communication (owner): ask questions in plain sentences, not in code blocks. One message may hold several questions; give a recommendation for each.
- Working rules (owner): the main session writes no code. Coder agents write code. At most 2 or 3 agents in parallel. Tests run on GitHub CI; locally only `cargo check`, `cargo fmt` and the specific new tests. CI is reviewed at the end of a round. Ask the owner only for hard architecture decisions.

## 2. What exists now (technical)

### 2.1 B.U.D. storage writes in block (done, ADIM 1 to 5)

All storage registry writes are moving from out-of-block RPC commands to signed in-block transactions. The registry is part of the account state root, so out-of-block writes make nodes diverge; that is why the chain actor refuses every storage command on mainnet today (`storage_economics_disabled_on_mainnet`, `src/chain/chain_actor.rs`).

- `TransactionType::Storage(StorageTx)`, type tag 47, proto `STORAGE = 47` / `ProtoStorageTx` (bincode payload). Code: `src/domain/storage_tx.rs`, executor arm in `src/execution/executor.rs`, preimage in `src/core/transaction.rs` (`encode_storage_tx`), wire in `src/network/proto_conversions.rs`.
- Variants: `RegisterManifest` (signer must be the manifest owner; preimage commits the whole manifest; Generated sources are refused for now because running the recipe in block is not priced), `DeclareOperatorClass` (Mobile refused while the sender operates any active deal), `DeclareSelfHostPolicy` (owner only; policies keyed by `(manifest_id, shard_id)`), `OpenDeal` (payer is `tx.from`; operator consent is a `GrantAuthorization` over a digest binding chain id, payer, nonce and every field; domain tag `BDLM_STORAGE_OPEN_DEAL_TX_V1`; refused on mainnet).
- `src/domain/deal_open.rs`: `open_deal_escrowed` and `accept_reallocation_escrowed` check everything first, then write. Escrow is priced on `held_bytes`. `STORAGE_MIN_OPERATOR_BOND` (1_000_000) is the in-block minimum bond (`src/domain/storage_params.rs`). The RPC wrappers in `src/chain/blockchain.rs` call the same functions. Fixed: a reallocation used to debit the payer and then fail without refund.
- `AccountState.current_block_unix_secs` (not hashed, not persisted) gives the executor deterministic block time; `produce_block` computes the planned timestamp before collecting transactions.
- Removed: the out-of-block `SetStorageOperatorClass` command.
- Still out of block (to move): retrieval challenge open and answer, coding audit open and answer, payouts and maintenance, the RPC mutation paths.

### 2.2 Repository is Rust and BudL only (done, ADIM 8)

All Python and shell tooling was ported to Rust and the scripts deleted: `xtask/tools` subcommands (audit-deps, generate-sbom, smoke-rpc, devnet-multinode-smoke, docker-smoke-mainnet, audit-guard, step-reachability, clippy-extra-report, fsf-project-fit), `bud/src/bin/measure_ratios.rs` (byte-identical corpus, same ratios), `crates/bpqs/examples/checksum_domination_exact.rs`. Workflows and docs call the Rust tools. Exceptions kept by owner decision: the Solidity verifier (`contracts/external-domain/`, runs on Ethereum; to be bound to Rust-generated ABI and vectors) and the budscan Firefox layer (paused, see 5).

### 2.3 BudZKVM (partly done, in progress)

- Done (commits b64163d, cf9a35f): tighter AIR row transitions (sequential `next_pc`, last row is Halt, expansion rows bound to their opcode, VerifyMerkle block closure) and `Plonky3Adapter::verify_with_activation` (the default verifier refuses opcodes the mainnet activation does not allow; relayer, bench, CLI `--activation` and the cross-domain adapter pass the activation explicitly). `verify_merkle_enabled` stays false.
- Not trustworthy yet: the register and memory arguments and the run boundary have confirmed soundness gaps. Closing them is the work queue in section 4. Until then zkVM proofs must not be relied on for value. VerifyMerkle stays closed on mainnet until the queue is done and an external review has signed off.
- Owner decision on channel (1 C): fixes go to the public branch with neutral commit messages.

### 2.4 Tooling for sessions

- `STATUS.md` (this file), `CLAUDE.md`, `MODEL_ROUTING.md`, `MODEL_ROUTING_KURULUM.md`, `BUD-AI-KAPSAMLI-DIREKTIF.md` at the repo root; `tree-is-english` and typos gates were given a narrow exemption for these Turkish owner documents.

## 3. Verification state

- Session tooling installed: Karpathy behaviour principles appended to CLAUDE.md (MODEL_ROUTING_KURULUM 3.2). Not installed: cloudflare security-audit skill (needs owner approval and an `opus xhigh` session), autoharness (owner approval and source review required), memanto (needs Docker and Ollama, not available in the cloud container; durable decisions stay in this file).

- Local checks used so far: `cargo check --lib --tests`, `cargo fmt --check`, clippy, the xtask gates (`cargo run -q --release --manifest-path xtask/gates/Cargo.toml -- <gate>`) and filtered tests. No full `cargo test` has been run locally.
- Opus verification passed with fixes applied for: ADIM 1 (manifest signing), ADIM 5 (operator class and policy), ADIM 2b (deal open and reallocation).
- zkVM commits b64163d and cf9a35f: Opus verification PASS with notes. New forgery tests verify on the parent AIR and are refused on the fixed AIR. Notes: VerifyInference expansion length is not enforced (superseded by queue step 7, which makes 0x1F fail closed); an honest VerifyMerkle with an out-of-bounds path address is unprovable (fix before activation); some callers (cross-domain adapter, ai_inference verify) use the default closed activation implicitly, which is correct.
- Pending verification: ADIM 2c `OpenDeal` (e649082) has no separate Opus verification yet.
- CI was last read at run number 12 (head 8bb1962); nothing newer was read in the latest session (owner order: read CI at the end of the round). See 6.2 and 6.8. CI on GitHub (PR #1) was red when last looked at (before the latest commits) on: Dependency Review, Typos, Repo Lint, Gates, Budlum Core, docker-smoke. The owner enabled Actions; the latest runs were not read. See section 6, step 2.
- Baseline edits approved by owner: four lines removed from `.github/idle-code-baseline.txt` (items gained production callers).

## 4. Work queues (in order)

### 4.1 BUD directive gates (current focus, see BUD-AI-KAPSAMLI-DIREKTIF.md)

- K0 discovery: done. `docs/bud/BUD-KESIF-RAPORU.md` (commit 2e4c5ce) has the module map, the NFT burn flow and an 18-row gap table against Priority Zero. Key facts: the QR video system exists (`src/storage/qr_*.rs`, `three_pipe.rs`) but is reachable only through the read-only `bud_storageQrFeedPreview` and reveal RPCs, not through upload, validation, NFT mint or commitments; zero bytes are refused in 6 places; the effective size ceiling is about 0.8 MB at the default block size (about 11.9 MB at the best block size) because the 64 MiB constants are unreachable through the video; no real transport path is simulated; the "two decoders" are the same receiver used twice; the decoder takes the stream commitment from the video itself; no fuzz target, no property tests, no cross-platform determinism job, no audit record type; the chain holds only the manifest so a validator cannot verify bytes; no `StorageClaim`, heartbeat or personal storage node; NFT burn has the owner as the only authority, no DAO path and no tombstone; `ChainCommand::StoragePrune` mutates the registry outside a block; zlib via miniz_oxide makes commitments depend on the crate version; four different ContentId hash definitions exist (budzero bud-node, bud/ crate). Scout numbers were wrong in three places (drop header is 24 bytes, max block length is 8168, the second decoder is the same path), which is why scout output is never evidence.
- K1 blockers (resolved by the owner, see section 5): four owner questions were asked (empty-content encoding, size ceiling or segmentation, carrier, validator verification design). Claude's recommendations: A for all four (explicit empty marker; segmentation; PNG-sequence BDLV with a Rust transport simulation, no real video codec; validator re-checks the canonical recipe and commitment bindings, byte-level roundtrip at client and reader). Do not start K1 coding before the owner answers. Claude decided: the second decoder is written as a verifier only, reports stay under `docs/bud/` in English, and the ContentId definitions are unified in a separate step.
- K1 Priority Zero: close the gap table (zero-byte content, fuzz target, lossy transport, differential second decoder, determinism across platforms, audit record, three verification points). Next step after K0: an architect plans K1 as small steps for coder agents.
- K2 BUD 1.0 (personal storage node, `StorageClaim`, heartbeat, challenge), K3 BUD 2.0 (compression pipeline, cost measurement, wire erasure, audit, repair, access counter design note), K4 BUD 3.0 (recipe NFT, lifetime pricing, owner-or-DAO burn, tombstone), K5 duplicate and spam research and interface, K6 AI audit pipeline. The directive says each gate ends with a report and an owner approval; the owner said to continue, so stop only at the directive's stop conditions.
- Open questions from the directive (section 15) must be resolved before constants are fixed; use configurable parameters until then.

### 4.2 B.U.D. in-block queue

- ADIM 3: retrieval challenge open and answer as in-block transactions.
- ADIM 4: stored coding audit with a deadline; a failure records and applies the 6 hour operator cooldown (owner decision B2 A, no bond slash). Today the maintenance sweep derives audits and discards them, and `AnswerCodingAudit` accepts a caller-built audit.
- ADIM 6: remove the RPC mutation paths; then open storage on mainnet (owner: Claude opens the gate when payouts, challenges and audits run in block).
- Low: `declare_operator_class` scans all deals; index by operator later.

### 4.3 BudZKVM queue (architect plan done; each step: a forgery test that verifies before the fix and is refused after; Opus verifies where marked)

Design choices (architect, accepted): sorted register and memory tables with a range-checked lexicographic order (32-bit decompositions; no 2^16 lookup table); initial image through a lookup against a preprocessed image (BudAir carries the image; initial_state_root limbs become a native hash of it); memory bus table ids 1 memory, 2 stack, 3 storage; trace width about 754 to 856; max constraint degree 6. One PROOF_FORMAT_VERSION bump to 2 for the whole series (nothing is released yet). The verifying key now depends on the image, so it is per execution.

Order (single coder at a time, all steps edit `budzero/bud-proof/src/plonky3_air.rs`):
1. B1: make VerifyInference (0x1F) fail closed (VM rd = 0 with no expansion; AIR asserts the selector and expansion flag are 0; flip the tests; fix ISA comments and docs/AI_VERIFICATION_STATUS.md). Also closes the free-filler-row finding.
2. R1: `Store` through r0 creates memory demand; canonical immediates in the VM; EQ inverse in the field. (coder, no Opus)
3. R2: register bus write only for opcodes that write rd. (coder, Opus)
4. R3: register table activity as a boolean prefix and strict (index, time) order. (coder-deep, Opus)
5. R4a: memory tables by id, same-address inverse, order and prefix. (coder-deep, Opus)
6. R4b: address bounds, word alignment, stack floor, resolved dynamic storage slot in address and digest. Check first that the compiler emits aligned addresses and non-negative slots; stop if not. (coder-deep, Opus)
7. R5: initial image lookup instead of the linear folds; update `cross_table_checks` gate columns without weakening it. (coder-deep, Opus)
8. R6: computed trace length, exit code and gas bound. The consumer must stop reporting `final_state_root` as proof-verified: this changes meaning, ask the owner before doing it. (coder, Opus)
9. R7: Poseidon chain for `event_digest`. (coder, Opus)
10. R8: PROOF_FORMAT_VERSION to 2, ARCHITECTURE table, four gates. (coder-lite, Opus reviews the whole R diff)
11. A1 VerifyMerkle sponge node hash and window (reads tid 1, aligned); 12. A2 AIR and prover for A1 (no second version bump); 13. A3 BudL builtin; 14. B2 HashMem 0x23 (needs R4b); 15. A4 storage proofs checkable end to end (tid 3 init through the R5 image); 16. B3 bind MLP guest; 17. AIR audit of remaining opcode groups.
Gates touched: air-selectors-are-opcode-bound (B1, B2), logup-multipliers-are-boolean (R1, R2, R5), cross-table-checks-use-last-row (R5), every-opcode-has-a-forgery-test (R1, R2, R4b, R6, R7, B2).

### 4.4 Tooling and platform

- CI repair pass (see section 3). Then ADIM 9: move inline shell in CI YAML into Rust xtask commands. ADIM 10: bind the Solidity verifier to Rust-generated ABI and test vectors.

## 5. Owner decisions on record

- B1 A: all B.U.D. storage writes become signed in-block transactions. B2 A: a failed coding audit records the failure and applies the 6 hour cooldown, no slash. Z1 A: VerifyMerkle gate stays closed.
- D1 A operator consent in the transaction; D2 consent digest binds chain id, payer and nonce; D3 A protocol constant minimum bond; D4 A escrow on `held_bytes`; D5 any payer may fund; D6 A OpenDeal refused on mainnet until payouts and challenges run in block; F3 C Generated sources refused in block for now.
- Language rule: Rust and BudL only (Solidity stays; budscan Firefox layer replaced later by a Rust engine shell). budscan: owner said it must not change for now (paused). Plan kept: Blitz recommended, decisions on engine and MPL-2.0 license scope still open.
- K1 decisions (owner approved all recommendations): (1) zero-byte content gets an explicit empty marker in the payload format so every content class converts; (2) content above the ceiling is segmented, each segment is its own QR video and the recipe lists the segments; (3) the PNG-sequence BDLV container is the canonical carrier and real transport (resize, JPEG and lossy re-encoding, frame loss) is simulated in Rust, no real video codec; (4) the validator re-checks the canonical recipe and the commitment bindings in the block (consensus surface: separate design note, separate PR), the byte-level roundtrip is verified by the uploading client and the reader, with the operator challenge as the third point. This deviates from directive section 1.3.5 and must be reported as a deviation.
- K1 decisions confirmed by the owner in plain words (A, A, A, A). Scope rule (owner): do not go beyond these decisions; ask only about serious questions that would fundamentally change the directive.
- BUD 3.0 as the owner states it: content is held only as a recipe. A recipe becomes a QR video and then the content (forward). On upload the content becomes a QR video and then a recipe on the receiving side (reverse). The process continues that way.
- Owner correction on 3.0: there is no split between generated and organic content in the plan. Every content has a recipe, and the recipe type comes from the existing QR video system (directive 1.1.5). Do not design around such a split and do not ask about it again.
- BUD 2.0 as the owner states it: the aim is the very low storage cost of 0.016 USD per TB per month. Measure it per content mix as the directive says.
- Tools (owner approved): install the cloudflare security-audit skill at project scope. The first audit run on `src/registry/` needs an owner-opened session (`claude --model opus --effort xhigh`, directive KURULUM 6.3.3), so it is not run from the main session. Not approved: autoharness (source review and a separate decision first).
- Security-audit skill (installed locally, not committed): the vendored cloudflare skill contains JavaScript validators and em dashes, which conflict with the Rust and BudL only rule and with the no-unicode-dashes gate. Decision by Claude: keep it out of the repo (listed in .git/info/exclude) and install it per session with `npx skills add https://github.com/cloudflare/security-audit-skill --skill security-audit --agent claude-code` (project scope, never --global). Safety review passed: no hooks, MCP, settings changes or load-time network; the full audit needs an OS sandbox for target code and stays `needs_validation` without one; output goes outside the repo by default. The first audit run on `src/registry/` needs the owner-opened session.
- Security channel 1 C; baseline edits approved; mainnet gate for B.U.D. opened by Claude when ready; continue in new sessions.
- Standing rule: ask the owner only for hard architecture changes; record decisions made by Claude here.

## 6. Next step (do this first in a new session)

### 6.0 State at the end of the latest session (written 2026-10-08, read this first)

Working directory note: the shell may be inside budzero/; run git from /home/user/budlum.

Done, pushed, local targeted tests passed (SHA, subject): d903aa9 E4-a global header time from the tip; 5f2d909 ADIM 3a block entropy; 990fdcd B1 opcode 0x1F closed; 7bcff68 K1-VI-T tests; 3dcbe16, 574a6fa, a56d04a ADIM 4a audit state with its two fix rounds; 2babb74, 0fc7f90, 99ba45b R1; a59afac R2; 311bf03 R2b; ed5624e R3; dcb2efb R3b; 78212be R4a-1; f7618dd R4a-1b test; 1c3bd7f, 834607e, ede96dd, ff1d03f K1-01 series (empty content end to end); 170661e K1-02; c0ad763 K1-03; 4db95e8 K1-04; 5e64e53 K1-05; 48a8be4 K1-08a; 362e716 K1-08b; 1bac039 K1-09. Opus verification passed for: E4-a, 3a, B1, 4a-fix2 (conditional), R2, R3, R3b and R4a-1 (conditional, test added in f7618dd), K1-01 series, K1-04 (library only, see 6.5j), K1-05. Not yet Opus verified: R1 (not required by the plan), K1-02, K1-03, K1-08a, K1-08b, K1-09.

Stopped mid-work at the owner's order (limit nearly out): zkVM R4a-2 (memory table order and 32 bit address range, design option B). The last commit labelled wip holds the partial edits in budzero/bud-proof (plonky3_air.rs, plonky3_prover.rs, trace_layout_tests.rs) and budzero/bud-vm/src/lib.rs. `cargo check -p bud-proof -p bud-vm --tests` passes, but its tests were never run and the agent stopped while editing test helpers. Next session: run `cd budzero && cargo test -p bud-proof --lib memory`, finish the step per the handoff in 6.5i (red forgery tests first, one mutation per rule, then a fresh Opus verification), or revert the wip commit if it is not close. Chain ADIM 1 and 2 (shared replay function and PoA stamp) were launched and stopped before any edit: nothing to clean up.

Open owner questions, all parked (ask in plain words): (1) deleting a stored file while the network checks its keeper (G1-P, 6.5g); (2) bridge messages and bridge state change outside blocks yet sit in the state root, so nodes can disagree: remove them from the root, or turn each into a signed in-block transaction (recommended); (3) K1 reader wiring, sealed content bound to its plain text, where to keep verification records (6.5j); (4) F-DET-1 sealed frames differ per call (6.5e); (5) S1 to S3 challenge proof and penalty (6.6); (6) where unfixed security findings are kept between sessions (suggested: the private channel of docs/SECURITY.md).

Security findings: a Haiku sweep and an Opus finder scan were started for the whole repository. The finder scan of src/chain is done: 19 findings (3 critical: slashing outside blocks driven by invalid votes and by gossiped proofs, and live versus replay state roots diverging after about 1010 blocks). The architect verified the replay-root cluster as true (CH-3, CH-5a, CH-5b, CH-6, CH-7) and wrote five handoffs (shared end-of-block state function pair; PoA stamp; unfreeze queue and AI pruning as an end-of-block hook; reorg must not overwrite state from disk). Details are NOT in this repository by rule (MODEL_ROUTING 12.10). They lived in the session scratchpad and are lost when the container goes away. To regenerate: run the finder (Opus xhigh) on src/chain again, then the architect verification for the replay-root cluster, with the mechanism choice already made: one shared function pair used by the live path and all five replay entries (option A); do not move end-of-block hooks to the start of a block. Haiku sweep: spec is summarized in 6.5k; group 1 (src/chain) is done, groups 2 to 9 are not.

Next steps in order: (a) finish or revert R4a-2, Opus verify; (b) chain replay-root steps 1 to 5 (coder-deep, Opus verify each) after the owner answers question 2 only for the message and bridge part, steps 1 to 5 do not wait for it; (c) finder scans: consensus and crypto, settlement and cross_domain and registry, network, tokenomics and execution, bud-proof, rpc and storage; (d) K1: K1-07 segmentation, K1-06 transport simulation, a resource budget before decode (V4), K1-10 fuzz, K1-12 determinism goldens, K1-11 test suite; (e) B.U.D. 4b and 4c wiring after the owner answers question 1; 3b to 3e wait for S1 to S3; (f) end of round: read CI (6.8) and fix with coders.

Rules that held this session: main session writes no code; at most 3 agents in parallel; every R3 change gets a fresh Opus verification; forgery test red first and one mutation per rule; scout output is a lead, not evidence; commit messages neutral for security fixes.


Written 2026-10-08. Work branch of this session: `ccr-9d3ed79c-9wpla0` (cut from `claude/zkvm-bud-completion-84r6jc` at 8bb1962; PR #1 still tracks the older branch name). Another branch `wip/zkvm-b1-reserve-0x1f` exists; its single commit was cherry-picked here (cf35384). Push only builds that pass their targeted tests.

### 6.1 Owner instructions of this session (binding)

- Do not take economic decisions in this session. Do not ask economic questions. Steps that depend on one stay unstarted and are listed in 6.5.
- Priority is a full audit and improvement of Budlum, module by module, then coded fixes. Plans alone are not progress: the branch had 4865 added product lines against main (about 4795 more in xtask tools) and 15 of 31 commits were status-only. Prefer code with tests.
- CI is read at the end of the round, not between steps.
- Ask the owner only in plain, short sentences with a recommendation.
- BUD-AI-KAPSAMLI-DIREKTIF.md binds all B.U.D. work (the uploaded copy is byte-identical to the repo copy). Owner: nothing coded may stay unwired (every component needs a production call site, see directive 2.2.3 and gate definition 11); B.U.D. 3.0 (recipe NFT, two-way recipe and QR video conversion, owner-or-DAO burn) is a new design with no precedent, so do not judge or shape it by existing systems or habits. The directive wins over general working habits where they differ; report the conflict. Priority Zero (directive 1) outranks every other B.U.D. step: K1 must finish before K2 to K4 code is merged.

### 6.2 Live facts (evidence, read 2026-10-08)

- PR #1 head 8bb1962, CI run number 12. Red: Budlum Core (steps "Feature matrix: pq-ml-dsa solo" and "Clippy"; Test, Format, doc pass), Typos and Repo Lint (typos: `flate` twice, `tru` at xtask/tools/src/json.rs:342), docker-smoke (step "Trivy IMAGE gate", fixable CRITICAL/HIGH), Dependency Review (fails in about 1 second; likely a repository setting, check before coding). Cancelled at the 6 hour limit: Gates, StorageProvider Gate, BNS Name Registry, Fork-Choice Invariants, Coverage. Green: determinism on 3 OS, fuzz quick, PoA isolation, B.U.D. E2E, economy, governance, network hardening, BudZero.
- Branch against main: 96 files, +11868 -2307. Rust +9660 -415 (product code in src, budzero, crates, bud: +4865 -394; xtask: +4795 -21).
- memanto is not installed in the cloud container; this file is the memory.

### 6.3 Agents running at the end of this note (check git log before redoing anything)

- coder-deep, zkVM B1: finish AIR part of 0x1F fail closed (4.3 step 1). Files: plonky3_air.rs, plonky3_prover.rs, bud-isa, docs/AI_VERIFICATION_STATUS.md, ai_verification_status_locks.rs.
- coder, ADIM E4-a: GlobalBlockHeader.timestamp_ms taken from the chain tip block (blockchain.rs about line 1917) with a test.
- coder-deep, ADIM 3a: `AccountState.current_block_entropy` (hash of previous_hash and vrf_output, tag BDLM_BLOCK_CONTEXT_ENTROPY_V1), not in state root, not persisted.
- Each pushes its own commit. If the tree is dirty at start, read `git status` and `git diff --stat` before acting. Do not commit another agent's half work.

### 6.4 Mainnet blocker matrix (architect, evidence-based, HEAD 8bb1962; classes only, no exploit detail)

Wired, tested: tokenomics (100M fixed supply, burns), settlement flow, cross-domain nonce store, PoW and PoS engines, wallet-core, genesis validation, snapshot v2 write path.
Not wired or closed:
1. Mainnet default PoS validator exits at start (PKCS#11 VRF backend missing; main.rs about 726). Economic or architecture: owner.
2. Three of four mainnet domains cannot finalize (validator_set_hash stays zero for PoS and BFT; PoA domain starts Frozen). BFT domain plugin branch never runs.
3. Consensus state written outside blocks: external root anchoring and domain advance (RPC and gossip), message_registry insert, operator RPC stake bonding, bud_submitZkProof fee cut, storage maintenance (challenges, deadlines, audits).
4. GlobalBlockHeader is sealed only by an operator RPC; timestamp_ms was the header count (E4-a fixes it); the GlobalHeader gossip receiver does not validate.
5. Snapshot manifest signing has no production caller; mainnet remote snapshot sync is refused (safe close, missing feature; pruning is forbidden on mainnet v1 so restart is not affected).
6. config/mainnet-genesis.json is a template (validators=0): operational.
7. PQ anchor and cold wallet are gated off.
8. DomainForkChoice has no production user.
9. bridge_relayer, network egress privacy and identity resolver are not wired. Account abstraction registry, TEE attestation and private transfer auth are not wired.
10. Validator reward pool is intentionally unwired; block_reward does not mint: economic, owner.
Also: storage challenge proof check is a constant false (proof format needs zkVM queue item 15); coding audit is not bound to shard commitments (format change); VerifyMerkle and VerifyInference are closed by activation; privacy is closed on mainnet; AI inference structural path is open for non-proof models; crypto::mainnet_policy test names and a dedicated settlement and cross-domain CI gate were not verified (KANITSIZ).

### 6.5 Handoffs ready (architect, B.U.D. in-block queue; order 3a, 3b, 3c, 3d, 3e; 4a parallel to 3a; 4b needs 3a and 4a; 4c needs 4a)

- 3a block entropy: running (see 6.3).
- 3b `StorageTx::OpenChallenge` (tag 4; opener is tx.from; bond debited after registry accepts; entropy from current_block_entropy, sender, nonce). Ready after 3a. coder-deep, Opus verify.
- 3c `StorageTx::AnswerChallenge` (tag 5; refund opener bond on Answered or Mismatched; Mismatched starts operator cooldown). BLOCKED on owner decision S3 (is the penalty the bond only, or bond plus the same amount again). Do not start.
- 3d finalize missed challenges inside apply_block_effects (replay equals live root). BLOCKED on S3 (same).
- 3e protocol challenge issuance inside apply_block_effects. Depends on 3a and 3d. BLOCKED through 3d.
- 4a stored coding audit state in the registry (StoredCodingAudit, outcome enum, open, answer, finalize; failure means cooldown only, no slash per owner B2 A; new fields serde default and in root()). Ready now. coder-deep, Opus verify.
- 4b open and finalize audits in apply_block_effects on the first block of each epoch (replaces the discard loop in chain_actor.rs about 3141-3185). Needs 3a and 4a.
- 4c `StorageTx::AnswerCodingAudit` (tag 6). Needs 4a. coder.
- Findings from the architect to keep: maintenance work outside apply_block_effects makes replayed registry roots differ (F3); a failed operator is charged the bond at open and again at slash (F4, owner question S3); opener bond is never refunded on Answered, Mismatched or Missed (F5); auto challenges use the zero address as opener with an undebited bond of 1, so a refund rule must skip the zero address (F6).
- Entropy and deadline choices made by Claude: entropy from the including block's previous_hash and vrf_output; coding audit deadline reuses the 10 epoch window.
- Handoff E4-a done above. Not started (questions open, not economic): S1 mainnet snapshot sync need (recommend: not needed in v1; remove from blocker list as v2 feature), S2 automatic global header sealing policy (recommend: seal at finality checkpoints, deterministic; gossip validation later), S3b DomainForkChoice wiring versus deletion (recommend: wire it).

### 6.5b K1 Priority Zero plan (architect, evidence read at HEAD 5f2d909; src/storage is R2, src/gateway is R1, so agent is coder, not coder-deep)

Waves (steps in one wave touch different files and may run in parallel; W3 steps both edit storage/mod.rs, use stacked branches):
- W1: K1-01 explicit empty marker (A0, A1, emit; flag bit1 EMPTY; golden vectors of non-empty content unchanged) | K1-02 PNG deflate fails closed (qr_png.rs:140-145 silent stored fallback removed) | K1-03 tolerant PNG reader (qr_video.rs:319-321 filters 1 to 4, colour types 0/2/4/6 at 8 bit, chunk CRC) | K1-08a reveal checks stream id at open (three_reveal.rs:75-90) | K1-08b gateway stored paths rehash against ContentId (gateway/service.rs) | K1-09 bud-node ContentId parity with hash_fields_bytes (budzero/bud-node/src/store.rs; do not touch bud/ SHA3, owner question).
- W2: K1-04 verify_qr_video with ExpectedCommitments, VerifyError, VerificationRecord (new storage/qr_verify.rs; emit.rs:909 calls it; record is node-local, emitted via tracing). Needs 01.
- W3: K1-05 independent verifier decoder, verifier only (new qr_verify_indep.rs; header layouts from spec; no imports of qr_frame, qr_carousel, qr_receive, qr_payload parsers) | K1-07 segmentation (new qr_segment.rs, local tag BDLM_THREE_SEGMENTS_V1; emit plan segments instead of refusing). Both need 04.
- W4: K1-06 transport simulation inside verify (new qr_transport.rs: resize, 8x8 quantization, bounded noise, frame drop, duplicate, reorder, grey re-encode; integer math; SHA-256 counter PRNG). Needs 02, 03, 04.
- W5: K1-10 fuzz targets (fuzz/fuzz_targets/qr_*.rs) | K1-12 cross-platform determinism goldens (tests named qr_determinism_* so determinism.yml:129 runs them).
- W6: K1-11 Priority Zero test suite (new src/tests/qr_priority_zero.rs: class matrix, boundaries 0/1/block multiples/MAX_K/max/max+1, proptest roundtrip, lossy transport, differential, negative).
- Opus verify: K1-01, 04, 05, 06, 07, 11. Not needed: 02, 03, 08a, 08b, 09, 10, 12.
- Consensus surface, no handoff, separate design note and separate PR each: DN-1 docs/bud/K1-DN1-validator-recipe-binding.md; DN-2 docs/bud/K1-DN2-operator-challenge-third-point.md (touches storage_deal.rs).
- Deviations to report: (a) directive 1.3.5, validator does not check bytes (owner decision K1-4); (b) directive 1.3.4, symbol layer stays shared (rqrr), the verifier is independent only at A1 to A3, a second QR reader would be a new decoder (forbidden by 1.1.3); (c) no production client call site yet, today the client point is the bud_storageQrFeedPreview library path, wiring into wallet-core is an architecture question; (d) audit records are node-local (open question 6 unanswered); (e) RPC body cap 1 MiB stays, segmentation is reached above 819,200 bytes; (f) if an xtask gate needs every domain tag in domain_tags.rs, K1-07 must stop and ask.
- Status at time of writing: K1-01 started (coder). Others not started.

### 6.5h zkVM queue state (after R3, Opus verification PASS)

- Done and verified: B1 (990fdcd), K1-VI-T (7bcff68), R1 (2babb74, 0fc7f90, 99ba45b), R2 (a59afac, Opus PASS), R2b (311bf03), R3 (ed5624e, Opus PASS). R3: register table active flag is boolean and a prefix; strict (index, time) order with 32 bit step columns 754..786 (TRACE_WIDTH 786); old 754 wide proofs are refused by the width check; PROOF_FORMAT_VERSION still 1 (bump in R8). bud-proof lib 201 passed / 0 failed; four gates green (33 opcodes, 73 forgery tests); prove run about 345 s wall for the lib suite.
- New findings from the R3 verification (classes only): (F1) the binding of the initial register image to the committed state is not tied to the active flag or to the first row of a register block (older than R3); (F2) the memory table has no active flag, prefix rule, inverse witness for same-address or order; addresses reach about 2^60 so a 32 bit step is not enough, a range design is needed (this is queue step R4a); (F3) a test where a register's events are split in two blocks is missing.
- Next: architect plan started for R3b (F1 and F3) and R4a (memory table, address range design). Then R4b, R5 to R8 as in 4.3. Public-channel rule: commit messages stay neutral; no exploit steps in the repository (MODEL_ROUTING 12.10).

### 6.5i zkVM next steps (architect plan, code read; R3b and R4a-1 started)

- Order, all in plonky3_air.rs so one coder-deep at a time: R3b (register init flag tied to the active flag and block start, plus split-block regression test; width stays 786), R4a-1 (memory table active flag, prefix, same-address inverse witness, init binding; COL_MEM_SAME_INV 786, width 787), R4a-2 (memory order and address range), R4b, R5 to R8 as in 4.3. Each step: forgery test red first, one mutation per rule, Opus verification.
- Design decision recorded (not economic): option B for the memory address space. It is the design already accepted in 4.3: table ids 1 memory, 2 stack, 3 storage on the bus (today the bus uses table id 1 for all three and only address bases separate the regions), address 32 bit inside a table id, VM memory limited to 2^32 bytes (larger is refused in the constructor), key = table_id * 2^32 + address, order step 34 bit, width about 854. Option A (keep the 2^60 bases, 62 bit address and step columns, about 124 columns, width about 911) was not chosen. R4a-2 stops and asks if the compiler emits a negative static slot (codegen.rs about 243 and 673).
- R4b note: dynamic SRead and SWrite use imm -1 as the CPU address while the table holds the resolved slot (prover about 283-303), so dynamic storage may not be provable today; the first R4b test must show it.
- R5 note: the init fold uses fixed constants (documented in plonky3_air.rs about 108-117 and 413-417) until R5 replaces it with a lookup against the image; consider moving R5 earlier.

### 6.5d B.U.D. audit state (4a verified by Opus, conditional pass)

- 4a done (3dcbe16): StoredCodingAudit, outcomes, open, answer, finalize in storage_deal.rs; 16 tests. Persistence note: new registry fields change the bincode row; an old sled row makes the node exit at start with a CRITICAL log (fail closed, blockchain.rs about 817-826). Nothing is released, so accepted; release notes must say resync.
- 4a-fix started (coder-deep): at most one open audit per deal; new outcome Void (deal not Active or manifest gone: closed without cooldown); finalized audits pruned by an epoch queue after a retention window; WIRING labels aligned (4b = open and finalize in apply_block_effects at epoch start; 4c = StorageTx::AnswerCodingAudit); golden root test for an empty audit set and a wrong-length column test.
- Binding rules for 4b and 4c (F5): now_secs comes from the block timestamp, the responder is the transaction signer, the entropy must not be choosable by the block proposer (grind), and 4c removes the old ChainCommand::AnswerCodingAudit that takes a caller built audit. Open risk: audits are not bound to column commitments (S2, owner parked).
- no-idle-code gate items remain (see 6.8 item 4b). dead_pub_api accepts a `WIRING:` doc line within 14 lines above an item; use it only for a step that really lands next.

### 6.5c zkVM queue state (architect verified claims at 7bcff68 by reading code)

- Done and Opus verified: B1 (990fdcd), K1-VI-T tests (7bcff68; mutation shows the IS_EXPAND constraint hides a Halt row from Program CTL if removed, so it is a real security constraint).
- R1 started (coder): Store through r0 makes a memory demand (`is_real_mem_op = is_load * rs1_idx_z + is_store`), VM immediates canonical (P-|imm|), Eq/Neq/SumConservation inverse computed in the field. No Opus.
- R2 next (coder, Opus verify): register bus write only for the 23 opcodes that write rd; others read rd at its current value; prover register_events fixed for Push/Call/Ret (the VM does not write there). Forgery tests: Store with rd=5 and Push with rd=5 writing a free value. Mutation: replace writes_rd with constant one, both forgery tests must go red.
- R3 after R2 (coder-deep, Opus verify): r_active boolean and prefix ((1-r_active)*nr_active = 0), strict (idx, clk*4+sub_clk) order with 32 bit decomposition columns 754..786 (TRACE_WIDTH 786). Tests: rejects_register_read_after_inactive_gap, rejects_reordered_register_writes. Two mutation checks so the tests do not mask each other.
- Observation for R6: VM Syscall sender and nonce values may be >= P (bud-vm lib.rs about 517-519).
- Known leftover from B1 review: stale comments about VerifyInference expansion in plonky3_air.rs (about 223, 2391-2398, 2617-2621) and plonky3_prover.rs (about 108-112, 1543), dead filler code in the prover (about 685-700, 850-866, 1042-1066, 1335), docs/ARCHITECTURE.md line about 714, VM gas_cost still 10 for VerifyInference, columns 691-693 unconstrained (harmless). Do as a mechanical coder-lite step after R3. locks test (ai_verification_status_locks.rs) only pins the VM arm text; add an AIR assert lock (T3).

### 6.5f Audit binding rules (Opus verification of 574a6fa, FAIL with small fixes; fix step 4a-fix2 started)

- Findings being fixed in 4a-fix2: G1 predictable or collusive Void (deal expiring inside the audit window, or prune_content) escapes the cooldown; G2 busy selected deal rejects the whole open instead of picking another holder; G3 queue root encoding is not injective (add a length prefix); G4 weak tests.
- G5 (existing, out of scope): ChainCommand::StoragePrune mutates consensus state outside a block (chain_actor.rs about 3443-3451) and now also influences Void or Missed. Needs its own step (ADIM 6 family).
- Binding rules for 4b and 4c: (1) sweep uses the constant CODING_AUDIT_RECORD_RETENTION_EPOCHS, never a config or RPC value; (2) open, finalize and sweep run only inside apply_block_effects, at epoch start, in fixed order: finalize audits, then expire, prune and slash, then open, then sweep; (3) the number of audits opened per epoch is bounded by a constant; (4) in 4c the epoch and now_secs come from the block context, never from the transaction payload; the responder is the transaction signer; an unknown audit id is a refusal; (5) 4c removes the old ChainCommand::AnswerCodingAudit path (chain_actor.rs about 1291).

### 6.5g Audit state after 4a-fix2 (a56d04a, Opus verification: conditional pass)

- Closed: G2 (selection filters first, picks among free holders), G3 (queue root encoding is injective: epoch, then length, then ids; empty queue adds no bytes; golden roots unchanged), G4 (tests, four mutations go red). Retention window is the constant; the helper with a window argument is private and used by the public wrapper.
- Open, owner decision parked (touches the delete lifecycle, directive 8.5): G1-P. Burning the NFT or pruning content while an audit is open turns the deal Expired and removes the manifest, so the audit closes as Void with no cooldown. Cost to the attacker: the NFT and the content. Gain: skipping a 6 hour cooldown (bond untouched). Options: A) prune delays closing of deals with an open audit until deadline plus one (closes it fully, recommended by the verifier; the delay is at most the audit window); B) if the burner equals the audited operator, close as Missed (cheap, open to two cooperating addresses). Decide before 4b, because 4b wires Void live. Plain-language question for the owner: when someone deletes a stored file while the network is checking the keeper of that file, should the deletion wait until the check ends? Recommendation: yes (option A).
- Extra binding rules for 4b: (a) in the same epoch step, finalize_missed_coding_audits runs before finalize_expired_storage_deals (blockchain.rs about 6708); test the deal_end = deadline plus one boundary; (b) order: finalize, sweep, open; (c) open errors must not stop block application, they are swallowed deterministically; (d) the entropy is the consensus block entropy (block_context_entropy), not a caller value; (e) the G1-P decision is applied.
- Hygiene: the counter and audit records are folded into the root without a field tag (G5 low); add a tag when the counter is above zero (no golden impact).
- Unwired until 4b and 4c: open, finalize and sweep have no production call site. This is a mainnet blocker.
- Out of scope, separate scan needed (O1): ChainCommand::StoragePrune mutates live state outside a block (chain_actor.rs about 3443-3451).

### 6.5j K1 progress and the honest gap list (Opus verification of K1-04, 4db95e8)

- In the branch and verified or tested: K1-01 series (1c3bd7f, 834607e, ede96dd, ff1d03f: empty content through A0, A1, plan, seal and RPC), K1-02 (170661e: no silent stored-zlib fallback), K1-03 (c0ad763: PNG filters 1 to 4, colour types 0/2/4/6, chunk CRC; grey value is taken from the first channel, check luma on real re-encodes in K1-06), K1-04 (4db95e8: verify_qr_video with ExpectedCommitments; library correct and fail closed), K1-08a (48a8be4: reveal checks the stream id at open; recipes pinned with another repair rate are now refused, none exist because nothing is released), K1-08b (362e716: gateway checks ContentId on all three stored paths).
- K1-04 does not close directive 1.3.5. The only production call (emit.rs about 930-939) compares the encode output with itself. Gaps: V1 reader point is UNWIRED (no production call that verifies a video against the on-chain manifest commitments; the client side only tests its own encode; a real client call site is an architecture question, wallet-core has no storage dependency); V2 for sealed content the ContentId is of the ciphertext body (optional plaintext expectation for key holders; architecture decision); V3 ExpectedCommitments lacks the payload kind (small fix); V4 no resource budget for untrusted video input before decode (blob size and total work cap) and no fuzz target, must close before the reader is wired (K1-10); V5 no persistent append-only verification record, rejections are not recorded (storage place is a design question); V6 no independent decoder (K1-05).
- Next K1 steps: K1-05 (independent verifier plus V3), K1-07 (segmentation, stacked after K1-05 because both edit storage/mod.rs), then K1-06 (transport simulation), a resource budget step (V4), K1-10 fuzz, K1-12 determinism goldens, K1-11 test suite. Owner decisions parked: V1 reader wiring, V2 plaintext expectation, V5 record storage, F-DET-1.

### 6.5k Whole-repo sweep with the cheap model (owner request, deviation from MODEL_ROUTING section 1 noted)

- Owner asked for the whole of Budlum to be checked by the Haiku model (scout). MODEL_ROUTING says scout does not decide and its output is not evidence. So the sweep is a mechanical net only: every result is a lead (path:line with the exact source line), and a lead becomes a finding only after an Opus agent or the main session has read the code. Do not report a lead as a finding.
- Twelve checks per module: C1 unwrap, expect, panic, todo in non-test code; C2 narrowing casts and unchecked integer maths on amount, fee, nonce, height, epoch, len, size, balance, stake, bond, timestamp; C3 floats on consensus-reachable paths; C4 HashMap or HashSet iteration feeding a hash, root or order; C5 clock and randomness in consensus files; C6 unsafe, allow, ignore, placeholder words; C7 unwrap_or_default and unwrap_or hiding a failure; C8 public functions with no caller outside tests (UNWIRED leads); C9 unbounded growth from network input; C10 process, file and network calls outside designated modules, secret-looking strings; C11 SAFETY, TODO, HACK comments; C12 every mainnet gate.
- Nine batches in order: (1) src/chain; (2) consensus, crypto, mempool, light_client, privacy; (3) settlement, cross_domain, registry, relayer; (4) network, rpc, cli, bin, main.rs, gateway; (5) core, execution, tokenomics, account_abstraction, bns, socialfi, pollen, deed, sharding, prover, sdk, budlumxyz; (6) storage, domain; (7) ai, ai_inference, crates/ai-inference; (8) budzero crates; (9) crates (wallet-core, bpqs, note-packing, budscan) and bud/. src/tests and budzero/target are skipped. Each scout returns at most 120 lines and says what it did not cover.
- Lead results are kept outside the repository (security rule, MODEL_ROUTING 12.10); only closed items are written here.
- Opus finder scans run in parallel on the highest-risk modules; first one (src/chain) started in this session.

### 6.5e K1 state

- K1-01 (1c3bd7f): A0, A1, plan and emit accept empty content (flag bit1 EMPTY). Not yet complete: the sealed RPC path still refuses empty plaintext (payload_crypt.rs SealError::Empty). K1-01b started (coder-deep): seal accepts empty plaintext through the AEAD (Claude decision: option A, because skipping the seal would open a bypass in the UnsealedGated visibility gate). Opus verify needed after. Mechanical follow-ups: update docs/bud/BUD-KESIF-RAPORU.md line about 98 (coder-lite); transformed.rs from_bytes still refuses empty and has only test callers (small separate step, coder).
- F-DET-1 (owner decision parked, crypto architecture): on the sealed path, the same content with the same seal seed gives a different nonce, recipe commit and frames on every call (payload_crypt.rs about 141-144, three_pipe.rs about 171). Directive 1.3.6 says the same content gives the same recipe and frame set. Options: (a) deterministic misuse-resistant nonce (for example AES-GCM-SIV, or an XChaCha nonce derived from key and plaintext); (b) 1.3.6 excludes sealed feeds. Existed before; not empty-specific.
- Tier note: payload_crypt.rs is under src/storage (R2 in MODEL_ROUTING section 3) but handled as R3 because of its crypto meaning; propose the tier change to the owner.

### 6.6 Open owner questions (do not decide; ask later in plain words)

S1 challenge proof check can follow in the zkVM queue (recommend A: move challenges into blocks now, mainnet gate stays closed). S2 parity audit fingerprints (recommend A: audit now, fingerprints later; changes manifest format). S3 penalty is bond only (recommend A). Plus items 1 and 10 of 6.4. All are economic or architecture and are parked by owner order.

### 6.7 Audit plan (the main work of the next rounds)

Rule set: MODEL_ROUTING sections 2, 4, 5. finder (Opus xhigh) hunts findings per module, read-only; it never runs in parallel with finder-max. Each finding gets its own handoff from architect, a coder or coder-deep fixes only that finding, a fresh architect call verifies the diff. The finder does not verify its own fix. At most 3 agents at once. Local checks: cargo check, fmt, filtered tests, clippy on touched crates; full tests only in CI.
Order of finder scans (R3 first scan each; later scans use architect on `git diff` only):
1. src/chain: blockchain.rs, snapshot, persistence, fork choice, chain_actor.
2. src/consensus and src/crypto (signing, verification, key policy, PKCS#11).
3. settlement, cross_domain, registry.
4. network/node.rs (gossip validation, scoring, DoS limits, the unvalidated GlobalHeader receiver).
5. tokenomics and execution/executor.rs (every TransactionType arm, fee and nonce on refusal).
6. budzero/bud-proof (continue 4.3 queue R1 to R8; the AIR bus and ordering gaps).
7. rpc and B.U.D. storage (all RPC mutation paths, mainnet gates).
8. wallet-core against node verify (cross test missing), account_abstraction, ai_inference.
Record each finding id, path:line, impact in a private note outside the repo; never in a public issue, PR or this file (MODEL_ROUTING section 12 item 10). Record only closed items here.
Expected output per module: findings list, fixed items with commit SHA, remaining engine blockers, tests added per file.

### 6.8 CI repair list (do at the end of a round, one pass with coders)

1. Budlum Core: read the failing step logs for "Feature matrix: pq-ml-dsa solo" and "Clippy" (the Format step is green and `cargo fmt --all -- --check` passes locally).
2. Typos: `flate` (crate name flate2 in comments) and `tru` (test string); fix by renaming where possible or a narrow typos config entry. No weakening.
3. docker-smoke: Trivy gate; find the fixable CRITICAL or HIGH package and update the dependency.
4. Dependency Review: check repository dependency graph setting first.
4b. no-idle-code gate is red at HEAD 3dcbe16 on three items not from the latest steps: verify_canonical_program (budzero/bud-proof/src/plonky3_prover.rs), open_deal_consent_digest and StorageDealOpen (src/domain/storage_tx.rs). Wire them or remove them; do not touch the baseline file without the owner.
5. Five cancelled jobs reached the 6 hour limit: find the hang (likely a test that waits forever) and fix it; do not raise timeouts.
6. Then rewrite sections 1 to 6.

## Official Anthropic skills (github.com/anthropics/skills, looked at, none installed)

Install: `/plugin marketplace add anthropics/skills`, then `/plugin install document-skills@anthropic-agent-skills` or `/plugin install example-skills@anthropic-agent-skills`. Skills in the repository: academy-guide, algorithmic-art, brand-guidelines, canvas-design, claude-api, discernment-nudge, doc-coauthoring, docx, frontend-design, internal-comms, mcp-builder, pdf, pptx, skill-creator, slack-gif-creator, theme-factory, web-artifacts-builder, webapp-testing, xlsx.

Useful for this work: doc-coauthoring (design notes and the K0 to K6 reports), skill-creator (own project skills), frontend-design and webapp-testing (later, for the wallet and budscan shell). Not relevant to the Rust core: the rest. Read a skill's source before installing it.

## EFFORT LOG

(no finder-max calls)
