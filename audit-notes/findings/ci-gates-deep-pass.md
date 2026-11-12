# Findings: CI workflows, quality gates, second-pass deep audit

## F-0121

- **ID**: F-0121
- **Location**: .github/workflows/conformance-nightly.yml lines 46, 150, 186; ci.yml lines 1709, 1739, 1795, 3450
- **What is wrong**: Seven rust-cache action references use v2.9.2 (`6323deb102c322ba6fcbdcafc7e3dddab59af2b6`) while the remaining 30+ references across all workflows use v2.9.3 (`d1527c8f67ef9af357290475d4af4b0fab8f12e0`). The v2.9.2 pins appear in conformance-nightly (3 places), mobile-core-targets, mobile-ffi, budlum-explorer, and budlum-types jobs.
- **Why**: Version drift from incomplete update. The bulk update caught most references but missed the cross-workspace jobs.
- **What it causes**: The stale jobs run a different cache implementation. A v2.9.3 fix or behavioral change does not apply to them. Supply chain consistency is weakened.
- **How to fix**: Update all seven pins to v2.9.3 (`d1527c8f67ef9af357290475d4af4b0fab8f12e0`).

## F-0122

- **ID**: F-0122
- **Location**: .github/workflows/ci.yml (3498 lines, 40+ jobs)
- **What is wrong**: The main CI workflow is 3498 lines in a single YAML file containing over 40 jobs. This is the largest file in the CI directory by a wide margin.
- **Why**: Incremental addition of jobs to a single file.
- **What it causes**: Reviewing CI changes is difficult. A YAML syntax error anywhere in the file can break all jobs. The file is slow to parse and render in editors. Conflict resolution on parallel branches is painful.
- **How to fix**: Consider splitting into domain-specific workflow files (core, mobile, gates, security, compliance). The `needs:` graph can span files through `workflow_call`.

## F-0123

- **ID**: F-0123
- **Location**: .github/workflows/ci.yml lines 395, 614, 619, 2488
- **What is wrong**: Turkish text appears in CI step names and comments. Line 395: "Clippy pedantic/nursery ratchet (izleme modu)". Line 2488: "Istatistiksel zamanlama regresyon testi (cargo bench)". Lines 614 and 619 use "izleme modu" in inline comments.
- **Why**: Same language issue as F-0089 (Turkish in Dockerfile). The CI was authored by Turkish speakers and some annotations were not translated.
- **What it causes**: Non-Turkish-speaking operators and reviewers cannot understand the step names or comments without translation.
- **How to fix**: Replace Turkish with English equivalents. "izleme modu" means "monitoring mode". "Istatistiksel zamanlama regresyon testi" means "statistical timing regression test".

## F-0124

- **ID**: F-0124
- **Location**: All 36 files in .github/workflows/
- **What is wrong**: Every workflow triggers on the `main` branch. The repository default branch (origin/HEAD) is `mains`, not `main`. Both `main` and `mains` exist as remote branches and are completely divergent. `mains` has web-upload commits ("Add files via upload"). `main` has the development tree ("Consensus Of Consensuses").
- **Why**: The repository was likely forked or the default branch was renamed. The CI was written for `main` which is the actual development branch.
- **What it causes**: Pull requests targeting `mains` as the base branch will not trigger CI. The audit branch PR #1 targets `mains`. Branch protection rules on `mains` may have no required status checks because no workflow reports to that branch.
- **How to fix**: Either change the GitHub default branch to `main`, or update all 36 workflows to also trigger on `mains`. The first option is simpler.

## F-0125

- **ID**: F-0125
- **Location**: xtask/gates/src/gates/concept_names.rs, .github/concept-names-baseline.txt
- **What is wrong**: The concept name gate is in "count" (ratchet) mode, not "absolute" mode. The baseline records the current count for each deprecated name. New uses of deprecated names are blocked, but existing uses (759 combined references) are allowed to persist indefinitely.
- **Why**: The gate was designed as a two-phase tool: "count" mode prevents growth while renames proceed, then "absolute" mode enforces zero after renames complete.
- **What it causes**: TLG-24 requires all deprecated names removed. The gate cannot enforce TLG-24 until switched to "absolute" mode. The 759 references (Pollen 455, Filiz 261, Mikoriza 43) must be renamed first.
- **How to fix**: Complete the concept name rename (IP-05), lower the baseline to zero, then switch the gate to "absolute" mode.

## F-0126

