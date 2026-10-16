# P33 scan: ai-serve and ai-integrations (read-only audit, Tier R1 to R2)

Base commit: f90796c. Scanner: Sonnet, read-only. Known and not repeated: STATUS.md, docs/audit/FINDINGS.md, .github/dead-pub-api-baseline.txt (the idle-code-baseline entries for the economics modules are mentioned only to say what is Known).

## COVERAGE

Read in full, every line, tests included:
- crates/ai-inference/crates/ai-serve/src/bridge.rs 1-542
- crates/ai-inference/crates/ai-serve/src/chain.rs 1-67
- crates/ai-inference/crates/ai-serve/src/config.rs 1-888
- crates/ai-inference/crates/ai-serve/src/cost_forecast.rs 1-542
- crates/ai-inference/crates/ai-serve/src/fee_scenario.rs 1-557
- crates/ai-inference/crates/ai-serve/src/health.rs 1-88
- crates/ai-inference/crates/ai-serve/src/lib.rs 1-26
- crates/ai-inference/crates/ai-serve/src/metric.rs 1-321
- crates/ai-inference/crates/ai-serve/src/residency.rs 1-1011
- crates/ai-inference/crates/ai-serve/src/staging.rs 1-334
- crates/ai-inference/crates/ai-serve/src/validator_cost.rs 1-449
- crates/ai-inference/crates/ai-integrations/src/lib.rs 1-107
- crates/ai-inference/crates/ai-integrations/src/rest.rs 1-136
- crates/ai-inference/crates/ai-integrations/src/ws.rs 1-125
Total 5193 lines, matches the AUDIT_PLAN row for P33.

Docs read: docs/AGENT_MAP.md 1-56; docs/AUDIT_PLAN.md lines 110-165; notes/P29-scan.md lines 1-30.

Outside the part (rg first, at most 80 lines each): crates/ai-inference/crates/ai-serve/Cargo.toml and ai-integrations/Cargo.toml; crates/ai-inference/Cargo.toml 1-40; crates/ai-inference/crates/ai-ops/src/main.rs 95-125 and 145-156; crates/ai-inference/crates/ai-core/src/system_prompt.rs (rg DECLARED_LIMITS only); xtask/gates/src/gates/ai_inference_prompt_is_true.rs 118-130; README.md 140-152; rust-toolchain.toml; .github/idle-code-baseline.txt (rg on ai-serve and ai-integrations lines, and the comment block at 1995-2040); .github/workflows/ci.yml line 572-573 (rg).

Not read: ai-core (system_prompt body, tier.rs, model.rs), ai-ops beyond the ranges above, ai-tune/eval.rs, the Cargo.lock. No cargo, no test run. Every claim comes from reading source; where a compile or run result would settle a claim, the finding says Likely and names the step.

