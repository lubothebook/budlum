# P42 scan: R1 apps (relayer, gateway, bns, socialfi, budlumxyz, deed, developer_os)

Base commit: 7ad75a9. Scanner: Sonnet, read-only. Known and not repeated: STATUS.md section 4.5, docs/audit/FINDINGS.md, .github/dead-pub-api-baseline.txt (items from it are marked Known where they come up).

## COVERAGE

Read in full (every line, tests included):
- src/relayer/mod.rs 1-7
- src/relayer/policy.rs 1-959
- src/relayer/worker.rs 1-1841 (reads 1-1500, 1500-1841)
- src/gateway/mod.rs 1-15
- src/gateway/atlas.rs 1-695
- src/gateway/passport.rs 1-540
- src/gateway/service.rs 1-575
- src/bns/mod.rs 1-5, src/bns/README.md 1-33
- src/bns/registry.rs 1-435
- src/bns/types.rs 1-93
- src/socialfi/mod.rs 1-382, src/socialfi/types.rs 1-31, src/socialfi/README.md 1-38
- src/socialfi/vault.rs 1-709
- src/budlumxyz/mod.rs 1-571, src/budlumxyz/types.rs 1-49, src/budlumxyz/README.md 1-31
- src/deed/mod.rs 1-195
- src/developer_os.rs 1-529

Docs read: docs/AGENT_MAP.md 1-56; docs/AUDIT_PLAN.md lines 110-165 (sections 5 and 6); notes/P29-scan.md lines 1-30.

Outside the part (rg first, at most 80 lines each): src/execution/executor.rs 640-790, 813-853, 976-1040, 1320-1480, 2450-2500; src/core/account.rs 606-618, 1700-1770, 2470-2490 (rg); src/rpc/server.rs 5410-5480, 6721-6737 (rg -A); src/cross_domain/evm/adapter.rs 322-372; src/main.rs 1500-1560; src/network/node.rs 170-225; src/storage/manifest.rs 942-990; src/core/bounded_read.rs (rg); src/storage/render.rs (rg); src/registry/role.rs (rg); baselines (grep only); STATUS.md (grep only); docs/audit/FINDINGS.md (grep only).

Not read: the body of src/storage/generated.rs and render.rs, the Pollen registry (pollen_asset_for_content), the full executor NftBoost/NftTransfer arms, ChainActor handlers for the relayer commands. No cargo, no test run: every claim comes from reading source.

Extra-check summary:
- Untrusted input: relayer and policy take no network input directly; the worker reads finalized blocks. Gateway RPC input is a BNS name and a render format string (bounded, parse_render_format handles errors). BNS, hub and vault payloads arrive as transaction data through the executor. Findings P42-F01, F02, F08, F12, F16, F17, F19.
- Auth: policy.rs has no signature on UserIntent or SolverBid (F07); BNS, hub and vault owner checks are present (vault checks ownership for every id, see F01 for the structural DoS).
- State written outside blocks: relayer worker writes relayer-cursor and relayer-pending.json (node-local, not consensus state; F05, F25). No consensus state written outside blocks in this part. Gateway generation cache is node-local.
- Unwired: policy.rs (self-documented, F07), deed (F21), Atlas query engine types (F22), BNS renew and transfer (F11), budlumxyz update_app and list_apps (F18), NftTag arm (F09), outbound EVM relay (F03).
- Z3: one `#[allow(clippy::too_many_arguments)]` at src/relayer/policy.rs:191; `git log -S` returns only the import commit e7e3a45, so it is not attributable to a new addition (see F26). No `#[ignore]` in the part.
- Z7: no `unsafe` in the part. Z9: no key, seed or token literal in the part (test fixtures use [n; 32] arrays). Z12: no "L1" or "layer 1" text in the part.
- Panic paths: no unwrap, expect or direct indexing in production code of the part. One recursion without a depth bound (F01). `register_app` and `BnsRegistry::register` use unchecked `+` (F14, F19).

