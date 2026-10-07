# STATUS

Read this file first in every session. It is the technical state of the work: what exists, what was decided, what is in flight and what to do next. Durable rules live in CLAUDE.md, MODEL_ROUTING.md and BUD-AI-KAPSAMLI-DIREKTIF.md.

## Maintenance rule (owner)

The main session rewrites sections 1 to 6 of this file at the end of every work round and before any session ends. A new session must be able to continue from this file alone. Write facts with commit shas, file paths and gate names. Do not write exploit details of unfixed security findings here.

## 1. Goal and branch

- Goal: Budlum testnet ready, then mainnet. Every part works, is wired to a production call site and is tested. No dead or idle code.
- Branch: `claude/zkvm-bud-completion-84r6jc`. PR #1 against `main` on `lubothebook/budlum`.
- Directives: CLAUDE.md (purpose, invariants, loop), MODEL_ROUTING.md (agents, effort, reading rules), BUD-AI-KAPSAMLI-DIREKTIF.md (B.U.D. 1.0, 2.0, 3.0, gates K0 to K6). Setup steps are in MODEL_ROUTING_KURULUM.md.
- Project settings: `.claude/settings.json` (model opusplan, read deny for target/ and Cargo.lock) and seven agents in `.claude/agents/` (scout Haiku; finder, architect, finder-max Opus; coder, coder-deep, coder-lite Sonnet).
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