Extra-check summary:
- Wiring (integration): the only non-test consumers of the part are ai-ops/src/main.rs (ServeConfig::for_tier, checked_system_prompt, rest::MAX_BACKOFF_SECS). Bridge, ChainClient (beyond NotConnected), Health, residency, staging, metric, fee_scenario, cost_forecast, validator_cost have no production caller anywhere in src/, budzero/, crates/, xtask/ (the one src/ hit, src/storage/qr_recipe.rs:180, is a doc comment). Findings P33-F01, F02.
- Untrusted input: the part has no network listener and no parser of external bytes. The only untrusted-input surfaces are the ai-integrations helpers (as_items, canonical_symbol, SessionTracker, RetryPolicy) that a future transport would feed with remote data, and the RpcError::MalformedResponse text. Findings P33-F11, F12, F13, F14.
- Auth: no authentication code exists. ws.rs SessionTracker is the only auth-state logic and it treats "connected" as authenticated: P33-F12. The bridge never checks the operator bond: P33-F06.
- Resource limits: no input in the part is size-bounded (as_items array, symbol length, shard list, device list). RetryPolicy.max_attempts is unbounded. P33-F11, F13.
- Secrets in logs (Z9): no secret value in the part. Debug derives on WsConfig, RestRequest, ServeConfig and an unbounded echo in as_items could carry a credential into logs: P33-F13, F14.
- Panics: `#![deny(clippy::indexing_slicing)]` in ai-serve and workspace `unwrap_used`/`expect_used = deny`. I found no unwrap, expect, panic!, todo!, direct integer indexing or unchecked subtraction in production code of ai-serve. One arithmetic overflow path in ai-integrations (P33-F11). One slice by byte index in ai-integrations/src/lib.rs:63 is safe today (char_indices) but that crate does not carry the indexing deny.
- Z3: `rg '#\[allow|#\[ignore|continue-on-error'` over the part returns no hit. No ignored test.
- Z7: both crates have `#![forbid(unsafe_code)]` (lib.rs:3 in each); `rg '\bunsafe\b'` hits only the comment lines at lib.rs:1.
- Z8: the part is not in the consensus directories listed in the Z8 scan, so f64 is not a Z8 hit. But fee_scenario.rs states it is "the single source every doc number quotes" for fees and wallet debits and uses f64 for money; if it is wired to settlement it falls under Z8. Noted in P33-F07.

Counts: 0 Critical, 0 High, 7 Medium, 11 Low, 1 Info (19 findings).

## FINDINGS

### P33-F01
File: crates/ai-inference/crates/ai-serve/src/bridge.rs:103-186, crates/ai-inference/crates/ai-serve/src/chain.rs:22-48, crates/ai-inference/crates/ai-serve/src/health.rs:38-58
Severity: Medium
Axis: integration
Confidence: Certain
Evidence:
```
$ rg -n 'Bridge::start|impl ChainClient|NotConnected' --glob '*.rs' crates/ai-inference | rg -v 'ai-serve/src'
crates/ai-inference/crates/ai-ops/src/cli.rs:46:    /// `Bridge::start`.
crates/ai-inference/crates/ai-ops/src/main.rs:151:                "ai-ops status: a skeleton - the chain connection is fail-closed (NotConnected)"
```
chain.rs:20-21: "The real client (based on ureq or reqwest) is added in the production phase". The only ChainClient implementors are `NotConnected` (chain.rs:36) and test mocks (bridge.rs:236, FixedChain). bridge.rs and its items (Bridge, StartupRefusal) are in neither .github baseline file; `rg 'ai-serve/src/bridge'` over both baselines returns nothing.
Description: `Bridge::start` has no production caller, and the only production `ChainClient` always returns `NotConnected`, which `attempt` turns into a refusal (bridge.rs:167-170). So no bridge can start outside tests. `Health::record_request`, `record_rejected_closed_loop`, `record_hash_failure` have no non-test caller (the last two are baselined), so the counters can only be zero in production.
Impact: the serving bridge, its startup checks and its health counters are documentation and tests, not a running path. README.md:148 describes ai-serve as refusing a consensus bridge, which is true only of a function nothing calls.
Known: partial. Idle-code baseline covers NotConnected, record_hash_failure, record_rejected_closed_loop, HealthSnapshot, assert_consensus_ready (idle-code-baseline.txt:956-965). Bridge, StartupRefusal, Bridge::start, Health::record_request, Health::record_refused_startup are not listed. Treat as a mainnet blocker per AUDIT_PLAN section 5.3 if the AI layer ships.