## FINDINGS

### P42-F01
File: src/socialfi/vault.rs:235-256 (called from 160 and 203); executor arm src/execution/executor.rs:2457-2480
Severity: High
Axis: correctness
Confidence: Likely (missing verification: the real per-frame stack use and the thread stack size of the block-apply path; the number of nested folders needed is therefore an estimate)
Evidence:
```
vault.rs:160  if self.folders.contains(&member) && self.reaches(member, folder) {
vault.rs:240  fn reaches_walk(&self, from: u64, target: u64, seen: &mut BTreeSet<u64>) -> bool {
vault.rs:251  if self.folders.contains(member) && self.reaches_walk(*member, target, seen) {
```
Description: The cycle test is a recursive depth-first walk with no depth limit. `seen` only stops revisits, it does not bound the recursion depth. A sender who owns N NFTs can register them as folders and nest them bottom-up as a chain f1 contains f2 contains ... fN. The last `AddMember` of the chain root walks N frames deep. There is no cap on folders, members or nesting depth anywhere in VaultRegistry.
Impact: Any account that pays about 3N transaction fees (mint, register, add) can make every node that applies the block run a native-stack recursion of depth N inside the executor. At a large enough N the stack overflows and the process aborts (not a catchable panic), on every node, on the live path and on replay. That is a network-wide halt at that block.
Trigger: precondition is fees for the NFTs and the add transactions, spread over many blocks. Entry: TransactionType::Vault(VaultTx::AddMember or MoveMember) at executor.rs:2457. Input shape: a chain of N registered folders, then AddMember { folder: X, member: chain_root } (or the same through MoveMember at vault.rs:203). Expected wrong result: stack overflow in `reaches_walk` instead of an `Err`.
Known: none

### P42-F02
File: src/rpc/server.rs:5410-5425 versus src/gateway/service.rs:205-211 and 320-323
Severity: High
Axis: security
Confidence: Likely (missing verification: that `pollen_asset_for_content` keys on the manifest id the way the guard assumes, and that protected bytes are served in clear; the Pollen registry was not read)
Evidence:
```
server.rs:5414  let manifest_id = resolved.as_ref().and_then(|r| {
server.rs:5415      r.content_id.or(r.storage_root.map(crate::storage::ContentId))
server.rs:5419  if self.chain.pollen_asset_for_content(cid).await.is_some() { return Err(... "Content is Pollen-protected" ...
service.rs:206  let storage_root = resolved.storage_root.ok_or_else(...)
service.rs:211  let cid = ContentId(storage_root);
```
Description: The Pollen guard in the RPC handler picks `content_id` first and `storage_root` only as a fallback. The gateway then serves `storage_root` and never looks at `content_id`. For a name that has both fields set to different values, the guard checks one content and the gateway serves the other. The same split exists in `gateway_render_content` (server.rs:5440-5475 and service.rs:320-328). The comment at server.rs:5410 says the order was changed to close the inverse hole (content_id set, storage_root unset); this order opens the other one.
Impact: A name owner (anyone who registers a name) can set storage_root to the manifest id of Pollen-protected content (ids are on chain) and content_id to any unprotected id, then call the gateway for that name and receive the protected bytes without an AccessGrant. Impact is limited if protected bytes are stored encrypted; recipe-born (Generated) content is served in clear by `render_from_recipe`.
Trigger: precondition: a registered name (cost: BNS registration), two owner-only transactions BnsSetContent and BnsSetStorage (executor.rs:696 and 746). Entry: RPC gateway fetch (server.rs:5430) or render (server.rs:5468). Input: the name. Expected wrong result: bytes of the storage_root content returned although the guard evaluated the content_id content.
Known: none (STATUS.md line 22 only says the gateway rehashes ContentId)

