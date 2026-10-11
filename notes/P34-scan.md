# P34 scan: AI helper crates (read-only audit)

Base commit: f90796c. Scanner: Sonnet, read-only. Known and not repeated: STATUS.md, docs/audit/FINDINGS.md, .github/dead-pub-api-baseline.txt (not re-read for this scan beyond the task statement; items below that may already be baselined are marked "Known: not checked").

## COVERAGE

Read in full, every line (tests included):
- crates/ai-inference/crates/ai-core/src: dataset.rs 1-197, lib.rs 1-14, manifest.rs 1-91, model.rs 1-150, system_prompt.rs 1-499, tier.rs 1-65
- crates/ai-inference/crates/ai-data/src: jsonl.rs 1-84, lib.rs 1-17, source.rs 1-104, template.rs 1-42, verify.rs 1-109
- crates/ai-inference/crates/ai-knowledge/src: cache.rs 1-187, chunk.rs 1-203, context.rs 1-247, embed.rs 1-252, lib.rs 1-45, memory.rs 1-330, redact.rs 1-463
- crates/ai-inference/crates/ai-ops/src: cli.rs 1-155, logparse.rs 1-254, main.rs 1-297
- crates/ai-inference/crates/ai-tune/src: eval.rs 1-620, lib.rs 1-14, lock.rs 1-60, plan.rs 1-120, schema.rs 1-153

Docs read: docs/AGENT_MAP.md 1-56; docs/AUDIT_PLAN.md 110-164 (sections 5 and 6); notes/P29-scan.md 1-30.

Outside the part (rg only, at most 80 lines): crates/ai-inference/Cargo.toml (head), the five crate Cargo.toml files, ai-tune/data/ai-behaviour-curriculum.jsonl (head 600 bytes, line count), .github/workflows/ci.yml 555-580, src/storage/content_id.rs (rg on constructors), src/ai_inference/mod.rs (rg MIN_OPERATOR_BOND, AiDatasetKind), src/domain/storage_params.rs (rg).

Not read: ai-serve, ai-integrations (not in this part), the rest of ai-tune/data, test-only bodies of the xtask gates that reference these files (ai_inference_prompt_is_true.rs was only matched by rg). No cargo, no test run: every claim is from reading source. Collision and panic claims below are derived by hand from the code shown; none was executed.

Extra-check summary:
- Z8 (float in consensus-reachable paths): f64 appears in embed.rs, context.rs:185, schema.rs:273-285, eval.rs (score, gate), main.rs:189. None of the five crates is a dependency of the root crate, budzero, or any other crate (rg on Cargo.toml files outside crates/ai-inference returns nothing; src/ hits are for an unrelated module named ai_inference). Not consensus-reachable today, so no Z8 finding. Info only: P34-F17.
- Determinism: BTreeMap/BTreeSet used in embed.rs, cache.rs, redact.rs report; one HashMap in main.rs:208 used only for lookup, never iterated. Sorting in context.rs/memory.rs/embed.rs is stable with explicit tie-breaks except memory.rs ties (F13). No determinism finding on a consensus path.
- Bounds and panics from external input: no unwrap/expect in production code (workspace lint denies them; hits are all under cfg(test)). One real slice panic on external text: P34-F06 (verify.rs:57). All other slices checked (redact.rs 213-233, logparse.rs 52-68 and 185-189, chunk.rs, context.rs) land on char boundaries or are bounded by find().
- Unwired parts: the whole ai-knowledge crate has no dependent (F05); ai-core lock/manifest path has no production caller (F07); ai-data source/verify helpers have no caller outside tests (F10); logparse.rs is dead behind an allow (F14).
- Z3: one `#![allow(dead_code)]` (logparse.rs:19), F14. No `#[ignore]` anywhere in the part.
- Z7: no `unsafe` in code; every crate has `#![forbid(unsafe_code)]` (ai-core lib.rs:3, ai-data lib.rs:3, ai-knowledge lib.rs:3, ai-tune lib.rs:3, ai-ops main.rs:5).
- Z9: no key, seed or token value in the part. Test secrets are assembled with format! at run time (cache.rs:167, chunk.rs:175, memory.rs:298, redact.rs:338). No finding.
- CI: crates/ai-inference is built and tested in .github/workflows/ci.yml:572-573 (clippy -D warnings plus cargo test), so the code compiles in CI.

## Findings