### P33-F02
File: crates/ai-inference/crates/ai-serve/src/bridge.rs:152-160, crates/ai-inference/crates/ai-serve/src/config.rs:356-393
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence:
bridge.rs:159: `let plan = assert_consensus_ready_on_device(&config, shards, budget, semantics)`
config.rs:362: `assert_consensus_ready(cfg)?;` which fails when `cfg.determinism` is `None` (config.rs:193-206).
config.rs:387-393: `if plan.streams_from_disk() { return Err(...cannot join the consensus path) }`
config.rs:116-117: "`None` means the bridge is for local/experimental use only". config.rs:343-345: "A streaming bridge serves locally, and is refused for consensus."
Description: `Bridge::start` is the only startup path, and it always runs the consensus-readiness check. A local bridge (no determinism profile) and a streaming bridge (the point of residency.rs and `ServeEngine::StreamingMoe`) are both documented as legitimate local modes but are both refused by `Bridge::start`. There is no local startup path at all.
Impact: the documented local and streaming operating modes cannot start; an operator on consumer hardware (the stated design target, residency.rs:1-18) can never get a `Bridge`. Also `disk_budget_bytes` (config.rs:125) can only change which refusal text appears: any plan with a nonzero disk footprint is refused by the streaming check after the bound passes, so the bounded-disk success path is unreachable through `Bridge::start`.
Known: none.

### P33-F03
File: crates/ai-inference/crates/ai-serve/src/bridge.rs:103-171
Severity: Medium
Axis: security
Confidence: Likely (missing step: the future ChainClient implementation and how model_id is derived; none exists in the tree)
Evidence:
```
103  pub fn start<C: ChainClient>(
104      config: ServeConfig,
105      model_id: &Hash32,
106      shards: &[WeightShard],
...
167      if !chain.model_registered(model_id)?  // model_id is a free parameter
```
Description: `model_id` is an unrelated caller parameter. Nothing binds it to `shards[*].content_id`, to `config.weight_source` or to `config.served_model_name`. The chain confirms only that some model with that id is registered. A bridge can pass the id of a registered model and serve other shards and another name.
Impact: the "registered on chain" gate (bridge.rs:162-166, "would answer requests for a model the chain does not know") is satisfiable by any registered id. Once wired, an operator loses nothing by lying about which model it serves, and the later settlement check is the only defence.
Trigger: precondition: a caller of `Bridge::start` (none today). Entry bridge.rs:103. Input: any registered `model_id`, arbitrary `shards`. Expected wrong result: `Ok(Bridge)`.
Known: none.

### P33-F04
File: crates/ai-inference/crates/ai-serve/src/config.rs:164-179, 424-437
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence:
```
166  if name.contains("example-org") || name.contains("upstream-base") {
172  if looks_like_multiplier(&cfg.served_model_name) {
```
`assert_served_name_is_ours` never compares the name to `cfg.tier.served_model_name(version)` or to the `ai_inference-{tier}-{version}` shape (config.rs:107). A name "my-custom-model", or the Normal name on a Light-tier config, returns Ok.
Description: the attribution rule is a deny-list of two placeholder strings (the same two strings that appear as the scrubbed weight_source "example-org/base-checkpoint-*", config.rs:139-140). Any real third-party model name passes. `looks_like_multiplier` also rejects innocuous names with a digit followed by `x` (for example "v0x1").
Impact: the check named "the served name is ours" (lib.rs:11-15, README.md in crates/ai-inference) does not enforce it. A bridge can serve "llama-3-70b" and start. Tier and name can disagree, so a Light-tier operator can be listed under the Normal name.
Known: none.

### P33-F05
File: crates/ai-inference/crates/ai-serve/src/config.rs:72-99, 191-213, 503-510
Severity: Medium
Axis: correctness
Confidence: Certain (code), Likely (impact; missing step: a real engine adapter that sets the flags)
Evidence:
config.rs:35 and 47-51: StreamingMoE "drives several backends at once ... a multi-backend engine is not sufficient on its own".
config.rs:79-81: `pinned_backend` is "A single fixed execution backend (CPU or CUDA or Metal -- not mixed)".
config.rs:97-99: `self.greedy && self.pinned_backend`.
Test at config.rs:503-510 asserts `StreamingMoe` plus `DeterminismProfile::for_consensus(42)` is accepted.
Description: the profile is two self-declared booleans. For an engine documented as multi-backend, `pinned_backend = true` is a contradiction the check accepts. `seed` is never checked, `greedy` is never verified against the engine, and nothing ties the profile to the engine actually running.
Impact: an operator running a heterogeneous streaming engine passes `assert_consensus_ready`, joins the `output_commitment` grouping, and its float-summation-order differences put it in a different group; the documented symptom (request never finalizes, config.rs:42-45) returns, now with a green gate.
Known: none (`is_bitwise_reproducible`, `assert_consensus_ready` are idle-baselined, which is the wiring gap, not this contradiction).

