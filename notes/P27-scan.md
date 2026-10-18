# P27 scan: Storage coding (read-only audit, Tier R3)

Base commit: 6066547. Scanner: Sonnet, read-only. Known and not repeated: STATUS.md section 4.5, docs/audit/FINDINGS.md.

## COVERAGE

Read in full (every line):
- src/storage/erasure.rs 1-1499 (two reads)
- src/storage/lrc.rs 1-583
- src/storage/msr.rs 1-206
- src/storage/living_threshold.rs 1-1278
- src/storage/one_share.rs 1-305
- src/storage/one_view.rs 1-216
- src/storage/payload_crypt.rs 1-301
- src/storage/pact_binding.rs 1-294
- src/storage/dictionary.rs 1-697
- src/storage/mobile_self.rs 1-701
- src/storage/server_admission.rs 1-149
- src/storage/social_delete.rs 1-171

Also read (docs): docs/AGENT_MAP.md 1-56; docs/AUDIT_PLAN.md sections 5 and 6 (lines 110-165); STATUS.md 4.5 and section 5.

Outside the part (rg first, then at most 80 lines): src/domain/storage_deal.rs 1590-1640, 1666-1680, 2075-2115, 2300-2345, 2960-3005 and rg hits for access_events, dictionaries, DEMAND_REPLICA_STEP_SCALED; src/storage/manifest.rs 275-300 and 729-800; .github/idle-code-baseline.txt and .github/dead-pub-api-baseline.txt (rg only); rg for call sites of every public symbol of the part in src/, budzero/, crates/ (tests excluded).

Not read: everything else in src/storage (manifest.rs beyond the ranges above, emit.rs, three_pipe.rs, view_grant.rs, three_hooks.rs, three_visibility.rs), src/account_abstraction (only rg), the rest of storage_deal.rs, src/tests. No cargo, no test run: every claim below comes from reading source. Git history is shallow at the file level (git log -S points to one large import commit), so Z3 attribution could not be tied to an adding commit.

Extra-check summary:
- Erasure threshold: encode, MDS Cauchy generator, reconstruct and column audit are sound on reading (k>=1, k+m<=255, n>=k enforced in ReedSolomon::new and ErasureScheme::validate; empty stripes refused; output re-verified against shard ContentIds). Gaps are in F04, F05, F06.
- Content id binding: shard ids are checked on reconstruct; verify_object_encoding deliberately skips manifest_id (provenance fields). content_size is not bound to the data region (F06). Payload seal is not bound to any content id (F08).
- Persistence format: LrcLayout deserializes through the constructor (good). DictionaryRegistry derives Deserialize with no validation and no version tag (F11). Pact has no Serialize at all (F10). Open with corrupt data: payload_crypt open_payload is correct (length, magic, version, AEAD all checked, no panic path).
- Parts not wired: F03, F04, F05 and the Known baseline list.
- Z3: F09. Z7: no unsafe in the part. Z8: no float in consensus-adjacent code; one f64 in one_share.rs (non-consensus, F12). Z9: no secret material in the part; test keys are literal 0x09 style fills, test-only.

## FINDINGS

### P27-F01
File: src/storage/living_threshold.rs:293-306, 346-372
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence:
```
293  const fn decayed_to(self, epoch: u64) -> u64 {
294      let elapsed = epoch.saturating_sub(self.last_epoch);
295      let halvings = elapsed / ACCESS_HALF_LIFE_EPOCHS;
...
364      let decayed_running = estimate.decayed_to(event.epoch);
365      estimate.scaled = decayed_running.saturating_add(scaled_count);
366      estimate.last_epoch = event.epoch;
```
Description: Decay is floor(elapsed / 720) halvings and the anchor epoch is then moved to the new event, so the remainder of elapsed time is thrown away. Two consecutive events fewer than 720 epochs apart cause zero decay, and the anchor still advances. An object read at least once per 719 epochs never decays; its estimate is the running sum of all reads.
Impact: Production replica target (required_replicas_with_demand, storage_deal.rs:1671-1678, used by the maintenance sweep at storage_deal.rs:3865) overstates demand for any steadily read object, up to the cap STORAGE_REPLICATION_TARGET. All nodes compute the same wrong number, so no fork, but the module's stated contract ("halves every half-life") is false and the discount/ladder is biased upward. Existing tests only use gaps of 720 or more epochs (storage_deal.rs:7640-7652; living_threshold.rs tests), so they do not catch it.
Known: none