- **ID**: F-0126
- **Location**: xtask/gates/src/gates/no_idle_code.rs (1939 lines)
- **What is wrong**: The gate documents 1962 idle public items out of 4600 total public items (42.6%). These are on a ratchet baseline: new idle items fail CI, but the existing 1962 items are allowed. The gate is the largest single gate file at 1939 lines.
- **Why**: The tree accumulated dead public API over time. The gate prevents growth while the team works down the baseline.
- **What it causes**: 42.6% of the public API surface is unreachable from production code. Each idle item compiles, is reviewed, and appears in docs but does no work. The independent review (TLG-22) must examine these items to distinguish intentional API reservation from dead code.
- **How to fix**: Gradually remove or wire idle items to drive the baseline toward zero. The gate's ratchet mechanism already prevents growth.

## F-0127

- **ID**: F-0127
- **Location**: xtask/gates/src/gates/ (167 files, 64960 lines total)
- **What is wrong**: The quality gate suite is 64,960 lines of Rust across 167 gate files. The gate binary is built and exercised in CI, but the gate code itself is only covered by clippy (enforced since 2026-09-11) and the per-gate `--self-test` canaries. There is no independent test coverage measurement for the gates themselves.
- **Why**: The gates were written as tree-scanning verification tools. They validate the main codebase but are not themselves subject to the same coverage measurement.
- **What it causes**: A gate with a bug in its scanning logic could report false negatives (missing violations) or false positives (blocking valid code). The `--self-test` canaries prove the gate can catch one known violation, but do not prove it catches all classes of violations.
- **How to fix**: Add the gates workspace to the coverage job, or add targeted tests for edge cases in the largest gates (no_idle_code, canonical_programs_reproduce, tree_is_english, capability_modules_are_wired, ast_security_gates).

## F-0128

- **ID**: F-0128
- **Location**: .github/workflows/testnet-rehearsal.yml lines 62-68
- **What is wrong**: The testnet-native-e2e job uses `actions/cache@4057802...` (v5.0.6) with a manual cache key `${{ runner.os }}-cargo-testnet-native-${{ hashFiles('**/Cargo.lock') }}`. All other workflows use `Swatinem/rust-cache` which manages cache keys automatically.
- **Why**: The job was added independently and adopted a different caching pattern.
- **What it causes**: The `**/Cargo.lock` glob matches every lockfile in the tree (root, fuzz, budzero, crates/*). A lockfile change in any workspace invalidates the entire cache even if the testnet-rehearsal dependencies did not change. The cache also stores `target/` without the cleanup logic that `Swatinem/rust-cache` applies, so it can grow unbounded.
- **How to fix**: Switch to `Swatinem/rust-cache@d1527c8f67ef9af357290475d4af4b0fab8f12e0` with `shared-key: testnet-rehearsal` to match the pattern used by other workflows.

## F-0129

- **ID**: F-0129
- **Location**: src/tokenomics/mod.rs line 210, src/chain/genesis.rs line 1121
- **What is wrong**: `mainnet_tokenomics()` sets `block_reward: 50` (line 1121 of genesis.rs). The module-level doc (mod.rs line 16-17) says "Validator income is fee-only: block and epoch emissions are disabled." The comment on the field (mod.rs line 209) says "Legacy wire value; Executor never mints it." The testnet template explicitly sets `block_reward: 0` (genesis.rs line 1219).
- **Why**: The field is retained for snapshot and wire compatibility. Mainnet keeps the legacy value 50 while testnet zeroed it.
- **What it causes**: The mainnet genesis advertises a block reward of 50 BUD in its parameters, but the executor never pays it. An external tool or block explorer reading the genesis parameters would display a non-zero block reward that does not exist. Validators or investors reading the genesis configuration could expect per-block emission.
- **How to fix**: Set `block_reward: 0` in `mainnet_tokenomics()` to match the testnet template and the documented behavior. If the field must remain non-zero for wire compatibility with old snapshots, add a comment clarifying that the value is never minted.

## F-0130

- **ID**: F-0130
- **Location**: src/tokenomics/mod.rs lines 181-182
- **What is wrong**: The `TokenomicsParams` struct contains a Turkish comment: "Stake Yield (Pasif Getiri)". Line 182 says "Removed hardcoded parameters in `advance_epoch` and brought them here for config-based governance." This reads as an implementation note, not documentation.
- **Why**: Same language issue as other Turkish annotations (F-0099, F-0089, F-0123).
- **What it causes**: The Turkish parenthetical is not understandable by non-Turkish readers. The implementation note will rot as the code evolves.
- **How to fix**: Remove the Turkish parenthetical. Remove or rewrite the implementation note.

## F-0131

- **ID**: F-0131
- **Location**: .github/workflows/ci.yml lines 1434-1436
- **What is wrong**: Three consecutive blank lines between the network-hardening-gate job (ending at line 1432) and the domain-tag-gate job (starting at line 1437). No other job boundary in the file has more than one blank line.
- **Why**: Editing artifact.
- **What it causes**: Inconsistent formatting. Minor.
- **How to fix**: Reduce to one blank line.
