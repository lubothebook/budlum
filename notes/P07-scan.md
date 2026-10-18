# P07 scan: src/cross_domain/external/{mod,intake,intake_quorum,quorum,registry,selftest,spec,versioning}.rs

Scope read in full: the eight files above (8064-line directory, these eight about 5300 lines). Also read for call paths only: ethereum.rs:250-440, rpc/server.rs:1640-1960, chain/blockchain.rs:2905-3215, storage/db.rs:1064-1087. Not repeated: STATUS.md section 4.5 (consensus state written outside blocks, external roots not in header, etc.).
Count: High 3, Medium 8, Low 8, Info 2. Total 21.

### P07-F01
File: src/rpc/server.rs:1712-1722 (entry); src/cross_domain/external/registry.rs:638-701 (check); src/cross_domain/external/intake.rs:379-405
Severity: High
Axis: security
Confidence: Certain (call chain read); Likely for public-listener exposure (missing step: check which listener serves bud_submitExternalEvidence)
Evidence: server.rs:1712 `async fn submit_external_evidence(&self, evidence: serde_json::Value)` has no `self.require_operator(...)` line, unlike every sibling (1654, 1806, 1825, 1840, 1888, 1920). The identity used for bond, vote and slashing is a plain field: spec.rs:125 `pub submitter: Address` inside the unsigned `RawConsensusEvidence`. registry.rs:692 `let Some(bond) = prover else { return Err(UnknownProver ..)}` looks up `reg.provers.get(&evidence.submitter)`.
Description: Anyone who can reach the endpoint can submit evidence naming any bonded address (poster addresses are public via the domain registration RPC) as submitter. Nothing proves the caller controls that address.
Impact: One caller can impersonate every bonded prover, so "one bond, one voice" in intake_quorum is void: a single actor reaches any quorum threshold alone, or fills a round with refusal votes under other provers' names. Attestations are recorded with the victim as carrier (registry.rs:430, 540), and the carrier is the slash target (registry.rs:785-795). Each call also rebuilds the adapter and persists plus flushes the whole intake (blockchain.rs:2951, db.rs:1068-1070), a cheap amplification.
Trigger: precondition: one registered domain with a bonded prover address A. Entry: server.rs:1712. Input: RawConsensusEvidence with submitter = A and a payload that is valid (for a vote-based domain) or garbage (to cast a refusal). Expected wrong result: the evidence is processed as A's own, counters and rounds move under A.
Known: none (related class only: STATUS.md 4.5 "consensus state written outside blocks")

### P07-F02
File: src/cross_domain/external/intake.rs:64-131 (PairingBls), src/cross_domain/external/ethereum.rs:393-406
Severity: High
Axis: security
Confidence: Certain for the code facts; Likely for end impact (missing step: read domain_bridge.rs and its consumers to see what an attestation unlocks)
Evidence: ethereum.rs:399-405 `verifier.verify_aggregate(&self.signing_root, &update.aggregate_pubkey, &update.signature, signers)` where `signing_root` is the constant held by the adapter (set at registration, intake.rs:142-145) and the key and signature are parsed from the payload. intake.rs:121-128 checks only `e(pk, H(signing_root)) == e(g1, sig)` with subgroup checks. The doc at intake.rs:56-63 says committee membership "belongs to the adapter's layout"; grep shows `next_committee_root` is parsed (ethereum.rs:237) and never compared to anything.
Description: The pairing check is over a fixed message and a key the prover supplies, and nothing ties the key to a known sync committee, the participation bits to the key, or the signed message to the finalized header or state root. Anyone can pick a secret scalar s, set pk = s*G1 and sig = s*H(signing_root), set the bitvector to all ones, and put any state_root in the payload.
Impact: A forged Ethereum finality attestation (arbitrary height and state root, SignatureSet with 512 signers) passes registry.check and is stored in the attestation book and committed to the digest. Any consumer that treats the attestation as finality is deceived. Severity could be Critical if value routing depends on it.
Trigger: precondition: a registered EthereumSync domain and a submitter address with a bond (see F01, spoofable). Entry: intake.rs:379. Input: 376-byte payload with self-made key and signature as above. Expected wrong result: attestation accepted.
Known: none

