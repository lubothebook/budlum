# P43 scan: Pollen and budscan (read-only audit, Tier R1)

Base commit: 7ad75a9. Scanner: Sonnet, read-only. Known and not repeated: STATUS.md, docs/audit/FINDINGS.md, .github/dead-pub-api-baseline.txt. Items that touch a baseline are marked Known below. budscan is paused (STATUS.md), only real defects are listed.

## COVERAGE

Read in full (every line, tests included):
- src/pollen/mod.rs 1-299
- src/pollen/data_rights.rs 1-773
- src/pollen/content_gate.rs 1-536
- src/pollen/offers.rs 1-1436 (two reads: 1-1000, 1000-1436)
- src/pollen/README.md 1-51
- crates/budscan/src/lib.rs 1-84
- crates/budscan/src/evidence.rs 1-213
- crates/budscan/src/content_id.rs 1-132
- crates/budscan/src/cid.rs 1-288
- crates/budscan/src/arweave.rs 1-230
- crates/budscan/src/bns_proof.rs 1-331
- crates/budscan/src/ens.rs 1-211
- crates/budscan/src/name_rule.rs 1-333
- crates/budscan/src/punycode.rs 1-185
- crates/budscan/src/query.rs 1-410
- crates/budscan/src/fetch.rs 1-291
- crates/budscan/src/resolve.rs 1-530
- crates/budscan/src/light_client.rs 1-443
- crates/budscan/src/evm_audit.rs 1-271
- crates/budscan/src/search.rs 1-471
- crates/budscan/src/patchset.rs 1-557
- crates/budscan/src/bin/budscan.rs 1-184
- crates/budscan/Cargo.toml 1-57
- crates/budscan/README.md 1-128
- crates/budscan/browser/README.md 1-87
- crates/budscan/browser/patches.txt 1-10
- crates/budscan/browser/mozconfig 1-77
- crates/budscan/browser/settings/policies.json 1-56
- crates/budscan/browser/settings/budscan.cfg 1-85
- crates/budscan/browser/l10n/en-US/budscan.ftl 1-43 (head 50)
- crates/budscan/browser/l10n/tr-TR/budscan.ftl: key set diffed against en-US (identical), lines 40-47 read
- crates/budscan/browser/patches/bud-protocol-handler.patch 1-441
- crates/budscan/browser/patches/address-bar-verification-badge.patch 1-206
- crates/budscan/browser/patches/name-bar-punycode.patch 1-219

Not read: crates/budscan/Cargo.lock (the tool refused the path, permission denied; 218 lines, dependency surface is only sha2, sha3, hex, serde per Cargo.toml, so the lock was not audited).

Docs read: docs/AGENT_MAP.md 1-56; docs/AUDIT_PLAN.md lines 110-165 (sections 5 and 6); notes/P29-scan.md lines 1-30.

Outside the part (rg first, then at most 80 lines): src/execution/executor.rs 1620-1650, 1722-1830, 2585-2610 and rg of `TransactionType::Pollen`; src/rpc/server.rs 4140-4170, 5169-5300; src/chain/chain_actor.rs 4240-4275, 4970-4985, 2857-2870; src/domain/storage_tx.rs 395-412; src/core/account.rs 2480-2492; src/core/hash.rs hash_fields_bytes (rg -A12); src/storage/manifest.rs 1027-1102 (manifest_id derivation); .github baselines (rg only); ci.yml and osv-scanner.yml (rg only).

Not done: no cargo, no test run, no `git log -S`. Every claim comes from reading source. The README test counts for src/pollen (46 tests: 15, 13, 10, 8) were recounted by hand and match.

