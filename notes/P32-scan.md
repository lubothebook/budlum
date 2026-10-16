# P32 scan: AI registry, execution, inference (read-only audit, Tier R2)

Base commit: f90796c. Scanner: Sonnet, read-only. Known and not repeated: STATUS.md sections 2 and 4 (4.5 lists "AI inference structural path open for non-proof models" as a class only), docs/audit/FINDINGS.md (P10-F02 AI maintenance outside block and replay, P13-F01 memory fold collision, P16, P18-F04 no rollback on Err), notes/P31-scan.md, .github/dead-pub-api-baseline.txt, .github/idle-code-baseline.txt (most dead pub items of this part are listed there).

## COVERAGE

Read in full, every line (tests included):
- src/ai/registry.rs 1-2042 (two reads: 1-1000, 1000-2042)
- src/ai/execution/guest.rs 1-1724 (two reads: 1-1000, 1000-1724)
- src/ai/execution/mod.rs 1-35
- src/ai/execution/model_class.rs 1-118
- src/ai/execution/stark.rs 1-44
- src/ai/execution/verify.rs 1-839
- src/ai_inference/effort.rs 1-294
- src/ai_inference/executor.rs 1-149
- src/ai_inference/i18n.rs 1-647
- src/ai_inference/inference.rs 1-270
- src/ai_inference/metrics.rs 1-107
- src/ai_inference/mod.rs 1-699
- src/ai_inference/perception.rs 1-539
- src/ai_inference/query.rs 1-59
- src/ai_inference/social.rs 1-232
- src/ai_inference/storage.rs 1-39
- src/ai_inference/verify.rs 1-192

Docs read: docs/AGENT_MAP.md 1-56; docs/AUDIT_PLAN.md sections 5 and 6 (lines 110-165); notes/P29-scan.md lines 1-30.

Outside the part (rg first, then at most 80 lines per read): src/execution/executor.rs 313-392 (operator bond), 1420-1534 (model register, request), 1535-1614 (result, rewards), 1654-1690 (fee reclaim), 1822-1990 (dispute slash, agent payment, release, reclaim), 2060-2377 (attach proof arm), 46-61 and 40-42 (dims check, backend allow); src/chain/chain_actor.rs 3325-3354 and 3385-3430 (AI maintenance call sites); src/core/account.rs 895-935 and 1255-1330 (operator unbond and withdraw checks); src/ai/types.rs 200-300, 300-460, 640-700, 736-800 (spec validate, leaf functions, request id, QoS); src/pollen/offers.rs 648-681 (validate_ai_read_ref); src/network/proto_conversions.rs 200-240 and 1030-1095 (model register wire form); src/core/transaction.rs 1770-1835 and 2415-2450 (signing preimage of the model spec); src/core/hash.rs hash_fields_bytes; STATUS.md (rg only), FINDINGS.md (rg only), both baseline files (rg only).

Not read: src/ai/mod.rs (huge test file, only lines 1-60), src/ai/types.rs beyond the ranges above, src/execution/proof_verifier.rs (validate_envelope_structure body), bud-proof verifier and adapter, src/rpc/server.rs AI handlers, src/tests. No cargo, no test run: every claim comes from reading source.

