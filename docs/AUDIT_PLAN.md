# Full code audit plan

Base commit for the plan: 25ede0b. Plan written by the architect agent (Opus) on 2026-10-09 and copied here so that a new session can run it without the old session. This file holds the method only. It holds no finding. Findings are never written to the repository (MODEL_ROUTING 12.10).

## 1. Scope and numbers

- 747 tracked .rs files, 366754 lines (measured with `git ls-files '*.rs'` and `wc -l`).
- 62 parts, 21 waves. At most 3 agents run at the same time (STATUS.md working rule).
- A part has at most about 8000 lines, so one Sonnet agent can read it in full.
- Non-Rust surfaces have their own small parts: CI workflows, supply chain, contract, protocol, config, ops.
- Not read: target/, Cargo.lock, fuzz/corpus.

## 2. Roles

- One Opus agent (the architect agent) plans, writes handoffs and verifies. It cannot start agents and cannot write files.
- The main session starts the Sonnet agents and writes the files.
- Sonnet agents (coder-deep for R3 parts, coder for the rest) only read and report. They never edit a file, never commit, never run cargo.
- No finding enters the report before the architect verifies it with evidence. The verification call is always a new call (the agent that found a bug does not approve it).
- A scout (Haiku) result is a lead, never evidence.

## 3. Part list

"ls[a-b]" means: `git ls-files '<pattern>' | sed -n 'a,bp'`. "dir/*" means every tracked .rs file in that directory. Split points in large files are item boundaries.

R3 parts