### P07-F03
File: src/cross_domain/external/registry.rs:452-471
Severity: High
Axis: correctness
Confidence: Certain
Evidence: registry.rs:452-466
`Err(err) => { ... if !already_faulted && reg.record.state.serves() { ... reg.record.state = DomainState::Faulted; ...} ... Err(err)`
Description: In the single-submission path every refusal of any kind faults the domain: UnknownProver (an address with no bond), DuplicateAttestation (a replay of accepted evidence), version-gate refusals, Stale, InsufficientDepth, malformed payload. The fault is a property of the caller's bad input, not of the domain.
Impact: Any caller (F01 removes the need for a bond) can move any Admitted or Active domain to Faulted with one malformed request. intake.rs test submitted_evidence_reaches_the_framework_registry_and_its_rules shows the refusal counter moves for an unbonded submitter. Recovery is blocked by F05.
Trigger: precondition: a domain in Admitted or Active state without a quorum policy. Entry: server.rs:1712. Input: any evidence that is refused, e.g. a one-byte payload. Expected wrong result: domain state Faulted, history entry with the error text.
Known: none

### P07-F04
File: src/cross_domain/external/registry.rs:51-52, 638-754; src/cross_domain/external/intake.rs:403, 461
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence: `rg -n 'NotServing|\.serves\(\)' src/` returns registry.rs:52 (definition), registry.rs:457 and 622 (only state transitions), mod.rs:665 (test). `check` (registry.rs:638) never reads `reg.record.state`.
Description: The Faulted state does not stop anything. submit, evaluate and commit_attestation keep accepting attestations for a Faulted domain; `NotServing` is never constructed. The docs (registry.rs:1-17, quorum.rs:10-11, intake_quorum.rs:34-36) say the domain "must stop serving" after a dispute.
Impact: After a quorum dispute (two bonded provers with different valid finalized headers) the domain stays writable, and a later round or single submission can still commit an attestation. The only effect of the fault is the label seen by readers.
Trigger (security): precondition: a Faulted domain. Entry: intake.rs:379. Input: valid evidence at a fresh height. Expected wrong result: attestation committed while state is Faulted.
Known: none

### P07-F05
File: src/cross_domain/external/registry.rs:337-371; src/chain/blockchain.rs:3022-3048
Severity: Medium
Axis: correctness
Confidence: Likely (missing step: run readmit_external_domain on a registered domain in a test; the derivation below says it always fails)
Evidence: registry.rs:354-360 `let digest = admission.digest(); if digest == reg.admission_digest { return Err(RegistryError::AdmissionDigestMismatch {..}) }`. blockchain.rs:3033-3037 rebuilds the adapter from the stored spec and golden and calls `admit(adapter.as_ref(), &entry.policy)`, the same inputs as registration (intake.rs:329-330).
Description: Admission is deterministic (compiled-in probes, same golden, same policy), so the re-run report has the same digest as the registered one and readmit refuses it. The error name is also inverted: a digest that is equal is reported as "does not match".
Impact: The only exit from Faulted (F03, quorum Disputed) is unreachable until the binary's probe set changes. Combined with F01 and F03, a domain can be killed permanently by an anonymous request. No test calls readmit (F12).
Known: none

### P07-F06
File: src/cross_domain/external/intake.rs:521-545; src/cross_domain/external/quorum.rs:131-155, 231-282
Severity: Medium
Axis: security
Confidence: Certain
Evidence: set_quorum_policy only refuses the two AcceptMostCommon behaviours (intake.rs:535-542). `strict(agreement_threshold: usize, max_participants: usize)` and the struct fields take any value. quorum.rs:245 `if valid < policy.agreement_threshold` and :259 `g.count >= policy.agreement_threshold` with threshold 0 make every non-empty group a winner.
Description: No validation of threshold against max_participants or against the number of bonded provers. Threshold 0 or 1 lets one prover decide; threshold at or below half of max_participants lets the first group to reach it win a race even when a larger honest group would form (intake_quorum.rs:300-302 decides on every submission); threshold above max_participants or above the bonded count can never decide and ends Disputed (intake_quorum.rs:347); max_participants 0 yields RoundFull for everyone; max_participants of usize::MAX voids the "memory bound" claimed in quorum.rs:136. Policies loaded from storage are not rechecked.
Impact: A mistaken operator call (or a malicious one, the call is operator-only) turns "quorum" into single-prover trust or a permanently dead domain without any warning.
Known: none