### P42-F03
File: src/relayer/worker.rs:687-689, 993-1004 with src/cross_domain/evm/adapter.rs:328-337 and src/main.rs:1517-1550
Severity: Medium
Axis: integration
Confidence: Certain
Evidence:
```
adapter.rs:328  async fn submit_transaction(&self, _ext_tx: &ExternalTransaction) -> Result<String, AdapterError> {
adapter.rs:332      Err(AdapterError::SubmissionFailed("EVM adapter cannot broadcast: it does not speak to Ethereum. ...
worker.rs:1001  | AdapterError::SubmissionFailed(_)
worker.rs:1003  | AdapterError::Other(_) => RelayOutcome::Retry,
worker.rs:689   break 'heights;
```
Description: The only adapter a node can register in production (main.rs builds the EVM adapter from --evm-bridge-address) always fails `submit_transaction` with SubmissionFailed, which `relay_outcome_for` maps to Retry. The Retry arm stops the height loop without advancing the cursor, so the first UniversalRelay request for Ethereum is retried every 5 seconds forever and every later request in that block and in all later blocks (any chain) is never reached. Consensus also refuses an Ethereum RelayerResult (executor.rs, `relayer_evm_package_required`), so even a working broadcast could not complete the loop.
Impact: On a node started with the EVM adapter, outbound relay does not work and the worker is wedged on the first Ethereum request. Users who paid the UniversalRelay fee get nothing. This is a mainnet blocker for the Universal Relayer (the module doc at worker.rs:157-177 describes the wiring as done).
Trigger (security only): not applicable.
Known: none (STATUS.md section 4.5 lists the bridge relayer as unwired, this is the separate worker)

### P42-F04
File: src/relayer/worker.rs:675-680 versus 114-127; src/execution/executor.rs:1015-1021 (UniversalRelay arm)
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence:
```
worker.rs:676  // A refusal at the adapter is a fact about the request: retrying it changes nothing
worker.rs:678  // and the chain-side deadline reclaims it.
worker.rs:126  /// `UniversalRelay` has no chain-side deadline to lean on.
executor.rs    UniversalRelay arm: sender.balance.checked_sub(tx.fee) ... sender.nonce += 1   (no escrow, no deadline)
```
Description: The Refused branch relies on a chain-side deadline that returns the user's fee. The same file says at line 126 that there is none, and the executor arm shows the fee is debited and credited to the block producer with no escrow. A Refused request (unsupported chain, adapter proof rejected) is marked settled and the cursor moves on.
Impact: A user who sends UniversalRelay for any chain without an adapter (all chains except a configured one, which is the default on every node) loses the fee with no result and no refund. No on-chain signal of the refusal exists either.
Known: none

### P42-F05
File: src/relayer/worker.rs:313-325, 654-712, 818-852
Severity: Medium
Axis: correctness
Confidence: Likely (missing verification: a run with two requests in one finalized block where one is retried long enough for the other result to be finalized, then a restart)
Evidence:
```
worker.rs:324  self.observed.remove(&request);            // reap_finalized: result finalized
worker.rs:661  if self.settled.contains(&tx.hash) { continue; }   // settled is in memory only
worker.rs:689  break 'heights;                            // cursor not advanced for the block
worker.rs:707  relayed_through = h; self.save_cursor(relayed_through);  // cursor persisted per block
worker.rs:855  self.observed.insert(request.to_string(), PendingResult { result: None, ...   // reserve and act again
```
Description: The cursor is persisted per block, the settled set is not persisted, and the pending entry of a request is deleted once its result is final. If a block holds requests A and B, A is submitted and later finalized (entry removed), and B keeps the cursor on that block with Retry, a restart replays A from scratch: no entry, not in settled, so the external action runs a second time. The module doc promises the external action runs at most once.
Impact: A relayed request can be broadcast to the external chain twice after a restart. The chain-side replay guard refuses the second result but cannot undo the second external transfer.
Known: none