| ID | Name | Files | Lines |
|---|---|---|---|
| P01 | Consensus | src/consensus/* | 5343 |
| P02 | Crypto | src/crypto/*, crates/bpqs/src/* | 5785 |
| P03 | Tokenomics, privacy, settlement | src/tokenomics/*, src/privacy/*, src/settlement/* | 6678 |
| P04 | Registry | src/registry/* | 8221 |
| P05 | Cross-domain core, EVM parsing | src/cross_domain/{bridge,bridge_relayer,chain_adapter,event_tree,message,message_registry,mod,nonce,relayer}.rs, src/cross_domain/evm/{mod,rlp,mpt,receipt,header,bud_to_eth}.rs | 6078 |
| P06 | EVM verification, external proof | src/cross_domain/evm/{adapter,sync_committee,verify}.rs, src/cross_domain/external/{domain_bridge,ethereum,evm_hybrid,prover,zkvm_proof,profile}.rs | 5835 |
| P07 | External intake and quorum | src/cross_domain/external/{mod,intake,intake_quorum,quorum,registry,selftest,spec,versioning}.rs | 5550 |
| P08 | Chain A | src/chain/blockchain.rs lines 1-3439, src/chain/{fee_market,mod,genesis}.rs | 5312 |
| P09 | Chain B | src/chain/blockchain.rs lines 3440-8393 (tests start at 6837), src/chain/storage_economics_tests.rs | 5601 |
| P10 | Chain actor, finality | src/chain/chain_actor.rs, src/chain/finality.rs | 6596 |
| P11 | Snapshot, network node | src/chain/snapshot.rs, src/network/node.rs | 6030 |
| P12 | ZK AIR, trace generation | budzero/bud-proof/src/plonky3_prover.rs lines 1-2019, budzero/bud-proof/src/plonky3_air.rs | 5285 |
| P13 | ZK tests A | plonky3_prover.rs lines 2020-6371, budzero/bud-proof/src/trace_layout_tests.rs, budzero/bud-proof/tests/* | 5633 |
| P14 | ZK tests B | plonky3_prover.rs lines 6372-11079, budzero/bud-proof/benches/* | 5047 |
| P15 | STARK core, adapter | budzero/bud-proof/src/bud_stark/*, budzero/bud-proof/src/{adapter,alarm_log,canonical_boot,canonical_recovery,canonical_set,lib,quarantine,relayer,transfer_verdict}.rs | 5317 |
| P60 | Contract, protocol (not Rust) | contracts/external-domain/BudlumFinalityVerifier.sol, proto/budlum/network/protocol.proto, buf.yaml | about 1000 |
| P62 | Repo-wide invariant scan (rg only) | whole tree, see section 5 | n/a |

R2 parts

| ID | Name | Files | Lines |
|---|---|---|---|
| P16 | ZK surroundings (proposed R3) | budzero/bud-vm/*, bud-isa/*, bud-state/*, verifier-registry/* | 6509 |
| P17 | Wallet, account abstraction (proposed R3 for wallet-core) | crates/wallet-core/*, crates/note-packing/*, src/account_abstraction/* | 6034 |
| P18 | Execution, mempool, prover, light client | src/execution/*, src/mempool/*, src/prover/*, src/light_client/* | 6908 |
| P19 | Core account and transaction | src/core/account.rs, src/core/transaction.rs | 6422 |
| P20 | Core rest | src/core/{address,block,bounded_read,chain_config,constitution,encoding,governance,hash,map_keys,metrics,mod,money}.rs, src/sharding/mod.rs, src/error.rs, src/lib.rs, src/sdk/mod.rs | 5551 |
| P21 | RPC server | src/rpc/server.rs, src/rpc/mod.rs | 7403 |
| P22 | RPC API, entry points | src/rpc/api.rs, src/rpc/tests.rs, src/main.rs, src/cli/* | 5911 |
| P23 | Storage deal A | src/domain/storage_deal.rs lines 1-4077, src/domain/{storage_params,storage_tx,deal_open}.rs | 5947 |
| P24 | Storage deal B | storage_deal.rs lines 4078-8122, src/domain/{regen_health,regeneration_stage}.rs | 5970 |
| P25 | Domain rest | src/domain/{finality_adapter,fork_choice,commitment_registry,mod,plugin,plugin_registry,registry,sovereign,types}.rs | 4691 |
| P26 | Storage core | src/storage/{db,merkle_trie,pruning,mod,traits,content_id,lifecycle,manifest,fixed_point,assignment,provider}.rs | 7083 |
| P27 | Storage coding | src/storage/{erasure,lrc,msr,living_threshold,one_share,one_view,payload_crypt,pact_binding,dictionary,mobile_self,server_admission,social_delete}.rs | 6400 |
| P28 | Storage derived | src/storage/{derived,emit,generated,render,transformed,view_grant}.rs | 7381 |
| P29 | Storage QR | src/storage/{qr_carousel,qr_codec,qr_encode,qr_frame,qr_matrix,qr_payload,qr_png,qr_receive,qr_recipe,qr_reemit,qr_verify,qr_verify_indep,qr_video}.rs | 7562 |
| P30 | Storage three and reveal | src/storage/{three_gate,three_hooks,three_meter,three_nft,three_pipe,three_recipe,three_regime,three_reveal,three_rpc,three_visibility,reveal_gateway}.rs | 4318 |
| P31 | AI core | src/ai/mod.rs, src/ai/types.rs | 5962 |
| P32 | AI registry, execution, inference | src/ai/registry.rs, src/ai/execution/*, src/ai_inference/* | 8029 |
| P33 | ai-serve | crates/ai-inference/crates/ai-serve/src/*, crates/ai-inference/crates/ai-integrations/src/* | 5193 |
| P34 | ai helper crates | crates/ai-inference/crates/{ai-core,ai-data,ai-knowledge,ai-ops,ai-tune}/src/* | 4772 |
| P35 | BudL compiler | budzero/bud-compiler/* | 7085 |
| P36 | Network rest | src/network/{gossip_dedup,mobile,mod,peer_manager,privacy,proto_conversions,protocol,sync_codec}.rs | 6449 |
| P37 | B.U.D. 1 | bud/src/*.rs ls[1-20] | 7860 |
| P38 | B.U.D. 2 | bud/src/*.rs ls[21-46] | 7869 |
| P39 | B.U.D. 3 | bud/src/*.rs ls[47-76] | 7764 |
| P40 | B.U.D. 4 | bud/src/*.rs ls[77-106] | 6536 |
| P59 | Supply chain (not Rust) | every Cargo.toml, .quality/{deny.toml,osv-scanner.toml,grype.yaml}, supply-chain/*, .github/git-dep-audit.toml, rust-toolchain.toml, clippy.toml, flake.nix | small |

R1 parts

| ID | Name | Files | Lines |
|---|---|---|---|
| P41 | Tools and binaries | budzero/bud-node/*, budzero/bud-cli/*, src/bin/*, examples/print_genesis_hash.rs, build.rs | 4998 |
| P42 | R1 apps | src/relayer/*, src/gateway/*, src/bns/*, src/socialfi/*, src/budlumxyz/*, src/deed/mod.rs, src/developer_os.rs | 7631 |
| P43 | Pollen, budscan | src/pollen/*, crates/budscan/* | 8208 |
| P44 | Test harness | benches/*, fuzz/fuzz_targets/*, kani/src/lib.rs, crates/bpqs/examples/*, bud/tests/*, bud/fuzz/*, bud/kani/* | 6292 |
| P45 | src/tests 1 | src/tests/*.rs ls[1-17] | 7299 |
| P46 | src/tests 2 | ls[18-40] | 5809 |
| P47 | src/tests 3 | ls[41-52] | 7181 |
| P48 | src/tests 4 | ls[53-70] | 6685 |
| P49 | src/tests 5 | ls[71-86] | 7358 |
| P50 | xtask gates 1 | xtask/gates/src/gates/*.rs ls[1-21] | 7396 |
| P51 | xtask gates 2 | ls[22-44] | 7537 |
| P52 | xtask gates 3 | ls[45-69] | 7577 |
| P53 | xtask gates 4 | ls[70-87] | 6701 |
| P54 | xtask gates 5 | ls[88-99] | 7269 |
| P55 | xtask gates 6 | ls[100-119] | 7368 |
| P56 | xtask gates last | ls[120-130], xtask/gates/src/main.rs | 5652 |
| P57 | xtask tools | xtask/tools/* | 5873 |
| P58 | CI (not Rust) | .github/workflows/*, .github/codeql/codeql-config.yml, .github/dependabot.yml, .github/CODEOWNERS, .github/*-baseline.txt (read only, with git history) | small |
| P61 | Config, ops | config/*, ops/*, budzero/*.bud, .claude/agents/* (model and effort against MODEL_ROUTING section 1) | small |

Totals: R3 88311 lines in 120 files, R2 161609 lines in 309 files, R1 116834 lines in 318 files. Together 747 files, 366754 lines. No file is missing. Re-measure at the start of a new audit, because HEAD moves.

## 4. Waves

D1 P62 P01 P02 | D2 P03 P04 P05 | D3 P06 P07 P60 | D4 P08 P09 P10 | D5 P11 P12 P13 | D6 P14 P15 P58 | D7 P59 P16 P17 | D8 P18 P19 P20 | D9 P21 P22 P23 | D10 P24 P25 P26 | D11 P27 P28 P29 | D12 P30 P31 P32 | D13 P33 P34 P35 | D14 P36 P37 P38 | D15 P39 P40 P61 | D16 P41 P42 P43 | D17 P44 P45 P46 | D18 P47 P48 P49 | D19 P50 P51 P52 | D20 P53 P54 P55 | D21 P56 P57

Waves are a guide. A free slot may take the next part (rolling start). R3 verification runs as soon as a part ends. R2 and R1 verification may be batched, two waves at a time.

## 5. Audit criteria

Five axes for every part:

1. Correctness: overflow and truncation (`as`, unsigned subtraction), boundary cases, ordering determinism (HashMap iteration), half-finished state on an error path, panic paths on external input (unwrap, indexing).
2. Security: input bounds (size, depth), skipped signature or proof checks, replay, authority checks, DoS (unbounded loop or allocation).
3. Integration (CLAUDE.md section 1): for each public entry, find a production call site with `rg -n '<symbol>' src/ budzero/ crates/`. A call only from a test is a finding "not wired". A feature behind a flag or marked experimental is written as a mainnet blocker.
4. Test coverage: is there a negative test, does the test call the production path, is an assert empty, is there an `#[ignore]`.
5. Dead code: a public item with no caller. Items already in `.github/dead-pub-api-baseline.txt` or `.github/idle-code-baseline.txt` are marked "Known". Baseline files are never edited.

Extra checks by part type:

- Consensus and chain (P01, P03, P04, P08 to P11, P25): fork choice and finality irreversibility, double vote and slashing proof, epoch transition, replay path and live path give the same root, no float (Z8), total supply 100M and the double burn invariant, PoA flow stays apart from the permissionless flow (Z11).
- Crypto (P02, P17): domain separation, constant-time compare, nonce and randomness source, key zeroization, PKCS11 error paths, WOTS one-time use.
- Cross-domain (P05 to P07, P60): nonce store, message id binding, MPT and RLP parse bounds, sync committee threshold, the Solidity verifier uses the same encoding as Rust.
- ZK (P12 to P16, P35): an AIR constraint for every opcode, unconstrained columns, LogUp multipliers, every public input enters the Fiat-Shamir transcript, the VerifyMerkle 64 gate stays closed in production, the compiler never emits a negative static slot.
- Storage (P23, P24, P26 to P30, P37 to P40): persistence format versioning, open with corrupt data, erasure threshold, content id binding, B.U.D. parts wired to the node.
- Network and RPC (P11, P21, P22, P36): message size limit, peer scoring, state change on an endpoint without identity, rate limit, secret leak in error text.
- Tools (P41 to P57): a gate can really fail, silent success, wrong file pattern, the gate is wired into a workflow.
- CI and supply chain (P58, P59): Z3 (continue-on-error, `|| true`, loosened baseline, pinned old commit), action pins by SHA, permissions, deny.toml exceptions, git dependencies.

P62 repo-wide scan commands (run once, rg and git only):

- Z3: `rg -n '#\[allow\(|#\[ignore' --type rust`. Tie each hit to the adding commit with `git log -S`. Note: a shallow clone hides history. Also search workflows for `continue-on-error`, `|| true`, `; true`, `|| exit 0`, `set +e`.
- Z6: `git ls-files | rg '\.(sh|bash|py)$'`, and shell or script blocks inside yml.
- Z7: `rg -n '\bunsafe\b' --type rust` (separate comments and strings from real use).
- Z8: `rg -n '\bf(32|64)\b' src/consensus src/chain src/tokenomics src/settlement src/registry src/cross_domain`.
- Z9: key, seed and token patterns (64 hex, "BEGIN PRIVATE", mnemonic). Never write the value, only file:line and type.
- Z12: `rg -n -i '\bL1\b|layer[ -]?1' README* docs budzero/docs crates/*/README* bud/README.md`.
- MODEL_ROUTING 12.9: `rg -n -A15 'fn to_bytes' --type rust | rg unwrap_or_default`. Label "LABEL-12.9". Do not mix into focused steps. Do not propose a fix.

## 6. Finding format (Sonnet output)

```
### Pxx-Fnn
File: path:lines
Severity: Critical | High | Medium | Low | Info
Axis: correctness | security | integration | test | dead code | invariant(Zn) | LABEL-12.9
Confidence: Certain | Likely (if Likely, name the missing verification step)
Evidence: raw quote (at most 15 lines) with file:line; for a call path the rg command and its output
Description: 1 to 3 short sentences
Impact: who, with which precondition, loses what
Trigger (security only): precondition, entry point file:line, input shape, expected wrong result. No working exploit code.
Known: none | STATUS.md section | baseline file
```

Severity:

- Critical: fund mint or loss with no precondition, finality or consensus safety break, acceptance of a forged ZK proof, network-wide halt.
- High: same class but needs a precondition (role, minority stake, configuration), or a remote single-node crash, or lasting data corruption.
- Medium: limited or local impact, wrong error handling, a mainnet part not wired to production, missing negative test on an R3 path.
- Low: defense in depth, maintenance risk, dead code.
- Info: observation, language issue, LABEL-12.9.

Never write "probably". A claim without evidence is not a finding.

## 7. Sonnet agent prompt template

Use coder-deep for R3 parts and coder for the rest. Set the model to sonnet.

```
TASK: Budlum audit, part <Pxx> <name>. Base commit <SHA>. Tier <R>.
FILES: <exact list and line ranges from section 3>
READ ONLY: Do not use Edit, Write or NotebookEdit. Do not run git commit, checkout or stash. Do not run cargo.
Bash only with: rg, sed -n 'a,bp', git log, git diff, git show, git ls-files, wc -l. The Read tool may use offset and limit.
READING: At most 1500 lines per call. Outside the part, use rg first and read at most 80 lines. Do not read target/ or Cargo.lock.
CRITERIA: sections 5 and 6 of docs/AUDIT_PLAN.md, the extra checks for this part type, and the invariants Z3 Z7 Z8 Z9 LABEL-12.9.
KNOWN: Mark items from STATUS.md and the baseline files as "Known".
OUTPUT: Write no file. In the reply give the full text of Pxx.md:
  1) COVERAGE: every file and range read; every range not read and why
  2) FINDINGS in the format of section 6, IDs start at Pxx-F01
  3) If there is no finding, list the axes checked and the rg commands used
