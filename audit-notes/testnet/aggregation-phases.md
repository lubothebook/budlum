# Aggregation directive: phase inventory

Measured against `budlum-stack` (head `462d7d5d`) on 2026-11-17 by reading the
tree, then re-measured against `agg/phase-5-pcs-claims` (head `834ee878`) on
2026-11-18. Every row names the file where the phase's code lives, or says
plainly that the code is not there. Nothing in this file is copied from a plan.

The phases that the aggregation branch finished are marked **corrected** below,
with the line numbers that were read. Read the branch, not the stack, before
trusting a "missing" line in this file: four of them were already false.

## Phase 0: baseline measurement

Done, and better than the phase asks for.

- `benches/agg_baseline.rs` measures the witness cost per scheme (name,
  public key bytes, signature bytes, one verification) and, for blocks of 100
  and of 1000 transfers in the V5 and V7 form, the protobuf size of the block
  with and without witnesses, the witness share and the time to verify every
  transaction. Sizes come from `ProtoBlock::encoded_len`, not from the scheme
  table, and the harness exits 2 rather than print a number it cannot trust.
- `benches/agg_tick_steps.rs` is the phase 2 measurement: where one signature
  verification spends its steps, and per tick size the steps, the VM time and
  how many prover rows that many steps need. It self-checks `tick_pi`.
- Both are registered in `Cargo.toml` with `required-features = ["wallet-ml-dsa"]`.
- Status: done.

## Phase 1: claim layer

Done, and it is the part the whole module rests on.

- `src/aggregation/claim.rs` holds `Claim::{Sig, Prog, Poly}` and the digest
  rules, with a tag test per tag in `src/aggregation/tests/claims.rs`.
- `src/aggregation/schemes.rs` is the scheme registry: name, native verifier
  and `max_object_bytes` per `SchemeId`.
- `src/aggregation/backend.rs` holds the `AggBackend` trait exactly as the
  directive writes it.
- `src/aggregation/native.rs` is `NativeBackend`. Its proof is the list of
  encoded objects, its `verify` re-verifies each object. It is compiled for
  tests and internal test harnesses; the xtask gate
  `xtask/gates/src/gates/agg_native_backend.rs` (`agg-native-backend-is-devnet-only`)
  enforces that this native test backend is strictly isolated from production release targets.
- `src/aggregation/pool.rs` is the object pool with the 6.1 admission rules,
  `src/aggregation/aggregator.rs` is the tick loop and the merge.
- `src/network` carries `agg/object` and `agg/frame`.
- `src/aggregation/envelope.rs` and `src/aggregation/object.rs` define the
  object form; `src/core/transaction.rs` carries `TxAuth` and `claim_refs`;
  `src/core/block.rs` carries `claim_frame` and `frame_hash`.
- `src/aggregation/block.rs` is the block rule set; `src/aggregation/fees.rs`
  is the fee rule; pruning is in `src/aggregation/pool.rs` and
  `src/aggregation/aggregator.rs`.
- `src/aggregation/params.rs` holds `AggMode` and `AggParams`.
- Status: done.

## Phase 2: signature claims, the tick proof

The programs exist as BudZKVM bytecode and are exercised by a real VM.

- `src/aggregation/programs/verify_sig.rs` is the reference signature program.
- `src/aggregation/programs/agg_tick.rs` is `AggTick`: it verifies each
  object, checks that the signer is the address of the key, computes the claim
  digest, the claim Merkle root and `tick_pi`, and writes `tick_pi` as eight
  storage writes.
- `src/aggregation/tests/programs.rs` runs those programs on `bud_vm::Vm` and
  covers the differential and the negative cases (a forged signature stops the
  run with `VmError::AssertionFailed`).
- Status: **corrected on 2026-11-18 against `agg/phase-5-pcs-claims`.** The
  earlier text here said the programs were proved nowhere in
  `src/aggregation`. That was true of the tree it measured (`budlum-stack`
  `462d7d5d`) and is false of the aggregation branch today.
- A prover for the tick program now exists: `src/aggregation/programs/chip_offload.rs`
  carries `prove_tick_chips` (`:273`) and `verify_tick_chips` (`:315`) over the
  real AIRs under `budzero/bud-proof/src/chips/` (`keccak.rs`,
  `mldsa_ntt.rs`, `batch.rs`, `fold.rs`), built on `bud-proof`'s
  `prove_chip` (`plonky3_prover.rs:436`) and `verify_chip` (`:453`). The proof
  type carries the Keccak outputs and proof, the NTT outputs and proof, and the
  tick public input, so the hash and signature hot spots a tick over one
  ML-DSA-87 object needs are offloaded. `src/aggregation/tests/programs.rs:1140`
  proves and verifies a chip tick.
- What is still missing is narrower than a prover: `StarkBackend::prove_tick`
  (`stark.rs:313`) does not call `prove_tick_chips`, and `chip_offload.rs:19`
  says so (`WIRING: unwired`). The chip proof is exercised by its own test and
  is not yet the statement the aggregator folds.

