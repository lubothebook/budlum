# Findings: network, settlement, infrastructure, unwired inventory

## F-0086

- **ID**: F-0086
- **Location**: src/network/peer_auth.rs lines 1-2, 75
- **What is wrong**: The P2P peer authentication module has 3 unwired markers. No transport code reads the TLS exporter value (`EXPORTER-budlum-pq-auth-v1`, 32 bytes) from the live session. No peer is asked for an ML-DSA-65 proof. No unproven validator peer is dropped. Peer identity stays libp2p's own PeerId identity key.
- **Why**: The module documents the intended binding and tests its mechanics. The `PROOF_DEADLINE` (5 seconds) is defined. The verification function `verify_peer_proof` works. The enforcement wrapper has not landed.
- **What it causes**: P2P connections between validator nodes have no post-quantum peer authentication. A classical attacker can impersonate a validator peer. This is relevant for TLG-18 (NAT traversal and P2P hardening) and K-2 (key exchange uses ML-KEM). The RPC path has TLS 1.3 (F-0040), but the P2P path does not enforce PQ peer identity.
- **How to fix**: Wire the TLS exporter reading and the `verify_peer_proof` call into the libp2p transport adapter. Apply the `must_drop` rule for unproven validator peers.

## F-0087

- **ID**: F-0087
- **Location**: src/network/privacy.rs lines 1-14
- **What is wrong**: The network egress privacy policy seam is unwired. The `check_egress` function classifies outbound dials by purpose (Bootstrap, PeerDial, DnsSeed, LocalDiscovery, RpcClient) and routes them through Direct or Proxy paths. No network adapter consults it.
- **Why**: The egress policy is part of the FSF adaptation plan (phase 5). The seam is written and tested before the network adapter that calls it.
- **What it causes**: No outbound traffic is classified or routed through privacy policies. All dials go directly. Not a testnet blocker but a missing privacy layer.
- **How to fix**: Wire into the libp2p dial path when the adapter lands.

## F-0088

- **ID**: F-0088
- **Location**: Full inventory across src/ and budzero/
- **What is wrong**: The total "WIRING: unwired" count is 72, not the 39+ reported in F-0015. The breakdown by module: core (11), consensus (8), storage (6), settlement (6), crypto (6), cross_domain (6), network (5), ai (4), persistence (3), chain (3), security (2), account_abstraction (2), relayer (1), pollen (1), object_layer (1), light_client (1), ai_inference (1), plus 5 in budzero/.
- **Why**: F-0015 used a quick grep that counted files, not occurrences. Some files have multiple unwired markers.
- **What it causes**: 72 declared API surfaces with no production callers. Each is a potential gap if needed for testnet. The highest-concentration modules (core 11, consensus 8, settlement 6, cross_domain 6) are on the critical path.
- **How to fix**: Triage by TLG dependency. The core, consensus, and settlement unwired surfaces are the most likely to block testnet gates.

## F-0089

- **ID**: F-0089
- **Location**: ops/Dockerfile line 78
- **What is wrong**: The Dockerfile compiler version check uses a Turkish error message: `echo "HATA: imaj rustc $actual tasiyor, rust-toolchain.toml $pinned pinliyor"`. This is the only Turkish text in operational output (not in localization resources).
- **Why**: The Dockerfile was written in Turkish and this error message was not translated during the English migration.
- **What it causes**: An operator who does not read Turkish gets an unintelligible error message when the Docker compiler version check fails. TLG-24 (concept name gate, English naming) applies to operational output.
- **How to fix**: Replace with English: `echo "ERROR: image carries rustc $actual but rust-toolchain.toml pins $pinned"`.

## F-0090

- **ID**: F-0090
- **Location**: src/settlement/graft.rs line 1, line 571
- **What is wrong**: The Graft module has 2 unwired markers. `verify_app_binding_payment` has no in-tree caller by design: the RPC returns the proof to the caller, who verifies it against their own trust anchor. The second marker at line 571 is a declared surface.
- **Why**: The Graft payment verification is client-side. The node produces the proof but does not verify it. The unwired marker on line 571 is a future surface.
- **What it causes**: The Graft RPC (`bud_graftPaymentProof`) works. The verification is client-side by design. No testnet impact.
- **How to fix**: No fix needed. The unwired markers are intentional for this module.

## F-0091

- **ID**: F-0091
- **Location**: src/settlement/anchor_production.rs line 815
- **What is wrong**: Anchor production has 1 unwired surface.
- **Why**: The anchor emission pipeline is wired in blockchain.rs. This specific surface is a declared future entry point.
- **What it causes**: No testnet impact if the main anchor emission path is wired (it is, through `Blockchain::anchor_emission`).
- **How to fix**: Verify the production path covers all anchor emission scenarios for testnet.

## F-0092

