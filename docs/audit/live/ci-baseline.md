# WP-04 CI baseline

## Capture

- Captured 2026-11-06 UTC with `gh api repos/lubothebook/budlum/actions/workflows/<file>/runs?branch=main&per_page=1` for every workflow file present on `cccf1da1f4bcfe97503a3a00cfba24b172b49b31`.
- This is the latest run on `main` per workflow, not a claim that every run tested the current SHA. The SHA column exposes older evidence.
- `none` means the GitHub API returned no run on `main`. It is not a pass.
- A cancelled or absent run is not green. The seven-night requirement cannot be evaluated from one latest run.

## Latest run per workflow

| Workflow file | Latest main run | Conclusion | Tested SHA | Event | Created UTC |
|---|---|---|---|---|---|
| `.github/workflows/air-mutants-nightly.yml` | [42003158526](https://github.com/lubothebook/budlum/actions/runs/42003158526) | cancelled | `14d9a18130b5` | schedule | 2026-11-05T09:32:06Z |
| `.github/workflows/audit-guard.yml` | [42050658969](https://github.com/lubothebook/budlum/actions/runs/42050658969) | failure | `cccf1da1f4bc` | push | 2026-11-05T15:01:03Z |
| `.github/workflows/benchmark.yml` | [42050658968](https://github.com/lubothebook/budlum/actions/runs/42050658968) | success | `cccf1da1f4bc` | push | 2026-11-05T15:01:03Z |
| `.github/workflows/cargo-vet.yml` | [42050659263](https://github.com/lubothebook/budlum/actions/runs/42050659263) | failure | `cccf1da1f4bc` | push | 2026-11-05T15:01:03Z |
| `.github/workflows/ci.yml` | [42050659038](https://github.com/lubothebook/budlum/actions/runs/42050659038) | failure | `cccf1da1f4bc` | push | 2026-11-05T15:01:03Z |
| `.github/workflows/dependency-review.yml` | none | no run | `none` | - | - |
| `.github/workflows/determinism.yml` | [42050659224](https://github.com/lubothebook/budlum/actions/runs/42050659224) | failure | `cccf1da1f4bc` | push | 2026-11-05T15:01:03Z |
| `.github/workflows/diverse-double-compiling.yml` | [42050659274](https://github.com/lubothebook/budlum/actions/runs/42050659274) | failure | `cccf1da1f4bc` | push | 2026-11-05T15:01:03Z |
| `.github/workflows/docker-smoke.yml` | [42048407414](https://github.com/lubothebook/budlum/actions/runs/42048407414) | failure | `ed43314c63f1` | push | 2026-11-05T14:47:23Z |
| `.github/workflows/extra-tooling.yml` | [42050659051](https://github.com/lubothebook/budlum/actions/runs/42050659051) | failure | `cccf1da1f4bc` | push | 2026-11-05T15:01:03Z |
| `.github/workflows/fmt-diff-surface.yml` | [41902433875](https://github.com/lubothebook/budlum/actions/runs/41902433875) | cancelled | `451ea26e13e4` | pull_request | 2026-11-04T20:00:37Z |
| `.github/workflows/fuzz-nightly.yml` | [42015083239](https://github.com/lubothebook/budlum/actions/runs/42015083239) | success | `14d9a18130b5` | schedule | 2026-11-05T10:58:22Z |
| `.github/workflows/grype.yml` | [42050658971](https://github.com/lubothebook/budlum/actions/runs/42050658971) | success | `cccf1da1f4bc` | push | 2026-11-05T15:01:03Z |
| `.github/workflows/kani-slow.yml` | none | no run | `none` | - | - |
| `.github/workflows/miri-storage-weekly.yml` | none | no run | `none` | - | - |
| `.github/workflows/miri.yml` | [42050659073](https://github.com/lubothebook/budlum/actions/runs/42050659073) | cancelled | `cccf1da1f4bc` | push | 2026-11-05T15:01:03Z |
| `.github/workflows/osv-scanner.yml` | [42050658932](https://github.com/lubothebook/budlum/actions/runs/42050658932) | failure | `cccf1da1f4bc` | push | 2026-11-05T15:01:03Z |
| `.github/workflows/pkcs11-pq.yml` | [42050658983](https://github.com/lubothebook/budlum/actions/runs/42050658983) | success | `cccf1da1f4bc` | push | 2026-11-05T15:01:03Z |
| `.github/workflows/provenance.yml` | [42050659347](https://github.com/lubothebook/budlum/actions/runs/42050659347) | success | `cccf1da1f4bc` | push | 2026-11-05T15:01:03Z |
| `.github/workflows/release-signature.yml` | [42048407458](https://github.com/lubothebook/budlum/actions/runs/42048407458) | success | `ed43314c63f1` | push | 2026-11-05T14:47:23Z |
| `.github/workflows/rust.yml` | none | no run | `none` | - | - |
| `.github/workflows/security-audit.yml` | [42050659081](https://github.com/lubothebook/budlum/actions/runs/42050659081) | failure | `cccf1da1f4bc` | push | 2026-11-05T15:01:03Z |
| `.github/workflows/security-hardening.yml` | none | no run | `none` | - | - |
| `.github/workflows/semgrep.yml` | [42050659229](https://github.com/lubothebook/budlum/actions/runs/42050659229) | failure | `cccf1da1f4bc` | push | 2026-11-05T15:01:03Z |
| `.github/workflows/semver.yml` | [42050659042](https://github.com/lubothebook/budlum/actions/runs/42050659042) | failure | `cccf1da1f4bc` | push | 2026-11-05T15:01:03Z |
| `.github/workflows/solidity.yml` | [42050659174](https://github.com/lubothebook/budlum/actions/runs/42050659174) | success | `cccf1da1f4bc` | push | 2026-11-05T15:01:03Z |
| `.github/workflows/supply-chain-extra.yml` | [42050659086](https://github.com/lubothebook/budlum/actions/runs/42050659086) | success | `cccf1da1f4bc` | push | 2026-11-05T15:01:03Z |
| `.github/workflows/testnet-rehearsal.yml` | [42002851792](https://github.com/lubothebook/budlum/actions/runs/42002851792) | failure | `14d9a18130b5` | schedule | 2026-11-05T09:29:57Z |
| `.github/workflows/typos.yml` | [42050658939](https://github.com/lubothebook/budlum/actions/runs/42050658939) | failure | `cccf1da1f4bc` | push | 2026-11-05T15:01:03Z |
| `.github/workflows/watchdog.yml` | [42163252239](https://github.com/lubothebook/budlum/actions/runs/42163252239) | failure | `cccf1da1f4bc` | schedule | 2026-11-06T04:33:54Z |

Inventory count: 30 workflow files. Conclusions: cancelled=3, failure=14, no run=5, success=8.

## Current-SHA core CI failure surface

Run [42050659038](https://github.com/lubothebook/budlum/actions/runs/42050659038), event `push`, SHA `cccf1da1f4bcfe97503a3a00cfba24b172b49b31`. The table names every failed job and failed step returned by the jobs API. It does not infer a compiler diagnostic from an exit code.

| Failed job | Failed step | Evidence |
|---|---|---|
| Gates (a suite that cannot run is not a protection) | Test log for the badge gate (the verdict lives in budlum) | [job 127488353707](https://github.com/lubothebook/budlum/actions/runs/42050659038/job/127488353707), step 13 |
| Gates (a suite that cannot run is not a protection) | Is the badge current (README = measured test count) | [job 127488353707](https://github.com/lubothebook/budlum/actions/runs/42050659038/job/127488353707), step 15 |
| Gates (a suite that cannot run is not a protection) | Unreachable gate count did not grow | [job 127488353707](https://github.com/lubothebook/budlum/actions/runs/42050659038/job/127488353707), step 27 |
| Gates (a suite that cannot run is not a protection) | Dead public API did not grow | [job 127488353707](https://github.com/lubothebook/budlum/actions/runs/42050659038/job/127488353707), step 29 |
| Gates (a suite that cannot run is not a protection) | A new file adds no panicking index | [job 127488353707](https://github.com/lubothebook/budlum/actions/runs/42050659038/job/127488353707), step 46 |
| Gates (a suite that cannot run is not a protection) | Tally (every red gate, annotated) | [job 127488353707](https://github.com/lubothebook/budlum/actions/runs/42050659038/job/127488353707), step 55 |
| Budlum Core PQ build | Feature matrix: pq-ml-dsa solo | [job 127488354234](https://github.com/lubothebook/budlum/actions/runs/42050659038/job/127488354234), step 6 |
| Budlum Core library test | Test | [job 127488354235](https://github.com/lubothebook/budlum/actions/runs/42050659038/job/127488354235), step 6 |
| Cargo Deny (fuzz) | cargo deny check (advisories, bans, licenses, sources) | [job 127488354282](https://github.com/lubothebook/budlum/actions/runs/42050659038/job/127488354282), step 4 |
| Budlum Core quality | cargo doc (Item 6 - public API documentation) | [job 127488354480](https://github.com/lubothebook/budlum/actions/runs/42050659038/job/127488354480), step 7 |
| Budlum Core quality | Clippy | [job 127488354480](https://github.com/lubothebook/budlum/actions/runs/42050659038/job/127488354480), step 8 |
| Budlum Core quality | Format | [job 127488354480](https://github.com/lubothebook/budlum/actions/runs/42050659038/job/127488354480), step 10 |
| Network Hardening | network hardening tests | [job 127488354561](https://github.com/lubothebook/budlum/actions/runs/42050659038/job/127488354561), step 7 |
| Cargo Deny (root) | cargo deny check (advisories, bans, licenses, sources) | [job 127488354574](https://github.com/lubothebook/budlum/actions/runs/42050659038/job/127488354574), step 4 |
| Coverage (nextest + llvm-cov, ratchet) | Measurement + ratchet gate | [job 127488354624](https://github.com/lubothebook/budlum/actions/runs/42050659038/job/127488354624), step 8 |
| StorageProvider Gate | StorageProvider tests | [job 127488354679](https://github.com/lubothebook/budlum/actions/runs/42050659038/job/127488354679), step 9 |
| Economy Invariants | named invariant tests | [job 127488354733](https://github.com/lubothebook/budlum/actions/runs/42050659038/job/127488354733), step 7 |
| Fuzz Quick (60s x 12 targets) | Her target 60 saniye fuzz | [job 127488354794](https://github.com/lubothebook/budlum/actions/runs/42050659038/job/127488354794), step 8 |
| Repo Lint (actionlint + buf + genesis + zizmor) | typos gate + canary (whole-tree spell check, calibrated) | [job 127488354807](https://github.com/lubothebook/budlum/actions/runs/42050659038/job/127488354807), step 6 |
| Serialize Map Keys Are Strings | Every map key is a string, a number, or carries a serde with | [job 127488354808](https://github.com/lubothebook/budlum/actions/runs/42050659038/job/127488354808), step 4 |
| BudZero / BudZKVM | Test | [job 127488354831](https://github.com/lubothebook/budlum/actions/runs/42050659038/job/127488354831), step 7 |
| Fork-Choice Invariants | fork-choice tests | [job 127488355011](https://github.com/lubothebook/budlum/actions/runs/42050659038/job/127488355011), step 7 |
| Budlum Core | Require every core split | [job 127535129483](https://github.com/lubothebook/budlum/actions/runs/42050659038/job/127535129483), step 2 |

### Retrieved annotations (verbatim substance)

- Gates job 127488353707: `FAIL [no-out-of-block-consensus-writes]: canary: an unmodified copy was refused: chain_actor.rs calls required_fee on self.blockchain.state, which is not a read`.
- Same job: `FAIL [indexing-is-not-new]: 2 file(s) outside the baseline index at runtime`.
- Same job: `FAIL [dead-public-api-is-ratcheted]: dead public api: 182 | baseline: 173`.
- Same job: `FAIL [guards-are-reachable]: unwired guards: 5 | baseline: 3`.
- Same job: `FAIL [badges-are-current]: test log records failures; refusing to compare badge against a red run`.
- Quality job 127488354480: format diffs in `src/tests/bud3_door.rs:242`, `src/storage/bud3/bof1.rs:278`, `crates/bud-codec/src/lib.rs:304`; cargo doc and clippy steps also fail with exit 101.
- Library job 127488354235: `Test` exits 101. The check-run annotation does not contain the compiler/test diagnostic.
- PQ feature-matrix job 127488354234: `Feature matrix: pq-ml-dsa solo` fails. The annotation does not establish which command or source line failed.
- Watchdog run 42163252239 on the current SHA: both `Main health pulse (locks / workflows / regen gate)` and `RustSec drift (fresh advisory DB vs unchanged locks)` fail.

Full runner logs redirect to a host outside the workspace network allowlist. The GitHub jobs/check-run API is reachable and is the evidence used here. An exit code alone is not labeled a build failure, a test assertion or an OOM.

## Runner memory baseline

The workflow sources select GitHub-hosted `ubuntu-latest`, explicit Ubuntu 24.04 x86/ARM, macOS and Windows labels. No job declares a 16 GiB or 32 GiB minimum-memory reservation. The directive requires at least 16 GiB for core/execution and 32 GiB for heavy nightly suites. GitHub label names do not prove the RAM actually allocated to a run.

`semver.yml` prints `free -m` for OOM evidence, but that is a diagnostic snapshot, not a capacity guarantee. No RAM measurement is claimed here. WP-42 owns runner capacity enforcement and the missing workflow structure; WP-04 records the gap without editing workflows locked by other streams.

## Ownership and action

| Failure family | Owning package | Action from this audit |
|---|---|---|
| Consensus-write gate / core build and tests | WP-01 verification; WP-17/owning implementation package | Record red base; do not mask it or raise a baseline. |
| B.U.D. 3.0 format diffs | WP-10 and open storage PR #8 | Leave locked files untouched; recheck after merge. |
| Gate indexing/dead-API/reachable-guard ratchets | WP-42 | Fix the cause or wire/remove idle code; never inflate the baseline. |
| Cargo Deny/RustSec/OSV/vet supply-chain failures | WP-42, with WP-07 removals where applicable | Preserve fail-closed jobs; record per-lock advisory evidence when logs become available. |
| Network hardening | WP-15 | Reproduce on the owning package before changing behavior. |
| Missing/older/cancelled nightly evidence | WP-42 | Establish required workflows and seven green nights; no readiness claim now. |

## User instruction handling

The user asked not to hold commits waiting for CI and to intervene only when a running check is observed red. This audit records the already-red base immediately. Commits proceed without waiting. This PR does not change another package's implementation to chase inherited failures. Its own new red checks will be diagnosed and fixed in this branch when observed.