### P34-F01
File: crates/ai-inference/crates/ai-knowledge/src/redact.rs:312-328 (used by cache.rs:91 and memory.rs:129)
Severity: Medium
Axis: security
Confidence: Certain
Evidence:
```
312 pub fn redact_model_strings(value: &serde_json::Value) -> serde_json::Value {
314         serde_json::Value::String(s) => {
315             let r = redact_text(s);
...
321         serde_json::Value::Object(map) => serde_json::Value::Object(
322             map.iter()
323                 .map(|(k, v)| (k.clone(), redact_model_strings(v)))
```
Description: Each JSON string is masked in isolation, so the object key (`password`, `api_key`, `token`) is never seen. Only values that match a known shape (sk-, ghp_, AKIA, JWT...) are masked. The cache test (cache.rs:162-175) passes only because its value is an `sk-` shape.
Impact: An operator who stores an LLM answer or a task record whose JSON holds `{"password":"hunter2"}` or `{"api_key":"<non-prefixed 40 hex>"}` gets the plain value written to the cache or memory JSONL, against the promise in system_prompt.rs:221-226 ("Masking happens before storage"). Precondition: the secret has no known prefix.
Trigger: LlmCache::set (cache.rs:87) or TaskMemory::append (memory.rs:116) with an object value whose key names a secret and whose value string has no known shape. Expected wrong result: value stored verbatim on disk. Note memory.rs:127-135 serialises the struct, so a field called `output` holding `password=x` is masked by the line rule, but a nested JSON value in cache is not.
Known: none

### P34-F02
File: crates/ai-inference/crates/ai-knowledge/src/redact.rs:246-276 (tokeniser at 266-271) with predicates 27-118
Severity: Medium
Axis: security
Confidence: Certain
Evidence:
```
266     for ch in line.chars() {
267         if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' || ch == '.' {
268             current.push(ch);
...
 27 fn is_generic_sk_key(v: &str) -> bool {
 28     v.len() >= 18
 29         && v.starts_with("sk-")
 30         && v.bytes()
 31             .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
```
Description: A word is cut only at characters other than alphanumerics, `_`, `-` and `.`. A secret followed by a full stop (end of a sentence, or `key.txt`) becomes one word that ends in `.`. Every predicate except the JWT ones rejects `.` (sk-, AKIA with exact length 20, ghp_, Stripe, Google with exact length 39, scoped keys), so the word is not masked.
Impact: "The key is sk-abcdefghijklmnopqrstuvwxyz123." in a document, chunk, cache value or memory record keeps the key. The chunk path (chunk.rs:70) and the prompt rule "a credential found inside a document is unreadable" fail on the most common prose shape.
Trigger: any text line with a known-shape secret directly followed by `.`. Expected wrong result: report().total() == 0 and the key survives.
Known: none

### P34-F03
File: crates/ai-inference/crates/ai-knowledge/src/redact.rs:195-243
Severity: Medium
Axis: security
Confidence: Certain
Evidence:
```
199     for (i, c) in line.char_indices() {
200         if c == '"' || c == '\'' {
201             in_quote = !in_quote;
...
204         if !in_quote && (c == ':' || c == '=') {
205             sep = Some(i);
206             break;
...
215     if !SECRET_KEYWORDS.iter().any(|kw| key_norm.contains(kw)) {
216         return line.to_string();
```
Description: Only the first separator on a line is considered, and a single apostrophe flips the quote state. (a) Compact JSON or JSONL such as `{"user":"bob","password":"hunter2"}` stops at the first `:` (key `{"user"`), has no keyword, and returns the line unchanged. (b) A line like `don't set password: hunter2` toggles in_quote at the apostrophe, never finds a separator, and returns unchanged. (c) YAML `private_key: |` masks only the `|`; the following block lines are plain text.
Impact: Secrets in one-line JSON, in prose containing an apostrophe, or in multi-line values pass into chunks (chunk.rs:70) and records. The same function feeds all three sinks (chunk, cache, memory).
Trigger: input lines of the shapes above with a non-prefixed secret value. Expected wrong result: no masking.
Known: none