### P07-F07
File: src/cross_domain/external/intake_quorum.rs:251-271, 357-363; src/cross_domain/external/intake.rs:424-444
Severity: Medium
Axis: security
Confidence: Certain
Evidence: intake_quorum.rs:261-271 `let height = evidence.declared_height; let round = self.rounds.entry(RoundKey { domain, height }).or_insert_with(...)`. Adapter refusals are folded in as answers (intake_quorum.rs:291, 389-397), including DeclarationMismatch and Malformed.
Description: Rounds are keyed by a prover-chosen number and a refusal is a vote, so a bonded prover opens a new retained round for every garbage height it submits, each kept for ROUND_RETENTION_BLOCKS (10,000) and included in every state_digest and every persist. Also, a refusal quorum (AgreedRefusal) for a height blocks that height for all later honest evidence (`RoundClosed`, line 273) until the sweep, although the refusal only says that those particular bytes were bad. With F01 two spoofed identities are enough.
Impact: Unbounded state growth bounded only by call rate, and height-level censorship of an attestation for up to 10,000 local blocks, repeatable after each sweep.
Trigger: precondition: quorum policy with threshold 2 and two bonded addresses (spoofable, F01). Entry: server.rs:1712. Input: two evidence values with the true external height and corrupt payloads. Expected wrong result: round AgreedRefusal, honest evidence for that height answered with RoundClosed.
Known: none

### P07-F08
File: src/cross_domain/external/intake.rs:391-403 (policy source); src/cross_domain/external/spec.rs:317-321, ethereum.rs:417-425
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence: intake.rs:392 `let policy = entry.policy.clone();` is the policy stored at registration (intake.rs:350-355). spec.rs:318-321 `now: u64` "supplied by the caller". `rg -n '\.now' src/cross_domain/external/intake.rs src/chain/blockchain.rs` finds no update. ethereum.rs:418 `let age = policy.now.saturating_sub(update.finalized_slot);`
Description: `now` is frozen at the value given at registration, so `max_age` is measured against a stale clock forever. For newer evidence the subtraction saturates to 0 and the Stale check can never fire; max_age is dead in practice.
Impact: A registrant who sets max_age to demand fresh evidence gets no freshness: old (but valid) updates stay acceptable and fresh ones are indistinguishable.
Known: none

### P07-F09
File: src/cross_domain/external/intake.rs:240-246; src/cross_domain/external/registry.rs:97-98; src/storage/db.rs:94-97; src/chain/blockchain.rs:844-846
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence: intake.rs:244-246 `/// ... default keeps old serialized states readable ... #[serde(default)] pub quorum: QuorumRounds`; db.rs:95 `bincode::serialize(value)`; blockchain.rs:844 `if let Ok(Some(stored_external_intake)) = store.load_external_intake() { ... }`.
Description: Storage is bincode, which is not self-describing, so `serde(default)` on a missing trailing field does not apply; a state written before `quorum` or `attestation_provers` existed fails to decode. The load site swallows the error and keeps the empty intake.
Impact: After an upgrade a node can silently start with no external domains and no bonds, then disagree with peers on `bud_getExternalIntakeDigest`. The doc comment gives false assurance. (Load site is in blockchain.rs, outside P07; flagged here because the claim sits in intake.rs.)
Known: none

### P07-F10
File: src/cross_domain/external/registry.rs:173-293; src/rpc/server.rs:1651-1653
Severity: Medium
Axis: integration
Confidence: Certain
Evidence: server.rs:1651-1652 `// Operator-only: the bond figure in this build is a declared number, not a signed stake transfer`. registry.rs:191-192 `bond_atoms: u128, poster: Address` are stored as given; bond_prover (registry.rs:573-605) likewise. slash (registry.rs:796-802) only edits the stored ProverBond.
Description: No balance is locked, moved or burned by registration, bonding or slashing. The economics that the "honesty_is_cheaper" gate (intake.rs:326) reasons about are bookkeeping.
Impact: Mainnet blocker class: the security argument of the whole external-domain design (bond scaled to ceiling, challenger reward from the slash) has no money behind it. The challenge reward is a number nobody receives.
Known: none (adjacent to STATUS.md 4.5 "stake bonding" only in a different subsystem)

