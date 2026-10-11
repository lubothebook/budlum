# P41 scan: Tools and binaries (read-only audit)

Base commit: 7ad75a9. Scanner: Sonnet, read-only. Known and not repeated: STATUS.md, docs/audit/FINDINGS.md, .github/dead-pub-api-baseline.txt (idle-code-baseline.txt entries are marked Known where they apply).

## COVERAGE

Read in full (every line, tests included):
- budzero/bud-node/Cargo.toml 1-39
- budzero/bud-node/src/lib.rs 1-46
- budzero/bud-node/src/bitswap.rs 1-458
- budzero/bud-node/src/discovery.rs 1-435
- budzero/bud-node/src/sharding.rs 1-238
- budzero/bud-node/src/store.rs 1-328
- budzero/bud-cli/Cargo.toml 1-26
- budzero/bud-cli/src/main.rs 1-1310 (two reads: 1-700, 700-1310)
- budzero/bud-cli/tests/toolchain_end_to_end.rs 1-351
- src/bin/bud.rs 1-583
- src/bin/budlum-light.rs 1-190
- src/bin/budlum-relayer.rs 1-1003
- examples/print_genesis_hash.rs 1-20
- build.rs 1-36

Docs read: docs/AGENT_MAP.md 1-56; docs/AUDIT_PLAN.md sections 5 and 6 (lines 110-165); notes/P29-scan.md lines 1-30.

Outside the part (rg first, at most 80 lines): src/rpc/api.rs 13-30, 360-368, 451-456; src/rpc/server.rs 443-453, 520-547, 1409-1450; src/core/transaction.rs (DEFAULT_CHAIN_ID, Transaction::new); src/core/chain_config.rs 29-38; src/crypto/primitives.rs 648-651; src/light_client/mod.rs 459-481; src/main.rs 1140-1165; src/network/node.rs 1720-1770 and rg hits for bud_node; budzero/bud-proof/src/relayer.rs 164-205 and rg; budzero/bud-state/src/lib.rs 158-209; .github/idle-code-baseline.txt 768-785 (rg); .github/workflows (rg only: supply-chain-extra.yml, semgrep.yml); Cargo.toml (rg).

Not read: the libp2p, clap, reqwest sources; src/network/node.rs beyond the hits above; the bud-proof prover; no cargo, no test run. Every claim comes from reading source.

Extra-check summary:
- Z9 (keys and secrets): bud.rs takes the signing seed from --priv-key-file or BUD_PRIV_KEY, never argv (good; bud.rs:243-255). No hard-coded keys or seeds in the part. The seed is held in plain String/[u8;32] with no zeroization and the file mode is not checked: P41-F07. budlum-relayer refuses --relayer-key (budlum-relayer.rs:405-414, good).
- CLI input validation: bud.rs validates hex length, port, address; budlum-light validates anchors through LightClient::new; bud-cli silently drops args beyond 31 and does not bound file sizes: P41-F09, P41-F10, P41-F15.
- Panics: no unwrap, expect, panic or direct indexing in production code of the 13 files except build.rs (expect, accepted at build time with a justified allow) and a debug_assert in bud-cli main.rs:589. Arithmetic overflow candidates: P41-F10.
- Mainnet gating: bud tx send always signs with the devnet chain id (P41-F01); budlum-relayer has no network gate (P41-F05); bud-cli has no network concept and a default chain_id of 1 (P41-F11); bud-node has memory-only storage (P41-F08).
- Unwired: the relayer EthToBud direction cannot relay (P41-F03); BudToEth scan calls the RPC with wrong parameters (P41-F02); ContentDiscovery cache and ShardManager::should_cache have no production caller (Known: idle-code-baseline.txt 773-785).
- Z3: no continue-on-error or ignore in the part. Allows: build.rs:4 (expect_used, justified), budlum-relayer.rs:1 (blanket pedantic and nursery, P41-F13), bud.rs:278 (too_many_arguments). Z6: no shell or python files in the part. Z7: bud-node has forbid(unsafe_code) (lib.rs:124); no unsafe in the other files.

Counts: Critical 0, High 0, Medium 8, Low 10, Info 2 (total 20).

## Findings