Extra-check summary:
- Untrusted input: pollen types decode hex/JSON with length checks (AssetId::from_hex, Signature64::from_hex, AiDataInputRef::decode rejects wrong length). No unwrap, expect, panic or direct index in production code of src/pollen (all hits are #[cfg(test)]). `bytes[offset..offset+32]` in AiDataInputRef::decode is guarded by the exact length test at data_rights.rs:585. In budscan the only reachable panic path is the u16 sum in evm_audit (P43-F18, abort under panic=abort plus overflow-checks); `bytes[0]`/`bytes[1]` in cid.rs:155 are guarded by `len == 34`; arweave `nodes[0]` is guarded because split_chunks never returns empty.
- Secrets (Z9): none found in any file of the part. No key, seed, token or 64-hex secret. The 64-hex constants in evm_audit.rs:40-47 are public EIP-1967 slot ids, the four hex digests in tests are public vectors.
- Z6: no .sh, .bash or .py file in the part. browser/ holds .patch, .cfg (Gecko JS config), .json, .ftl, .mjs inside patches: this is the budscan Firefox layer, the documented exception (CLAUDE.md, STATUS.md section "Repo is Rust and BudL only"). Z7: no `unsafe` in either part; budscan has `unsafe_code = "forbid"` (Cargo.toml:46). Z3: see P43-F23.
- Unwired parts: see P43-F02, F03 (Pollen), F07, F08 (budscan). Everything in budscan src has no production consumer outside the crate itself and the xtask gates (rg shows budscan only in docs, ci.yml and xtask/gates); that is the pause and is not repeated per item.

## Findings

### P43-F01
File: crates/budscan/src/fetch.rs:121-127; crates/budscan/src/content_id.rs:76-83; crates/budscan/src/resolve.rs:166-169, 203-213
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence:
```
fetch.rs:121  Target::Bud(manifest_id) => {
fetch.rs:122      if bytes_match(*manifest_id, &bytes) {
content_id.rs:81  pub fn bytes_match(expected: ContentId, bytes: &[u8]) -> bool {
content_id.rs:82      ContentId::of(bytes) == expected
src/storage/manifest.rs:1041  buf.extend_from_slice(b"BDLM_MANIFEST_V4");
src/storage/manifest.rs:1043  for s in shards { ... buf.extend_from_slice(&s.shard_id.0); ... }
```
Description: The Bud fetcher treats `manifest_id` as `ContentId::of(page bytes)`. In the chain a `manifest_id` is the hash of the manifest preimage (domain tag BDLM_MANIFEST_V4, shard ids, erasure scheme, encryption tag, sizes, provenance), not the hash of the content. `ContentId::of(chunk)` is the shard id domain (BDLM_CONTENT_V1).
Impact: For a real stored object the equality can never hold, so every `bud://` page, `.bud` name target and `bud://0x..` content id would be labelled Refused, and the "verified" tier of the browser has no reachable input. The README claim "`manifest_id` is the hash of the bytes" (README.md line 42, lib.rs:24) is wrong. A Transport that joins shards cannot fix it, because the compared identity is the wrong one.
Known: none

### P43-F02
File: src/pollen/offers.rs:693-720, 734-780; src/pollen/content_gate.rs:244-265; reader sites src/rpc/server.rs:4155, 4231, 4263; src/chain/chain_actor.rs:4975-4979
Severity: Medium
Axis: integration
Confidence: Certain
Evidence:
```
$ rg -n 'bind_content_to_asset|authorize_content_read|\.authorize_read\(|\.bind\(' src/ crates/ budzero/ --glob '!src/pollen/**'
(no output)
$ rg -n 'Pollen[A-Z][A-Za-z]*\(' src/core/transaction.rs
331: PollenRegisterDataAsset(..)   333: PollenAuthorizeSale(..)   335: PollenGrantAccess(..)
337: PollenRevokeGrant(..)        339: PollenRevokeDataAsset(..)
```
Description: `protected_content` has no writer in production: there is no transaction type, RPC or executor arm that calls `bind_content_to_asset`, and `authorize_content_read` and `ProtectedContent::authorize_read` have no caller. The readers (`pollen_asset_for_content`, `check_content_may_be_public`, shard withholding in `storage_get_manifest`) therefore always see an empty map.
Impact: The module README and the content_gate.rs module doc present a read gate that keeps paid content behind the grant; in the running node paid content is never gated, never kept out of the plaintext (deduplicated) class, and the registered grants only gate AI input refs. Written as a mainnet blocker per AUDIT_PLAN criterion 3.
Known: partly. idle-code-baseline.txt:1731-1737 lists authorize_read, is_protected, authorize_content_read, bind_content_to_asset and dead-pub-api-baseline.txt:170-171 lists is_protected and bind_content_to_asset. The baselines do not record that the whole gate is inert because no transaction can create a binding.

### P43-F03
File: src/pollen/offers.rs:505-636; src/rpc/server.rs:5249-5270; src/execution/executor.rs:1779-1799
Severity: Medium
Axis: integration
Confidence: Certain
Evidence:
```
$ rg -n 'issue_grant_from_sale_authorization' src/ --glob '!src/pollen/**' --glob '!src/tests/**'
src/rpc/server.rs:5258:            .issue_grant_from_sale_authorization(
server.rs:5249  let mut registry = crate::pollen::MarketplaceRegistry::new();   (throwaway, "prepare-only; no state mutation")
executor.rs:1779 TransactionType::PollenGrantAccess(grant) => { ... if grant.owner != tx.from {...}
executor.rs:1790     .create_access_grant(grant)
$ rg -n 'purchase_receipts|record_issued_grant' src --glob '!src/pollen/**'
src/chain/chain_actor.rs:5020 (read only)
```
Description: The sale path (buyer signature, receipt, `grants_issued` counter, `max_grants`) is only reachable in a preview RPC on a scratch registry. On chain the only way to get a grant is `PollenGrantAccess`, submitted by the asset owner with any `grantee`, `payer` and `price_paid` the owner writes. `payment_commitment` is any non-zero 32 bytes and is never matched to a transfer, escrow or burn anywhere.
Impact: No payment is collected or verified for pollen on chain, `purchase_receipts` is never written, `SaleAuthorization.max_grants` and `unit_price` are never enforced against a real issue. A buyer-side "purchase" has no on-chain effect. Marketplace economics are a mainnet blocker; the buyer-signature code is exercised only by tests and the preview.
Known: partly (idle-code-baseline.txt:1736-1741 lists purchase_signing_hash and get_sale_authorization as idle). The missing payment and missing on-chain issue are not recorded.

### P43-F04
File: src/pollen/offers.rs:327-335, 693-720; src/execution/executor.rs:1737-1760
Severity: Medium
Axis: security
Confidence: Likely (missing verification step: a transaction type that binds content to an asset does not exist yet, see P43-F02, so this is latent until the binding is wired)
Evidence:
```
offers.rs:711  if asset.manifest_id != manifest_id {
offers.rs:712      return Err("cannot bind content that does not match the asset's registered manifest".into());
executor.rs:1737 PollenRegisterDataAsset: only `asset.owner != tx.from` is checked, then derive_id, then register_data_asset
offers.rs:328  asset.validate()?;  (id preimage only; no check that manifest_id exists or is owned)
```
Description: The CWE-639 fix in `bind_content_to_asset` compares the manifest with the asset's own `manifest_id`, but `PollenRegisterDataAsset` lets any account declare any `manifest_id` (also one it never uploaded) with itself as owner. The fix is therefore a tautology for an attacker: register asset(owner=attacker, manifest_id=victim content), then bind.
Trigger: precondition: a bind entry point exists (not today). Entry: PollenRegisterDataAsset at executor.rs:1737, then bind_content_to_asset. Input: a ContentId of someone else's stored object. Expected wrong result: the victim's manifest is gated behind the attacker's asset, `check_content_may_be_public` (storage_tx.rs:405, chain_actor.rs:4265) refuses the true owner's plaintext registration, and reads need a grant sold by the attacker. Bindings are permanent by design (content_gate.rs module doc), so it cannot be undone.
Impact: content squatting and denial of registration for any not yet registered or already public ContentId, once binding is wired. The ownership link must be checked against the storage registry (uploader/owner of the manifest) at register or bind time.
Known: none

### P43-F05
File: crates/budscan/src/resolve.rs:192-214; crates/budscan/src/bns_proof.rs:148-152
Severity: Medium
Axis: security
Confidence: Likely (missing verification step: the Registry proof reproduces the on-chain bns_v1 root only after the proof format is completed, bns_proof.rs:169-181, so today the Verified branch of evaluate is not reachable against the real chain)
Evidence:
```
bns_proof.rs:148  let found = entries.iter().any(|e| {
bns_proof.rs:149      e.name == resolved.name && e.owner == resolved.owner && e.content_id == resolved.content_id });
resolve.rs:203    let Some(id) = record.content_id.or(record.storage_root.map(ContentId)) else {
resolve.rs:210    Ok(Plan::Fetch { target: Target::Bud(id), evidence })
```
Description: The proof entry carries only `content_id`. If the resolver returns `content_id: None` and a `storage_root`, `found` is true (None equals None) and the root can reproduce, so evidence is Verified, but the fetch target is the unproven `storage_root` supplied by the same untrusted node. The fetch then hashes the bytes against that attacker chosen id and adds a second Verified claim.
Impact: the exact "verified and wrong" outcome the crate exists to prevent (bns_proof.rs doc lines 3-9): a lying node picks the page under a proven name. Not reachable until the proof format is completed, but the binding must cover `storage_root` before then.
Known: none

### P43-F06
File: crates/budscan/src/bns_proof.rs:104-117, 148-153
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
bns_proof.rs:111  if resolved.is_expired { ...Refused...  return }
RegistryEntry { name, owner, expires_at, content_id }   // expires_at is hashed into the root but never compared
```
Description: Expiry is taken from the resolver's own `is_expired` flag. The proven entry's `expires_at` is never compared with a height, and `evaluate` has no height input.
Impact: A node can report `is_expired: false` for a lapsed name with a root-matching proof and the badge says Verified for a name that no longer binds content.
Known: none

### P43-F07
File: crates/budscan/browser/patches/bud-protocol-handler.patch:36-37 (USE_LIBS), 261-274 (spec passed to the name rule), 396 (budscan_open)
Severity: Low
Axis: integration
Confidence: Likely (missing verification step: the Rust side of the C ABI does not exist, so the call shape cannot be run)
Evidence:
```
$ rg -n 'budscan_ffi|budscan_open|budscan_check_name' --glob '!crates/budscan/browser/patches/*' .
./crates/budscan/browser/README.md:74: ... (`budscan_ffi.h`) is part of the
bud-protocol-handler.patch:+ USE_LIBS += ["budscan_ffi"]
bud-protocol-handler.patch:+ aURI->GetSpec(spec); BudNameVerdict verdict = budscan_check_name(spec.get());
```
Description: (1) No Rust crate, cdylib or staticlib named `budscan_ffi` exists in the tree (no `#[no_mangle]`, no crate-type), so the patch cannot link; the name-bar patch admits the bridge "does not exist yet" while the protocol handler patch states it as present. (2) The patch hands the full URI spec (`bud://ayaz.bud/`, with scheme separator and the trailing slash that nsStandardURL adds) to `check_name`, which refuses `:` and `/` (name_rule.rs:167-171); `classify_name("ayaz.bud/")` in resolve also refuses. A literal implementation refuses every load, and a path part (`bud://name/page`) has no route at all.
Impact: the bud:// handler cannot build or, once built per the header, cannot open a valid name. Paused, so Low; fix before un-pausing.
Known: none

### P43-F08
File: crates/budscan/browser/patches/address-bar-verification-badge.patch:120-130
Severity: Low
Axis: integration
Confidence: Certain
Evidence:
```
+      let channel = aRequest.QueryInterface(Ci.nsIBudChannel);
$ rg -n 'nsIBudChannel' .
(only that line; no .idl, no XPIDL entry, BudChannel.h declares no nsIBudChannel)
```
Description: The badge reads strength through an interface no patch defines; `BudChannel` is a plain C++ `nsBaseChannel` subclass with `Strength()` and `BadgeText()` that JS cannot call. The catch block swallows the failure and leaves strength 0.
Impact: every bud:// page, including Verified ones, shows the "refused" badge while its content is displayed. Fails closed (no false "verified"), but the security signal is wrong for all pages. The swallowed exception also hides the defect.
Known: none

### P43-F09
File: crates/budscan/src/resolve.rs:251-254; crates/budscan/src/ens.rs:60-63, 111; crates/budscan/src/arweave.rs:3-10, 144-153
Severity: Low
Axis: correctness
Confidence: Likely (missing verification step: no ENS arweave-ns example with a real record was checked; the length claim follows from the EIP-1577 body being the 32-byte transaction id and from arweave.rs:148-153 requiring 48 bytes)
Evidence:
```
ens.rs:62   /// `arweave-ns` (0xb29910): the body is a transaction id.
resolve.rs:251  ContentHash::Arweave(root) => Plan::Fetch { target: Target::Arweave(root), ...
arweave.rs:153  let matches = expected_root.len() == produced.len() && expected_root == &produced[..];   // produced.len() == 48
arweave.rs:5-8  "That id is the hash of the signature and NOT the hash of the content; the field bound to the content is data_root"
```
Description: The ENS contenthash body is passed unchanged as the expected `data_root`. A transaction id is not a data_root and is not 48 bytes, and the exact-length rule (CWE-354 fix) then refuses it.
Impact: `.eth` names that point at Arweave can only ever be Refused; the Arweave fetcher is reachable in tests but not from a real ENS record. Fetching the transaction header to read its `data_root` is the missing step (and must itself be labelled claim only).
Known: none

### P43-F10
File: src/pollen/offers.rs:418-432; src/pollen/data_rights.rs:68-98; src/pollen/offers.rs:460-465
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
offers.rs:423  self.encryption_policies.values().filter(|p| p.active).min_by_key(|p| p.max_grant_duration_blocks)
data_rights.rs:79  if let Some(deprecated) = self.deprecated_after_block { if issued_at_block >= deprecated { return Err(...) } }
```
Description: The ceiling is taken from the active policy with the smallest duration, but deprecation is not part of "active". Once that policy's `deprecated_after_block` passes, `check_grant_duration` errors for every grant, even when a newer active policy with a valid ceiling exists. `active_encryption_policies` also lists deprecated policies.
Impact: after the DAO sets a deprecation block on its strictest policy and publishes a replacement, all grants (create_access_grant and the purchase path) are refused until the old policy is manually set inactive. Self-inflicted halt of the grant flow, no attacker needed. No test covers two policies with one deprecated.
Known: none

### P43-F11
File: src/pollen/offers.rs:353-391; src/pollen/data_rights.rs:79-86, 212-250; src/execution/executor.rs:1761-1799
Severity: Low
Axis: correctness
Confidence: Likely (missing verification step: tx admission in src/core/transaction.rs was not read for a height check on these fields)
Evidence:
```
offers.rs:387  self.check_dao_grant_duration_ceiling(grant.issued_at_block, grant.expires_at_block)?;
executor.rs:1779-1799 (PollenGrantAccess): owner check, create_access_grant(grant); no comparison of issued_at_block with the block height
```
Description: `issued_at_block`, `expires_at_block`, `valid_from_block` are owner-chosen and never tied to the block that includes the transaction. The DAO ceiling bounds the length of the window only, and the deprecation test uses the owner's `issued_at_block`.
Impact: an owner can backdate `issued_at_block` below `deprecated_after_block` to obtain a grant under a deprecated policy (window must still reach the present, so the benefit is limited to the grace of `max_grant_duration_blocks`), and can create grants valid only far in the future. Low: owner is the data seller, the effect is on DAO policy enforcement.
Known: none

### P43-F12
File: src/pollen/data_rights.rs:196-210, 466-482; src/pollen/offers.rs:246-274
Severity: Low
Axis: security
Confidence: Likely (missing verification step: whether the transaction envelope signature already binds the chain id for Pollen transactions was not checked)
Evidence:
```
data_rights.rs:200  hasher.update(b"BDLM_POLLEN_SALE_AUTHORIZATION_SIGNING_V1");   // no chain id, no network id
offers.rs:262       hasher.update(b"BDLM_POLLEN_PURCHASE_V1");                       // buyer signature: no chain id
```
Description: The three embedded signatures (seller authorization, owner grant, buyer purchase) sign a domain tag and the fields but no chain id or genesis hash. Same key on testnet and mainnet (or after a restart with a new genesis) validates the same bytes.
Trigger: precondition: the same ed25519 address key is used on two Budlum networks. Entry: create_sale_authorization (offers.rs:487) or create_access_grant (offers.rs:369). Input: a signature captured on one network. Expected wrong result: it is accepted on the other. On the executor path `tx.from == owner` is also required, which limits the replay to sellers who re-submit themselves.
Impact: cross-network replay of sale terms and grants for reused keys. Compare D2 "consent digest binds chain id" for storage deals, which Pollen lacks.
Known: none

### P43-F13
File: src/pollen/offers.rs:815-858; src/core/account.rs:2486-2487
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
account.rs:2486  final_hasher.update(b"pollen_v1"); final_hasher.update(self.marketplace.root());   // unconditional
offers.rs:820-851  for offer in &self.offers {...} for asset in &self.data_assets {...} for grant in &self.access_grants {...} ... (all records, every call)
```
Description: `MarketplaceRegistry::root()` re-hashes every offer, asset, grant, authorization, receipt and policy on every state root calculation and every record is created for a transaction fee only. There is no per-account cap, no deposit, no pruning of revoked or expired grants, and `create_offer`, `register_data_asset`, `create_access_grant` have no size bound.
Impact: state root cost and snapshot size grow linearly with attacker-paid records; compare P21-F03 (RPC side) which is the same state. Maintenance and DoS risk, not a safety break.
Known: none

### P43-F14
File: src/rpc/server.rs:5249-5262 (with src/pollen/offers.rs:418-432)
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
server.rs:5249  let mut registry = crate::pollen::MarketplaceRegistry::new();
server.rs:5250  registry.register_data_asset(asset)...; create_sale_authorization(authorization)...; issue_grant_from_sale_authorization(...)
```
Description: The preview builds an empty registry, so `encryption_policies` is empty and the DAO duration ceiling (offers.rs:423) is never applied; the same scratch registry holds no `protected_content` either.
Impact: `expectedAccessGrant` can describe a grant that the real registry would refuse. The preview also uses `get_height()` as the issue block, not the next block. Misleading result for clients.
Known: none

### P43-F15
File: src/pollen/offers.rs:693-720, 734-780, 793-797 (tests at 861-1436)
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
$ rg -c 'bind_content_to_asset|authorize_content_read' src/tests
(no output)
offers.rs tests (13) call none of bind_content_to_asset, authorize_content_read, check_content_may_be_public, revoke_data_asset, revoke_access_grant, consume_ai_read_grant after exhaustion by revoked asset.
```
Description: The registry-level wrappers carry the CWE-639 fix (offers.rs:706-715) and the reader-side checks (wrong grantee, revoked asset, owner mismatch, wrong asset) but have no test; content_gate.rs tests only the inner map with caller-supplied owner. Also no test for `check_dao_grant_duration_ceiling` with two policies, and none for `revoke_data_asset` followed by a read.
Impact: a regression of the manifest-match check, or of the asset-before-liveness ordering, would pass the suite.
Known: none

### P43-F16
File: crates/budscan/src/patchset.rs:210-226, 321-328
Severity: Low
Axis: security
Confidence: Certain
Evidence:
```
patchset.rs:322  if !allowed_roots.iter().any(|root| path.starts_with(root)) {
touched_files("+++ b/browser/../../etc/x")  ->  ["browser/../../etc/x"]  passes starts_with("browser/")
```
Description: The allowed-tree check is a string prefix. A path with `..` segments, or an absolute path after the `b/` strip (`+++ /etc/x` gives `/etc/x`, rejected, but `+++ b//etc` is not normalised), passes. No test uses `..`. Also `parse_list` does not validate entries, so a list line `../x.patch` or an absolute path is accepted and compared as a plain string with the disk listing.
Impact: the gate `budscan-patchset` reports a pass for a patch that writes outside browser/ once applied. Defense in depth for a CI gate on a paused component.
Known: none

### P43-F17
File: crates/budscan/src/patchset.rs:210-226, 240-285
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
patchset.rs:213  let Some(rest) = line.strip_prefix("+++ ") else { continue };   // no hunk-state, any line
patchset.rs:255  || l.starts_with("--- ") || l.starts_with("+++ ")               // ends a hunk body
```
Description: Body lines are classified by prefix alone. A removed line whose text begins with `-- ` (SQL or Lua comment, `--- ` after the leading `-`) or an added line beginning with `++ ` is read as a file header: `touched_files` reports a bogus path, and `hunk_count_errors` ends the hunk early and reports a count error on a correct patch.
Impact: false failures and false touched paths on valid patches; the second can mask a real count error in the same hunk. Low (the three shipped patches do not hit it).
Known: none

### P43-F18
File: crates/budscan/src/evm_audit.rs:162-197
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
evm_audit.rs:163  let total_w = weights.onchain_pct + weights.static_analysis_pct + weights.behavior_pct + weights.metadata_pct;   // u16 sum
evm_audit.rs:181-188  area_weight(area): any area containing "onchain" gets onchain_pct (substring match)
evm_audit.rs:195  u8::try_from(acc / 100).unwrap_or(u8::MAX)
Cargo.toml:53-54  overflow-checks = true, panic = "abort"
```
Description: (1) The weight sum is an unchecked u16 addition, so caller weights such as 65535 + 1 panic and, with panic=abort, kill the process instead of returning Err. (2) Areas are summed per distinct string but weighted by substring, so "onchain" and "onchain-proxy" both add the full onchain weight; a report can exceed 100 (up to 255 clamp) and `rating` maps >100 to "poor". (3) A missing area contributes 0, so a clean report scores 0 ("poor") and a contract with one Info item scores 32; scores reward having findings.
Impact: wrong risk score and rating from a function documented as a deterministic hint. No caller exists yet (paused), so no live exposure.
Known: none

### P43-F19
File: crates/budscan/src/name_rule.rs:219-240
Severity: Low
Axis: security
Confidence: Certain
Evidence:
```
name_rule.rs:228  if label.is_ascii() { out.push_str(label); }      // control characters are ASCII
name_rule.rs:291 (test) "ayaz.bud\u{0}x" is refused by check_name, display_form returns it unchanged
```
Description: `display_form` punycodes only non-ASCII labels. A refused name containing NUL, newline, ESC, DEL or other ASCII control characters, and the `[?]` escape for failures, is returned raw. `Page::address_bar` (resolve.rs:64-70) and the CLI (budscan.rs:34, 54) print it.
Impact: control characters and terminal escape sequences reach the address bar text or a terminal for refused input, i.e. the display gap the module says it closes (name_rule.rs:213-217). Only for names that are refused, so no resolution risk.
Known: none

### P43-F20
File: crates/budscan/src/fetch.rs:72, 110-118; crates/budscan/src/cid.rs:58-82, 129-136; crates/budscan/src/query.rs:152-154
Severity: Low
Axis: security
Confidence: Likely (missing verification step: the production Transport is not in the tree, so whether it caps while reading was not checked)
Evidence:
```
fetch.rs:72   fn fetch(&self, target: &Target) -> Result<Vec<u8>, String>;       // returns the whole body
fetch.rs:112  if bytes.len() > MAX_CONTENT_BYTES { return Err(..) }               // checked after buffering
cid.rs:69     out.insert(0, (carry & 0xff) as u8);                                // O(n) insert in an O(n) loop
query.rs:153  return Query::Cid(rest.trim().to_string());                         // ipfs:// accepts any length
```
Description: The 10 MiB cap is applied after the transport returned the full `Vec`, so memory is bounded by the transport, not by `MAX_CONTENT_BYTES`; the trait doc promises "the size limit being exceeded" is the transport's error but gives it no limit argument. `b58_decode` is quadratic in the input length and `ipfs://z...` or a `z` prefixed CID string of any length is passed to it (the `Qm` branch is length-gated, the `z` branch is not).
Impact: a hostile node can make the browser buffer an arbitrarily large body before the check; a pasted very long `ipfs://z...` string costs quadratic CPU. Local, user-triggered, so Low.
Known: none

### P43-F21
File: crates/budscan/src/light_client.rs:124-156
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
light_client.rs:133  if header.index == last.index { if header.hash != last.hash { Err } return Ok(()); }   // state_root not compared
light_client.rs:125  only index % EPOCH_LENGTH is checked; `epoch` is never compared with index / EPOCH_LENGTH; state_root and hash are free Strings
```
Description: `accept` takes any epoch-aligned index above the last one (no continuity, no parent link, no hex or length check), ignores a different `state_root` at an index with the same `hash`, and never checks `epoch` against `index`.
Impact: the store is only as honest as its feeder and the "fork" detector can be bypassed by changing `state_root` while repeating `hash`. The module doc already says finality is an RPC assertion, so this is a hardening gap for when a feeder is written.
Known: none

### P43-F22
File: crates/budscan/src/name_rule.rs:51-54; crates/budscan/browser/patches/name-bar-punycode.patch:8-12; crates/budscan/browser/settings/policies.json:15-17, 40-45; crates/budscan/browser/README.md (FORBIDDEN_BRAND_TOKENS)
Severity: Info
Axis: correctness
Confidence: Certain
Evidence:
```
name_rule.rs:51-53  MixedScript: "a name wholly in Cyrillic is accepted and displayed as punycode"
name_rule.rs:167   non-[a-z0-9.-] characters are refused (test at name_rule.rs:306-312 asserts a Cyrillic name is an error)
name-bar-punycode.patch:11-12  "A fully Cyrillic name is displayed fully in Cyrillic."  (display_form returns punycode)
policies.json:40-45  EnableTrackingProtection "Locked": false  under the comment "not as a default the user can change"
policies.json:15  key "//_crash" followed by DisableFirefoxScreenshots
```
Description: Documentation and configuration comments contradict behaviour: check_name refuses every non-ASCII name (Cyrillic is not accepted); display_form punycodes it (not shown in Cyrillic); the policy comment calls tracking protection non-changeable while it is unlocked; the crash comment sits above a screenshots policy (crash reporting is off in mozconfig, not here); the browser README names `budscan::patchset::FORBIDDEN_BRAND_TOKENS` but the code has `FORBIDDEN_BRAND_SYLLABLES` and `forbidden_brand_tokens()`; the policy comment claims `InstallAddonsPermission.Default=false` enforces signed extensions (signing is enforced by `MOZ_REQUIRE_SIGNING` in mozconfig, the policy only blocks site-initiated installs).
Impact: reader is misled about the shipped guarantees. Docs only.
Known: none

### P43-F23
File: src/pollen/data_rights.rs:142, 400, 440; src/pollen/offers.rs:49, 95, 504, 928; crates/budscan/src/query.rs:110
Severity: Info
Axis: invariant(Z3)
Confidence: Likely (missing verification step: `git log -S` for the adding commits was not run; the scan is read-only and the history was not inspected)
Evidence:
```
#[allow(clippy::too_many_arguments)]   x7 in src/pollen (offers.rs:928 is a test helper)
#[allow(clippy::too_many_lines)]       crates/budscan/src/query.rs:110
```
Description: Seven `too_many_arguments` allows in src/pollen (src is inside the suppression gate scan) and the budscan allow outside the gate scan. The budscan one is Known (docs/audit/FINDINGS.md P62-F03, query.rs:110). The pollen ones were not found in the baselines read by rg (idle and dead-pub baselines only); the suppression budget file was not located under .github/.
Impact: possible Z3 entries; cannot be judged without the suppression budget and the commit dates.
Known: query.rs:110 yes (P62-F03); the pollen allows: unverified.

## Counts

Total 23: Critical 0, High 0, Medium 5 (F01 to F05), Low 16 (F06 to F21), Info 2 (F22, F23).

Test-coverage notes besides P43-F15: budscan tests are real (they call the production functions; `the_self_test_passes_in_this_tree` runs the canaries), no `#[ignore]`, no empty asserts found in the 21 files; the 107-test README count was not verified (no test run).