### P33-F06
File: crates/ai-inference/crates/ai-serve/src/bridge.rs:12-19, 137-172; chain.rs:28-29
Severity: Low
Axis: integration
Confidence: Certain
Evidence: bridge.rs:18-19: "an operator whose device cannot hold the model has no reason to be told about their bond." `ChainClient::operator_bond` exists (chain.rs:29) but `attempt` calls only `chain.model_registered` (bridge.rs:167). `rg -n 'operator_bond|pollen_grant_active' crates/ai-inference/crates/ai-serve/src` shows definitions, mocks and the NotConnected impl only.
Description: the startup order text promises a bond check that does not exist; no minimum bond is read, compared or configured. `pollen_grant_active` also has no caller.
Impact: a bridge starts with zero bond once a chain client exists. The documented startup contract and the code differ.
Known: none.

### P33-F07
File: crates/ai-inference/crates/ai-serve/src/fee_scenario.rs:57-59, 275-289
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence:
```
280  let fee = tokens as f64 / 1_000_000.0 * dollars_per_million_tokens;
281  if fee > self.balance_usd { return Err(...) }
287  self.balance_usd -= fee;
```
`PromptWallet::new` (line 268) accepts any f64. `transaction_fee_usd` (line 57-59) adds `amount_usd * rate / 10_000` with no sign check.
Description: a negative `dollars_per_million_tokens` makes `fee` negative, the guard `fee > balance` is false, and `balance_usd -= fee` increases the balance and returns `Ok(negative)`. A NaN rate makes `fee` NaN: the guard is false, the balance becomes NaN, and `Ok(NaN)` is returned. A negative `amount_usd` gives `transaction_fee_usd` a fee below the base or below zero. Negative initial balance is also accepted.
Impact: if the wallet or the fee function is wired to settlement with a rate or amount from any untrusted source, funds can be minted or fees refunded; and f64 money is not exact (the file describes itself as the single source for published fees). No test passes a negative or NaN input. Today there is no caller.
Known: partial. The economics modules are idle-baselined (idle-code-baseline.txt, comment block 1995-2040); the sign and NaN hole is not recorded.

### P33-F08
File: crates/ai-inference/crates/ai-serve/src/residency.rs:331-383, 247-276
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: the dense loop (residency.rs:359-383) places EveryToken shards first-fit in input order and returns `DensePartDoesNotFit` on the first shard that fits neither pool. Worked case: accelerator 700, system 400, shards [A=300 EveryToken, B=700 EveryToken]. dense_needed 1000 <= 1100. A goes to accelerator (400 left), B (700) fits neither remainder: Err. Input [B, A]: B to accelerator, A to system: Ok.
Description: the plan for the dense part depends on input order, and the refusal depends on it too. The doc claims "two operators with the same device producing the same plan" (residency.rs:259-261) and `the_plan_is_deterministic_across_shard_order` (residency.rs:768-792) tests only one dense shard plus equal-size routed shards. Placements Vec order also follows input order for dense shards, so `ResidencyPlan` (derives PartialEq over Vec) differs between orderings.
Impact: a device that can hold the model is refused, or two operators on identical devices get different plans, depending only on shard list order. No tie-break for dense shards (routed shards have one).
Known: none.

