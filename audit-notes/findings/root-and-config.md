# Findings: Root files and configuration

## F-0001

- **ID**: F-0001
- **Location**: Cargo.toml line 194
- **What is wrong**: `bud-node` is still listed as a workspace member dependency. The directive Section 2 and IP-07 require removing `bud/` workspace and `budzero/bud-node`.
- **Why**: B.U.D. 3.0 replaces all earlier storage. The old crate must not ship.
- **What it causes**: Dead code in the binary. Dependency surface stays large. TLG-03 cannot pass.
- **How to fix**: Remove `bud-node` from `[workspace.members]` and all references. Move needed code to `src/persistence/` or `src/storage/`.

## F-0002

- **ID**: F-0002
- **Location**: Cargo.toml line 52
- **What is wrong**: `sled` is a direct dependency. Decision KR-37 states mobile validators cannot use sled.
- **Why**: sled is not maintained, does not build on Android NDK, and uses mmap which is unsafe on mobile.
- **What it causes**: Mobile validator binary cannot link. KR-37 is violated.
- **How to fix**: Replace sled with the B.U.D. 3.0 persistence backend. If a temporary bridge is needed, gate sled behind a feature flag that mobile builds disable.

## F-0003

- **ID**: F-0003
- **Location**: .gitignore line 7
- **What is wrong**: `THE_BUDLUM_DIREKTIF.md` is listed in .gitignore but the file exists in the repository root.
- **Why**: The file was added before the ignore rule, or was force-added. Git tracks it but the ignore rule hides new changes from `git status`.
- **What it causes**: Edits to the directive are invisible in `git diff` and `git status`. An operator may believe the file is untracked.
- **How to fix**: Either remove the .gitignore entry (if the file should be tracked) or remove the file from the index with `git rm --cached` (if it should not be tracked).

## F-0004

- **ID**: F-0004
- **Location**: README.md line 15 (badge area)
- **What is wrong**: The README claims "4444 lib" tests. This number is hardcoded in a badge, not generated from CI.
- **Why**: Badge text is static Markdown. The real test count changes with every commit.
- **What it causes**: The badge becomes stale. TLG-23 requires README.md to equal Appendix A of the directive. A wrong number is a factual error.
- **How to fix**: Replace the static badge with a CI-generated badge that reads the actual test count, or remove the number.

## F-0005

- **ID**: F-0005
- **Location**: src/main.rs line 1536-1539
- **What is wrong**: `bud_node::BudBitswap`, `bud_node::ShardingConfig`, `bud_node::ContentStore`, and `bud_node::FileContentStore` are used directly in main.rs for the storage node initialization.
- **Why**: The directive requires removing bud-node (IP-07). main.rs imports types from it.
- **What it causes**: main.rs depends on a crate that must be removed. The storage init path must be rewritten for B.U.D. 3.0.
- **How to fix**: After B.U.D. 3.0 storage is built (IP-10 through IP-25), replace these types with the new storage API. Until then, this is a blocking dependency on bud-node removal.

## F-0006

- **ID**: F-0006
- **Location**: src/main.rs line 754
- **What is wrong**: The version string is hardcoded as "v0.2.0 (Framework Edition)".
- **Why**: Version should come from Cargo.toml or a build-time constant. A hardcoded string drifts from the actual package version.
- **What it causes**: Operators see a wrong version. Debugging and triage use the wrong version. TLG-26 requires reproducible release artifacts with correct metadata.
- **How to fix**: Use `env!("CARGO_PKG_VERSION")` or a build.rs-generated constant.

## F-0007

- **ID**: F-0007
- **Location**: src/main.rs lines 205-438
- **What is wrong**: The `genesis build` subcommand parses CLI arguments manually with index arithmetic instead of using clap (which is already a dependency and used for the main config on line 523).
- **Why**: Two argument-parsing styles in the same binary. The manual parser does not support `--help`, does not validate combinations, and silently ignores trailing arguments after a matched pair.
- **What it causes**: Inconsistent UX. The keygen subcommand (line 444) has the same manual style. Both diverge from the clap-based main path.
- **How to fix**: Define `genesis build` and `keygen` as clap subcommands. This is a quality finding, not a blocker.

