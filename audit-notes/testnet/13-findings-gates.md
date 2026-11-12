# How to build: Findings and feature gates

Testnet index Sections 13 and 14. TLG gates TLG-01, TLG-02.

---

## Current state

28 closing tests (`*_is_closed_on_testnet`) exist across the codebase (F-0058). These prove that closed features are rejected on chain ID 45261.

The audit has found 112 findings (F-0001 to F-0112) plus 11 from the ops/config audit (F-1200 to F-1210). Total: 123 findings.

## What must be built

1. **TLG-01: No open Critical or High findings**. Review all findings. Classify each as Critical, High, Medium, Low, or Informational. Critical and High findings must either be fixed or the finding's feature must be closed on testnet with a closing test.

2. **TLG-02: Closing test coverage**. Every "Closed" and "Conditional" feature in Section 14 of the testnet index must have a `<feature>_is_closed_on_testnet` test. Verified features (F-0058):
   - private_dedup: yes
   - compression_context: yes
   - bud3_live / live_stream: yes
   - object_layer: yes
   - shared_security: yes
   - private_transfer: yes
   - ai_execution_proof / ai_layer: yes
   - ethereum_sync_committee: yes
   
   Need verification:
   - SVM and Move asset pointers: check if covered under object_layer or foreign module tests
   - Remote App Binding: check if covered under object_layer tests

3. **Closing test for bridge** (if bridge stays closed). Add `bridge_is_closed_on_testnet` test that proves bridge transactions are rejected on chain ID 45261.

4. **Closing test for AI** (if AI stays closed). The executor gate (`check_ai_layer`) already refuses AI transactions on non-devnet chains. Verify a closing test exists that exercises this gate on chain ID 45261.

## Key files

- `src/tests/`: 116 test files, 28 closing tests
- `src/execution/executor.rs`: feature gating for privacy and AI
- `src/core/chain_config.rs`: network profiles and feature flags

## Blocking findings

- F-0058: closing test coverage needs verification for 2 features
- F-1200: truncated address in ceremony genesis (Critical)
- F-0044: mobile share store in-memory (High, testnet blocker)
- F-0001/F-0005/F-0030: bud-node coexistence (High, TLG-03)