### P07-F11
File: src/cross_domain/external/registry.rs:764-803
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: registry.rs:776-801 checks that the digest was accepted and who carried it, then `bond.slash(height, evidence_digest, penalty, ...)` with no check of `bond.slashings` for the same digest; the accepted attestation stays in `reg.attestations`.
Description: The same evidence digest can be slashed repeatedly until the bond is empty, and a slashed attestation keeps being returned by latest_attestation (registry.rs:113, used at server.rs:2226).
Impact: Operator-only call, so the risk is a repeated or mistaken slash and a contradicted attestation that stays served as valid.
Known: none

### P07-F12
File: src/cross_domain/external/registry.rs (whole file, 823 lines, no `#[cfg(test)]`)
Severity: Medium
Axis: test
Confidence: Certain
Evidence: `grep -c '#\[test\]'` gives registry.rs 0, selftest.rs 0, spec.rs 0 (covered only through mod.rs). `rg -n 'slash\(|readmit|mark_faulted|DuplicateAttestation|ProverUnderbonded|AttestationProverMismatch|set_version_policy|schedule_external_fork' src/tests tests src/cross_domain/external/*.rs` finds no test that calls them in the external tree.
Description: The R3 registry has no direct tests for slash and its carrier check, readmit, fault transitions, the duplicate-slot rule, the underbonded gate, set_version_policy, or a persistence round trip of IntakeState (bincode). The intake tests cover only register, unknown prover, bonding and the happy quorum path. The defects in F03, F05 and F11 would have been found by any of these tests.
Impact: Missing negative tests on an R3 path.
Known: none

### P07-F13
File: src/cross_domain/external/intake.rs:1122-1161
Severity: Low
Axis: test
Confidence: Certain
Evidence: test name `a_split_that_cannot_recover_freezes_the_domain_faulted`; its comment (1146-1152) admits "the round agrees ... To force the dispute, the policy would need threshold 3"; the last assert is only `!matches!(round.state, RoundState::Open)`.
Description: The test named for the dispute-freezes-domain path does not reach Disputed. The intake branch that calls mark_faulted (intake.rs:464-487) and the AgreedRefusal branch have no test through IntakeState; intake_quorum tests cover the round only.
Impact: The dispute-to-fault wiring is untested and the test name misstates coverage.
Known: none

### P07-F14
File: src/cross_domain/external/mod.rs:381-392, 665
Severity: Low
Axis: test
Confidence: Certain
Evidence: mod.rs:381-391 `for (name, _) in &report.probes { let mut corrupted = Tiny.golden_evidence()...; corrupted.payload.clear(); assert!(Tiny.verify(&corrupted, ...).is_err(), ...) }` never uses the probe. mod.rs:665 `assert!(!p.state.serves() || p.state == DomainState::Admitted);` with the fixture state Admitted is always true.
Description: Two assertions that cannot fail: the loop re-checks the same emptied payload four times and the comment claims it checks each probe's corrupted evidence; the staleness test's last assert holds for its own fixture by construction.
Impact: False coverage in the admission and profile tests.
Known: none

### P07-F15
File: src/cross_domain/external/intake.rs:441-489
Severity: Low
Axis: correctness
Confidence: Likely (missing step: check whether the chain actor rolls back or persists after a returned Err; blockchain.rs:2944-2951 persists on both arms)
Evidence: intake.rs:442-444 `self.quorum.submit(...)?` mutates the round, then 461 `self.registry.commit_attestation(&winner, carrier)?;` and 479 `self.registry.mark_faulted(...)?;` can still return Err.
Description: The round is already AgreedClaim or Disputed when a later step fails, and the error path persists the half-applied state. The commit can fail with DuplicateAttestation if the slot was filled by another path; the round then says agreed with nothing committed and refuses all later votes (RoundClosed).
Impact: Stuck round with an inconsistent book until the 10,000-block sweep.
Known: none