Extra-check summary:
- Z8 float: none in a consensus path. f64 exists only in AiVerifierQos::reliability_score and AiAgentReputation::trust_score (src/ai/types.rs 743-751, 955-971), reached from AiRegistry::verifiers_by_reliability and agents_by_trust_score (registry.rs 1492-1500, 1885-1897) which feed RPC (server.rs 6403, 6560, chain_actor.rs 3943, 3999). Neither value enters calculate_leaf. Effort tiers use integer tenths (effort.rs 58-72). See P32-F18.
- Fixed-point overflow: eval_fixed_point_mlp (guest.rs 120-158) uses i64 with checked_add and an i32 clamp, no panic. The guest uses the Goldilocks field with no clamp, so host and guest diverge on large accumulators (P32-F12 note). No consensus effect, the AI MLP is only used by the prover.
- Proof verification soundness: P32-F02, P32-F03.
- Bounds on sizes and dims: P32-F12, P32-F13. FixedPointMlpSpec::validate is sound (checked arithmetic, memory bound). AiModelSpec::validate is looser than the proof path.
- Replay and nonce: request ids are content hashes, results are unique per (request, verifier), result_nonce is only checked for non-zero (P32-F18). No replay found. Pruning then resubmission needs a live deadline, so it is bounded.
- Unwired parts: P32-F09, P32-F10, P32-F17.
- Panics from external bytes: no unwrap, expect, panic or direct index in production code of these files (all hits are under cfg(test)). One alloc-before-validate site: P32-F12.
- State written outside blocks: AI maintenance is Known (P10-F02). New instances of state written on an error path: P32-F11.
- Z3: four `#[allow(clippy::too_many_arguments)]` (registry.rs 592, executor.rs 55, inference.rs 62, mod.rs 319), no `#[ignore]`. Z7: no unsafe. Z9: no key, seed or token strings.

## FINDINGS

### P32-F01
File: src/ai/registry.rs:722-753 (prune_expired), 824-882 (reclaim_fee), src/execution/executor.rs:1512-1533 (escrow debit), src/chain/chain_actor.rs:3336-3345
Severity: High
Axis: correctness
Confidence: Certain (timing in wall clock depends on the slot length; the block count is certain)
Evidence:
```
registry.rs:730  let expired_request_ids: Vec<AiRequestId> = self.requests.iter().filter(|(id, req)| {
registry.rs:734      if self.outcomes.contains_key(id) { return false; }
registry.rs:743      let effective_deadline = std::cmp::max(req.deadline_block, result_deadline);
registry.rs:744      current_block > effective_deadline.saturating_add(retention_blocks)
registry.rs:749  for id in &expired_request_ids { self.requests.remove(id); self.results.remove(id); ...
registry.rs:829  let request = self.requests.get(request_id).ok_or_else(|| format!("Request {} not found", ...
chain_actor.rs:3340  let retention = crate::ai::registry::DISPUTE_WINDOW_BLOCKS;   // 10_080
executor.rs:1529     .checked_sub(req.max_fee)  // max_fee debited from the requester at request time
```
Description: An unfinalized request whose requester has not sent `AiFeeReclaim` within 10_080 blocks after its effective deadline is deleted by prune_expired. The max_fee was debited at request time and the only refund paths (reclaim_fee, cancel_request) need the request row. Nothing refunds on prune.
Impact: Any requester whose request goes unanswered and who does not reclaim inside the window loses max_fee permanently (the amount is neither returned nor burned nor paid, it just disappears from supply accounting). No attacker is needed, an offline requester or a stalled operator set is enough. MAX_DEADLINE_HORIZON_BLOCKS (registry.rs 24-32) says six-second slots, which makes 10_080 blocks about 17 hours; the DISPUTE_WINDOW comment (registry.rs 18-22) says 7 days at one block per minute, so the two comments disagree on the block time.
Trigger: precondition: a request that never reaches agreement_threshold (no operator, operators offline, or model deactivated meanwhile) and a requester who misses the window. Entry: AiInferenceRequest tx (executor.rs:1464) then no AiFeeReclaim (executor.rs:1654). Result: balance debited, row pruned, reclaim returns "not found".
Known: P10-F02 covers where pruning runs (outside blocks) and the replay mismatch, not the lost escrow. STATUS.md section 2 line 44 says the pruning moves into the end-of-block hook; the refund gap stays after that move.