### P27-F02
File: src/storage/living_threshold.rs:346-357; src/domain/storage_deal.rs:2970-2982
Severity: Low
Axis: correctness
Confidence: Likely (missing step: confirm no other writer of access_events than record_proven_read)
Evidence:
```
350  if event.epoch < last_epoch || event.epoch > current_epoch {
...
356      break;
```
Description: from_events stops at the first out-of-order or future-dated event and silently drops every later (valid) event. record_proven_read folds late events into the newest entry, so out-of-order input should not arise, but the guard turns any future-dated entry (current_epoch smaller than a logged epoch, e.g. a query with a stale epoch argument) into a silent truncation instead of an error.
Impact: access_estimate(manifest, epoch) with epoch below the newest logged event returns a partial estimate with no signal. Local effect on replica target only.
Known: none

### P27-F03
File: src/storage/dictionary.rs:242-271, 348-394 (header claim at lines 46-49)
Severity: Medium
Axis: integration
Confidence: Certain
Evidence (rg -n 'register_dictionary|release_dictionary|delete_dictionary|deletable_dictionaries|dictionary_root|\.dictionaries' src, outside dictionary.rs):
```
src/domain/storage_deal.rs:1624:  self.dictionaries.has_dictionary(&manifest.manifest_id);
src/domain/storage_deal.rs:1625:  self.dictionaries
src/domain/storage_deal.rs:4459:  reg.dictionaries   (test)
src/domain/storage_deal.rs:4460:  .register_dictionary(dict, 4_096)   (test)
src/domain/storage_deal.rs:4478-4479 (test)
```
Description: The only production call is acquire_dictionary inside register_manifest_with_source. Nothing in production registers a dictionary, releases a reference, deletes one, or folds dictionary_root into any state root. The module header says "WIRING: wired" but the registration half has no caller.
Impact: Any manifest that sets dictionary_id is refused with UnknownDictionary on every node, so the dictionary feature cannot be used; and if a path is added later without release_dictionary, refs only grow (prune_content leaves the reference, see P23-F02). dictionary_root is not committed, so two nodes could disagree on refs without detection.
Known: baseline (.github/dead-pub-api-baseline.txt and idle-code-baseline.txt list register_dictionary, release_dictionary, delete_dictionary, dictionary_root); the header wording and the consequence are new.

### P27-F04
File: src/storage/erasure.rs:783-866
Severity: Medium
Axis: integration
Confidence: Certain
Evidence (rg -n 'reconstruct_object\(' src, outside erasure.rs):
```
src/tests/manifest_commitment_locks.rs:189:    let out = reconstruct_object(&m, &present).unwrap();
```
Description: reconstruct_object, the only function that rebuilds an object from surviving shards, has no production caller. Production uses ReedSolomon only for the coding audit column check (storage_deal.rs:2320) and verify_object_encoding only from the RPC storage_verify_encoding (server.rs:3324). No node path reads or repairs an object by decoding.
Impact: The "any k of n recovers the object" promise that the manifest declares (and that repair triggers rely on) is exercised by tests only; a regression in the decode path would not be seen by any node binary. Mainnet blocker class: part not wired.
Known: none (STATUS 4.5 does not list it).

