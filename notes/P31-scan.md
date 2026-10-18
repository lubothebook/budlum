# P31 scan: AI core (read-only audit, Tier R2)

Base commit: f1b8a4a. Scanner: Sonnet, read-only. Known and not repeated: STATUS.md sections 2 and 4, docs/audit/FINDINGS.md (P13, P16 AI items), .github/dead-pub-api-baseline.txt. A rg for the AI symbols below in STATUS.md, FINDINGS.md and the baseline files returned no hit, so the findings are written as not known.

## COVERAGE

Read in full (every line):
- src/ai/types.rs 1-1265 (one read)
- src/ai/mod.rs 1-4697 (four reads: 1-1500, 1501-3000, 3001-4500, 4501-4697)

Docs read: docs/AGENT_MAP.md 1-56; docs/AUDIT_PLAN.md lines 108-167 (sections 5 and 6); notes/P29-scan.md lines 1-30.

Structure of the part: src/ai/mod.rs production code is lines 1-31 only (module list, re-exports, the const FULL_AI_STARK_VERIFICATION_LIVE). Lines 33-4697 are #[cfg(test)] unit tests that drive src/ai/registry.rs. All consensus logic of the part is in types.rs.

Outside the part (rg first, then at most 80 lines):
- src/ai/registry.rs 180-300 (register_model, submit_request), 440-465 (QoS record), 1485-1500 (verifiers_by_reliability), 1905-1925 (state_root)
- src/execution/executor.rs 1420-1475 (AiModelRegister arm)
- src/core/transaction.rs 1791-1840 (encode_model_spec)
- src/chain/chain_actor.rs 3990-4005 (ranking command)
- rg only: src/core/account.rs (state_root use), src/chain/snapshot.rs, src/rpc/server.rs, src/network/proto_conversions.rs, .github/*baseline.txt, STATUS.md, docs/audit/FINDINGS.md.

Not read: the rest of registry.rs, src/ai/execution/*, src/ai_inference/*, src/tests. No cargo, no test run: every claim comes from reading source.

Extra-check summary:
- Z8 float: f64 appears only in AiVerifierQos::reliability_score and AiAgentReputation::trust_score (types.rs:743, 955). Neither value enters any leaf (calculate_leaf uses the integer counters only). Consumers are RPC and the chain_actor read commands, plus a sort in registry.rs:1495. Not a consensus path: P31-F08 (Info).
- Fixed-point overflow: the MLP fixed-point code is in src/ai/execution (outside the part). In the part the only integer arithmetic risk is the running average at types.rs:722: P31-F07.
- State transitions: the escrow status enum (types.rs:998-1007) has no transition function in the part; transitions live in registry.rs. Terminal-state tests exist in mod.rs (reclaim, cancel, proof after cancel). Not verified for payments beyond tests that read.
- Bounds on sizes and dims: validate() bounds ref bytes, deadlines and execution_dims (types.rs:202-251). Gaps: P31-F05, P31-F06.
- Replay and nonce: request_id and the result signing hash omit chain_id (documented in-file, types.rs:413-432): P31-F09.
- Unwired parts: P31-F10, P31-F11.
- Panics from external bytes: no unwrap, expect, panic, indexing, or unsafe in the production code of types.rs (rg confirmed zero hits). mod.rs unwraps are all under #[cfg(test)].
- State written outside blocks: none in the part. The types are plain data; mutation is in registry.rs.
- Z3: no #[allow] and no #[ignore] in either file (rg). Z7: no unsafe. Z9: no key, seed or token literals (test addresses are small constants).

## Findings

### P31-F01
File: src/ai/registry.rs:186-243 (register_model) with src/execution/executor.rs:1420-1456; type definition src/ai/types.rs:25-33
Severity: Medium
Axis: security
Confidence: Likely (missing verification step: the full register_model body between lines 243 and the end of the function was read, but the RPC and mempool admission paths outside the part were not read, so a pre-check there is not excluded)
Evidence:
```
src/execution/executor.rs:1420  TransactionType::AiModelRegister(spec) => {
src/execution/executor.rs:1421      let mut spec = spec.clone();
src/execution/executor.rs:1422      if spec.owner != tx.from { spec.owner = tx.from; }
...
src/execution/executor.rs:1455      .register_model(spec)
src/ai/registry.rs:236  if self.models.contains_key(&spec.model_id) {
src/ai/registry.rs:237      return Err(format!("Model ID {} is already registered", ...
src/ai/registry.rs:241  let id = spec.model_id;
src/ai/registry.rs:242  self.models.insert(id, spec);
```
rg -n 'AiModelId::of' src --glob '!src/tests/*' gives only src/rpc/server.rs:5650 (the RPC helper that builds a spec), src/ai/execution/guest.rs:1614 and src/ai/execution/verify.rs:165 (tests). No consensus code recomputes it.
Description: types.rs:25-33 defines the canonical model id as a hash of owner, model_hash and version, but register_model and the executor take `spec.model_id` as sender-chosen and never compare it to AiModelId::of. The executor overwrites owner with tx.from, so the id and the owner can disagree.
Impact: any account can register an arbitrary model_id, including one derived from another owner's address and hash. A user who sees a pending registration can submit the same model_id first with a higher fee; the honest registration then fails with "already registered" and the attacker owns the id requesters will address. Requesters pay max_fee against that id.
Trigger: precondition: a pending or announced registration whose model_id is known. Entry: AiModelRegister at executor.rs:1420. Input shape: AiModelSpec with the victim's model_id and the attacker's own fields. Expected wrong result: attacker is recorded as owner of the victim's id.
Known: none

### P31-F02
File: src/ai/types.rs:253-284 (AiModelSpec::calculate_leaf); src/core/account.rs:2454
Severity: Medium
Axis: correctness
Confidence: Likely (missing verification step: whether any mutation path other than registration can change execution_dims or modalities was not checked in registry.rs; update_model_spec at registry.rs:595-665 was seen only through its argument list)
Evidence:
```
types.rs:253  pub fn calculate_leaf(&self) -> [u8; 32] {
types.rs:269      match &self.execution_weights_digest { ... }
types.rs:276      match &self.execution_program_hash { ... }
types.rs:283      hasher.finalize().into()
```
The leaf hashes model_id, model_hash, owner, counts, deadlines, version, active, require_execution_proof, execution_class, weights digest and program hash. It does not hash execution_dims (types.rs:187) or modalities (types.rs:198). By contrast the transaction preimage does include both (src/core/transaction.rs:1824-1837, with comments saying a relay rewrite must not verify).
Description: the registry root that feeds the block state root (account.rs:2454 final_hasher.update(self.ai_registry.state_root())) is blind to two spec fields that control behavior: dims select the guest program a proof is checked against, modalities gate request admission.
Impact: two nodes whose specs differ only in execution_dims or modalities produce the same state root, so a divergence (a bug, or a tampered snapshot that passes a root check) is not detected at the root and only shows later as different accept/reject decisions. Snapshot hashing uses a serialized form (snapshot.rs:909) and does cover them, which limits this to the block-root path.
Known: none

### P31-F03
File: src/ai/types.rs:661-673 (AiExecutionProof::calculate_leaf)
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
types.rs:663  hasher.update(b"BDLM_AI_EXEC_PROOF_V1");
types.rs:664  hasher.update(self.model_id.0);
...
types.rs:668  hasher.update(self.gas_used.to_le_bytes());
types.rs:670  // Note: proof_bytes is NOT hashed (too large, and the
```
Description: the leaf omits proof_bytes, weights_digest and public_inputs. weights_digest is the field the verifier compares to the registered digest (types.rs:549-556), so it decides acceptance but is not committed. The code comment justifies omitting proof_bytes only.
Impact: stored proofs that differ in weights_digest or public_inputs give an identical root. Today require_execution_proof registration is refused (registry.rs:191, FULL_AI_STARK_VERIFICATION_LIVE is false), so the proof path is closed; the gap becomes live when the flag flips.
Known: none (the flag itself is a mainnet blocker written in mod.rs:19-25 and tested in src/tests/f20_priority_findings.rs:68)

### P31-F04
File: src/ai/types.rs:944-951, 982-995 (AiAgentReputation::update_activity, calculate_leaf)
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
types.rs:946  if self.first_active_block == 0 {
types.rs:947      self.first_active_block = current_block;
types.rs:950  self.active_block_span = current_block.saturating_sub(self.first_active_block);
types.rs:993  hasher.update(self.active_block_span.to_le_bytes());
```
The leaf hashes active_block_span but neither first_active_block nor last_active_block.
Description: 0 is used as the unset marker, so activity at block 0 is overwritten by the next event. Two committed-state inputs (first and last block) are not in the leaf while a value derived from them is.
Impact: low; reputation is informational (RPC only). A node with a wrong first_active_block computes future spans differently while the leaf still matches until the next update.
Known: none

### P31-F05
File: src/ai/types.rs:202-251 (AiModelSpec::validate)
Severity: Low
Axis: security
Confidence: Certain
Evidence:
```
types.rs:203  if self.min_verifier_count == 0 {
types.rs:206  if self.agreement_threshold == 0 || self.agreement_threshold > self.min_verifier_count {
types.rs:209  if self.max_input_ref_bytes > MAX_INFERENCE_REF_BYTES as u64 {
```
Description: no upper bound on min_verifier_count or agreement_threshold (u32::MAX accepted); max_input_ref_bytes and max_output_ref_bytes of 0 accepted; execution_class (u8) is not checked against the whitelisted set documented at types.rs:146 (only executor.rs:2250 rejects nonzero classes at proof time). dims are bounded (2 to 32 layers, no zero) but individual layer sizes are not (u16 up to 65535 per layer, 32 layers), so a guest program build from dims at registry.rs:220 can be large.
Impact: an owner can register a model that can never finalize (self-harm and registry clutter) or one whose guest program build is expensive at registration time. Bounded by the registration fee. No remote crash found.
Known: none

### P31-F06
File: src/ai/types.rs:87-117 (BoundedBytes)
Severity: Low
Axis: security
Confidence: Likely (missing verification step: which serde-based decode paths, snapshot JSON, RPC JSON or block bincode, take AiInferenceRequest or AiInferenceResult from external bytes was not traced)
Evidence:
```
types.rs:87  #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
types.rs:88  pub struct BoundedBytes(pub Vec<u8>);
```
The protobuf conversion goes through try_new (src/network/proto_conversions.rs:1127, 1195), but the derived Deserialize and the public field do not enforce the 64 KiB bound.
Description: the bound exists only on the constructor. Registry admission checks length against the model spec (registry.rs:288-ish, "input_ref exceeds model specification limits"), so oversize data is refused later, but allocation happens at decode time.
Impact: decode-time allocation up to the transport limit on serde paths. The size is also not capped for the stored state if any path skips submit_request. Defense in depth.
Known: none

### P31-F07
File: src/ai/types.rs:716-726, 955-962
Severity: Info
Axis: correctness
Confidence: Certain
Evidence:
```
types.rs:722  (self.avg_response_blocks * total + response_blocks) / (total + 1)
types.rs:956  if self.payments_completed + self.payments_defaulted == 0 && ...
types.rs:961  let total_payments = self.payments_completed + self.payments_defaulted;
```
Description: unchecked u64 multiply and add in a field that is committed to the state root (types.rs:761). response_blocks comes from registry.rs:452-456 with saturating_sub and a submitted_at_block window of MAX_DEADLINE_HORIZON_BLOCKS, so the average stays small and the product needs about 2^64 / horizon results to overflow. The counters elsewhere use saturating_add, so the style is inconsistent. In release builds a wrap would be silent; in debug it would panic.
Impact: not reachable in practice; maintenance risk only.
Known: none

### P31-F08
File: src/ai/types.rs:738-751, 953-979 (reliability_score, trust_score); src/ai/registry.rs:1492-1498
Severity: Info
Axis: invariant(Z8)
Confidence: Certain
Evidence: f64 arithmetic at types.rs:748-750 and 963-978. rg -n 'reliability_score|trust_score' src gives callers only in src/rpc/server.rs (6403, 6428, 6560, 6579), src/chain/chain_actor.rs:3943 and 3999 (read commands) and the sort at registry.rs:1495. The leaf functions (types.rs:754-764, 982-995) hash integers only.
Description: floats are present in the part but outside the state root and outside block validation. The sort order is deterministic enough for display (stable sort over a BTreeMap, partial_cmp with Equal fallback; NaN cannot occur because divisors are checked nonzero).
Impact: none for consensus today. Anyone who later feeds these scores into verifier selection or fees would violate Z8.
Known: none

### P31-F09
File: src/ai/types.rs:357-385, 433-443
Severity: Info
Axis: security
Confidence: Certain
Evidence: calculate_id (V3 tag, types.rs:359) hashes requester, model, commitment, input_ref, fee, callback, blocks, effort and perception; no chain_id. The comment at types.rs:413-432 states this and says the transaction envelope (chain_id in Transaction::signing_hash) is what stops cross-network replay.
Description: the same request produces the same request_id on testnet and mainnet. Safe today because the outer signature binds the chain; recorded here as a standing precondition for the inner signature (calculate_signing_hash has zero production callers, rg found only src/tests/advisory_reachability.rs:462,497).
Impact: none now. If calculate_signing_hash is wired without chain_id, the inner result signature is replayable between networks.
Known: documented in-file only

### P31-F10
File: src/ai/types.rs:1119-1175 (AiAgentPaymentReceipt::from_settlement, calculate_receipt_id); src/ai/registry.rs:1723
Severity: Low
Axis: integration
Confidence: Certain
Evidence: rg -n 'get_agent_payment_receipt|AiAgentPaymentReceipt|accountability_receipt' src crates --glob '!src/ai/*' returns no line. Inside src/ai the only non-test caller chain is registry.rs:1723-1729 get_agent_payment_receipt, which has no caller. The doc comments at types.rs:1122-1125 and 1151-1153 say "no other file names it yet, which is what the idle-code gate is reporting".
Description: the receipt type and its builder have no production call site; the RPC or CLI read path the comments promise does not exist.
Impact: dead code; a public accountability feature that cannot be reached.
Known: the idle-code gate reports it (per the in-file comment); not found in the dead-pub-api baseline by rg

### P31-F11
File: src/ai/types.rs:74-84 (AiResultId); src/ai/mod.rs:29
Severity: Low
Axis: dead code
Confidence: Certain
Evidence: rg -n 'AiResultId' src crates returns only the definition (types.rs:78-84) and the re-export in mod.rs:29. No struct field, function or test uses it.
Description: public type with constructor and no user.
Impact: dead code, API surface.
Known: not found in the dead-pub-api baseline by rg

### P31-F12
File: src/ai/mod.rs:3007-3018, 3508-3526, 4249-4263, 4675-4696
Severity: Low
Axis: test
Confidence: Certain
Evidence:
- test_state_root_v2_not_equal_v1 (3007): the name claims V2 differs from V1, the only assertion is `assert_ne!(root, [0u8; 32])` (3014).
- test_qos_changes_state_root (4249): `let _ = root_before;` (4255); the root changes because a request is submitted (4254) before the result, so the final assert_ne (4262) does not isolate QoS.
- test_prune_includes_new_fields (3508): comment says cancellation is cleared; the assert checks only get_request is None (3522-3525), not cancelled_requests.
- test_reputation_top_agents (4675): asserts only that the top two are a1 or a3 (4695), which also holds if the ranking is wrong within the tie.
Description: four tests whose names or comments promise more than they assert.
Impact: regressions in QoS root coverage, cancellation pruning or ranking would pass. Linked gaps: no test sets execution_dims or modalities and checks the root (the P31-F02 gap is untested); no test asserts the validate() messages for execution_dims (rg 'at most 32 layers|at least 2 layers|may not be 0' src finds them only in types.rs); no test that registration rejects a model_id that is not AiModelId::of (P31-F01).
Known: none

### P31-F13
File: src/ai/mod.rs:3589-3603
Severity: Info
Axis: test
Confidence: Certain
Evidence: test_consensus_kind_ai_inference and test_consensus_kind_ai_distinct_from_custom test crate::domain::types::ConsensusKind, not the AI registry.
Description: tests of another module sit in the AI test file; they would be lost or missed if the AI module is moved or the file split.
Impact: maintenance only.
Known: none

## Observations not raised as findings

- AiModelId::of binds a version (types.rs:26) but update_model_spec increments spec.version without changing the id (mod.rs tests 3021-3042), so the id no longer matches AiModelId::of for any updated model. Tied to P31-F01: if F01 is fixed by recomputing the id at registration, updates must keep the original version input.
- AiInferenceOutcome::calculate_leaf (types.rs:788) hashes the verifier list with no count prefix; the list is followed by fixed-width fields of 8 bytes and a 2-byte or 5-byte tag, and every element is 32 bytes, so no ambiguity was found. Same for calculate_id: input_ref is variable but input_commitment (a hash of it, enforced at admission) precedes it.
- The execution proof design gap (weights not bound by the AIR, FULL_AI_STARK_VERIFICATION_LIVE false) is documented in types.rs:149-167 and mod.rs:19-25 and pinned by src/tests/f20_priority_findings.rs:68. It is a mainnet blocker, not repeated as a finding.