### P32-F02
File: src/execution/executor.rs:2146-2193; src/ai/execution/verify.rs:110-155; src/ai/execution/stark.rs:25-44; src/ai/mod.rs:25
Severity: High (mainnet blocker; gated off today by FULL_AI_STARK_VERIFICATION_LIVE = false, so not exploitable now)
Axis: security
Confidence: Likely (missing step: confirm in bud-proof that event_digest, initial_state_root, block_height, nonce and sender are free public inputs that the AIR only ties to the trace, not to the request)
Evidence:
```
executor.rs:2146  let Some(ref claimed_inputs) = proof.public_inputs else { ... };
executor.rs:2165  if claimed_inputs.program_hash != registered_program_hash { ... }
executor.rs:2171  if claimed_inputs.exit_code != 0 { ... }
executor.rs:2184  if claimed_inputs.chain_id != tx.chain_id { ... }
executor.rs:2190  let expected_inputs = claimed_inputs.to_execution_inputs();
stark.rs:35       if envelope.public_inputs_hash != expected_inputs.hash() { ... }
rg 'expected_initial_state_root' src/ budzero/ crates/ --glob '!src/ai/execution/*'  -> no hit
.github/idle-code-baseline.txt:1115  src/ai/execution/verify.rs fn expected_initial_state_root
```
Description: The STARK is verified against public inputs that the prover supplies. Only program_hash, exit_code and chain_id are compared with the node's own data. The doc comment of verify.rs:94-109 says the verifier derives initial_state_root itself, but expected_initial_state_root has no production caller, and it needs the weights, which are not on chain (the model registers only a digest and dims). event_digest (the Poseidon fold of the outputs logged by the guest) is never compared with proof.output_commitment (SHA3 over limbs), and weights_digest and input_commitment are fields on the proof, not values proven by the STARK.
Impact: When the flag flips, a proof only shows that the registered program ran to exit code 0 over some memory image. A prover can pair it with any claimed output_commitment, weights_digest and input_commitment, and can reuse one proof for several requests of the same model (block_height, nonce and sender are not bound to the transaction). The guard that makes a proof-required model "trustless" would not hold. mod.rs:19-25 names only F-08 and F-09 (memory image and Fiat-Shamir) as the conditions for flipping the flag; this binding gap is not on that list.
Trigger (security): precondition: FULL_AI_STARK_VERIFICATION_LIVE flipped and a proof-required model exists; entry executor.rs:2089 (AiAttachExecutionProof) by an active AI_OPERATOR; input: a valid proof of the registered program over any weights, with a different claimed output_commitment; expected wrong result: has_execution_proof true for a result the proof does not attest.
Known: P13-F01 covers the linear memory fold only. Related baseline entry for expected_initial_state_root (idle-code-baseline.txt:1115).

### P32-F03
File: src/execution/executor.rs:2141-2144, 2210-2215, 2312-2365; src/ai/registry.rs:1323-1388, 1478-1481, 1587-1591
Severity: High
Axis: security
Confidence: Likely (missing step: read ProofVerifier::validate_envelope_structure in src/execution/proof_verifier.rs to confirm it does not inspect the inner proof_bytes)
Evidence:
```
executor.rs:2142  if model.as_ref().is_some_and(|spec| spec.require_execution_proof) {   // only then is the STARK program built
executor.rs:2210  None => crate::ai::execution::verify_execution_proof_structural_with_model(...)
executor.rs:2312  if let Ok(envelope) = postcard::from_bytes::<bud_proof::ProofEnvelope>(&proof.proof_bytes) {
executor.rs:2342      ProofVerifier::validate_envelope_structure(&structural)   // five structural checks
registry.rs:1385  self.execution_proofs.insert((*request_id, verifier.0), proof);
registry.rs:1587  if payment.require_proof && !self.has_execution_proof(rid, &payment.to_agent) {
```
Description: For every model with require_execution_proof = false (all models that can be registered today), the attach transaction runs only structural checks and an envelope shape check, never the STARK. The stored entry then satisfies has_execution_proof, which is the gate of `AiAgentPayment { require_proof: true }` release.
Impact: An active AI_OPERATOR who is the to_agent of a proof-gated escrow and an agreeing verifier of the linked outcome can attach an envelope with plausible header fields and arbitrary inner bytes, then release the escrow. The payer asked for a proof and got none. The same entry is kept in the state (see P32-F13 for size).
Trigger (security): precondition: a payment with require_proof = true linked to a finalized request; the recipient is a bonded operator who finalized it. Entry: executor.rs:2089, then executor.rs:1919 (release). Input: a postcard ProofEnvelope with backend "plonky3", valid version fields, junk proof_bytes. Expected wrong result: release_agent_payment returns Ok.
Known: STATUS.md section 4.5 names "AI inference structural path open for non-proof models" as a class. The release of proof-gated escrow through it is not described there.