## Phase 3: recursion, master proof, `StarkBackend`, `Shadow`

- `src/aggregation/frame.rs` already carries the merge shape, `merge_pi`,
  `AggProgram` and the frame hash rule, and `src/aggregation/programs`
  describes the tick program a merge would recurse over.
- `StarkBackend` **exists**: `src/aggregation/stark.rs`, 450 lines, `pub struct
  StarkBackend` at `:84`, `impl AggBackend` at `:312` with `prove_tick`
  (`:313`), `prove_merge` (`:368`) and `verify` (`:418`). It does not
  concatenate: the proof is a Plonky3 `AggFoldAir` over the postfix tree, magic
  `BSTK`, parameter set `2`, and `verify` re-derives the fold statement and
  checks the parameter set, the domain, the program hash against the
  `agg_tick_program_hash` / `agg_merge_program_hash` allowlist and the master
  public input before running the fold verifier (`:423-448`). A segment's
  `tick_pi` commits to `claims_root`, so editing a claim changes the master
  public input and is refused.
- `AggMode::Shadow` exists in `src/aggregation/params.rs`, and the 10 000-block
  shadow run exists: `src/aggregation/tests/phase3.rs:368
  shadow_auditor_matches_native_across_10000_blocks`.
- Status: **corrected** to done on `agg/phase-5-pcs-claims`, negative cases
  included (`phase3.rs:197` refuses a wrong domain, a wrong parameter set, a
  redirected aggregator fee and a tampered program hash).

## Phase 4: program claims, hidden multisig

- `Claim::Prog` and the `auth_program_hash` slot exist in the claim and
  account shapes.
- The hidden-multisig program **exists**:
  `src/aggregation/programs/hidden_multisig.rs`, 446 lines, with
  `HiddenMultisigProgram` (`:66`), `HiddenMultisigPolicy` (`:80`),
  `MemberSignatureWitness` (`:98`) and `compile` (`:209`).
- Status: **corrected** to done on `agg/phase-5-pcs-claims`.

## Phase 5: PCS claims, `VerifyStark`, `AirRegistry`

- `Claim::Poly` exists with canonical field elements.
- `AirRegistry` **exists**: `src/aggregation/air_registry.rs`, 366 lines, with
  `pub struct AirRegistry` (`:284`), `with_builtins` (`:292`) registering
  `bud_air_id`, `agg_fold_air_id` and `external_fib_air_id`, and
  `verify_stark_tx` (`:333`); `TransactionType::VerifyStark` and `Claim::Poly`
  are the wire path into it. `src/aggregation/programs/pcs_claim.rs` (594
  lines) and `prog_claim.rs` (380 lines) carry the program side.
- Status: **corrected** to done on `agg/phase-5-pcs-claims`.

## Why phases 2 and 3 were unfinished on `budlum-stack`, and what changed

The reasoning below was correct for the stack tree it was written against, and
it is kept because it explains the order the work took. It is no longer a
statement about the aggregation branch: the chipping it calls for as a
precondition has been done, and the backend it says to write second exists.

Measured, not assumed:

- On `budlum-stack`, `src/aggregation/tests/programs.rs` ran the reference
  programs on `bud_vm::Vm` and threw the trace away as it went
  (`if vm.trace.len() >= 1 << 12 { vm.trace.clear(); }`), with the comment
  that a run of millions of steps "does not fit in memory with it". A tick
  over one ML-DSA-87 object is such a run.
- `ProverAdapter::prove` takes the whole `&[Step]` trace, so a tick proof first
  needed the hot spots moved to chips, the way `src/domain/storage_deal.rs`
  does for `VerifyMerkle`.
- The order the work then took was: chip the tick program (`chip_offload.rs`,
  done), write `StarkBackend` against it (`stark.rs`, done but not yet wired to
  the chip proof), then turn `Shadow` on for the 10 000-block run
  (`phase3.rs:368`, done).

What is still true: `StarkBackend::prove_tick` folds an `AggFoldAir` statement
and does not consume `prove_tick_chips`, so the aggregator's own tick proof is
not the chip proof. That single wiring step is the remaining phase 2 item.

## What the tree already gives the next phase

`budzero/bud-vm` is a real VM and `budzero/bud-proof` is a real Plonky3
prover and verifier for it: `plonky3_prover::prove_chip`, `verify_chip`,
`Plonky3Adapter::verify_canonical_program` (which also refuses a program
outside `canonical_set`), and `bud_stark::{prover,verifier}`. The canonical set
holds a program that uses `VerifyMerkle`, so a verifier that has not activated
that opcode refuses it rather than accepting it by way of the set.

That is the material `StarkBackend` should be built on. A hand-written backend
that "looks like" a STARK would be worse than none: it would carry the frame
rules and none of the soundness.

## Constraint that stays in force

`VerifyMerkle` (64-depth) stays disabled in production. While the aggregation
circuit depends on it, mainnet keeps `AggMode::Off` and testnet may run `On`.