### P27-F05
File: src/storage/lrc.rs (whole), src/storage/msr.rs (whole), src/storage/one_share.rs, src/storage/one_view.rs, src/storage/server_admission.rs, src/storage/mobile_self.rs:158-381, src/storage/living_threshold.rs:390-622
Severity: Medium
Axis: integration
Confidence: Certain
Evidence (rg for production callers outside the part, tests excluded):
```
LrcLayout / new_lrc_group            : only src/storage/mod.rs:100 (re-export)
msr_* / lrc_repair_traffic_scaled    : only src/storage/mod.rs:111 (re-export)
OneShareRegistry / SingleScreenView  : only src/storage/mod.rs:114-115 (re-export)
CustodyLedger, decide_upload_custody,
admit_device_as_server               : mod.rs re-exports only
break_even_rate_scaled / decide      : mod.rs re-export only (module header says so)
```
Description: LRC has a layout description but no encoder, decoder or repair; the durability figures (guaranteed_loss_tolerance = global_parity + 1, overhead 1.031x) have no code behind them. MSR states "WIRING: unwired". The B.U.D. 1.0 share, single screen, custody ledger and server admission have no node or RPC caller. msr.rs header says lrc.rs wires it; lrc.rs lines 45-48 say msr "reads this module's repair-traffic function so the module is wired"; the msr header says the opposite. Both are inconsistent with the call graph (nothing outside the pair calls either).
Impact: These are mainnet parts not wired (Directive: everything coded must be wired). Redundancy numbers quoted in docs are not backed by running code.
Known: baseline (idle-code and dead-pub-api baselines list these symbols).

### P27-F06
File: src/storage/erasure.rs:783-866; src/storage/manifest.rs:729-800
Severity: Low
Axis: correctness
Confidence: Likely (missing step: confirm no later check of content_size against the data region in the registration path)
Evidence:
```
erasure.rs:835  out.truncate(manifest.content_size() as usize);
manifest.rs:   if self.content_size > self.total_size { ... }   (total_size = sum of ALL shards incl. parity)
```
Description: validate_untrusted bounds content_size by the size of all shards, not by the data shards only. truncate is a no-op when content_size exceeds the data region, so reconstruct_object would return padded bytes (up to k times stripe) as if they were the object; a content_size far below the data region silently shortens the object. verify_object_encoding catches this, but it is only called from the optional RPC.
Impact: A client-built manifest can declare a content_size that does not match its shards and still register; a reader trusting content_size gets padded or truncated bytes. Self-inflicted by the uploader, so Low.
Known: none