### P42-F06
File: src/main.rs:1502-1517 with src/relayer/worker.rs:654-671
Severity: Medium
Axis: integration
Confidence: Likely (missing verification: adapter-level idempotency through external_nonce; no adapter can broadcast today, see F03)
Evidence:
```
main.rs:1502  let relayer_identity = if config.role == "relayer" || config.role == "validator" {
worker.rs:655  if let TransactionType::UniversalRelay(ext_tx) = tx.tx_type {   // every request, no assignment
worker.rs:796  /// `ExternalTransaction::external_nonce` remains the adapter's job
```
Description: Every node with role relayer or validator and an address runs the worker and relays every finalized UniversalRelay request. There is no assignment, bid, lock or leader per request. With N relayers there are N external broadcasts per request, each signing its own RelayerResult. Idempotency is left to a future adapter.
Impact: When a real broadcasting adapter exists, each request is executed N times on the external chain and N relayers pay external gas, with the chain accepting one result. Together with F07 the policy layer that would pick one solver is not connected.
Known: none

### P42-F07
File: src/relayer/policy.rs:1-48, 345-439 (whole file)
Severity: Medium
Axis: integration
Confidence: Certain
Evidence:
```
policy.rs:1   //! WIRING: unwired - no transaction type carries an intent, no settlement path
rg -n 'RelayerPolicyRegistry|UserIntent|SolverBid' src --glob '!src/relayer/*' --glob '!src/tests/*'  -> only src/developer_os.rs fixture field names, no use of the types
policy.rs:398 pub fn settle_intent(&mut self, intent_id, relayer: Address, paid_fee: u64, current_block: u64)
```
Description: The module is unwired by its own header (no tx type, no settlement, no slash). Besides being unreachable, the design has gaps that matter the day it is wired: `UserIntent` and `SolverBid` carry no signature, so `owner` and `relayer` are plain claimed fields; `settle_intent` takes any caller-chosen relayer and any `paid_fee` up to the quote and does not require the best bid; `bond` is a number that is never locked or checked against the registry stake; `PolicyEnvelope.requires_multisig`, `requires_hsm` and `session_key` are stored and hashed but never enforced; `spending_cap` is compared per intent, never summed over time.
Impact: Unwired relayer policy layer (mainnet blocker for intent-based relaying). If wired as written, anyone could submit intents in another owner's name and settle bids without authority.
Known: partly, dead-pub-api-baseline.txt lines 186-188 (best_bid, submit_bid, submit_intent); settle_intent, prune_* and the authority gaps are not listed.

### P42-F08
File: src/bns/registry.rs:58-108 and executor src/execution/executor.rs:644-690
Severity: Medium
Axis: security
Confidence: Certain
Evidence:
```
registry.rs:66  let char_count = name.chars().count();
registry.rs:67  if !(3..=32).contains(&char_count) { return Err(BnsError::InvalidName); }
```
Description: The only check on a name is its character count. No charset, no lowercase rule, no Unicode normalization, no ban on control characters, '/', '..', dots, spaces or confusable characters. The gateway and passport validators (passport.rs:27-38, atlas.rs validate_label) reject names with '/' and '..' and control bytes, so the registry accepts names the rest of the stack refuses. `Ayaz.bud`, `ayaz.bud` and a name with a Cyrillic letter are three different, all registrable, names.
Impact: Name spoofing and squatting through case and homoglyph variants of a brand name; names that can be paid for but never resolved through the gateway (a fee loss for the registrant); hidden control characters in names shown in UIs.
Trigger: any account with the registration cost sends BnsRegister with such a name; executor.rs:644 passes it unchanged.
Known: none