No claim without evidence. No fix code.
```

After each wave the main session runs `git status --porcelain`. It must be empty. Compare COVERAGE with the plan range. If more than 20 percent of a part was not read, open a part Pxx-b for the gap.

## 8. Opus verification protocol

1. Every batch is a new architect call. The agent that verified an earlier batch is not reused.
2. A batch has at most 8 findings, at most 4 when they are Critical or High. A batch comes from one part or from neighbouring parts of one module.
3. For each finding: read the cited lines with `sed -n`, build the call path with `rg`, follow the trigger step in the code. Decision: Confirmed, Rejected or Uncertain, with the file:line read.
4. Severity is re-set. The Sonnet severity and the architect severity are written in separate columns, with one sentence for any change.
5. Duplicates: one root cause (same function or same invariant break) is merged under the smallest ID.
6. An R3 finding that stays Uncertain goes to finder-max (MODEL_ROUTING section 5), at most one call per finding, logged in STATUS.md "EFFORT LOG". If it stays Uncertain, ask the owner with the section 8 template. R2 and R1 Uncertain findings stay in the "uncertain" section of the report.
7. For an R3 part with no finding, the architect checks the COVERAGE statement and samples at least 2 high-risk functions.
8. After verification, one handoff per finding (MODEL_ROUTING section 7). Handoffs are a separate job.

## 9. Report layout and where it lives

- One merged report. It is NOT committed. Keep it in the session scratchpad (or another private place) and file Critical and High items through the private channel in docs/SECURITY.md. Reason: MODEL_ROUTING 9 and 12.10.
- Part files: findings/Pxx.md in the scratchpad.
- Report layout: 1 summary table (tier by severity, confirmed only; mainnet blockers) | 2 coverage table (part, planned lines, ranges read, ranges not read, agent, verification call, status) | 3 confirmed findings (by severity, then part; extra field "Verification: call number, lines read") | 4 invariant table (Z3 Z6 Z7 Z8 Z9 Z12 LABEL-12.9) | 5 integration table (parts not wired) | 6 uncertain findings | 7 rejected findings with reason and evidence | 8 merged findings | 9 open decisions (section 8 template).
- The trigger step of a security finding lives only in the private report. Memanto may get only: audit done, SHA, count.

## 10. Stop and deviation rules

Stop and ask with the MODEL_ROUTING section 8 template when:

- `git status --porcelain` is not empty, or HEAD differs from the audit base: an agent may have written a file.
- A confirmed Critical finding exists: execution continues, the finding goes to the owner at once for the private channel decision.
- A Z9 hit may be a real secret: report file:line only, never the value.
- An agent transcript shows a model or effort that differs from the agent file (MODEL_ROUTING 2.9).
- The same part gave invalid output twice: apply the section 5 ladder (architect high).

## 11. Tier proposals that differ from MODEL_ROUTING section 3

The plan proposes these tiers. They are labels only; the owner may refuse and only the label changes.

- crates/bpqs and crates/wallet-core: R3 (crypto, key handling).
- budzero/bud-vm, bud-isa, bud-state, verifier-registry: R3 (ZK soundness).
- src/network files other than node.rs: R2. bud/src (B.U.D.): R2. budzero/bud-compiler: R2. src/mempool and src/light_client: R2.

## 12. Progress at the time of writing (2026-10-09)

- Done by the Sonnet agents in the old session: P62, P01, P02, P03. P04 and P05 were running and are NOT counted as done.
- The Opus verification had started for P01 only. Nothing else was verified.
- The findings and the verification notes were kept in the old session scratchpad. They are not in this repository on purpose. A new session must treat every part as NOT done unless a private report exists. If the owner kept the private report, start from the first part with no verified result.
- Ask the owner first: file the earlier results through the private channel in docs/SECURITY.md before any rescan.
- Z12 note for the owner: some public files still use the banned term (README and docs text). The P62 scan lists them. Fixing LICENSE or NOTICE text needs an owner decision.
- Tooling note: the `memanto` command was not installed in that container, so no recall was done.