### P32-F04
File: src/network/proto_conversions.rs:202-222, 1046-1088; src/core/transaction.rs:1790-1810 (encode_model_spec)
Severity: Medium (latent mainnet blocker, tied to the same flag as F02)
Axis: integration
Confidence: Certain
Evidence:
```
proto_conversions.rs:1069  require_execution_proof: false,
proto_conversions.rs:1070  execution_program_hash: None,
proto_conversions.rs:1071  execution_class: 0,
transaction.rs:1790  put_u8(out, u8::from(spec.require_execution_proof));
transaction.rs:1791  put_u8(out, spec.execution_class);
transaction.rs:1792  match spec.execution_program_hash { Some(ph) => {...
```
Description: The wire message ProtoAiModelRegister carries neither require_execution_proof, execution_class nor execution_program_hash, and decoding hardcodes false, 0 and None. The signing preimage includes all three.
Impact: A model registration that asks for an execution proof verifies on the node that built it and fails signature verification on any peer after the wire round trip. So the proof-required path cannot be reached over p2p or block propagation, and the registered spec on a decoding node differs from the signed one. Today the flag blocks such registration (registry.rs:191), so nothing breaks, but flipping the flag alone would not make the path usable, and a local-only registration would fork the producer.
Known: none.

### P32-F05
File: src/execution/executor.rs:2371; src/ai/registry.rs:1401-1473, 834-840
Severity: Medium (latent: only reachable for proof-required models, which cannot be registered today)
Axis: correctness
Confidence: Certain
Evidence:
```
executor.rs:2371  let _ = state.ai_registry.try_finalize_with_proofs(request_id);
registry.rs:1457  self.outcomes.insert(*request_id, outcome.clone());
registry.rs:835   if self.outcomes.contains_key(request_id) { return Err(... "fee belongs to verifiers") }
executor.rs:1577  if let Some(finalized) = outcome {   // reward loop exists only in the AiInferenceResult arm
```
Description: When an attach transaction completes the agreement, the outcome is inserted and the return value is dropped. The reward loop (max_fee split among agreeing verifiers), the SocialFi mint and the callback-driven follow-ups run only in the AiInferenceResult arm.
Impact: For a proof-required model finalized through the attach path the request has an outcome, so reclaim_fee and cancel_request refuse it, but no verifier is paid. The max_fee stays escrowed forever. Becomes a fund lock when the flag in F02 flips.
Known: none.

### P32-F06
File: src/ai/types.rs:253-282 (AiModelSpec::calculate_leaf), 661-672 (AiExecutionProof::calculate_leaf), used at src/ai/registry.rs:1914-1917 and 1995-2001
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence:
```
types.rs:255-280  leaf hashes model_id, model_hash, owner, min_verifier_count, agreement_threshold, max_*_bytes,
                  request/result_deadline_blocks, version, active, require_execution_proof, execution_class,
                  execution_weights_digest, execution_program_hash.
                  Not hashed: execution_dims, modalities.
types.rs:661-672  proof leaf: model_id, input_commitment, output_commitment, program_hash, steps, gas_used.
                  Not hashed: proof_bytes, weights_digest, public_inputs.
```
Description: Two spec fields that decide consensus behaviour are outside the AI state root: `modalities` (admission in admit_inference_request, mod.rs:126) and `execution_dims` (the guest program is rebuilt from it, executor.rs:2191). The stored proof leaf leaves out weights_digest and public_inputs, which the structural check and the STARK path read.
Impact: A snapshot or state transfer in which these fields are altered still matches the state root, so two nodes can hold different admission rules for the same model with equal roots, and the difference shows only when a request or proof touches the model. Live and replay paths stay equal because both run the same transactions; the exposure is state import and snapshot integrity.
Known: none.