### P07-F16
File: src/cross_domain/external/versioning.rs:138-200, 244-265; src/cross_domain/external/registry.rs:314-335
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: schedule_fork never compares new_version with the adapter's `accepted_evidence_versions`; set_version_policy (registry.rs:331-332) overwrites `reg.record.accepted_evidence_versions` with the window list. is_consistent doc (versioning.rs:244-245) says "at least one open window" but the code only checks non-empty, duplicates and sunset >= valid_from.
Description: An operator can schedule a version the adapter does not accept and sunset the one it does, leaving a domain with no usable version; a registration whose only window already sunset is accepted. The profile then advertises versions the adapter refuses.
Impact: Operator-only misconfiguration bricks a domain; with F03 each refusal also faults it.
Known: none

### P07-F17
File: src/cross_domain/external/selftest.rs:214-222, 326-330
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: `&self.probes.len().to_le_bytes()` and `&probes.len().to_le_bytes()` hash a `usize`.
Description: usize is 4 bytes on 32-bit targets and 8 on 64-bit, so the admission digest and probe commitment differ by platform. The digest is compared at readmit and stored in consensus state.
Impact: A 32-bit build would compute different digests for the same admission. Fix is `as u64`.
Known: none

### P07-F18
File: src/cross_domain/external/quorum.rs:112-129, 245-265, 305-314; src/cross_domain/external/registry.rs:51-52
Severity: Low
Axis: dead code
Confidence: Certain
Evidence: `rg -n 'AcceptMostCommon' src/` shows production use only in the refusal at intake.rs:535-539; the behaviours are constructed otherwise only in tests (quorum.rs:496-497, intake.rs:997-1001). `RegistryError::NotServing` is never constructed (F04).
Description: The permissive resolution branches (winner_of, both AcceptMostCommon arms) cannot be reached in production because the only installer refuses them, and `QuorumRounds::set_policy` (pub) does not enforce that refusal itself.
Impact: Maintenance risk and a pub path (`QuorumRounds::set_policy`) that bypasses the guard.
Known: none (baseline files not checked for these symbols; grep of both baselines found no match)

### P07-F19
File: src/cross_domain/external/intake_quorum.rs:357-363; src/cross_domain/external/intake.rs:460
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: intake_quorum.rs:360 `self.rounds.retain(|_, round| local_height.saturating_sub(round.opened_at) <= ROUND_RETENTION_BLOCKS);` applies to every state; intake.rs:460 `.unwrap_or(evidence.submitter)`.
Description: A Disputed round (the evidence for the "challenge game", intake_quorum.rs:99-101) is swept after 10,000 blocks like any other, while the domain stays Faulted. The carrier lookup falls back to the current submitter if the entry is missing, which would credit the closer; the doc says the first carrier.
Impact: The record of who disputed what disappears before anything resolves it; the silent fallback hides a broken invariant instead of failing.
Known: none

### P07-F20
File: src/cross_domain/external/registry.rs:295-337
Severity: Info
Axis: correctness
Confidence: Certain
Evidence: the doc block "Re-admits a domain under a new probe set ..." (295-302) sits above and is merged with the doc of `set_version_policy` (303-313); `pub fn readmit` (337) has no doc; the `# Errors` list under it describes readmit.
Description: Misplaced doc comments, so rustdoc shows readmit's description on set_version_policy.
Impact: Misleading docs only.
Known: none

### P07-F21
File: src/cross_domain/external/mod.rs:39-60; src/cross_domain/external/intake.rs:16-25
Severity: Info
Axis: integration
Confidence: Certain
Evidence: mod.rs:50 "the `GlobalBlockHeader` gains a root when the first real external domain earns one"; intake.rs:584-587 state_digest is "the commitment consumers compare until a header root is earned".
Description: The module headline "Wired through intake" is true for call sites but the attestation is not part of any block or header; the digest is an RPC value only. This is the same class as STATUS.md 4.5 and is noted so the WIRING claim is read with that limit.
Impact: None beyond the known blocker class.
Known: STATUS.md section 4.5 (external roots)