### P41-F01
File: src/bin/bud.rs:304-308 (also :288, no chain id option in the Cli struct at :40-46)
Severity: Medium
Axis: integration
Confidence: Certain
Evidence:
```
src/bin/bud.rs:304:    let mut tx = Transaction::new(from, to_addr, amount, Vec::new());
src/bin/bud.rs:305:    tx.fee = fee;
src/bin/bud.rs:306:    tx.nonce = nonce;
src/bin/bud.rs:307:    tx.tx_type = TransactionType::Transfer;
src/bin/bud.rs:308:    tx.sign(&keypair);
src/core/transaction.rs:471: pub fn new(from, to, amount, data) -> Self { Self::new_with_chain_id(..., DEFAULT_CHAIN_ID, TransactionType::Transfer)
src/core/transaction.rs:20: pub const DEFAULT_CHAIN_ID: u64 = 45262;   (devnet)
src/core/chain_config.rs:34-36: Mainnet 45260, Testnet 45261, Devnet 45262
```
Description: `bud tx send` builds the transaction with Transaction::new, which fixes the chain id to the devnet value 45262. There is no --chain-id or network flag and the node's chain id is not fetched before signing.
Impact: the only user CLI for L1 transfers cannot produce a valid transaction for mainnet (45260) or testnet (45261); signed transactions are bound to the wrong chain (if the node accepts them, replay across chains is the risk; if it rejects them, the CLI is unusable). Mainnet blocker for the wallet CLI.
Known: none

### P41-F02
File: src/bin/budlum-relayer.rs:769-771 versus src/rpc/api.rs:16-17
Severity: Medium
Axis: integration
Confidence: Certain
Evidence:
```
budlum-relayer.rs:770:    let params = serde_json::json!([format!("0x{height:x}"), true]);
budlum-relayer.rs:771:    let block = client.rpc_call("bud_getBlockByNumber", params).await?;
src/rpc/api.rs:16:    #[method(name = "bud_getBlockByNumber")]
src/rpc/api.rs:17:    async fn get_block_by_number(&self, number: u64) -> ...
```
Description: the relayer sends a hex string plus a second boolean to a method that takes exactly one u64. A jsonrpsee positional call with a string for a u64 and an extra parameter is an invalid-params error, so every block scan fails and the loop retries the same height forever (budlum-relayer.rs:745-750). `bud.rs:349-353` sends a plain number and is correct. The unit tests only exercise burn_hashes_from_block, so this never ran.
Impact: the BudToEth burn scan never finds a burn; the operator sees only retry log lines. No test calls the RPC shape.
Known: none

### P41-F03
File: src/bin/budlum-relayer.rs:285-343, 352-361, 606-617
Severity: Medium
Axis: integration
Confidence: Certain
Evidence:
```
budlum-relayer.rs:334:                    amount: 0,
budlum-relayer.rs:357:        Err(
budlum-relayer.rs:358:            "EthToBud deposit proof assembly is not implemented yet; refusing to submit placeholder proofs"
budlum-relayer.rs:288-296: filter without topics ("topics": [["0x..."]] is a placeholder; fetch every log)
```
Description: EthToBud never relays: get_deposit_logs returns placeholder events (amount 0, zero recipient, every log of the bridge address with no topic filter) and build_deposit_proof always returns Err. BudToEth refuses to submit by design (budlum-relayer.rs:699-703). The relayer is a mainnet blocker for the cross-chain path; the refusal is fail-closed and honest, but the binary has no mode that does useful work and the usage text (print_usage, :444-474) and module doc (:1-37) describe a working relay.
Impact: an operator who bonds 1000 BUD and runs the binary relays nothing and earns nothing; the module doc and --help overstate the capability.
Known: none (related Z-rule: experimental part must be listed as mainnet blocker)

### P41-F04
File: src/bin/budlum-relayer.rs:600-603, 606-617, 638-652
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence:
```
608:            let proof_json = match eth_client.build_deposit_proof(&dep).await {
609:                Ok(p) => p,
610:                Err(e) => { eprintln!(...); continue; }
...
638:                Err(e) => { eprintln!("EthToBud: submit_relay_proof failed for {}: {e}", dep.tx_hash); }
652:        last_block = to;
```
Description: the scan cursor advances to `to` after the batch whatever happened to each deposit. A transient proof-build error or a transient submit error (timeout, node busy) drops that deposit permanently; there is no retry queue and no persisted cursor. The BudToEth loop got the opposite treatment (cursor advances only past scanned heights, :728-751), so the two loops disagree.
Impact: once the proof builder exists, any transient failure loses a deposit relay; restart also resets the cursor to head minus confirmations (:556-566), dropping deposits that happened while the relayer was down.
Known: none