### P32-F07
File: src/ai_inference/mod.rs:113-159; src/pollen/offers.rs:654-656; src/execution/executor.rs:1490-1495
Severity: Medium
Axis: security
Confidence: Likely (missing step: whether operators act on perception.asset_id and content_id without an off-chain grant check; the chain does not enforce it)
Evidence:
```
mod.rs:140   if let Some(data_ref) = AiDataInputRef::decode(req.input_ref.as_slice())? {
mod.rs:143       if data_ref.asset_id != perception.asset_id { return Err(...) }
offers.rs:654  let Some(reference) = AiDataInputRef::decode(input_ref)? else { return Ok(None); };
executor.rs:1494  .validate_ai_read_ref(req.input_ref.as_slice(), &tx.from, current_block)
```
Description: The perception declaration (asset_id, content_id, kind, units) is part of the request id, but a Pollen grant is checked only when input_ref starts with the Pollen prefix. With any other input_ref, validate_ai_read_ref returns Ok(None), admit_inference_request skips the asset comparison, and the request is accepted with a declaration that names any asset and content id.
Impact: A requester can pay for and publish a request that tells operators to read content of an asset it holds no grant for. The data-rights check ("closed loop") holds only if the requester volunteers the Pollen reference. Operators that follow the declaration read data without authorization.
Trigger (security): precondition: a model that declares the modality; entry executor.rs:1464 (AiInferenceRequest); input: input_ref opaque bytes, perception.asset_id of another party's asset; expected wrong result: request accepted, no grant consumed.
Known: none.

### P32-F08
File: src/ai/registry.rs:490-496, 1440-1443; src/ai_inference/inference.rs:100-121
Severity: Medium
Axis: security
Confidence: Likely (missing step: whether consumers re-hash output_ref against output_commitment off chain; the chain does not)
Evidence:
```
registry.rs:493  output_ref: result.output_ref.clone(),        // submit_result: the threshold-reaching submitter
registry.rs:1440 let output_ref = entries.iter().find(|r| r.output_commitment == output_commitment)
registry.rs:1443     .map_or_else(BoundedBytes::empty, |r| r.output_ref.clone());   // try_finalize: the first match
inference.rs:108 hasher.update(b"AI_OUTPUT_COMMIT_V1"); hasher.update(&output);   // helper binds, the registry does not
```
Description: Agreement is counted on output_commitment only. output_ref is never checked against the commitment, and the two finalization paths pick different submitters' output_ref for the stored outcome.
Impact: Among verifiers that agree on the commitment, the last one to submit (in the normal path) decides the output_ref that the outcome, the callback event consumer and the SocialFi mint (executor.rs 1607-1650, ContentId of the output) use. One agreeing verifier can point the outcome at data that does not hash to the agreed commitment, and the outcome differs between the two finalization paths for the same inputs.
Known: none.

### P32-F09
File: src/ai/registry.rs:1101-1161, src/ai_inference/effort.rs:28-49
Severity: Medium
Axis: integration
Confidence: Certain
Evidence:
```
rg -n 'declare_effort_ceiling|verifier_effort_ceilings' src --glob '!src/tests/**' --glob '!src/ai/registry.rs'
  -> src/ai_inference/effort.rs:34,35 (comments only)
.github/idle-code-baseline.txt:1124  src/ai/registry.rs fn declare_effort_ceiling
registry.rs:48   pub const DEFAULT_OPERATOR_CEILING: EffortTier = EffortTier::BASELINE;
```
Description: effort.rs states rule 2 ("declared capability gates eligibility") as "Enforced". No transaction or RPC calls declare_effort_ceiling, so every operator has the 1.0x default. unservable_reason (registry.rs 1140-1161) therefore refuses every request above 1.0x as soon as one authorized verifier exists, and accepts any tier when none exists. Eligibility is also not checked at submit_result, so an operator with a low ceiling can answer a 10.0x request that was admitted while the operator set was empty. The tier does not change the fee either (effort.rs 51-54).
Impact: Tiers above 1.0x are unusable in production, and the doc claim of enforcement is wrong. A mainnet part presented as live is not wired.
Known: dead symbol is in idle-code-baseline.txt; the consequence is not described.