### P33-F09
File: crates/ai-inference/crates/ai-serve/src/residency.rs:291-329, 438-453
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: `rebalance(&self, heat, budget)` (line 318) calls `plan_with_heat` (line 291), which calls `place_internal` and has no disk bound. `plan_bounded_by_disk` (line 438) calls only `Self::plan`.
Description: heat ordering fills fast memory with the hottest shards instead of the largest, so the bytes left for disk can be larger than in the size-first plan. The disk budget (`ServeConfig::disk_budget_bytes`) is applied at startup but not after a rebalance, and there is no bounded variant of `plan_with_heat`.
Impact: after a rebalance a plan can exceed the operator's stated disk budget with no refusal, the opposite of the "fail-closed" rule in the module doc (residency.rs:413-433). Both functions are idle (baselined), so no live effect today.
Known: partial. plan_with_heat, rebalance, RoutingHeat are idle-baselined; the unbounded result is not recorded.

### P33-F10
File: crates/ai-inference/crates/ai-serve/src/bridge.rs:152-154, crates/ai-inference/crates/ai-serve/src/config.rs:395-417, residency.rs:191-199
Severity: Low
Axis: test
Confidence: Certain
Evidence:
bridge.rs:152: `if !shards.iter().any(|s| s.demand == Demand::EveryToken)` (bytes not examined).
config.rs:399-401 and 411: the "unknown placement" and "semantics altered" checks compare the plan to the values the planner copied from the same input (`place_internal` builds placements from `shards`, residency.rs:377-382, 402-407, and returns `semantics` by value, line 410).
Description: (a) a single EveryToken shard with `bytes: 0` satisfies the no-dense-part guard, so a model with no real dense weights starts. (b) Duplicate `content_id` shards are not detected anywhere. (c) `SemanticProfile` is not validated (weight_bits 0, context_tokens 0, experts_per_token 0 are accepted) and is not related to shard bytes. (d) The two post-plan checks cannot fail for any input, and no test can make them fail; they read as defence in depth but are tautologies. The weak test `the_device_check_also_requires_the_prompt_to_pass` (config.rs:871-887) admits in its comment that it asserts wiring rather than a refusal.
Impact: the checks give more assurance than they provide; a malformed model description starts.
Known: none.

### P33-F11
File: crates/ai-inference/crates/ai-integrations/src/rest.rs:38-88
Severity: Low
Axis: correctness
Confidence: Certain (overflow); Likely (the rest, missing step: a transport that uses the policy)
Evidence:
```
85  pub fn should_retry(&self, status: u16, attempt: u32) -> bool {
86      attempt + 1 < self.max_attempts && self.retry_on_status.contains(&status)
```
Description: (a) `attempt + 1` overflows at `attempt == u32::MAX`: debug build panics, release build wraps to 0 and returns true, so a caller counting wrongly retries. `delay_for_attempt` is tested at `u32::MAX` (rest.rs:109) but `should_retry` is not. (b) `RetryPolicy` is Deserialize (config-fed) but `max_attempts` has no ceiling and `base_delay_secs` may be 0, giving a hot retry loop of up to 4 billion attempts; the cap at MAX_BACKOFF_SECS bounds the wait, not the count. (c) The policy has no method awareness: `RestRequest` carries `Post`/`Patch` and the status list (429, 5xx) retries them, so a non-idempotent request (the T/msg shape in lib.rs:31-33 is a market-data feed that also places orders on some providers) can be sent twice. (d) No jitter and no Retry-After handling.
Impact: low today (no transport). The first item is a panic or unbounded retry on a configuration/counter error.
Known: partial. RetryPolicy, should_retry, delay_for_attempt are idle-baselined (idle-code-baseline.txt:897-900).