### P42-F09
File: src/execution/executor.rs:999-1006 (arm of TransactionType::NftTag), src/socialfi/mod.rs:80-86
Severity: Medium
Axis: integration
Confidence: Certain
Evidence:
```
executor.rs:1000  let _ = (nft_id, tag);
executor.rs:1001  let sender = state.get_or_create(&tx.from);
executor.rs:1002  sender.balance = sender.balance.checked_sub(tx.fee) ...
rg -n 'add_tag' src --glob '!src/tests/*'  -> socialfi/mod.rs:80 and ai_inference/social.rs:160 only
```
Description: The NftTag transaction discards the tag and the id, checks nothing (not even that the NFT exists or that the sender owns it) and charges the fee. `NftRegistry::add_tag` is called only by an AI helper, not by this arm. The README lists tags as a live feature.
Impact: Users pay for a transaction that does nothing; tags on NFTs cannot be set by users. When someone later makes the arm call `add_tag`, the registry method has no owner check and no tag count or length bound (mod.rs:80).
Known: none

### P42-F10
File: src/bns/registry.rs:254-274 with src/gateway/service.rs:201-203, src/gateway/passport.rs:363-373
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
registry.rs:273  .filter(|r| !r.is_expired)
service.rs:201   if resolved.is_expired { return Err(format!("BNS name '{name}' expired")); }
passport.rs:363  status: if resolved.is_expired { EvidenceStatus::Unverified } else { EvidenceStatus::Verified },
```
Description: `resolve_full` computes `is_expired` and then filters those rows out, so it never returns an expired record. The expired branches in the gateway, the render path and the passport builder cannot run, and the passport for an expired name reports `exists: false` with the warning "Name not found".
Impact: Expired names are shown as never registered, and the `is_expired` evidence and error paths have no test and cannot be exercised in production.
Known: none

### P42-F11
File: src/bns/registry.rs:110-182
Severity: Low
Axis: integration
Confidence: Certain
Evidence:
```
registry.rs:115  /// # No transaction reaches this
registry.rs:119  /// From tests. On a live chain an owner cannot renew, and cannot hand a
idle-code-baseline.txt:1163  src/bns/registry.rs fn renew
```
Description: `renew` and `transfer` have no transaction type (doc admits it). Besides that, `transfer` changes only `owner`: `address`, `resolver`, `content_id`, `storage_root` and all `subdomains` stay as the old owner set them, so after a transfer the name still resolves to the previous owner's address (`address` defaults to the owner at registration, types.rs:34).
Impact: Owners cannot renew without losing remaining time, and cannot sell or move a name. If transfer is wired as written, the new owner inherits pointers to the old owner.
Known: partly, idle-code-baseline.txt line 1163 (renew); transfer and the pointer issue not listed.

### P42-F12
File: src/bns/registry.rs:184-204, 220-232
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
registry.rs:202  parent.subdomains.insert(sub_label, owner);        // no expiry check, no label check
registry.rs:230  record.content_id = Some(cid);                     // no expiry check
registry.rs:288  if rec.expires_at <= current_epoch { return Err(BnsError::Expired); }   // set_storage does check
```
Description: `set_content` and `register_subdomain` never check expiry (set_storage does), so the former owner of an expired name keeps writing to it until someone else re-registers, including after the grace window. Subdomain labels have no length, charset or emptiness check; only the count is capped at 1000, so one record can hold 1000 labels of the full transaction size.
Impact: Stale writes on expired names; state and snapshot growth bounded only by the tx size limit and the per-name count.
Known: none

### P42-F13
File: src/bns/registry.rs:7-32, 297-299; src/core/account.rs:615, 2470
Severity: Low
Axis: invariant
Confidence: Likely (missing verification: whether any supported snapshot version can carry `bns_registry: None`)
Evidence:
```
registry.rs:7   #[derive(Debug, Clone, Serialize, Deserialize, Default)]
registry.rs:30  base_cost: 100,                         // new()
registry.rs:298 self.names.is_empty() && self.base_cost == 100
account.rs:615  bns_registry: snapshot.bns_registry.clone().unwrap_or_default(),
```
Description: The derived `Default` gives `base_cost = 0`, while `new()` gives 100 and `is_empty` assumes 100. A snapshot with `bns_registry: None` restores a registry with `base_cost` 0, so registrations cost nothing (calculate_cost returns 0) and `is_empty()` is false, which adds the `bns_v1` section to the state root that a live node with an empty registry does not hash.
Impact: Free names and a state-root difference after a restore from such a snapshot. The snapshot digest covers `None`, so this needs a snapshot produced by older code.
Known: none