### P32-F10
File: src/execution/executor.rs:347-349; src/ai/registry.rs:1038-1076, 1083-1091, 1164-1202, 1792-1806
Severity: Low
Axis: integration
Confidence: Certain
Evidence:
```
executor.rs:347  let _ = state.ai_registry.lock_verifier_stake(&tx.from, crate::ai::registry::MIN_VERIFIER_STAKE);
rg 'verifier_stakes' src --glob '!src/tests/**' --glob '!src/ai/registry.rs'  -> no hit
idle-code-baseline.txt:1141  src/ai/registry.rs fn withdraw_verifier_stake
executor.rs:1839  let (_slashed_operator, _legacy_unbacked_stake) = state.ai_registry.slash_equivocator(...)
```
Description: The registry-level verifier stake is a phantom 1000 units added on each bond (result ignored), unrelated to the real bond held in the role registry. Unbond and withdraw (account.rs 906-935) never touch ai_registry.verifier_stakes or verifier_effort_ceilings, and withdraw_verifier_stake has no production caller. The seized amount of a slash is discarded.
Impact: After AiOperatorWithdraw the address stays "authorized" in registry-level checks (is_verifier_authorized, authorized_ceilings), and the state keeps an entry per address that ever bonded. The transaction path is protected by the separate AI_OPERATOR role check (executor.rs 1540), so no direct bypass was found; the registry-level guard documented as the Sybil defense (registry.rs 39-41, mod.rs 42-60) is not the one that holds. Repeated bond cycles add 1000 each with saturating_add.
Known: dead symbols in the baselines.

### P32-F11
File: src/ai/registry.rs:406, 415-429, 434-448; src/execution/executor.rs:1560-1571
Severity: Low
Axis: correctness
Confidence: Likely (missing step: whether the block builder rolls back state on a failed tx; P18-F04 says it does not)
Evidence:
```
registry.rs:406  let entries = self.results.entry(result.request_id).or_default();
registry.rs:434  if self.cancelled_requests.contains(&result.request_id) { return Err(...) }
registry.rs:443  if self.reclaimed_fees.contains(&result.request_id) { return Err(...) }
registry.rs:415  self.equivocation_events.insert(...);  ... return Err(format!("{} verifier ...", EQUIVOCATION_ERROR_PREFIX, ...
executor.rs:1560 Err(error) if crate::ai::registry::is_equivocation_error(&error) => { ... None }
```
Description: submit_result inserts an empty results row before the cancelled and reclaimed checks, so a refused result leaves an empty Vec in `results` (hashed into state_root, changes is_empty). The equivocation record is also written on an Err return and survives only because the executor matches the error string prefix and swallows it, so correctness rests on string matching.
Impact: When an Err is not rolled back (P18-F04 class), a refused result changes the root on the node that built the block. Fragile contract between registry and executor; any other caller of submit_result loses the evidence or leaves stray state.
Known: P18-F04 (class).