## F-0008

- **ID**: F-0008
- **Location**: src/error.rs (entire file)
- **What is wrong**: `BudlumError` has no `Execution` variant. The UEL (Section 8 of directive) needs execution errors.
- **Why**: The error enum covers Storage, Consensus, Settlement, Bridge, Network, Rpc, Validation. Execution is missing.
- **What it causes**: UEL errors must be shoehorned into another variant or bypass the central error type. IP-26 (Execution Coordinator) cannot use the standard error path.
- **How to fix**: Add `Execution { code: &'static str, message: String }` variant to BudlumError.

## F-0009

- **ID**: F-0009
- **Location**: fable.py (entire file)
- **What is wrong**: AWS credentials are read from environment variables and passed to boto3. The script has no error handling for missing credentials. It calls `claude-fable-5-2` model directly.
- **Why**: This is a development utility, not production code. But it sits in the repo root alongside production files.
- **What it causes**: Confusion about what is production code. The file is not in .gitignore. It could expose credential patterns if copied.
- **How to fix**: Move to a `scripts/` or `tools/` directory. Add to .gitignore or mark clearly as developer-only.

## F-0010

- **ID**: F-0010
- **Location**: modelleri_dene.py (entire file)
- **What is wrong**: Same as F-0009. A development/testing script in the repo root. Lists and tests AWS Bedrock models. Not production code.
- **Why**: Developer convenience files mixed with source.
- **What it causes**: Repo root clutter. Not relevant to testnet.
- **How to fix**: Move to `scripts/` or `tools/` directory.

## F-0011

- **ID**: F-0011
- **Location**: src/lib.rs lines 12-13
- **What is wrong**: `account_abstraction` module is gated behind `#[cfg(feature = "wallet-ml-dsa")]`. The directive Section 8 (IP-45) marks account abstraction as conditional for testnet.
- **Why**: The feature gate name (`wallet-ml-dsa`) describes the crypto backend, not the feature boundary. The directive uses the term "account abstraction" as a testnet-conditional feature.
- **What it causes**: The gate name does not match the directive's feature taxonomy. A reader cannot tell from the gate name that this is a testnet-conditional feature.
- **How to fix**: This is a naming observation. The feature gate works correctly. Consider renaming to match the directive's terminology if the concept name map (IP-05) requires it.

## F-0012

- **ID**: F-0012
- **Location**: Cargo.toml, rust-toolchain.toml
- **What is wrong**: Rust 1.97.1 is pinned. The `libp2p` dependency is pinned to a git SHA for an unpublished 0.57.0.
- **Why**: libp2p 0.57.0 is not on crates.io. The project uses a git fork.
- **What it causes**: Reproducible builds depend on the git remote staying available. If the fork is deleted or force-pushed, builds break. TLG-26 requires reproducible release artifacts.
- **How to fix**: Track when libp2p 0.57.0 is published to crates.io and switch to the registry version. Document the git dependency in the release notes.

## F-0013

- **ID**: F-0013
- **Location**: .cargo/audit.toml
- **What is wrong**: The file exists but the ignore list is empty. All advisory management is in osv-scanner.toml.
- **Why**: Two vulnerability scanning tools with separate config files. cargo-audit reads .cargo/audit.toml, osv-scanner reads osv-scanner.toml.
- **What it causes**: No direct harm. But an auditor checking .cargo/audit.toml sees no exceptions and may think none exist, while osv-scanner.toml has detailed VEX entries. This is a documentation gap.
- **How to fix**: Add a comment to .cargo/audit.toml pointing to osv-scanner.toml as the primary exception list. Or consolidate.