- **ID**: F-0092
- **Location**: src/settlement/anchor_ledger.rs line 482
- **What is wrong**: The anchor ledger has 1 unwired surface.
- **Why**: The anchor ledger is used by the settlement layer. This specific function is a declared future surface.
- **What it causes**: No testnet impact if the settlement path reads the anchor ledger through its main entry points.
- **How to fix**: Verify the settlement paths cover anchor ledger reads for testnet.

## F-0093

- **ID**: F-0093
- **Location**: src/settlement/cold_wallet.rs line 827
- **What is wrong**: Cold wallet key rotation is unwired. The comment says "key rotation is an operator action on the same".
- **Why**: Cold wallet rotation is an operational procedure. The cold wallet state machine is in blockchain.rs but the rotation entry point is not yet wired.
- **What it causes**: An operator cannot rotate cold wallet keys through the node. Key rotation would require a manual genesis-level operation. Not a testnet blocker if the cold wallet is not activated on testnet.
- **How to fix**: Wire the rotation entry point before mainnet if cold wallet is used.

## F-0094

- **ID**: F-0094
- **Location**: src/settlement/pq_anchor_verify.rs line 1
- **What is wrong**: PQ anchor light verification is unwired. `verify_anchor_light` is reached only from anchor-ledger tests. The finality consumer that calls it is not in the tree yet.
- **Why**: This is the light client verification path for PQ anchors. The full node verification path works (anchor_production.rs is wired into blockchain.rs).
- **What it causes**: Light clients cannot verify PQ anchors. This compounds with F-0064 (light client cannot follow validator set changes). TLG-21 (explorer) may depend on light client anchor verification.
- **How to fix**: Wire when the light client consumer lands.

## F-0095

- **ID**: F-0095
- **Location**: src/chain/chain_actor.rs line 601
- **What is wrong**: `get_blocks_range` on ChainHandle is unwired. The function reads a contiguous range of blocks in one round trip.
- **Why**: The function exists for batch block reads. Current callers read one block at a time.
- **What it causes**: No functional impact. Single-block reads work. The batch API would improve sync performance for range requests.
- **How to fix**: Not a testnet blocker. Wire when batch sync is needed.

## F-0096

- **ID**: F-0096
- **Location**: src/rpc/compat/ethereum/filters.rs lines 19-20
- **What is wrong**: Ethereum compatibility filter limits are TESTNET-DEFAULT values: MAX_FILTERS_TOTAL=128, MAX_FILTERS_PER_PEER=8, FILTER_IDLE_TTL_SECS=300, MAX_FILTER_SPEC_BYTES=16KB. Log delivery answers `-32004` until the EVM execution environment log query path merges.
- **Why**: The filter table bounds are set before measurement. Log delivery depends on the EVM execution environment which is not yet delivering logs.
- **What it causes**: TLG-13 (Hardhat, Foundry, ethers, viem tool compatibility) depends on `eth_newFilter` and `eth_getFilterChanges`. These methods currently return error -32004. Tools that poll for logs will fail on testnet.
- **How to fix**: Wire the EVM log query path. The filter table and limits are ready; only the log data source is missing.

## F-0097

- **ID**: F-0097
- **Location**: ops/Dockerfile line 93
- **What is wrong**: The runtime base image is `cgr.dev/chainguard/wolfi-base` with a digest pin. The builder stage uses `rust:1.97.1-bookworm` (Debian) with a digest pin. The toolchain check verifies the Rust version. The Chainguard migration is documented (lines 95-100) and resolves 305 Trivy advisories from Debian.
- **Why**: This is a positive finding. The Docker image uses digest-pinned base images, non-root runtime user (UID 1000), and a toolchain version gate.
- **What it causes**: Good supply chain security. The Dockerfile is well-hardened.
- **How to fix**: No fix needed. Maintain the digest pins and toolchain verification.

## F-0098

- **ID**: F-0098
- **Location**: .github/workflows/ (36 workflow files)
- **What is wrong**: 36 CI workflows exist. The nightly workflows include: conformance-nightly.yml, fuzz-nightly.yml, air-mutants-nightly.yml, miri.yml, miri-storage-weekly.yml, determinism.yml, kani-slow.yml. TLG-25 requires "all nightly workflows green for last 7 nights."
- **Why**: A large CI surface with nightly runs creates flaky-failure risk.
- **What it causes**: With 7+ nightly workflows running 7 nights, the probability of at least one flaky failure blocking TLG-25 is high. No dashboard or summary workflow aggregates the 7-night green streak.
- **How to fix**: Add a `nightly-status.yml` workflow that queries the last 7 runs of each nightly workflow and reports pass/fail summary. This gives TLG-25 a single check to verify.

## F-0015-UPDATE

- **ID**: F-0015 (update)
- **Location**: 72 markers across src/ and budzero/
- **What is wrong**: The total WIRING: unwired count is now confirmed at 72 (was "39+" in F-0015). See F-0088 for the full breakdown by module.
- **Why**: The original count used `grep -rl` (file count) not `grep -r` (occurrence count).
- **What it causes**: Updated severity: 72 is nearly twice the original estimate.
- **How to fix**: See F-0088.