### P32-F12
File: src/ai/execution/guest.rs:1108-1130; src/ai/registry.rs:218-232; src/ai/types.rs:237-247
Severity: Low (High if a non-protobuf ingress for AiModelSpec exists)
Axis: security
Confidence: Likely (missing step: whether any ingress other than the proto decoder, for example a JSON-RPC raw transaction or JSON block import, can set execution_program_hash with execution_dims)
Evidence:
```
guest.rs:1117  weights: vec![0i32; weight_count(dims)],     // allocated before spec.validate() at 1120
types.rs:243   if dims.len() > 32 { ... }  dims.contains(&0) only; per-layer value up to u16::MAX
registry.rs:218  if let (Some(registered_hash), Some(_)) = (spec.execution_program_hash, spec.execution_dims.as_ref())
proto_conversions.rs:1070  execution_program_hash: None,
```
Description: guest_program_for_model allocates a zeroed weights vector sized from unvalidated dims before the architecture check. With 32 layers of 65535 this is about 1.3e11 i32 values. register_model reaches it whenever both fields are present. The protobuf decoder always sets execution_program_hash to None, which closes that route. AiModelSpec::validate also accepts shapes (32 layers, width 65535) that FixedPointMlpSpec::validate and check_ai_exec_dims (5 layers, width 64) reject, so such a model can be registered and never proven.
Impact: A node abort (allocation failure) on a registration transaction if a non-proto route exists; otherwise a registered model that can never carry a proof (permanent trap, as the comment at registry.rs 204-217 warns). Related note: host eval saturates to i32 (guest.rs 146) while the guest computes in the field with no clamp, so large accumulators make prove_mlp_inference fail rather than diverge silently.
Known: c5459d4 (dims bound at attach, STATUS section 2) covers the attach arm only.

### P32-F13
File: src/ai/execution/guest.rs:508; src/execution/proof_verifier.rs:23; src/execution/executor.rs:2240-2272; src/ai/registry.rs:799-806
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
guest.rs:508            pub const MAX_PROOF_BYTES: usize = 256 * 1024;
proof_verifier.rs:23    pub const MAX_PROOF_BYTES: usize = 1 << 20; // 1 MB
executor.rs:2240        if proof.proof_bytes.len() > crate::execution::proof_verifier::MAX_PROOF_BYTES   // 1 MiB
executor.rs:2250        if model_spec.execution_class != 0 { ... max_params * 64 = 262144 }
proto_conversions.rs:1071  execution_class: 0
registry.rs:802         // ... up to 256 KiB of `proof_bytes` per verifier stayed in the map
```
Description: Two constants with the same name and different values. Models arriving over the wire always have execution_class 0, so the 256 KiB class bound does not apply to them and the bound in force is 1 MiB. Combined with F03 (no STARK for these models), a bonded operator can park up to 1 MiB of junk per (request, verifier) in the registry (proof_bytes is not in the state root, so it is also not covered by snapshot checks).
Impact: Memory and snapshot size amplification by operators, bounded by the number of results they submit.
Known: none.

### P32-F14
File: src/ai/registry.rs:807-811, 957-1003, 1905-2035
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
registry.rs:808  self.execution_proofs.retain(|(request_id, _), _| {
registry.rs:809      !expired_request_ids.contains(request_id) && !prunable_outcomes.contains(request_id)   // Vec::contains
registry.rs:958  self.results.iter().any(...)   // operator_has_open_obligations scans all results
registry.rs:1905 pub fn state_root(&self) -> [u8; 32] { ... hashes every map from scratch
```
Description: prune_expired does a Vec membership test per stored proof (quadratic in proofs times pruned ids) on every block, and state_root rehashes all sixteen maps on each call. operator_has_open_obligations scans all results on each unbond or mempool validation.
Impact: Per-block cost grows with registry size. Requests cost an attacker only max_fee >= 1 plus the tx fee, so the registry can be inflated to slow every block; no hard cap on request count exists.
Known: none.

### P32-F15
File: src/ai/registry.rs:1983-1993, 1953-1957
Severity: Low
Axis: correctness
Confidence: Likely (the preimage is ambiguous in theory; no concrete colliding pair was built)
Evidence:
```
registry.rs:1985  for (addr, events) in &self.callback_queue {
registry.rs:1986      hasher.update(addr.as_bytes());
registry.rs:1987      for event in events { hasher.update(event.request_id.0); ... }   // no count, no length
```
Description: The callback queue contribution has no per-address event count, unlike the results section that hashes each list into its own digest. Event bytes (32 + 32 + 8 + 32) can be re-read as an address followed by fewer events.
Impact: Two different queue contents could in principle produce the same bytes. Hard to construct (event fields are chain-derived), so defense in depth only.
Known: none.