### P41-F05
File: src/bin/budlum-relayer.rs:491-498, 504-508
Severity: Medium
Axis: security
Confidence: Certain
Evidence:
```
496:    if config.relayer_address == "0x0" || config.relayer_address.len() < 10 {
497:        eprintln!("WARN: relayer-address is placeholder - permissionless gate will skip active check (devnet mode). Set real Budlum address for mainnet.");
505:    if config.relayer_address == "0x0" {
506:        eprintln!("Devnet mode: relayer-address placeholder, skip active check (assume active).");
507:        return true;
```
Description: the registry gate is skipped by a value the operator controls (the default "0x0") with no network selector. The binary has no mainnet mode, so "devnet mode" is only a string compare. The skip is checked with `== "0x0"` while the warning also fires for any address shorter than 10 chars; a short address such as "0xab" is not skipped but is sent to the node as is. The node enforces the gate again (doc :22-25), so this is defense in depth, but the comment claims a devnet versus mainnet distinction that does not exist.
Impact: local only; an operator on mainnet who forgets the flag starts in "assume active" mode and learns of it only on the first rejected submission.
Known: none

### P41-F06
File: src/bin/budlum-relayer.rs:181-197
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
181:        let normalized = address.to_lowercase();
184:                if acc.to_lowercase() == normalized {
```
Description: the active-member check compares strings after lowercasing only. A configured address without the 0x prefix or with a different prefix form never matches a member reported with the prefix (and the reverse), so an active relayer is reported inactive and the EthToBud loop refuses to start (:547-553). The helper test (:961-985) re-implements the comparison inline and never calls is_active_relayer, so it tests nothing of the production function.
Impact: false negative at startup for a correctly bonded relayer; the test gives false assurance.
Known: none

### P41-F07
File: src/bin/bud.rs:243-265, 287-288
Severity: Low
Axis: invariant(Z9)
Confidence: Certain
Evidence:
```
245:        return std::fs::read_to_string(path).map(|s| s.trim().to_string())...
249:    std::env::var("BUD_PRIV_KEY")...
257: fn parse_seed(hex_str: &str) -> Result<[u8; 32], String> {
```
Description: the seed travels as an ordinary String and an unzeroized [u8; 32] and is never wiped; the key file permissions are not checked (a world-readable seed file is accepted silently); an environment-variable seed is visible in /proc/<pid>/environ and inherited by children. Error texts do not echo the seed (good). `KeyPair::from_seed` (src/crypto/primitives.rs:648) takes the array by reference with no zeroize either.
Impact: local key exposure on a shared host; not remote.
Known: none

### P41-F08
File: budzero/bud-node/src/store.rs:106-136, 154-162; src/main.rs:1150
Severity: Medium
Axis: security
Confidence: Certain
Evidence:
```
store.rs:131: /// Create a store with default capacity (10,000 chunks ≈ 2.5 GiB
store.rs:132: /// At 256 KiB per chunk).
store.rs:139: fn put(&self, id: ContentId, data: Vec<u8>) -> Result<(), StoreError> {   (no length check)
store.rs:155:        if map.len() >= self.max_capacity && !map.contains_key(&id) {
store.rs:156:            if let Some(oldest) = map.keys().next().copied() {   // lowest CID, not oldest
src/main.rs:1150:        let store = Arc::new(bud_node::MemoryContentStore::with_default_capacity());
```
Description: (a) capacity is counted in chunks, not bytes, and put has no per-chunk size cap; bitswap accepts messages up to 16 MiB + 256 (bitswap.rs:130), so 10,000 chunks can hold about 160 GiB, not the documented 2.5 GiB. (b) "evict oldest" actually evicts the lowest CID (BTreeMap first key), so any peer that serves valid but self-chosen data for requested CIDs can push out chunks with low CIDs, and eviction order is unrelated to age or deal status. (c) The only store the production node builds is this memory store (src/main.rs:1150); SledContentStore is "planned for mainnet" (store.rs:6-7), so a node restart loses all stored B.U.D. content.
Impact: remote memory exhaustion of a storage node (attacker with fetchable CIDs), loss of content on restart, unpredictable eviction of content a node has a deal for. Mainnet blocker (no persistent store).
Known: none

### P41-F09
File: budzero/bud-node/src/bitswap.rs:64-78, 129-130
Severity: Low
Axis: security
Confidence: Certain
Evidence:
```
74:        io.take(MAX_MESSAGE_SIZE as u64).read_to_end(&mut buf).await?;
130: const MAX_MESSAGE_SIZE: usize = 16 * 1024 * 1024 + 256;
```
Description: read_request reads up to 16 MiB for a request whose valid encoding is 32 bytes (a ContentId). A peer can hold 16 MiB per open inbound stream before decode runs, and bincode with allow_trailing_bytes (:260) accepts a 32-byte request followed by garbage. The module doc says "Rate limiting is handled by the libp2p swarm layer" (:20) but this crate builds no swarm and sets no concurrency limit; the limit, if any, lives in src/network/node.rs (not read here beyond the hits).
Impact: memory pressure per stream on a storage node; weak input discipline on the request side.
Known: none

### P41-F10
File: budzero/bud-cli/src/main.rs:292-296, 893, 331
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
292:    for (i, val) in config.args.iter().enumerate() {
293:        if i < 31 { vm.registers[i + 1] = *val; }
893:                let step_nonce = nonce.map(|n| n + index as u64);
331:            acc.nonce += 1;
```
Description: (a) arguments past the 31st are dropped silently, so the program runs and is proven with fewer inputs than the user gave. (b) `n + index as u64` and `acc.nonce += 1` are unchecked; in release without overflow checks they wrap, with them they panic. (c) The state nonce increments by exactly 1 whatever --nonce the user supplied (line 285-287 only changes vm.context.nonce, which becomes the public input), so the proof binds nonce N while the persisted account moves from M to M+1 and the two diverge.
Impact: local tool; silent input truncation and a nonce in the proof that does not match the persisted nonce.
Known: none

### P41-F11
File: budzero/bud-cli/src/main.rs:40-46, 369-371, 1088-1105
Severity: Low
Axis: invariant (mainnet gating)
Confidence: Certain
Evidence:
```
40: #[arg(long, default_value_t = 1, help = "The unique chain identifier for execution context")]
41: chain_id: u64,
```
Description: the chain id defaults to 1, which is not a Budlum network id (45260 to 45262, chain_config.rs:34-36); any value is accepted, including 0. A proof produced with the default is bound to a chain id no node uses. The integration test also hard-codes chain_id: 1 (toolchain_end_to_end.rs:78).
Impact: proofs from the default CLI invocation cannot match a production verifier that checks chain_id; an operator can produce a signed relay report for any chain id.
Known: none

### P41-F12
File: budzero/bud-cli/src/main.rs:12-30, 995-1021; budzero/bud-proof/src/relayer.rs:164-205
Severity: Medium
Axis: invariant (mainnet gating)
Confidence: Certain
Evidence:
```
main.rs:47-54: --activation is global; Full opens VerifyMerkle and VerifyInference ("never the production setting while they are closed")
main.rs:1016: activation: cli.activation.state(),
relayer.rs canonical_payload (164-205): fields are schema, status, time, program_hash, canonical flag, set digest, proof fingerprint, alarm. No activation field.
```
Description: `bud-cli relay --activation full` verifies and signs a report under the open activation state, and the signed payload does not record which activation state was used. Such a report is byte-compatible with one made under the closed default, so a consumer of the report cannot tell that a program the production verifier refuses (VerifyMerkle 64 gate, AUDIT_PLAN section 5 ZK check) was accepted. The flag also has no warning, no confirmation and no tie to a network.
Impact: a monitor or relayer operator can publish a "VALID" report for a program the closed production gate refuses; the report cannot be audited for this afterward.
Known: none

### P41-F13
File: budzero/bud-cli/src/main.rs:533-593, 649-678; budzero/bud-proof/src/relayer.rs:207-209, 348, 441
Severity: Low
Axis: integration
Confidence: Certain
Evidence:
```
main.rs:649: let mut ledger = NodeLedger { alarms: AlarmLog::new(), quarantine: QuarantineLedger::new() };
main.rs:589: debug_assert!(ledger.alarms.verify_integrity(), ...)
relayer.rs:208: keccak256(&self.canonical_payload()) == self.report_sig
```
Description: (a) the alarm log and quarantine ledger are created fresh in every CLI process, written only to tracing output and dropped; nothing persists the ban, so the "quarantine" never refuses a later pass as the comment (:536-541) claims. (b) The integrity check on the chained log is a debug_assert and vanishes in release builds. (c) The "keccak-signed report" is an unkeyed hash of the payload: anyone who can edit the file can recompute report_sig, so verify_report_sig detects accidental corruption only, not forgery. The CLI help text and the word "signed" (main.rs:173, 204, 612) suggest authentication.
Impact: K3 and K4 ledgers have no effect outside one process; a report's signature gives no origin guarantee.
Known: none

### P41-F14
File: src/bin/budlum-light.rs:121-189
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence:
```
127:    let mut next = client.next_checkpoint_height(cli.checkpoint_interval, target);
128:    while let Some(checkpoint_height) = next { ... }
184-189: println!("LIGHT CLIENT TRUSTED height=... "); ExitCode::SUCCESS
src/light_client/mod.rs:460: if interval == 0 || self.trusted.height >= target_height { return None; }
```
Description: with --checkpoint-interval 0, a --target-height at or below --trusted-height, or a range with no checkpoint multiple, the loop body never runs and the binary prints "LIGHT CLIENT TRUSTED height=<anchor>" and exits 0 without verifying any certificate. The header chain is checked, but no finality proof is. Also next_checkpoint_height (light_client/mod.rs:463-469) walks height by height up to target, so a huge --target-height with a large interval is a long CPU loop (u64 range) on the operator's machine. The default interval 10 (budlum-light.rs:59) is not read from the chain's config.
Impact: an operator or script that checks only the exit code can treat an unverified export as verified (wrong or mistyped interval).
Known: none

### P41-F15
File: src/bin/budlum-light.rs:80-96; src/bin/bud.rs:164-198; budzero/bud-cli/src/main.rs:461-470, 944
Severity: Low
Axis: security
Confidence: Certain
Evidence:
```
budlum-light.rs:80: let raw = match fs::read(&cli.chain_file) {      (whole file, no size cap)
bud.rs:196:  let body = text.split_once("\r\n\r\n").map(|(_, b)| b).unwrap_or(&text);
bud-cli main.rs:944: let bytes = fs::read(bytecode)...
```
Description: untrusted files are read whole with no size ceiling (chain export, bytecode, proof and state JSON; ProofEnvelope::from_json_bounded at main.rs:462 bounds the envelope only). bud.rs bounds the RPC response (64 MiB) but its hand-rolled HTTP client ignores the status line and does not decode chunked transfer encoding, so a node answering with Transfer-Encoding: chunked yields a body with chunk-size lines and a JSON parse error; IPv6 hosts such as [::1] fail (parse_rpc_url :150) and the CLI sends signed transactions in clear text over http only.
Impact: local resource use on hostile input; usability faults against standard HTTP servers; no TLS for signed transactions to a remote node.
Known: none

### P41-F16
File: src/bin/budlum-relayer.rs:127-131, 225-229, 154-157, 254-257, 297-299
Severity: Low
Axis: security
Confidence: Certain
Evidence:
```
129: .timeout(Duration::from_secs(15)).build().unwrap_or_else(|_| reqwest::Client::new());
154: let json: serde_json::Value = resp.json().await ...   (no body size cap)
298: .rpc_call("eth_getLogs", ...)   (range from..to unbounded)
```
Description: both RPC clients read unbounded JSON bodies from configured endpoints (Eth RPC is a third-party endpoint). The fallback to reqwest::Client::new() drops the 15 s timeout silently. eth_getLogs is requested for the whole interval since last_block with no chunking, so after an outage the call can exceed provider limits and fail on every poll (the loop continues without moving the cursor, :593-598). Endpoint URLs are not validated (scheme, host), http for the Ethereum endpoint is accepted.
Impact: a hostile or broken endpoint can exhaust relayer memory; a long outage can stall the relayer permanently.
Known: none

### P41-F17
File: src/bin/budlum-relayer.rs:1, 807-813 ; src/bin/bud.rs:278
Severity: Low
Axis: invariant(Z3)
Confidence: Certain
Evidence:
```
budlum-relayer.rs:1: #![allow(clippy::pedantic, clippy::nursery)]
bud.rs:278: #[allow(clippy::too_many_arguments)]
build.rs:4: #![allow(clippy::expect_used)]
```
Description: the relayer binary switches off two whole lint groups, which hides the unwrap and indexing style lints the workspace uses to keep panics out; the inline comment for build.rs:4 is a valid justification, the other two have none beyond the Semgrep note. No panic path was found in the relayer, so this is maintenance risk only. The Semgrep exclusion (semgrep.yml:85-94) for the two argv hits is documented.
Impact: lint coverage gap on a binary that handles remote JSON.
Known: none (allow-count budget in suppression baseline not checked here)

### P41-F18
File: budzero/bud-cli/tests/toolchain_end_to_end.rs:182-212, 254-277; src/bin/budlum-relayer.rs:957-985, 961-985
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
toolchain_end_to_end.rs:211:    let _ = public_inputs_for(&vm, &bytecode, &receipt.events);   // builds a value and discards it
toolchain_end_to_end.rs:272:    assert!(receipt.success, "kosum basarisiz: {:?}", receipt.error);
budlum-relayer.rs:952-958: relayer_config_permissionless_gate asserts the default struct value (min_stake == 1000) only
```
Description: the struct test is named "runs and proves" but proves nothing (comment admits it); the integration test file never invokes the bud-cli binary (no run, relay, call, batch or verify command is exercised end to end; the unit tests in main.rs call run_pipeline and write_signed_relay_report only on error paths and Halt programs); the relayer test relayer_config_permissionless_gate asserts constants and a test at :961 re-implements the filter inline. Turkish assertion text in two asserts (CLAUDE.md language rule if applicable). bud.rs has no test for load_priv_key, tx building or the RPC client.
Impact: regressions in the CLI commands and in key loading would not be caught.
Known: none

### P41-F19
File: budzero/bud-node/src/sharding.rs:415-433, 445-449; budzero/bud-node/src/discovery.rs:123-236
Severity: Info
Axis: dead code
Confidence: Certain
Evidence:
```
src: rg -n "should_cache|is_resource_buffer_sufficient|\.xor_distance|should_announce|mark_announced|add_provider\(|get_providers" src crates --glob '*.rs'
(no production call site; only budzero/bud-node tests)
src/network/node.rs:1615: let key = bud_node::ContentDiscovery::cid_to_key(&cid);   (the only discovery call)
```
Description: ShardManager is built (src/network/node.rs:789) but should_cache and xor_distance are never called, and is_resource_buffer_sufficient is a constant true placeholder (sharding.rs:429-433), so "active sharding" and "mobile mode" have no effect; the ContentDiscovery provider cache (add_provider, get_providers, should_announce, mark_announced, cids_needing_reannouncement) and DiscoveryConfig are unused in production. The doc in sharding.rs:341-343 says it is reached from startup; construction is, behavior is not.
Impact: a feature documented as active is inert; the module docs overstate wiring.
Known: .github/idle-code-baseline.txt lines 768-785 (listed there)

### P41-F20
File: build.rs:6-35; examples/print_genesis_hash.rs:1-20
Severity: Info
Axis: integration
Confidence: Certain
Evidence:
```
build.rs:7: println!("cargo:rerun-if-changed=proto/budlum/network/protocol.proto");
build.rs:12: std::env::var("PROTOC").map_or_else(...)
examples/print_genesis_hash.rs:45: let mainnet = mainnet_genesis().build_genesis_block();
rg -l "print_genesis_hash|MAINNET_HASH": docs/AUDIT_PLAN.md, examples/print_genesis_hash.rs only
```
Description: (a) build.rs does not declare rerun-if-env-changed=PROTOC nor rerun-if-changed for build.rs itself, so a changed PROTOC leaves a stale build; protoc is taken from fixed paths or PATH without a version check (build-time only). (b) The genesis hash example prints the three hashes but no workflow or script compares them with a pinned value (`rg` finds no caller), so it is an unwired operator helper and cannot itself fail; the real pin gate lives elsewhere (not read). It also exposes mainnet_genesis() output before mainnet launch, which is a value that changes with any genesis edit.
Impact: stale proto output after an env change; the helper provides no gate.
Known: none