- Local checks used so far: `cargo check --lib --tests`, `cargo fmt --check`, clippy, the xtask gates (`cargo run -q --release --manifest-path xtask/gates/Cargo.toml -- <gate>`) and filtered tests. No full `cargo test` has been run locally.
- Opus verification passed with fixes applied for: ADIM 1 (manifest signing), ADIM 5 (operator class and policy), ADIM 2b (deal open and reallocation).
- Pending verification: ADIM 2c `OpenDeal` (e649082) and the zkVM commits b64163d, cf9a35f. An architect run for the zkVM commits was started; if it did not report, re-run it first.
- CI on GitHub (PR #1) was red on: Dependency Review, Typos, Repo Lint, Gates, Budlum Core, docker-smoke. Several workflows are `disabled_fork` and must be enabled by the owner in the Actions tab. Review the CI logs together at the end of the next round and fix in one pass. Expected causes to check first: Turkish text in `.claude/agents/*.md` against Typos, the new proto message against Repo Lint (buf), the tool ports against Gates, and whether the Rust workspace tests pass.
- Baseline edits approved by owner: four lines removed from `.github/idle-code-baseline.txt` (items gained production callers).

## 4. Work queues (in order)

### 4.1 BUD directive gates (current focus, see BUD-AI-KAPSAMLI-DIREKTIF.md)

- K0 discovery: in flight. `docs/bud/BUD-KESIF-RAPORU.md` is being written by a coder agent from scout output, with a gap table against Priority Zero. Scout facts so far (verify before relying): the QR video system exists (`src/storage/qr_*.rs`, `three_pipe.rs` `encode_qr_video` / `decode_qr_video`, BDLV container, fountain code, EC level L, 64 MiB payload cap, 50,000 frame cap); zero-byte content is refused (`PayloadError::Empty`, conflicts with the universality rule); no fuzz target for the QR decoder; no real-transport (re-encode, resize) simulation; no `StorageClaim`, `Heartbeat` or personal storage node types; no DAO authority over NFT burn; NFT burn prunes the manifest (`prune_content`); no perceptual fingerprint code; no cost measurement for the 0.016 USD/TB/month target.
- K1 Priority Zero: close the gap table (zero-byte content, fuzz target, lossy transport, differential second decoder, determinism across platforms, audit record, three verification points). Next step after K0: an architect plans K1 as small steps for coder agents.
- K2 BUD 1.0 (personal storage node, `StorageClaim`, heartbeat, challenge), K3 BUD 2.0 (compression pipeline, cost measurement, wire erasure, audit, repair, access counter design note), K4 BUD 3.0 (recipe NFT, lifetime pricing, owner-or-DAO burn, tombstone), K5 duplicate and spam research and interface, K6 AI audit pipeline. The directive says each gate ends with a report and an owner approval; the owner said to continue, so stop only at the directive's stop conditions.
- Open questions from the directive (section 15) must be resolved before constants are fixed; use configurable parameters until then.

### 4.2 B.U.D. in-block queue

- ADIM 3: retrieval challenge open and answer as in-block transactions.
- ADIM 4: stored coding audit with a deadline; a failure records and applies the 6 hour operator cooldown (owner decision B2 A, no bond slash). Today the maintenance sweep derives audits and discards them, and `AnswerCodingAudit` accepts a caller-built audit.
- ADIM 6: remove the RPC mutation paths; then open storage on mainnet (owner: Claude opens the gate when payouts, challenges and audits run in block).
- Low: `declare_operator_class` scans all deals; index by operator later.

### 4.3 BudZKVM queue (each step: a forgery test that verifies before the fix and is refused after; Opus verifies)

1. Memory and register argument (architect plan first): booleanity, prefix contiguity and boundary rules for the active flags; pin the memory same-address flag with an inverse witness; order the tables by (index, clock) with a range check or use timestamped offline memory checking; commit the initial image soundly; make `Store` with `rs1 = r0` create memory demand; bound addresses and the stack pointer; separate stack and storage tables; only opcodes that write `rd` write it on the register bus.
2. Run boundary: force `cpu_active = 0` on the last row; derive `exit_code`; trace length counter with a transition; prove or stop exposing `final_state_root` (consumer `src/execution/proof_verifier.rs`).
3. Dynamic storage slots (`imm = -1`) in the address and the digest; no i32 truncation in the VM.
4. Canonical u64 values in the VM; field subtraction in `EQ_DIFF_INV`.
5. `event_digest` as a Poseidon chain.
6. Verifier checks `gas_used <= gas_limit`.
7. Make VerifyInference (0x1F) fail closed.
8. VerifyMerkle node hash: two full Poseidon permutations (rate 4, capacity 4), leaf and node domain tags, 4-limb digests, 265-word window, 329 rows, gas 2075 (decided by Claude).
9. AIR and prover for 8; `PROOF_FORMAT_VERSION = 2`.
10. BudL builtin `verify_merkle_proof(window_const)`.
11. Storage challenge proofs checkable end to end; then `storage_challenge_proofs_are_checkable()` may return true.
12. HashMem opcode 0x23 (decided by Claude); 13. bind MLP guest weights, input and output with it.
14. AIR audit of the remaining opcode groups (Poseidon, privacy opcodes, syscalls, storage digest, call and return).

### 4.4 Tooling and platform

- CI repair pass (see section 3). Then ADIM 9: move inline shell in CI YAML into Rust xtask commands. ADIM 10: bind the Solidity verifier to Rust-generated ABI and test vectors.

## 5. Owner decisions on record

- B1 A: all B.U.D. storage writes become signed in-block transactions. B2 A: a failed coding audit records the failure and applies the 6 hour cooldown, no slash. Z1 A: VerifyMerkle gate stays closed.
- D1 A operator consent in the transaction; D2 consent digest binds chain id, payer and nonce; D3 A protocol constant minimum bond; D4 A escrow on `held_bytes`; D5 any payer may fund; D6 A OpenDeal refused on mainnet until payouts and challenges run in block; F3 C Generated sources refused in block for now.
- Language rule: Rust and BudL only (Solidity stays; budscan Firefox layer replaced later by a Rust engine shell). budscan: owner said it must not change for now (paused). Plan kept: Blitz recommended, decisions on engine and MPL-2.0 license scope still open.
- Security channel 1 C; baseline edits approved; mainnet gate for B.U.D. opened by Claude when ready; continue in new sessions.
- Standing rule: ask the owner only for hard architecture changes; record decisions made by Claude here.

## 6. Next step (do this first in a new session)

1. Read this file, run the environment check from MODEL_ROUTING.md section 10, compare with `git log` and `git status`.
2. If the K0 report, the directive-documents commit or the zkVM verification are not in the log, finish them (their agents may have been cut off).
3. Review the CI results of PR #1 together and fix the red checks in one pass (coder agents).
4. Plan K1 with an architect (Opus) from the K0 gap table, then run K1 steps with coder agents. In parallel keep the zkVM queue moving, starting with step 1 (architect plan, then coder-deep).
5. Rewrite sections 1 to 6 of this file before ending the session.

## EFFORT LOG

(no finder-max calls)