### P33-F12
File: crates/ai-inference/crates/ai-integrations/src/ws.rs:79-103, crates/ai-inference/crates/ai-integrations/src/lib.rs:31-42
Severity: Medium
Axis: security
Confidence: Likely (missing step: a transport that uses SessionTracker; none exists)
Evidence:
```
93  pub fn observe(&mut self, item: &serde_json::Value) {
94      if crate::is_success_connected_or_authed(item) {
95          self.state = ConnState::Authenticated;
```
lib.rs:41: `matches!(msg, "connected" | "authenticated")`.
Description: the unauthenticated `"connected"` message moves the tracker straight to `Authenticated`. `Connecting`, `Connected` and `Failed` are never set by any code (`rg 'ConnState::' ` shows only Disconnected and Authenticated in production code). The state never goes back on disconnect or on an error message. `on_connected` returns the login message but nothing connects the reply to the state.
Trigger: precondition: a transport that gates sending authenticated requests on `state() == Authenticated`. Entry ws.rs:93. Input: `{"T":"success","msg":"connected"}` from the server before auth. Expected wrong result: state Authenticated without credentials accepted.
Impact: a feed that reports connected-but-not-authenticated is treated as logged in; later authenticated calls fail or, worse, the session is reported healthy. Also an observer that sees one error message after authentication stays Authenticated.
Known: partial. SessionTracker, ConnState are idle-baselined (idle-code-baseline.txt:901,906).

### P33-F13
File: crates/ai-inference/crates/ai-integrations/src/lib.rs:23-29, 49-68
Severity: Low
Axis: security
Confidence: Certain
Evidence:
lib.rs:27: `other => Err(format!("beklenen nesne/dizi, gelen: {other}")),`
lib.rs:61: `let cuts: Vec<usize> = s.char_indices().map(|(i, _)| i).skip(1).collect();`
Description: (a) `as_items` places the entire rejected JSON value in the error string with no length cap; a string or number message from the remote end is copied into the error and then into any log. A scalar auth echo can carry a token (Z9 type: token, not present in the tree). (b) `as_items` also returns an array of any length unbounded; there is no per-message item or size limit in the crate. (c) `canonical_symbol` allocates a `Vec<usize>` of 8 bytes per character of the input symbol (8x amplification) and does one set lookup per prefix; the symbol is not length-limited. (d) the error text and several doc comments are Turkish (lib.rs:27, rest.rs:7) in an English-only tree; the Turkish text reaches operators in an error string. (e) ai-integrations lacks the `#![deny(clippy::indexing_slicing)]` that ai-serve has (ai-serve/src/lib.rs:8); the `s[..i]` slices at lib.rs:63-64 are safe now only because of `char_indices`.
Impact: bounded memory/log amplification from a hostile peer once a transport exists; a possible secret echo. Language issue is Info-level on its own.
Known: partial. as_items and canonical_symbol are idle-baselined (idle-code-baseline.txt:894-895); the error echo and allocation are not recorded.

### P33-F14
File: crates/ai-inference/crates/ai-integrations/src/ws.rs:24-33, crates/ai-inference/crates/ai-integrations/src/rest.rs:8-13, crates/ai-inference/crates/ai-serve/src/config.rs:102-126, crates/ai-inference/crates/ai-serve/src/bridge.rs:36-45, 62
Severity: Low
Axis: security
Confidence: Likely (missing step: a real URL with credentials in config; none in the tree)
Evidence: `#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)] pub struct WsConfig { pub url: String, ... }`; `RestRequest<T>` derives Debug and carries `path` and `body`; `ServeConfig` derives Debug with `base_url: String`; `Bridge` derives Debug (bridge.rs:89) and so prints `config`; `StartupRefusal::Chain(e)` displays `{e:?}` (bridge.rs:62) where `RpcError::MalformedResponse(String)` (chain.rs:17) holds unbounded text from the remote node.
Description: URLs and bodies are stored as plain strings with derived Debug. A `wss://host/stream?key=...` or `http://user:pass@host` value prints in full under `{:?}`. Nothing validates URL scheme either: `base_url` may be non-loopback and plain http, and `port` and `base_url` can disagree (config.rs:112-113, defaults 8000 and "http://127.0.0.1:8000/v1"); `Bridge::attempt` checks neither. A remote-supplied MalformedResponse string is copied into the operator message with no length cap or control-character filter.
Impact: possible credential in logs, log injection from a node response, and an unvalidated base_url in a layer whose design principle is a closed circuit.
Known: none.