### P32-F16
File: src/ai/types.rs:206; src/ai/registry.rs:483-484; src/execution/executor.rs:1589-1605
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
types.rs:206   if self.agreement_threshold == 0 || self.agreement_threshold > self.min_verifier_count
registry.rs:483  if agreeing_verifiers.len() as u32 >= spec.agreement_threshold && ...
rg -n 'min_verifier_count' src --glob '!src/tests/**'  -> only spec validate, preimage encoders, update_model_spec
```
Description: min_verifier_count is only an upper bound for agreement_threshold, never a minimum of participating verifiers. A model with both equal to 1 finalizes on the first result and pays the whole max_fee to that one operator. The model owner chooses these values, not the requester, and the requester has no field to demand a threshold.
Impact: A requester pays for "agreement" that can be a single operator, and the owner of a model can run it as its own operator. Naming and documentation mismatch more than a code defect.
Known: none.

### P32-F17
File: src/ai_inference/i18n.rs (whole file), src/ai_inference/social.rs:72-126, 158-162, src/ai_inference/metrics.rs, src/ai_inference/storage.rs, src/ai_inference/mod.rs:62-78, 215-283
Severity: Low
Axis: dead code
Confidence: Certain
Evidence:
```
rg -n 'i18n::|OfflineLocalizationCatalog|AccessibilityTranscript|federated_ai_output_event|tag_ai_nft|AiMetrics\b|AiDatasetStorageDeal|prepare_ai_overview' src crates --glob '!src/ai_inference/**' --glob '!src/tests/**'
  -> no output
```
Description: 647 lines of i18n.rs (marked "WIRING: unwired"), FederatedAiOutputEvent and to_json, tag_ai_nft, AiMetrics (new, summary, record_slash), AiDatasetStorageDeal and register_operator (with the MIN_OPERATOR_BOND floor of 1000) have no production caller. The baselines list register_operator, TrainingDataGrant, AiMetrics members, query and storage items, but not the i18n module, the federation event, tag_ai_nft, AiMetrics::summary and record_slash.
Impact: Unreviewed surface that is compiled into the node; register_operator's bond floor is not what the executor enforces (executor uses required_ai_bond).
Known: partly (dead-pub-api-baseline.txt, idle-code-baseline.txt); i18n and the federation event are not listed.

### P32-F18
File: src/ai/registry.rs:1492-1500, 1885-1897; src/ai/registry.rs:399-401; src/ai_inference/verify.rs:56-65; src/ai/execution/guest.rs:558-576
Severity: Info
Axis: invariant(Z8)
Confidence: Certain
Evidence and notes:
- f64 in verifiers_by_reliability and agents_by_trust_score (partial_cmp with unwrap_or(Equal)). Not in the state root, RPC only; Z8 scope list does not include src/ai. No action unless the scores start to gate eligibility.
- result_nonce is checked only for non-zero (registry.rs 399-401) and not stored for replay, and AiInferenceResult.signature is never verified in the registry (calculate_signing_hash is in the dead-pub baseline). Authority comes from the executor, which sets res.verifier = tx.from (executor.rs 1550). Fine today, a trap if a second caller of submit_result appears.
- verify_inference_stark (ai_inference/verify.rs 56) decodes proof bytes with bincode and no size bound; it has no production caller and a lock test stops one from being added. Keep it that way or add the bound.
- validate_gas_budget is wired with u64::MAX on purpose (guest.rs 528-576, executor.rs 2304-2309), so gas estimates are never enforced; documented in the code, listed here for completeness.
- Four `#[allow(clippy::too_many_arguments)]` in this part (Z3 count only, each tied to a signature, none hides a bug).
Known: baselines for the dead symbols.
