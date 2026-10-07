# STATUS

Active work only. Durable decisions belong in memanto.

## Current goal

Complete B.U.D. and BudZKVM for mainnet. Branch `claude/zkvm-bud-completion-84r6jc`, PR #1 (draft).

## Owner decisions (2026-10-07)

- B1, storage writes: A. Move every B.U.D. storage write into signed in-block transactions (`TransactionType::Storage`). Mainnet gate is asked separately at the end.
- B2, coding audit failure cost: A. Record the failure and apply the 6 hour operator cooldown. No bond slash, because a column fault cannot yet be attributed to one operator.
- Z1, VerifyMerkle: A. The gate stays closed. Prepare the external review package.
- Z2, VerifyInference circuit: priority not set.
- D1 A: payer is tx.from; the tx carries the operator's ML-DSA consent (GrantAuthorization) over the deal digest.
- D3 A: in-block minimum operator bond is a protocol constant equal to today's default (1_000_000).
- D4 A: escrow is priced on held_bytes, the number the deal records.
- D2 (decided by Claude): the consent digest binds chain_id, payer and the payer's tx nonce, so a consent is single-use.
- D5 (decided by Claude): any payer may fund a deal, as today.
- D6 A: the executor refuses OpenDeal on mainnet until maintenance, payouts and challenges also run in block.
- F3 C: the in-block registration path refuses a Generated source for now. Stored and sealed recipes are allowed.
- Language rule (owner): the repo is Rust and BudL only.
  - 1B: the Solidity verifier stays (Ethereum runs it); it is bound to Rust-generated ABI and test vectors.
  - 2A: the budscan Firefox patch layer is replaced by a Rust-engine browser shell over the existing Rust core.
  - 3: inline shell in CI YAML moves into Rust xtask commands. Nix, Dockerfile, systemd and YAML stay as configuration formats.
- Security channel (owner, 1 C): BudZKVM soundness fixes go to the public branch with neutral commit messages, no exploit write-ups.
- Baseline edits (owner, 2): the four idle-code baseline line removals are approved.
- B.U.D. mainnet gate (owner, 3): Claude opens it when storage payouts, challenges and audits run in block and the RPC mutation paths are gone.
- Session (owner, 4): continue in a new session. The project settings (.claude/settings.json opusplan, .claude/agents) load at session start.
- Main session writes no code. Code is written by coder agents (Sonnet), discovery by scout (Haiku), R3 bug hunts and verification by finder/architect (Opus). At most 2 agents in parallel.
- Tests run on GitHub CI. Locally: cargo check and cargo fmt only, plus the specific new tests. CI is reviewed at the end.
- Standing rule: ask the owner only for hard architecture changes. Decide the rest, record it here.

## Steps

| ADIM | Scope | State |
|---|---|---|
| 1 | `StorageTx::RegisterManifest` | done, `9527a01`, Opus verification pending |
| 2 | `StorageTx::OpenDeal`, deterministic block time | planning |
| 3 | Open and answer a retrieval challenge in block | open |
| 4 | Stored coding audit with deadline, cooldown on failure | open |
| 5 | `DeclareOperatorClass` | open |
| 6 | Remove RPC mutation paths, mainnet gate (ask owner) | open |
| 7 | VerifyMerkle external review package | draft in scratchpad (not committed: open soundness leads, SECURITY.md process); finder running |
| 8 | Port Python and shell tooling to Rust (owner rule: Rust and BudL only) | coding |
| 9 | Move inline CI shell into Rust xtask | open, after ADIM 8 |
| 10 | Solidity verifier bound to Rust-generated ABI and vectors | open |
| 11 | budscan Rust-engine shell replacing the Firefox layer | paused by owner (budscan stays unchanged for now); plan kept: Blitz engine recommended, K1/K2 open |
| 12 | BudZKVM soundness fixes (details held privately per docs/SECURITY.md) | planning, local only |

## Open findings

- B2: `AnswerCodingAudit` accepts a caller-built `CodingAudit`, so an answerer can pick a column it computed honestly (`src/chain/chain_actor.rs`, AnswerCodingAudit arm). The maintenance sweep derives audits and discards them. Closed by ADIM 4.

## Deviations to report

- ADIM 1 removed one stale line from `.github/idle-code-baseline.txt`. The `no-idle-code` gate requires it because `register_manifest_with_source` gained a production caller. This is a tightening, but MODEL_ROUTING §12.6 forbids touching baseline files, so the owner should confirm.