### P33-F15
File: crates/ai-inference/crates/ai-serve/src/cost_forecast.rs:53-58, 144-162, 188-191; crates/ai-inference/crates/ai-serve/src/metric.rs:152-169
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
cost_forecast.rs:145: `GIB_PER_COLD_TOKEN * f64::from(weight_bits) / 4.0` returns 0.0 for `weight_bits == 0`, then line 160 divides by it: `disk_gib_per_second / 0.0 * speculation` gives `inf`. Line 157 guards only `<= 0.0`, so NaN passes. metric.rs:157 guards `tokens_per_second <= 0.0` only, so NaN passes and returns `Some(NaN)`; `inf` returns `Some(0.0)` cost.
cost_forecast.rs:189: `tb.accelerator as f64 / 1e9` (decimal GB) while shard sizes and budgets are in GiB (cost_forecast.rs:38) and the test helper documents the rate as "per gibibyte-hour" (cost_forecast.rs:312).
Description: zero precision or NaN input produces infinite throughput and a "free" cost per million tokens; a 7.4 percent unit mismatch (GB vs GiB) between the rates' stated unit in two places. `scaled_bytes(…, 0)` makes zero-byte shards, which then "fit" anywhere.
Impact: the published cost figures ("absurdly low cost" claim, cost_forecast.rs:293-297) are only as right as their inputs; invalid inputs yield a plausible zero rather than None. No caller yet.
Known: partial (functions idle-baselined).

### P33-F16
File: crates/ai-inference/crates/ai-serve/src/staging.rs:33-40, 94-160, 162-185
Severity: Low
Axis: integration
Confidence: Certain
Evidence: `stripe_plan` reads `StorageDevice.bandwidth_kib_s` (staging.rs:105, 117). `DeviceProbe.measured_kib_s`, `is_usable`, `weight_kib_s` (staging.rs:164-184) are read by no function in the crate or outside it; `rg 'weight_kib_s|is_usable' crates/ai-inference` shows only definitions and tests.
Description: the module doc says the probe "decides how much of one it reads" (staging.rs:13-15), but no code converts a probe into a `StorageDevice`; a probe that measured 0 while `device.bandwidth_kib_s` says 100 still receives a share. Duplicate `StorageDevice.id` entries are not rejected and give two chunks for one device. `stripe_plan` has no upper bound on the device list.
Impact: the invariant "a device that measures nothing receives nothing" holds only if the caller copies the probe value into the device by hand. The arithmetic itself (u128 shares, remainder to the last live device, checked adds) is sound: I found no overflow or gap.
Known: partial (StorageDevice, DeviceProbe, stripe_plan idle-baselined, idle-code-baseline.txt, staging.rs lines).

### P33-F17
File: crates/ai-inference/crates/ai-serve/src/config.rs:215-241, 294-324; validator_cost.rs:117-126; fee_scenario.rs:212, 408; crates/ai-inference/Cargo.toml (workspace.package rust-version) with validator_cost.rs:60-72
Severity: Info
Axis: dead code
Confidence: Certain (doc items); Likely (MSRV; missing step: build with the declared 1.75 toolchain, CI uses 1.97.1)
Evidence and description:
- config.rs:215-240 documents `checked_system_prompt` (including a `# Errors` section) but those doc lines run straight into the doc of `REQUIRED_PROMPT_PHRASES` (config.rs:241-248), so the public `checked_system_prompt` (config.rs:321) has no doc and the const carries two docs.
- `checked_system_prompt` verifies `AI_SYSTEM_PROMPT`, a `const &str`; the result is fixed at compile time. The check only protects against a modified source build; it cannot detect runtime tampering, and the module text (config.rs:233-236) presents it as a runtime half.
- `limit.to_string()` substring match (config.rs:298): the limits 1048576, 16777216, 3600000, 4096 are matched anywhere in the text, so a prompt with the number in another sentence passes. The CI gate checks the exact "- medium: at most N unit" line (system_prompt.rs:369-374); the runtime check is weaker than the gate it mirrors.
- validator_cost.rs:117-122 documents custody as one year ("user decision 2026-08-31: custody is one year") while fee_scenario.rs:71-78, 408 and the module doc say the floor is ten years; `one_year_storage_cost_usd_per_tb` is idle-baselined. Test comments (fee_scenario.rs:408, 499-503) still say the recipe "rides the upload base" and that the excess rule covers small items, contrary to fee_scenario.rs:122-129 ("the old excess rule is gone").
- MSRV: `[workspace.package] rust-version = "1.75"` (crates/ai-inference/Cargo.toml) but `const fn` with `f64` arithmetic and `u64 as f64` casts (validator_cost.rs:60-72, 105-115, 131-133, cost_forecast.rs not const f64) needs a newer compiler than 1.75; CI builds with 1.97.1 (rust-toolchain.toml), so the declared MSRV is not enforced for this workspace.
Impact: maintenance risk and misleading docs only.
Known: none.

