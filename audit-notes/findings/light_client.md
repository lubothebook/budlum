# Light Client Findings

Files reviewed: src/light_client/mod.rs (1437 lines)

## F-0041: Light client supports three trust modes

- **Location**: `src/light_client/mod.rs`
- **What is wrong**: Not a finding. The light client implements Classical, Hybrid, and PostQuantum trust modes. Hybrid mode requires both a classical BLS signature and a PQ signature for checkpoint verification. PostQuantum mode drops classical entirely.
- **Status**: closed (informational, positive assessment)

## F-0042: Light client enforces chain-specific PQ scheme

- **Location**: `src/light_client/mod.rs:236`
- **What is wrong**: Not a finding. `for_chain(checkpoint, chain_id)` looks up the network's PQ scheme and configures the client accordingly. Unknown chain IDs default to Classical trust.
- **Status**: closed (informational, positive assessment)

## F-0043: Header chain verification exists but needs fuzz

- **Location**: `src/light_client/mod.rs:510`
- **What is wrong**: `verify_header_chain` processes a sequence of headers and verifies parent hash chaining, height monotonicity, and timestamps. No fuzz target exists for this function.
- **Why it is wrong**: Without a fuzz target, malformed header sequences may not be caught.
- **What it causes**: A maliciously crafted header chain could potentially cause unexpected behavior in the light client.
- **How to fix it**: Add a libfuzzer target for `verify_header_chain` with random header sequences.
- **Status**: open (enhancement)

## F-0044: Sync plan computation is sound

- **Location**: `src/light_client/mod.rs:743`
- **What is wrong**: Not a finding. The sync plan computes which checkpoints to fetch between the trusted height and the target height, using the configured checkpoint interval.
- **Status**: closed (informational, positive assessment)
