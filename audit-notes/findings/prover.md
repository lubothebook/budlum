# Prover Module Findings

Files reviewed: src/prover/mod.rs (523 lines)

## F-0069: ZK proof submission uses binding hash

- **Location**: `src/prover/mod.rs:166`
- **What is wrong**: Not a finding. The `payload_binding_hash` binds the proof to a specific domain, height, and submitter. This prevents proof replay across domains or heights.
- **Status**: closed (informational, positive assessment)

## F-0070: Proof claim registry is bounded

- **Location**: `src/prover/mod.rs:346`
- **What is wrong**: Not a finding. The ProofClaimRegistry uses a BTreeMap and provides len() for monitoring. Claims are per (domain, height) pair.
- **Status**: closed (informational, positive assessment)

## F-0071: Program hash uses Keccak-256

- **Location**: `src/prover/mod.rs:103`
- **What is wrong**: Not a finding. `zk_program_hash` uses Keccak-256 over little-endian u64 words. This is deliberately the same hash the verifier uses, ensuring the allow list and the AIR agree on the program identity.
- **Status**: closed (informational, positive assessment)

## F-0072: Empty programs are detected

- **Location**: `src/prover/mod.rs:116`
- **What is wrong**: Not a finding. `is_empty_or_halt_only_program` detects trivial programs that do nothing. This prevents submitting proofs over no-op programs for farming rewards.
- **Status**: closed (informational, positive assessment)