### P34-F04
File: crates/ai-inference/crates/ai-knowledge/src/redact.rs:12-25 and 120-136
Severity: Medium
Axis: security
Confidence: Certain
Evidence:
```
12 const SECRET_KEYWORDS: &[&str] = &[
13     "api_key", "apikey", "access_key", "secret", "token", "password", "passwd", "pwd",
   "private_key", "client_secret", "auth", "credential",
125 const VALUE_PATTERNS: ... scoped_ant_key, scoped_proj_key, stripe_key, google_api_key,
    generic_sk_key, aws_access_key, github_token, slack_token, jwt, jwt_like_token
```
Description: There is no rule for PEM blocks (`-----BEGIN ... PRIVATE KEY-----` body lines are base64 and carry no key name), for 64-hex Ed25519/BLS seeds (the repo's own secret shape), for mnemonics, or for the key names `seed`, `mnemonic`, `privkey`, `wallet_key`. Only a line whose key contains one of the listed words is masked.
Impact: Budlum wallet material, the main secret type in this repo, is not masked when it appears as a bare value or a PEM body. The system prompt (system_prompt.rs:223) names "private key" explicitly.
Trigger: a document chunk containing `seed: <64 hex>` hits no keyword; a PEM body line hits none. Expected wrong result: stored unmasked.
Known: none

### P34-F05
File: crates/ai-inference/crates/ai-knowledge/ (whole crate), workspace members at crates/ai-inference/Cargo.toml:7
Severity: Medium
Axis: integration
Confidence: Certain
Evidence: `rg -n 'ai-knowledge|ai_knowledge' --glob '*.toml' --glob '*.rs' --glob '*.yml' .` outside the crate returns only the workspace member line (Cargo.toml:7) and three source-text strings inside xtask/gates/src/gates/ai_inference_prompt_is_true.rs (lines 130, 395, 400, 405). No Cargo.toml lists it as a dependency; ai-ops/Cargo.toml and ai-tune/Cargo.toml do not.
Description: redact, chunk, embed, context, memory and cache have no production caller. The gate reads their source text; it does not prove they run. The prompt (system_prompt.rs:221-226) and chunk.rs:63-69 state that masking happens before storage, but nothing in the shipped binaries (ai-ops, ai-serve) calls it.
Impact: The documented secret-masking guarantee is not enforced anywhere at runtime. Per CLAUDE.md section 1 a part with no production call site is a finding; F01 to F04 become live defects the moment it is wired.
Known: Known: not checked against the dead-pub-api baseline

### P34-F06
File: crates/ai-inference/crates/ai-data/src/verify.rs:51-59 (and 20-40)
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence:
```
51 fn hex_bytes(s: &str) -> Option<Vec<u8>> {
52     if s.len() % 2 != 0 { return None; }
55     (0..s.len())
56         .step_by(2)
57         .map(|i| u8::from_str_radix(&s[i..i + 2], 16).ok())
```
Description: `s.len()` is bytes, and `&s[i..i+2]` slices a `&str`. For an even-length string with a multi-byte character straddling a pair boundary, for example `"aéb"` (4 bytes), `s[2..4]` starts inside `é` and panics. Separately `u8::from_str_radix("+a", 16)` returns Ok(10), so `+a` is accepted as the byte 0x0a and a 64-char string with `+` signs decodes to a valid 32-byte hash (non-canonical aliases of one digest). The error text (line 22, 26) echoes the full untrusted string with no length cap.
Impact: A caller feeding an untrusted expected-hash string aborts the thread (release builds of the wider binary abort on panic, per the Cargo.toml comment). No production caller exists today (F10), so the live impact is nil; it becomes a remote crash when wired. No test covers non-ASCII.
Trigger: verify_sha256(data, "aéb") or any even-byte-length string with a split multi-byte char. Expected wrong result: panic instead of HashMismatch.
Known: none

### P34-F07
File: crates/ai-inference/crates/ai-core/src/manifest.rs:49-60, ai-core/src/model.rs:89-100, ai-tune/src/lock.rs:32-52
Severity: Medium
Axis: security
Confidence: Certain
Evidence:
```
manifest.rs
 51 pub fn digest(&self) -> Hash32 {
 52     let mut buf = Vec::new();
 53     buf.extend_from_slice(&self.base_model.0);
 54     buf.extend_from_slice(&self.rank.to_le_bytes());
 55     buf.extend_from_slice(&self.alpha.to_le_bytes());
 56     for r in &self.dataset_refs { buf.extend_from_slice(r); }
 59     placeholder_digest(&buf)
model.rs
 95     for (i, b) in bytes.iter().enumerate() {
 96         out[i % 32] = out[i % 32].wrapping_add(*b);
 97         out[(i * 7 + 3) % 32] ^= b.rotate_left((i % 8) as u32);
```
Description: Two defects in the one digest the output lock relies on. (1) The digest leaves out `adapter_sha256`, `dtype`, `framework` and `trained_at`, so lock::verify_lock accepts a manifest whose adapter hash or dtype was swapped; the module doc (lock.rs:1-3) says the lock binds the output. (2) placeholder_digest is not a hash: zero bytes contribute nothing, so a manifest with `dataset_refs = [[0;32]]` has the same digest as one with `dataset_refs = []`, and the mix is linear/XOR so collisions are constructible. model.rs:89-92 says it has no security purpose, but lock.rs and manifest.rs:49 use it for the lock.
Impact: Anyone who can edit a manifest keeps a valid lock while changing which adapter bytes it points to. Today there is no production caller of lock_output/verify_lock/LoRaManifest outside tests (rg over src, xtask, crates returns none), so the on-chain `register_ai_model` match described in model.rs:3-6 is not enforced.
Trigger: build LoRaManifest, lock_output it, then change adapter_sha256 and call verify_lock: returns Ok. Expected wrong result: lock accepted.
Known: Known: not checked against baseline (dead code part)

### P34-F08
File: crates/ai-inference/crates/ai-tune/src/eval.rs:210-236 and 257-285
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence:
```
228 pub fn canonical(&self) -> String {
229     format!("{}\n{}\n{}\n", self.name, self.prompt, self.check.canonical())
...
216 Check::Exact(s) => format!("exact:{}", s),
217 Check::ExactTrimmed(s) => format!("exact_trimmed:{}", s),
218 Check::ContainsAll(xs) => format!("contains_all:{}", xs.join("|")),
263-267 from_golden: name String::new(), prompt: r.user, check: ExactTrimmed(r.assistant)   // system dropped
```
Description: The digest input has no length prefixes or escaping. Two cases (p, a) and (q, b) give "\np\nexact_trimmed:a\n\nq\nexact_trimmed:b\n", which equals the single case with prompt p and answer "a\n\nq\nexact_trimmed:b". The `|` join in ContainsAll/Excludes is also ambiguous. In addition from_golden ignores `InstructionRecord.system`, so two training sets that differ only in the system turn have the same digest.
Impact: The digest is documented (eval.rs:238-243, 275-278) as pinning "this exact behaviour set" and the train/eval identity. Records that differ in system text, or sets with embedded newlines, are not distinguished. Local operator tooling only.
Trigger: eval.rs digest() on the two sets above. Expected wrong result: equal digests for different sets. Derived by hand, not executed.
Known: none

### P34-F09
File: crates/ai-inference/crates/ai-ops/src/main.rs:133-145 and 164-170; eval.rs:297-304
Severity: Medium
Axis: integration
Confidence: Likely (missing step: confirm that the intended registered id is the B.U.D. content id; only the formula difference is verified)
Evidence:
```
main.rs
133 let set = EvalDataSet::from_golden(dp.clone(), &records);
134 plan.dataset_hashes.push(set.digest());
137 if !set.registered_in(&plan) {
168 "content digest (B.U.D. hash): {}",
src/storage/content_id.rs:45  Self(hash_fields_bytes(&[b"BDLM_CONTENT_V1", chunk]))
ai-data/src/verify.rs:44 content_id_of = plain Sha256::digest(data), no domain tag
```
Description: (1) `registered_in` is called immediately after pushing the same digest into the same plan, so it cannot fail; it is the train/eval binding in name only, nothing else supplies the plan's hashes. (2) The output is labelled "B.U.D. hash", but it is a plain SHA-256 of a custom canonical string; the chain's content id is domain-separated `BDLM_CONTENT_V1` over the chunk bytes. The two never match, and ai-data::content_id_of (typed `Hash32 content_id`) likewise differs from the on-chain id.
Impact: An operator who copies the printed digest to register a dataset or to bind a plan would register an id the chain cannot resolve; the identity check gives false assurance.
Known: none

### P34-F10
File: crates/ai-inference/crates/ai-data/src/source.rs:30-66; ai-core/src/dataset.rs:69-77; ai-data/src/verify.rs:20,44
Severity: Low
Axis: integration
Confidence: Certain
Evidence: `rg -n 'require_source|reject_unknown_source|assert_closed_loop|verify_sha256' crates/ai-inference src xtask -g '*.rs'` outside tests returns only definitions and the xtask gate's text matching (ai_inference_prompt_is_true.rs:139, 923). `assert_closed_loop` (source.rs:30-34) is an exhaustive `match` returning `Ok(())` for every variant, so it cannot fail. ai-ops main.rs uses only DatasetMetadata::validate, jsonl::decode and verify::content_id_of (via eval.rs).
Description: The "closed-circuit" source gate, hash verification and the `SourceRef` types have no production call site; the one function named as a gate is a tautology.
Impact: The closed-loop rule in the system prompt (system_prompt.rs:175-180) has no code behind it in this part. Maintenance and false-assurance risk.
Known: Known: not checked against the dead-pub-api baseline

### P34-F11
File: crates/ai-inference/crates/ai-knowledge/src/memory.rs:136-142
Severity: Medium
Axis: correctness
Confidence: Certain
Evidence:
```
136 let mut text = std::fs::read_to_string(&self.path).unwrap_or_default();
...
142 std::fs::write(&self.path, text).map_err(...)?;
143 self.push_record(rec);
```
Description: append re-reads the whole file and rewrites it. If the read fails (non-UTF-8 byte, permission blip, file replaced), `unwrap_or_default()` yields an empty string and `fs::write` truncates the file to the single new line. The write is not atomic and O(file size) per record, and two processes interleave and lose records. open() (line 94) silently skips undecodable lines but they are kept by the rewrite only while the read succeeds.
Impact: Loss of the whole task-memory history on a transient read error; local file only.
Trigger: file containing one invalid UTF-8 byte after open() succeeded earlier (for example edited by hand or by a second process), then append. Expected wrong result: file reduced to one line.
Known: none

### P34-F12
File: crates/ai-inference/crates/ai-knowledge/src/cache.rs:36-120 and redact.rs:280-293
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
cache.rs 91  let redacted = redact_model_strings(&parsed);
cache.rs 119 std::fs::write(&self.path, text)
cache.rs 75  format!("{SCHEMA_VERSION}|{provider}|{model}|{task}|{h}")
redact.rs 282-290  text.lines().map(...).collect(); lines.join("\n")
```
Description: (a) set() runs every string through redact_text, which rebuilds text with `lines().join("\n")`, so every cached string loses a trailing newline and CRLF becomes LF even when nothing was masked (memory.rs:130 avoids this by comparing first; cache does not). (b) The key joins provider, model and task with `|` without escaping, so ("a|b","c") and ("a","b|c") collide. (c) open() reads the file without a size cap, and save() is a non-atomic truncate-and-write, so a crash mid-write empties the cache.
Impact: Cached LLM output differs from what the model returned; key collisions return another request's answer; cache loss on crash. Local only.
Known: none

### P34-F13
File: crates/ai-inference/crates/ai-knowledge/src/embed.rs:64-71 vs 16-22; memory.rs:214
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
 20 // ... written without diacritics because the tokeniser folds to ASCII before lookup
 65     let lower = text.to_lowercase();
 67         .split(|c: char| !c.is_ascii_alphanumeric())
memory.rs 214 scored.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.created_at.cmp(&b.1.created_at)));
```
Description: The tokeniser does not fold to ASCII; it splits at every non-ASCII letter. "değil" becomes "de" and "il", "çok" becomes "ok", "için" becomes "in" (dropped) and so on, so the Turkish stopwords (`icin`, `degil`, `cok`) never match accented input and diacritic words are cut into fragments. The comment states the opposite. Separately, equal-score memory runs are ordered by oldest created_at first, while the doc (memory.rs:4-5) speaks of recency.
Impact: Weaker Turkish retrieval and noise tokens; oldest-first tie-break. No safety impact.
Known: none

### P34-F14
File: crates/ai-inference/crates/ai-ops/src/logparse.rs:19 and 177-206
Severity: Low
Axis: dead code
Confidence: Certain
Evidence:
```
 4 //! WIRING: unwired. `main.rs` declares this module with `mod logparse;` but no command calls it
19 #![allow(dead_code)]
193     Some((app, msg)) if !app.contains(' ') && !app.is_empty() => {
```
Description: Whole module (254 lines) is unreachable and hidden by an inner `allow(dead_code)` (Z3: suppression added to keep -D warnings green; the module doc admits this). Its own test (line 242-248) never asserts appname, because for the sample line `Oct 11 22:14:15 mymachine su[123]: ...` the text before the first `:` contains spaces, so appname is always None for real syslog headers. Also the PRI is accepted up to 255 (facility 31), values above 191 are invalid.
Impact: Maintenance, suppression budget. The allow is an inner attribute on a binary-crate module.
Known: Known: not checked against .github suppression budget

### P34-F15
File: crates/ai-inference/crates/ai-ops/src/main.rs:26-45, 110-146, 208-213, 228
Severity: Low
Axis: correctness
Confidence: Certain
Evidence:
```
 36 match ai_data::jsonl::decode(line) {      // load_records: no size ceiling, no empty-field check
133 let set = EvalDataSet::from_golden(dp.clone(), &records);   // tune path never calls validate_records
212 answers.insert(r.user, r.assistant);       // duplicate prompt: last wins silently
228 run_eval(&set.cases, |p| answers.get(p).cloned().unwrap_or_default())
```
Description: `tune` and `eval` load datasets with load_records, which skips ai_tune::schema::validate_records (64 KiB line ceiling, empty user/assistant). Only `validate` runs the gate. A dataset with empty assistants is digested and "attached" by `tune`. Duplicate prompts in the responses file overwrite each other, and the golden set with the same prompt twice and different answers can never fully pass. A blank trailing line makes load_records exit 1. With an empty dataset and `min_score 0`, the gate passes with 0 cases, while line 232 prints "gate: all 0 cases passed" before the non-zero exit when the default gate is used. `register` accepts a model id argument and prints it without parsing or length check (main.rs:67-72, spec uses ModelId([0;32])).
Impact: The pre-run quality gate can be bypassed by using the other subcommands; confusing operator output. Local CLI.
Known: none

### P34-F16
File: crates/ai-inference/crates/ai-knowledge/src/redact.rs:12-25, 106-118
Severity: Low
Axis: correctness
Confidence: Certain
Evidence: keyword list contains `"auth"` and `"token"`; `is_bearer_jwt_like` accepts any 3 dot-separated alphanumeric segments of total length 40 or more.
Description: Over-masking. Keys `author`, `authority`, `max_tokens`, `tokenizer`, `secretary` match by substring, so `author: Bob` or `max_tokens = 100` become `<SECRET:MASKED>`. A filename or dotted identifier such as `my_long_document_name_final_v2_for_review.tar.gz` (3 segments, over 40 chars) is masked as a token. chunk.rs:71-75 then stores the rewritten text and hash, so ordinary code chunks change.
Impact: Lower retrieval quality and silently altered chunks; not a leak.
Known: none

### P34-F17
File: crates/ai-inference/crates/ai-ops/src/main.rs:86-92
Severity: Info
Axis: invariant(Z8)
Confidence: Certain
Evidence:
```
87  Some(a) if a >= 1_000 => { println!("draft bond: {a} (above MIN_OPERATOR_BOND=1_000)")
```
src/ai_inference/mod.rs:52: `pub const MIN_OPERATOR_BOND: u64 = 1_000;`
Description: The bond floor is a second hand-copied constant (K3 says rules are queried, not copied); the value matches today but no gate compares them. Z8 note: f64 in this part (embed.rs, context.rs:185, schema.rs:273, eval.rs:170, main.rs:189) is not reachable from consensus because no crate outside crates/ai-inference depends on these crates. NaN passed as `min_score` parses as f64 (main.rs:189) and clamp keeps NaN (eval.rs:326), which makes every gate fail closed; harmless but untested.
Impact: Drift risk only.
Known: none

### P34-F18
File: crates/ai-inference/crates/ai-core/src/system_prompt.rs:317-357 and 206-211 vs tier.rs:9
Severity: Low
Axis: test
Confidence: Certain
Evidence:
```
347 if !REFUSAL_NEGATIONS.iter().any(|n| sentence.contains(n)) {
 31 "do not", "does not", "cannot", "never", "no path", "not a transaction class"
system_prompt.rs 206: "from 0.5x for a shallow preview through 1.0x baseline up to 10.0x"
tier.rs 9: "Multiplier tier labels (such as "0.5x" or "10x") do **not** exist"
```
Description: The negation check is a substring test on the sentence, so "Do not hesitate to generate images on request" passes as a refusal, and a marker spelled differently ("generate pictures") is not looked for. The prompt also introduces "0.5x to 10.0x" effort labels while tier.rs states that such labels do not exist in this layer; the test at tier.rs:57-64 checks only the tier names. The numeric limits are tested against DECLARED_LIMITS (system_prompt.rs:367-376), which is good, but DECLARED_LIMITS is `pub` with a `u32` type and no production reader inside the part (the gate reads source text, per the comment at 254-262).
Impact: Defence-in-depth check is weaker than described; documentation contradiction.
Known: none

## Counts

Critical 0, High 0. Medium 10 (F01 F02 F03 F04 F05 F06 F07 F08 F09 F11). Low 7 (F10 F12 F13 F14 F15 F16 F18). Info 1 (F17). Total 18.
