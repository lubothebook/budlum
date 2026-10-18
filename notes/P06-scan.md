# P06 scan (cross_domain evm adapter, sync_committee, verify; external domain_bridge, ethereum, evm_hybrid, prover, zkvm_proof, profile)

Scope read in full: src/cross_domain/evm/{adapter.rs 921, sync_committee.rs 1418, verify.rs 982}, src/cross_domain/external/{domain_bridge.rs 317, ethereum.rs 594, evm_hybrid.rs 545, prover.rs 249, zkvm_proof.rs 589, profile.rs 220}. The 1100 lines of hex test constants in sync_committee.rs (known-answer data) were skimmed by shape only.
Read-only. No cargo run. Excluded as already known: FINDINGS.md Part 7 P05-F03 (mpt.rs keccak path), P05-F04 (header.rs parent hash and number only), STATUS.md 4.5.
Counts: Critical 0, High 2, Medium 9, Low 4, Info 1 (16 findings, P06-F01 to P06-F16).

### P06-F01
File: src/cross_domain/external/ethereum.rs:393-406 (verify), :88-145 (layout); src/cross_domain/external/intake.rs:64-131 (PairingBls)
Severity: High
Axis: security
Confidence: Certain (code behaviour); the admission gate that decides who may register an EthereumSync domain is outside P06 and sets the precondition
Evidence: ethereum.rs:399-406
    verifier.verify_aggregate(&self.signing_root, &update.aggregate_pubkey, &update.signature, signers)
intake.rs:121-125 (PairingBls)
    let h_msg = crate::chain::finality::hash_to_g2(signing_root);
    ... multi_miller_loop(&[(&pk, &h_msg.into()), (&g1_gen_neg, &sig.into())])
ethereum.rs:141-143 `NEXT_COMMITTEE_ROOT` is parsed into `SyncCommitteeUpdate.next_committee_root` and read nowhere (rg -n next_committee_root src: only ethereum.rs parse and struct).
Description: The aggregate public key that the signature is checked against is a field of the submitted payload (layout::AGGREGATE_PUBKEY). Nothing ties it to a known sync committee (no committee root, no stored period state, no Merkle proof of membership). `participants` is only a count and is never used to rebuild the key from a committee, and PairingBls comments say committee membership "belongs to the adapter's layout", but the adapter does not check it.
Impact: Whoever can submit external evidence for an admitted EthereumSync domain generates their own BLS key pair, sets 342 or more bits in the bitmap, signs the adapter's fixed signing root with their own key and puts their own public key in the payload. The check passes. The attestation then carries any height and state_root. The path is submit_verified_domain_commitment -> verify_domain_commitment_finality (blockchain.rs:1582, 1660 ff) -> external-root anchor write (AR-GE-6). A forged anchor feeds the executor relayer gate.
Trigger: Precondition: a domain admitted with AdapterSpec::EthereumSync. Entry: Blockchain::submit_verified_domain_commitment (blockchain.rs:1576). Input: 376-byte payload with attacker key at bytes 88..136, attacker signature, 43 or more bytes of 0xff in the bitmap, chosen state_root at 344..376 and the same value in declared_root. Expected wrong result: FinalityStatus::Finalized for a state the Ethereum chain never had.
Known: none (STATUS.md 4.5 lists "three of four mainnet domains cannot finalize" only as a class, no detail)

### P06-F02
File: src/cross_domain/external/ethereum.rs:255-257, 299-306, 344-376, 399-425
Severity: High
Axis: security
Confidence: Certain
Evidence: ethereum.rs:254-257
    /// The signing root is supplied per call by the caller that owns the
    /// beacon chain's fork version and genesis validators root.
    signing_root: [u8; 32],