### P33-F18
File: crates/ai-inference/crates/ai-serve/src/*.rs (tests), crates/ai-inference/crates/ai-integrations/src/*.rs (tests)
Severity: Low
Axis: test
Confidence: Certain
Evidence: tests exist in every file; no test uses hostile input for ai-integrations (no oversize array, no non-ASCII symbol with multi-byte upper-casing, no `should_retry(_, u32::MAX)`, no `SessionTracker` fed an error after authenticated, no server-sent "connected" before auth). No test passes negative or NaN to `PromptWallet::charge`, `transaction_fee_usd`, `cost_per_million_tokens_dollars`, or `weight_bits = 0` to `frontier_model`. No test feeds `assert_served_name_is_ours` a non-placeholder third-party name (config.rs:460-485 uses "upstream-base" and multiplier tags only). No test for dense shards in different input order (P33-F08). No `#[ignore]`.
Description: the negative tests that exist are good for the happy-path refusals (device too small, unknown model, no chain), but the hole classes in F04, F07, F08, F11, F12, F15 have no test.
Impact: the findings above would not be caught by the existing suite.
Known: none.

### P33-F19
File: crates/ai-inference/crates/ai-serve/src/residency.rs:165-183, crates/ai-inference/crates/ai-serve/src/cost_forecast.rs:70-77
Severity: Low
Axis: security
Confidence: Likely (missing step: the router that feeds RoutingHeat; none exists)
Evidence: `RoutingHeat::record(content_id)` inserts a key into a `BTreeMap` for any id (residency.rs:173-176) with no check that the id belongs to a known shard; `frontier_model` fills ids with `[n as u8; 32]` for `n in 1..=EXPERT_COUNT` (cost_forecast.rs:70-76, truncating cast, safe only while EXPERT_COUNT <= 255).
Description: the heat map grows with every distinct id recorded and is never pruned; `rebalance` ignores ids that are not shards. The truncating cast would silently duplicate ids if the expert count constant grows past 255.
Impact: unbounded memory if a router ever feeds it ids from untrusted routing output; latent duplicate-id risk on a constant change (duplicate ids are undetected, P33-F10).
Known: partial (RoutingHeat idle-baselined).

## Not findings (checked, no issue)

- staging.rs `stripe_plan`: chunks sum to `len`, offsets contiguous, remainder goes to the last live device, all arithmetic is checked or u128; tests cover rotation and dead last device.
- residency.rs `place_internal`: all subtractions follow a `<=` guard; fold uses `saturating_add`; sort has a content-id tie-break so routed order is deterministic.
- fee_scenario.rs `bud_extension_fee_usd_per_tb`: the range guard precedes the u32 subtraction; `recipes_for_content` guards zero item size.
- health.rs: relaxed atomics for monotonic counters, no ordering requirement.
- Both lib.rs have `#![forbid(unsafe_code)]`; no `#[allow]`, no `#[ignore]`, no `unwrap`/`expect` in production code of the part.
- No secret value, key or token string in any of the 14 files.