### P42-F14
File: src/bns/registry.rs:105, 92-104
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
registry.rs:105  let record = NameRecord::new(name.clone(), owner, current_epoch + duration);
registry.rs:92   if self.names.len() >= Self::MAX_NAMES {   // also true when the key already exists
registry.rs:97   .find(|(_, r)| r.expires_at.saturating_add(Self::GRACE_PERIOD) < current_epoch)
```
Description: `current_epoch + duration` is an unchecked add on a transaction-supplied u64. It is not reachable today only because `calculate_cost` saturates to u64::MAX and the executor requires the amount and spendable balance to cover it. At capacity the eviction runs even when the call re-registers an existing key (no growth), and it removes the first expired name in key order, while the comment calls it the oldest; the scan is O(n) over 100,000 names inside transaction execution.
Impact: Defense in depth (overflow protected only by the economic check); an unnecessary eviction of an unrelated expired name; linear scan per registration at capacity.
Known: none

### P42-F15
File: src/relayer/policy.rs:155-174, 527-551
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
policy.rs:167  for domain in &self.allowed_domains { hasher.update(domain.to_le_bytes()); }   // no count prefix
policy.rs:535  hasher.update(format!("{chain:?}").as_bytes());                                // Debug text of the enum
```
Description: `policy_hash` writes the domain list with no length or count, followed by fixed-size fields, so the encoding is not self-delimiting. `encode_action` hashes `format!("{chain:?}")`, so renaming an `ExternalChain` variant changes every intent id. Both are inside an unwired module.
Impact: Latent hash-ambiguity and a silent id change on a refactor once the module is wired.
Known: none

### P42-F16
File: src/socialfi/vault.rs:93-95, 225-230; src/execution/executor.rs:786, 823, 794-801
Severity: Low
Axis: security
Confidence: Likely (missing verification: block gas limits that bound how large the vault can grow per block)
Evidence:
```
vault.rs:226  self.members.iter().find(|(_, list)| list.contains(&candidate))   // referenced_by: O(total members)
executor.rs:786  if let Some(parent) = state.vault.referenced_by(id) {          // runs on every NftTransfer and NftBurn
executor.rs:794  if state.vault.is_folder(id) && state.vault.open(id).is_some_and(|m| !m.is_empty())
```
Description: No cap on folders, members per folder or total members. Every NftTransfer and NftBurn pays a linear scan over all memberships, and `contains` is linear per list. An empty registered folder can be burned (the check only blocks non-empty folders), which leaves its id in `folders` for a token that no longer exists.
Impact: Per-transaction cost of NFT transfers and burns grows with attacker-created memberships; dangling folder ids stay in state and the state root.
Known: none

### P42-F17
File: src/socialfi/mod.rs:48-78
Severity: Low
Axis: security
Confidence: Likely (missing verification: the transaction size limit that bounds author_name)
Evidence:
```
mod.rs:54  name: Option<String>,        // author_name stored as given
mod.rs:68  self.nfts.insert(id, nft);   // no cap on count, no check of name length or charset
```
Description: `mint` does not bound or validate `author_name` and the registry has no count cap. The README states the author name is "linked to a BNS name at mint time", but nothing checks that the name exists or belongs to the minter.
Impact: State growth bounded only by fees; a minter can put any name as author, including a BNS name it does not own (spoofed authorship).
Known: none

### P42-F18
File: src/budlumxyz/mod.rs:75-93 and 158-182
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
mod.rs:86  if let Some(url) = new_url { app.website_url = url; }
mod.rs:89  if let Some(manifest) = new_manifest { app.manifest_id = Some(manifest); }      // verified and developer_attested untouched
dead-pub-api-baseline.txt:60,61  src/budlumxyz/mod.rs list_apps, update_app
```
Description: `update_app` changes the URL and manifest of an app without clearing `verified` or `developer_attested`. `mark_verified_by_proposal` guards against exactly this change before the vote lands, but after the badge is set the developer can swap the content and keep the badge. `update_app` has no transaction type, so this is latent.
Impact: Latent: once an update transaction is added, a verified badge no longer refers to the content the voters reviewed.
Known: partly, baselines list update_app and list_apps as idle; the badge issue is not listed.

### P42-F19
File: src/budlumxyz/mod.rs:36-73; src/execution/executor.rs:1344-1400
Severity: Low
Axis: security
Confidence: Likely (missing verification: the transaction size limit)
Evidence:
```
mod.rs:71  self.next_app_id += 1;           // unchecked
mod.rs:45  let id = self.next_app_id;       // no cap on apps; name and website_url are not validated
```
Description: `register_app` accepts any name and URL (empty, control characters, any length up to the tx limit), has no cap on the number of apps, and increments the id with a plain `+=`. The fee of 100 is the only brake.
Impact: Registry growth bounded by fees, unvalidated text rendered by budlum.xyz clients. The u64 increment is not reachable in practice.
Known: none

### P42-F20
File: src/execution/executor.rs:648-690 (BNS), 1390-1400 (hub)
Severity: Low
Axis: invariant
Confidence: Likely (missing verification: the supply invariant test and how circulating supply counts these deductions)
Evidence:
```
executor.rs:683  .checked_sub(cost) ... sender.balance (no credit to any account, pool or burn counter)
executor.rs:1396 .checked_sub(crate::budlumxyz::BUDLUMXYZ_REGISTER_MIN_FEE)
```
Description: The BNS registration cost and the hub registration fee are taken from the sender and credited to nobody. They are not added to a burn counter or the treasury. The comments call them an "exact cost" or "anti-sybil fee". Any amount in `tx.amount` above the cost is left untouched.
Impact: Unaccounted token destruction (supply drops without a burn record); if a supply or double-burn invariant sums balances, burn counters and pools, these amounts are invisible.
Known: none

### P42-F21
File: src/deed/mod.rs:1-106
Severity: Low
Axis: dead code
Confidence: Certain
Evidence:
```
rg -n 'deed::' src --glob '!src/deed/*'  -> src/tests/deed.rs:11 only
rg -n 'RoleId\(1[01]\)|INDUSTRY_SPONSOR'  -> no registration of roles 10 and 11 in src/registry
idle-code-baseline.txt:1464-1467  INDUSTRY_SPONSOR, RESEARCH_CONTRIBUTOR, ArtifactKind, Visibility
```
Description: The deed module (Manifest, validate, id, role ids) has no production caller. The module doc says wiring is a follow-up. Roles 10 and 11 are declared but not part of the role table in src/registry/role.rs (1 to 9).
Impact: Unwired application domain; roles that cannot be registered.
Known: partly, idle-code-baseline.txt lines 1464-1467 (four symbols); Manifest and its methods are not listed.

### P42-F22
File: src/gateway/atlas.rs:141-463
Severity: Low
Axis: dead code
Confidence: Certain
Evidence:
```
rg -n 'AtlasQueryEngine|WalletContextGraph|CrossDomainTrace|EvidenceRecord|DomainSummary' src --glob '!src/gateway/*' --glob '!src/tests/*'  -> no hit
dead-pub-api-baseline.txt:142-147  all_domain_summaries, get_domain_summary, insert_evidence_record, query_evidence_by_*, upsert_domain_summary
atlas.rs:403  self.evidence_records.remove(0);      // O(n) eviction in a Vec of 10,000
```
Description: Only `build_wallet_context` (server.rs:5576) is wired. The query engine, the graph types, `CrossDomainTrace`, `EvidenceRecord`, `DomainSummary` and the node and edge enums have no production use. The module doc says they were moved here to stop being dead, but nothing calls them. Eviction at cap uses `Vec::remove(0)`.
Impact: Dead code kept alive by moving it; no Atlas evidence index exists in the node.
Known: partly, dead-pub-api-baseline.txt lines 142-147 (methods); the types are not listed.

### P42-F23
File: src/rpc/server.rs:5429, 5466; src/gateway/service.rs:36-43, 253-257
Severity: Low
Axis: integration
Confidence: Certain
Evidence:
```
server.rs:5429  crate::gateway::BudGateway::new(self.chain.clone(), Some(self.node.clone()), None);
service.rs:41   Err(_) => Ok(None),
```
Description: Both RPC call sites build the gateway with `storage = None`, so the local sled path (step 4) never runs in production. In `read_local_storage`, any storage error is turned into "not stored here" and the next source is tried, so a corrupt or unreadable local store is not reported.
Impact: Local content is never served by the gateway directly; storage faults are hidden.
Known: none

### P42-F24
File: src/gateway/service.rs:10, 116, 143-166, 230-234, 263-274
Severity: Low
Axis: security
Confidence: Likely (missing verification: the size bound inside NodeCommand::FetchRemoteContent)
Evidence:
```
service.rs:12  fn checked_gateway_content(source: &str, data: Vec<u8>)    // size check after the whole Vec exists
service.rs:116 pub const MAX_GENERATION_CACHE_ENTRIES: usize = 64;       // 64 x 10 MiB = 640 MiB worst case
service.rs:150 fn get(&self, id) -> Option<Vec<u8>> { ... bytes.clone() } // clone of up to 10 MiB under the mutex
```
Description: The 10 MiB limit is applied after `fetch_local_content` or `fetch_remote_content` returns the full buffer, so the limit does not bound the fetch itself. The cache bound is a count, not bytes (the doc computes the product but accepts 640 MiB), and a hit clones the buffer while holding a std Mutex on an async path.
Impact: Memory pressure from large remote responses and from a full cache; lock hold time proportional to object size.
Known: none

### P42-F25
File: src/relayer/worker.rs:359-410, 833-850
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
worker.rs:391  error!(request, "Relayer: reservation still held with the external action in flight; an operator must ...
worker.rs:849  return RelayOutcome::Held;
```
Description: A Held reservation (result `None`) is never expired when its request transaction is readable: `resubmit_lost` logs an error for it on every 5-second pass forever and `process_relay` does not clear it. Entries count against the 16 MiB pending-file ceiling; above it `save_pending` refuses to write (worker.rs:284-296), and the on-disk state stops following memory. Also, SubmissionFailed releases the reservation while ConnectionFailed keeps it, so a brief connection error during `submit_transaction` turns into a permanent manual hold even if nothing was sent.
Impact: Log spam and slow growth of the pending store; transient network errors need an operator.
Known: none

### P42-F26
File: src/relayer/policy.rs:191; src/relayer/policy.rs:1-48; src/bns/registry.rs:110-140; src/relayer/worker.rs:621
Severity: Info
Axis: invariant(Z3)
Confidence: Certain
Evidence:
```
policy.rs:191  #[allow(clippy::too_many_arguments)]
git log --oneline -S'too_many_arguments' -- src/relayer/policy.rs  -> e7e3a45 (import commit only)
worker.rs:621  "... cannot be                      loaded, and relaying without them ..."   (run of spaces inside the log string)
```
Description: One existing `#[allow]` in the part; history shows only the import commit, so no new Z3 addition is attributable. Several doc comments have odd capitalization at line starts (policy.rs:13-48, bns/registry.rs:110-140, atlas.rs header) and the worker log message at line 621 contains a long run of spaces from a lost line continuation. Atlas, vault and gateway comments contain Turkish words (atlas.rs:148, 190, 258, 312, 332, 333, 386; gateway tests "boyut").
Impact: Readability only.
Known: none