The adapter is built with `EthereumSyncAdapter::new(network, *signing_root, bls)` (intake.rs:177) from a constant stored in AdapterSpec::EthereumSync. `verify` passes `&self.signing_root` to the verifier, never a value derived from the update.
Description: The signed message is one constant per adapter. It is not derived from finalized_root, attested_slot, finalized_slot or state_root. No finality branch (finalized header inside the attested header's state) and no branch from finalized_root to state_root exist, so state_root (the value Budlum commits to) is a free payload field. `depth = attested_slot - finalized_slot` is also payload data and is not a depth of anything.
Impact: Even with a bound committee key (F01 fixed), one genuine signature over the constant signing root, once observed, validates every later payload with any state_root, slot and bitmap. Replaying a genuine Ethereum signature is enough. The comparison with verify.rs shows the right design: sync_committee.rs rebuilds the signing root from the beacon header fields and walks a branch (verify_execution_block_finality), the external adapter does neither.
Trigger: Precondition as F01 plus one signature observed (the mainnet data are public). Entry ethereum.rs:313 (verify). Input: payload with own state_root and slots, signature and key copied from the earlier valid update, bitmap with 342 set bits. Expected wrong result: attestation with attacker-chosen height and state_root.
Known: none

### P06-F03
File: src/cross_domain/external/ethereum.rs:361-425; src/cross_domain/external/spec.rs:315-324 (field), :335-342
Severity: Medium
Axis: security
Confidence: Certain for require_slashable and for the stored `now`; Likely for the age effect (missing step: confirm the value of `now` the admission path stores in entry.policy)
Evidence: rg -n 'require_slashable' src --glob '!src/tests/**' gives only spec.rs:324,341,354,363 (declaration and constructors). rg -n 'max_age' shows the only use is ethereum.rs:417-419. blockchain.rs builds the bridge with `entry.policy.clone()`, a stored value, and `VerificationPolicy::strict(now)` fixes `now` at construction.
Description: policy.require_slashable is never checked by any adapter or by the bridge, so a policy that refuses unslashable backing still accepts the sync-committee set (`slashable: false`, ethereum.rs:445). policy.now is a number frozen in the stored policy while the consensus path verifies at later heights, so `age = now - finalized_slot` (saturating) does not measure staleness at verification time, and max_age defaults to 0 in both constructors (check off).
Impact: A domain or operator relying on require_slashable or max_age gets no protection; an old but once-valid update is accepted.
Known: none

### P06-F04
File: src/cross_domain/external/domain_bridge.rs:108-160
Severity: Medium
Axis: security
Confidence: Likely (missing step: confirm in accept_domain_commitment and the consumers of event_root, tx_root and domain_block_hash that none of them re-binds these fields)
Evidence: domain_bridge.rs:138-148 compares only `attestation.height != commitment.domain_height` and `attestation.state_root != commitment.state_root`; line 154 compares the proof hash. For comparison finality_adapter.rs:252 `|| target.event_root != commitment.event_root` is used by the in-tree adapters.
Description: For an external domain only height and state_root are bound to the attestation. domain_block_hash, parent_domain_block_hash, tx_root, event_root, validator_set_hash, sequence, timestamp_ms, producer and state_updates of the commitment are free. blockchain.rs:1566 stores `com.domain_block_hash` as `last_committed_hash`. The ZK payload carries event_digest and state_writes_digest (inputs) that zkvm_proof.rs does not expose in the attestation and the bridge never compares.
Impact: A submitter who holds one valid evidence for (height, state_root) can attach any block hash, event_root and tx_root to it. Later message proofs or chain-link checks that rely on those fields rest on unattested data.
Trigger: Precondition: valid external evidence for the height. Entry domain_bridge.rs:108. Input: commitment with matching height and state_root and different domain_block_hash/event_root. Expected wrong result: Finalized.
Known: none

### P06-F05
File: src/cross_domain/external/zkvm_proof.rs:202-263
Severity: Medium
Axis: security
Confidence: Likely (missing step: whether bud-proof's canonical program set constrains block_height, final_state_root and chain_id so that a prover cannot choose them; that is P12 to P16 scope)
Evidence: zkvm_proof.rs:209-216 checks only declared_height == inputs.block_height and declared_root == inputs.final_state_root, then `bud_proof::DefaultAdapter::verify(&decoded.envelope, &decoded.inputs, &decoded.program)`. The attestation takes `height: decoded.inputs.block_height, state_root: decoded.inputs.final_state_root`. No use of inputs.chain_id, initial_state_root, sender, nonce, event_digest, state_writes_digest or program_hash against an expected program.
Description: The module header calls the three-way binding "the whole guarantee", but no field ties the proof to this network, to a pinned program, or to the previous attested root. Any canonical-set program run with chosen public inputs attests whatever height and root those inputs name. Continuity (initial_state_root equals the last attested root of this domain) is not checked.
Impact: If the AIR allows the public inputs to be chosen by the prover, the Trustless domain is forgeable by anyone, and an old height with a different root can be re-attested. If the AIR constrains them to the execution of a domain-specific program, this is only a missing program pin. Either way the code gives no domain-specific anchor.
Known: none

### P06-F06
File: src/cross_domain/evm/adapter.rs:153-172, 328-357, 221-272; src/relayer/worker.rs:735-754; src/execution/executor.rs:1027-1037
Severity: Medium
Axis: integration
Confidence: Certain
Evidence: adapter.rs:166-171 `generate_receipt_proof` always returns Err; :332 `submit_transaction` always returns Err(SubmissionFailed); :343-348 `wait_for_confirmation` calls generate_receipt_proof and propagates the Err. worker.rs:735-754 calls `wait_for_confirmation` and only then `adapter.verify_observation(&result)`. executor.rs:1036 refuses Ethereum relay results with "relayer_evm_package_required". .github/idle-code-baseline.txt:1401 lists `verify_deposit` as idle; .github/unwired-guards-baseline.txt:43 lists it.
Description: With EvmChainAdapter registered through --evm-bridge-address, the worker cannot get past submit_transaction/wait_for_confirmation, so the verify_observation override, verify_deposit and verify_evm_receipt run only in tests. The consensus side refuses Ethereum results. The Ethereum deposit path therefore has no live end to end route.
Impact: The bridge leg Ethereum to Budlum does not operate; a reader of the comments ("the real safe path", "the production path") would believe it does. Mainnet blocker class.
Known: idle-code-baseline.txt (verify_deposit, SyncAttestation, test_default, DEFAULT_DEPOSIT_TOPIC0); unwired-guards-baseline.txt:43. The end-to-end unreachability is not stated in the baselines.

### P06-F07
File: src/cross_domain/evm/adapter.rs:244-254; src/cross_domain/evm/verify.rs:177, 192-204, 293-301
Severity: Medium
Axis: security
Confidence: Certain
Evidence: adapter.rs:253 `sync_attestation: None,` is the only construction of EvmDepositProof outside tests (rg -n 'EvmDepositProof \{' src). DepositProofPackage has no attestation field (verify.rs:193-204). rg -n 'SyncCommitteeState' src outside evm/ and tests gives nothing; `rotate_period` has no caller (dead-pub-api-baseline.txt:123).
Description: Header authenticity is never checked in the production verifier: the target and confirmation headers are accepted on parent hash and number alone (P05-F04 covers header.rs; this finding covers that the strong path that would repair it, verify_execution_block_finality, cannot be reached because the adapter hard-codes None and no state source exists). verify.rs:5-6 and the module text still say the chain is verified.
Impact: A relayer can fabricate an entire header chain of the required length with any receiptsRoot and a receipt MPT built under it; verify_observation returns Ok. Today closed by F06 (executor refusal), so Medium; it becomes High when the EVM leg is opened.
Trigger: Precondition: EvmChainAdapter registered and the executor refusal lifted. Entry adapter.rs:221. Input: DepositProofPackage with synthetic headers. Expected wrong result: a deposit accepted that never happened on Ethereum.
Known: related to P05-F04 (header.rs), not the same item.

### P06-F08
File: src/cross_domain/evm/sync_committee.rs:20-23, 410-439, 441-446
Severity: Medium
Axis: correctness
Confidence: Likely (missing step: check against the Ethereum consensus-spec light-client sync protocol, where finality requires finality_branch from the attested header's state to the finalized header; this repo carries no such branch)
Evidence: sync_committee.rs:20-21 "the sync committee ... gives real PoS finality"; verify_execution_block_finality checks only that the signed beacon header's body holds the execution block hash (branch gindex 812) and a pairing over that header. BeaconBinding has no finality branch and no check that the header is at or below the finalized checkpoint. rotate_period (:443) swaps `next_sync_committee` in with no proof that it is the chain's next committee.
Description: A sync aggregate signs the head of the chain; two thirds signing a head block does not make it final. The module treats a signed head as finalized. Committee trust root and rotation are also unproven: SyncCommitteeState is an unauthenticated input, and next_sync_committee is never verified by a branch to the finalized state.
Impact: A deposit block that is later reorged away can still carry a valid sync aggregate. Weaker than the "PoS finality" the documents and VerifyError::SyncCommittee text claim.
Known: none

### P06-F09
File: src/cross_domain/evm/sync_committee.rs:355-358, 430-436, 281-292
Severity: Low
Axis: correctness
Confidence: Likely (missing step: confirm with the Altair spec that the committee is chosen by the signature slot's period, which is the attested slot plus one)
Evidence: sync_committee.rs:356 `self.slot / SLOTS_PER_SYNC_COMMITTEE_PERIOD` with binding.slot being the signed (attested) header slot; :431 compares it to state.current_period.
Description: The committee that signs is the one in force at the signature slot (the block after the signed header). For the last slot of a period the code selects the wrong period and refuses a genuine aggregate. BeaconChainParams has one fork_version; the version for the signed slot changes at each fork, so a fixed value refuses every aggregate after a fork until the config is changed.
Impact: Fail closed, liveness only, at period and fork boundaries.
Known: none

### P06-F10
File: src/cross_domain/evm/verify.rs:245-314; src/cross_domain/evm/adapter.rs:255-272; src/cross_domain/evm/receipt.rs:151-155 (find_log)
Severity: Medium
Axis: correctness
Confidence: Certain for the code facts; the impact depends on the future consumer of VerifiedDeposit (none exists, see F06)
Evidence: rg -n 'PayloadMismatch|deposit_log_data' src: the variant is declared (verify.rs:59) and displayed, never constructed; deposit_log_data is set (verify.rs:310) and read only by tests. adapter.rs:255-272 uses `verified.receipts_root` only and drops the rest. find_log (receipt.rs:151) is `.iter().find(...)`, first match only. VerifiedDeposit identity is (block_hash, receipt_key) without a log index.
Description: A successful verification proves "some log from the bridge address with topic0 exists in some successful receipt". The observation is not tied to the transaction the worker broadcast: tx_hash is compared only against two relayer-supplied strings (adapter.rs:233, worker.rs:738), and the log data (amount, recipient) is compared with nothing (ExternalTransaction.payload is never compared). A receipt with two deposit logs yields only the first, and a replay set keyed on (block_hash, receipt_key) conflates both logs.
Impact: A relayer can present any older bridge deposit receipt as the observation of the transaction it broadcast. A later mint consumer keyed on (block_hash, receipt_key) would pay one of two deposits, or pay the same log under another tx_hash intent.
Known: none

### P06-F11
File: src/cross_domain/external/evm_hybrid.rs:183-202, 210-250; contracts/external-domain/BudlumFinalityVerifier.sol:92-112, 423-445
Severity: Medium
Axis: correctness
Confidence: Likely (missing step: run the pairing equation of the Solidity _verifyBls in P60 scope; here it is read from the comments and field types)
Evidence: evm_hybrid.rs fields `g1_hashed_message` (128), `g2_aggregate_pubkey` (256), `g1_generator` (128), `g2_negated_signature` (256). Solidity: "e(H(m), agg_pubkey) * e(-signature, g1_generator) == 1" with H(m) in G1, pubkey in G2, signature in G2. Rust side (sync_committee.rs:6-10, intake.rs:55-63): public keys in G1, signatures and hash-to-curve in G2.
Description: The group assignment of the EVM proof does not describe either BLS variant. With H(m) in G1 and the key in G2 the signature must be in G1 and pair with the G2 generator; here the signature is in G2 and pairs with the G1 generator. Ethereum sync aggregates (key G1, signature G2, H(m) in G2) cannot be put in this struct. The structure also supplies the hashed message point, the generator and the key from the caller (the planner's `MessageBinding` is a flag the caller states).
Impact: Either no genuine attestation can verify (liveness), or, because the caller provides H(m), the key and the signature, an equation of the form e(H, PK) = e(G1, SIG) can be satisfied with chosen points (for example H = G1 and SIG = PK) when the contract does not pin the key and the message point. P60 must decide which. In the planner this makes FullCryptographic a statement about declared flags.
Known: none

### P06-F12
File: src/cross_domain/external/prover.rs (whole file); rg -n 'ProverBond|DomainEconomics|honesty_is_cheaper' src
Severity: Medium
Axis: integration
Confidence: Certain
Evidence: rg for ProverBond, DomainEconomics, ProverFee, ChallengeReward, required_bond_atoms, honesty_is_cheaper outside prover.rs finds only the re-export list (external/mod.rs:107-108) and mod.rs unit tests (mod.rs:674-780). No call in chain/, execution/, registry/ or rpc/. No code applies slash(), pays a ProverFee or accepts a fraud proof.
Description: The module describes bonds, slashing by fraud proof, fees and unbonding, and the module header says the bond "is slashed when an accepted attestation is contradicted". None of it is wired. Domain admission (intake.rs) takes bond_atoms as data and does not call DomainEconomics::admits.
Impact: The economic backing shown in the DomainProfile (bond_atoms, bond_posters) is not enforced or even produced by a prover role; a domain operator reading the documents would expect a deterrent that does not exist. Mainnet blocker class under CLAUDE.md section 1 (public item without production caller).
Known: none (not in dead-pub-api-baseline.txt or idle-code-baseline.txt: unreachable only through the re-export and tests)

### P06-F13
File: src/cross_domain/external/prover.rs:92-111, 185-192, 227-249
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: prover.rs:189-190 `let part = ... / BPS_DEN; part.max(self.floor_atoms)`; slash() takes `challenge_reward_atoms` as an argument and stores it with no relation to `taken`; prover.rs:245-248 `let gain = economics.fee.for_value(value_atoms); let loss = economics.penalty_for(value_atoms, economics.required_bond_atoms()); gain < loss`.
Description: (1) The floor can make the reward larger than the slashed amount, contrary to the header text that the reward "comes out of the slashed bond rather than from nowhere"; slash() does not cap it. (2) honesty_is_cheaper compares the prover's fee with the penalty, but the profit of a false attestation is the value it diverts (bounded by value_atoms), not the fee. The bond is 10 percent of the ceiling, so for value above the bond the penalty is the bond while the attacker's gain is the value. The function returns true for exactly the case it is meant to catch.
Impact: Wrong economic safety signal in any future caller. Unwired today (F12), so Low.
Known: none

### P06-F14
File: src/cross_domain/external/zkvm_proof.rs:86-108; src/cross_domain/evm/adapter.rs:222-232
Severity: Low
Axis: security
Confidence: Likely (missing step: confirm the bincode version's `deserialize` default accepts trailing bytes; the zkvm decode uses the top-level `bincode::deserialize`, while the adapter uses `bincode::options()`)
Evidence: zkvm_proof.rs:96 `bincode::deserialize(payload)` (the top-level function accepts trailing input), after only a length cap of 12 MiB (:62). adapter.rs:222 uses `bincode::options().with_fixint_encoding().with_limit(...)`, which rejects trailing bytes.
Description: ZkFinalityEvidence payloads with arbitrary trailing bytes decode to the same value but hash to a different evidence.digest() and a different finality_proof_hash. A valid proof can be re-wrapped into many distinct commitments' proof hashes. The 12 MiB cap also has no bincode limit, so memory is bounded only by the cap.
Impact: Duplicate attestation records for the same proof and evidence_digest-based dedup (if added) can be bypassed; resource use per verify up to 12 MiB decode plus the STARK verify.
Known: none

### P06-F15
File: src/cross_domain/evm/adapter.rs:1; src/cross_domain/evm/sync_committee.rs (test fn mainnet_kat_state, attribute allow large_stack_arrays); src/cross_domain/evm/adapter.rs:742-786
Severity: Low
Axis: invariant(Z3) and test
Confidence: Certain
Evidence: adapter.rs:1 `#![allow(clippy::pedantic, clippy::nursery)]` (a module-wide suppression with no reason text, whole file including production code); sync_committee.rs test helper `#[allow(clippy::large_stack_arrays)]` with a comment as reason. adapter.rs:744-745 `include_str!("adapter.rs")` and `include_str!("../../relayer/worker.rs")`, then `contains("verify_observation(&result)")`. verify.rs `verify_does_not_panic_on_garbage` (verify.rs:675-690) and sync_committee `garbage_aggregate_does_not_panic` end in `let _ = ...` with no assertion on the result.
Description: The file-level allow turns off two lint groups for the production code of the EVM adapter; the module test `the_relayer_reaches_verify_deposit` checks source text instead of behaviour (it passes if the string occurs in a comment-stripped line, even when the branch is unreachable, see F06); two garbage tests would still pass if verification returned Ok.
Impact: Maintenance risk and false assurance on an R3 path. Remaining negative tests (wrong emitter, topic, floor, root, tx) are real.
Known: none checked against P62 output (FINDINGS.md has no entry for these files)

### P06-F16
File: src/cross_domain/evm/adapter.rs:343-357
Severity: Info
Axis: LABEL-12.9
Confidence: Certain
Evidence: adapter.rs:354 `receipt_proof: bincode::serialize(&proof).unwrap_or_default(),`
Description: unwrap_or_default on a serialize result in a result constructor. Unreachable today because generate_receipt_proof returns Err first (line 348).
Impact: none now. Label only.
Known: none