### P27-F07
File: src/storage/erasure.rs:818-866
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
846  for &drop in &live {
847      let mut trial = present.to_vec();
848      trial[drop] = None;
...
857  let all = rs.reconstruct(present)?;   // third, redundant full attempt
```
Description: The retry drops exactly one survivor, so two or more corrupted survivors fail even when k clean shards remain; recovery is not "any k clean of n". Each attempt clones all shards and runs a full decode, and the last block repeats the first attempt (dead logic whose result is always the first error). Cost is O(live) full reconstructions on attacker-influenced shard bytes, with no caller today (F04).
Impact: Denial of recovery under two bad shards; CPU amplification if the function is later exposed to a network-supplied shard set.
Known: none

### P27-F08
File: src/storage/payload_crypt.rs:40-48, 146-167
Severity: Low
Axis: security
Confidence: Certain
Evidence:
```
40  pub struct PayloadKey(pub [u8; 32]);          // Clone, no Zeroize, no Drop wipe
164 let cipher = XChaCha20Poly1305::new_from_slice(&key.0)...
166 cipher.decrypt(&nonce, ct)                   // no AAD, no length cap on open
```
Description: (a) No associated data: the header (magic, version) and any content id or manifest id are not authenticated, so a sealed blob can be moved to another content item under the same key and still opens. (b) open_payload has no upper bound (MAX_SEAL_PLAINTEXT is enforced on seal only). (c) The key type is Clone with a public field and is never zeroized; plaintext returned is not wiped. Random nonces are an owner decision (Q4 B), not repeated here.
Impact: Defense in depth: ciphertext substitution between items sharing a seal_seed is not detected at the AEAD layer; memory residue of key material.
Known: Q4 B covers nonce choice only.

### P27-F09
File: src/storage/pact_binding.rs:38, 85, 107
Severity: Low
Axis: invariant(Z3)
Confidence: Likely (missing step: git log -S shows only the import commit e7e3a45, so the adding commit is not identifiable in this clone)
Evidence (rg -n '#\[allow' on the part):
```
38:    #[allow(clippy::missing_const_for_fn)]
85:    #[allow(clippy::missing_const_for_fn)]
107:    #[allow(clippy::missing_const_for_fn)]
```
Description: Three `#[allow(clippy::missing_const_for_fn)]` attributes with a comment that the nightly jobs reject const. This is a lint suppression of the kind Z3 forbids for new code; whether it is new depends on suppression-budget accounting.
Impact: Maintenance and CI-softening risk only.
Known: none (check the suppression baseline).

### P27-F10
File: src/storage/pact_binding.rs:20-30, 115-118; src/account_abstraction/registry.rs:169-180
Severity: Low
Axis: integration
Confidence: Certain
Evidence (rg for register_with_pacts / add_pact / PactRegistry::new outside tests): only src/account_abstraction/registry.rs:169 (definition) and its in-file tests.
Description: PactRegistry and Pact are built only in tests; no production code constructs a pact set, so the pact_root check is vacuous in production. In addition: Pact does not derive Serialize/Deserialize (no persistence format); add_pact has no duplicate-id check and no count cap and recomputes the whole root each call (quadratic); Pact::new does not tie mode_flag to residual_commitment (mode 0 with a non-zero residual and mode 2 with zero residual are both accepted, and is_pure_production reads both).
Impact: Dead binding today; once wired, a caller can register the same pact id repeatedly and build an arbitrarily large registry.
Known: STATUS 4.5 (account abstraction registry unwired); idle-code baseline lists add_pact, is_pure_production, is_residual_only.

### P27-F11
File: src/storage/dictionary.rs:217-221, 242-271, 364-394
Severity: Low
Axis: security
Confidence: Likely (missing step: confirm that StorageRegistry deserialization is the only load path and has no post-load validation)
Evidence:
```
217 #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
218 pub struct DictionaryRegistry { entries: BTreeMap<ContentId, DictionaryEntry> }
```
Description: (a) Deserialize accepts any entry (size 0, size above MAX_DICTIONARY_BYTES, refs 0 with no grace marker, deletable_at_epoch set while refs > 0) and carries no version tag. (b) register_dictionary records only a claimed size: nothing ties the id to dictionary bytes, so an entry can exist whose bytes are held by nobody, and acquire_dictionary only proves the entry exists, not that the dictionary is readable (the stated purpose). (c) An entry with refs == 0 and deletable_at_epoch == None can never be deleted (delete_dictionary returns GraceNotElapsed with u64::MAX and deletable_dictionaries skips it), so a registration that is never referenced is permanent state; registration is permissionless by design and has no bond or cap here.
Impact: Once register_dictionary is wired (F03), unbounded permanent entries and manifests that reference unreadable dictionaries; corrupt snapshot data loads without refusal.
Known: none

### P27-F12
File: src/storage/mobile_self.rs:59-76, 158-181, 275-283, 335-381; src/storage/one_share.rs:75-109, 148-154, 170-172
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
mobile_self.rs:70  AlwaysOnReplica => SelfHostOnly; Scheduled|Opportunistic => AddPaidReplica   (recommendation_for_content)
mobile_self.rs:166 else if content_size <= profile.max_storage_bytes { UserHeld }              (decide_custody, ignores availability)
one_share.rs:94    decide_custody(profile, size, false)  // per-share only
one_share.rs:150   ledger.record_user_held(s.content_id, s.size) // bypasses the capacity check
one_share.rs:170   pub const fn storage_fee_owed_usd(&self) -> f64 { 0.0 }
```
Description: Two answers for the same question (recommendation_for_content vs decide_custody) disagree for a Scheduled or Opportunistic device. OneShareRegistry.create checks each share against max_storage_bytes but not the running total; admit_as_server rebuilds a ledger with record_user_held, which has no capacity check, so a device can hold shares far above its declared capacity and still show device_is_server = true. CustodyLedger.put_user_content keeps a stale user_held row when the same content id is later put as network-held. All CustodyLedger collections are Vec with linear duplicate search and no cap; next_id saturates (re-use at u64::MAX). storage_fee_owed_usd returns f64 (not consensus, Z8 not violated, but a float money API).
Impact: Wrong or inconsistent 1.0 custody reporting; quadratic cost on large ledgers. No funds. Not wired (F05), so Low.
Known: baseline lists all_user_held and storage_fee_owed_usd as dead.

### P27-F13
File: src/storage/server_admission.rs:58-75; src/storage/mobile_self.rs:259-269
Severity: Low
Axis: security
Confidence: Certain
Evidence:
```
server_admission.rs:61  if ledger.network_default_attempts() > 0 { return Err(TriedToDefaultCustody) }
mobile_self.rs:259      pub struct CustodyLedger ... #[derive(Default)]  (counters are plain local state)
```
Description: The doc calls admission "the network-side check", but its only evidence is a CustodyLedger the device itself fills, can drop and recreate (Default), and never signs or commits on chain. A device that tried to default custody can present a fresh ledger.
Impact: Not a usable admission control if wired as described; no code uses it today (F05).
Known: baseline (admit_device_as_server unwired).

### P27-F14
File: src/domain/storage_deal.rs:2101-2108 (caller), src/storage/social_delete.rs:30-73
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
storage_deal.rs:2100  let mut hook = crate::storage::three_hooks::NopThreeHook;
storage_deal.rs:2101  Ok(crate::storage::process_social_delete(&mut self.view_grants, content_id, owner, at_epoch, &mut hook))
social_delete.rs:56   let key_rotated = crate::storage::three_visibility::delete_implies_key_rotate();
```
Description: process_social_delete always reports key_rotated = true, but the only production caller passes a no-op hook, so the KeyRotated event is dropped and nothing rotates the payload key id. Owner authority is checked by the caller (good: derived owner, digest, signature). The delete digest binds content, caller and epoch but no chain id or nonce, so a signed delete can be replayed at the same epoch (idempotent, harmless today). Failed per-grant revokes are swallowed (`is_ok()` count only).
Impact: The DeleteOutcome claim "key rotated" is not true on the production path; the T3 threat-model step 2 is a signal with no receiver.
Known: none

### P27-F15
File: src/storage/msr.rs:50-56, 71-101, 112-131; src/storage/lrc.rs:208-210
Severity: Info
Axis: correctness
Confidence: Certain
Evidence:
```
msr.rs:52  /// The repair degree must be able to rebuild: d >= k and d <= n - 1
msr.rs:94  if d < k || d - k >= k { return Err(BadRepairDegree) }
lrc.rs:209 self.data_shards + self.local_groups + self.global_parity   (plain add in a const fn)
```
Description: The error doc names d <= n-1 but the code enforces d < 2k and has no n. msr_speedup_over_lrc_scaled divides an LRC at k=2000 by an MSR at an unrelated k=20 (the test comment admits the comparison is not like for like). lrc_total_shards uses unchecked addition; it is safe only because constructors cap the total at 4096, but the struct fields are pub, so a struct literal can bypass the cap (the wire path is safe via try_from).
Impact: Misleading doc and metric; latent overflow if a layout is built by literal (release has overflow-checks = true, so it would abort).
Known: baseline (unwired).

### P27-F16
File: src/storage/erasure.rs:683-696, 744-769; src/rpc/server.rs:3324 (caller)
Severity: Info
Axis: security
Confidence: Likely (missing step: none, already tracked as P21-F02 for the RPC cost)
Description: verify_object_encoding re-encodes unbounded caller data (stripe times n allocation plus parity rows, rayon above 256 KiB of parity work) with only the structural manifest check first; the module offers no size cap of its own. Noted here because the cap belongs in this function for any future caller. The check also deliberately skips manifest_id, so provenance fields (encryption, source, dictionary) are not covered by it.
Known: P21-F02 (FINDINGS.md:1049).

## NOT FINDINGS (checked, sound on reading)
- Cauchy generator uses x=k+i, y=j as u8 with k+m <= 255, so x XOR y is never zero and the MDS property holds; the GF tables and inverse are correct; tests cover submatrices exhaustively at (4,4).
- encode_object refuses empty stripes (the k=5, len=6 case) and equal-id shards; reconstruct re-verifies every recovered shard against manifest ContentIds.
- column_is_correctly_encoded and parity_coefficient return false/None on out-of-range indices, no panic path for challenge input.
- LrcLayout checks zero data, zero groups, empty local group, total above 4096, on both construct and deserialize.
- msr_repair_traffic_scaled avoids 2*k overflow and uses u128; living_threshold products are checked_mul; no panic or unwrap on external input in non-test code of the part.
- payload_crypt open_payload: length, magic, version and AEAD are all checked; deterministic-nonce helper is cfg(test) only.
- pact_binding root V2 commits every field and the count; empty set is zero.