## EFFORT LOG

(no finder-max calls)

## Handoff to the next session (2026-10-07)

Branch head pushed. Working tree clean. PR #1 open.

### Done this session (commits on the branch)
- Storage in block: RegisterManifest, DeclareOperatorClass, DeclareSelfHostPolicy, OpenDeal (with operator consent), shared check-first deal open and reallocation (src/domain/storage_tx.rs, src/domain/deal_open.rs). Out-of-block operator class command removed.
- Deterministic block clock on AccountState.
- All Python and shell tooling ported to Rust (xtask/tools, bud/src/bin/measure_ratios.rs, crates/bpqs/examples).
- BudZKVM AIR row transitions (b64163d, cf9a35f): sequential next_pc rule, last row Halt, expansion rows bound to their opcode, VerifyMerkle block closure (64 rounds, constant imm), verifier refuses opcodes not activated.

### Not yet verified
- b64163d and cf9a35f: the Opus verification was running when the session ended. Re-run it first (architect, fresh context).

### CI (owner: review at the end, then fix in one pass)
Failing on the PR: Dependency Review, Typos, Repo Lint, Gates, Budlum Core, docker-smoke. Several workflows are disabled_fork and must be enabled in the Actions tab by the owner.

### BudZKVM work queue (in order; each step: coder writes a forgery test that verifies before the fix and is refused after; Opus verifies)
1. Memory and register argument (needs an architect plan first; the plan was in progress):
   - Booleanity, prefix contiguity and boundary rules for COL_REG_ACTIVE and COL_MEM_ACTIVE.
   - Pin memory m_same to (next addr == addr) with an inverse witness, like the register side.
   - Order the register and memory tables by (index, clk) with a range check, or timestamped offline memory checking (prev_clk < clk).
   - Commit the initial image soundly (not a linear fold); gate on active; forbid index 0 writes.
   - Store with rs1 = r0 must create memory demand (is_real_mem_op = is_load*rs1_nonzero + is_store).
   - Bound addresses; separate stack and storage tables; word alignment.
   - Only opcodes that write rd may write rd on the register bus.
   - Bound sp on pop/ret and the stack depth.
2. Run boundary and outputs: force cpu_active = 0 on the last row; derive exit_code; trace_len counter with a transition; prove or stop exposing final_state_root (consumer src/execution/proof_verifier.rs:248-254).
3. Dynamic storage slots (imm = -1): slot = is_dyn*rs2 + (1-is_dyn)*imm in the address and the digest; no i32 truncation in the VM.
4. Canonical u64: the VM must keep register values canonical (imm sign extension, saturating adds); EQ_DIFF_INV must use field subtraction.
5. event_digest: Poseidon chain instead of a field sum.
6. Verifier: check gas_used <= gas_limit.
7. B1: make VerifyInference (0x1F) fail closed (VM rd = 0 with no expansion; AIR forbids the selector).
8. A1: VerifyMerkle node hash = two full Poseidon permutations (sponge rate 4, capacity 4), leaf and node domain tags, 4-limb digests, 265-word window, 329 rows, gas 2075.
9. A2: AIR and prover for A1 via the shared Poseidon gadget; PROOF_FORMAT_VERSION = 2.
10. A3: BudL builtin verify_merkle_proof(window_const).
11. A4: storage challenge proofs checkable end to end (Hash32 <-> 4 limbs; canonical program binds root, leaf and key through state_writes_digest); then storage_challenge_proofs_are_checkable() can return true.
12. B2: HashMem opcode 0x23 (4-lane sponge over a memory window).
13. B3: bind MLP guest weights, input and output commitments with HashMem.
14. AIR audit group 2 (finder): Poseidon, privacy opcodes, syscalls, SRead/SWrite digest, Call/Ret stack.
VerifyMerkle stays closed on mainnet until all of the above and an external review.

### B.U.D. queue
- ADIM 3: retrieval challenge open and answer in block.
- ADIM 4: stored coding audit with deadline; failure records and applies the 6 hour cooldown.
- ADIM 6: remove RPC mutation paths; then open storage on mainnet (owner decision 3).
- Low: declare_operator_class scans all deals (index by operator later).

### Paused
- ADIM 11 budscan (owner). ADIM 10 Solidity ABI binding not started.
