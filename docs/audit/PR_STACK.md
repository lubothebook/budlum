# The open pull request stack

81 pull requests are open against this repository. This file records what they
do to each other, measured from the GitHub API on 2026-11-07: every open pull
request's file list, the files more than one of them touches, and the order the
stack can land in.

It exists because the stack cannot be read one pull request at a time. Four of
the pathologies below only appear in the aggregate.

## 1. The directive is deleted 26 times

`THE_BUDLUM_DIREKTIF.md` is in the diff of 26 open pull requests, each of them
removing the same 6044 lines. That is the boundary rule working as designed and
also a merge conflict waiting: the first one to land removes the file, and the
remaining 25 become no-op diffs that have to be rebased away. One of them
should land, and the rest should drop the file from their diff.

## 2. Two shared files are rebuilt independently by every package

The directive names `docs/work/PACKAGES.md` and
`.github/concept-names-baseline.txt` as shared addition files whose conflicts
are resolved by rebase. In practice 25 pull requests create the package
registry and 7 create the concept-name baseline, each with its own contents.
Only one of each can win. `concept_map.toml` has the same problem in a worse
form: it is written verbatim from Appendix C, so every copy claims to be the
authoritative one, and 6 pull requests carry a copy.

`docs/work/PACKAGES.md` in this pull request is measured from the live stack, so
that the copies can be compared against something.

## 3. `ci.yml` accumulates 19 pull requests' gate steps

Each package that adds a gate adds a canary step and a run step to
`.github/workflows/ci.yml`. Nineteen open pull requests do. The steps are
appended, so they merge textually and still break semantically: two packages
that both add a `concept-names` pair produce two steps running the same gate.

## 4. The B.U.D. 3.0 tree is being built by several packages at once

25 open pull requests touch `src/storage/bud3/`, and `src/storage/bud3/mod.rs`
alone is touched by 17. That directory does not exist on `main` yet. Every one
of those pull requests creates the module root, which means 17 competing
skeletons for one file. WP-10 owns the skeleton; the other packages should be
adding files to it rather than creating it.

## 5. Integration pull requests sit on top of unfinished ones

Seven pull requests are integration stacks rather than packages: #24, #36, #43,
#45, #75, #96 and #113. They carry between 8500 and 30000 added lines because
they absorb the packages under them. Landing one of these before its inputs
lands duplicates the inputs, and landing it after them leaves only the
remainder, which nobody has measured. Each one needs a rebase that drops what
already landed before it can be reviewed.

## 6. CI cannot be read as a verdict

Of the last 100 workflow runs, 45 are queued, 40 are cancelled and none is
complete: the queue is saturated by 81 pull requests pushing 30 workflows each,
and a branch that pushes again cancels its own run. So "CI is red" on any one
pull request is usually not a statement about that pull request. A red check is
evidence only when the run completed.

## The files the stack collides on

| File | Open pull requests | Pull requests |
|---|---|---|
| `docs/work/PACKAGES.md` | 25 | #13, #14, #29, #30, #36, #41, #47, #48, #51, #56, #57, #59, and 13 more |
| `.github/workflows/ci.yml` | 19 | #24, #29, #30, #40, #44, #51, #56, #59, #60, #73, #75, #77, and 7 more |
| `src/storage/bud3/mod.rs` | 17 | #40, #45, #55, #56, #57, #59, #60, #65, #69, #75, #84, #86, and 5 more |
| `xtask/gates/src/main.rs` | 16 | #13, #29, #30, #40, #51, #56, #59, #60, #69, #73, #75, #84, and 4 more |
| `xtask/gates/src/gates/no_out_of_block_consensus_writes.rs` | 16 | #42, #60, #63, #74, #75, #77, #80, #93, #95, #96, #98, #101, and 4 more |
| `src/rpc/server.rs` | 15 | #34, #41, #53, #60, #71, #72, #73, #75, #83, #89, #93, #95, and 3 more |
| `src/bin/support/http.rs` | 15 | #36, #60, #77, #79, #80, #93, #95, #96, #98, #101, #104, #106, and 3 more |
| `src/storage/bud3/bof1.rs` | 14 | #45, #55, #60, #77, #79, #80, #93, #95, #96, #98, #101, #104, and 2 more |
| `crates/bud-codec/src/lib.rs` | 14 | #36, #60, #77, #79, #80, #93, #95, #96, #98, #101, #104, #106, and 2 more |
| `xtask/gates/tests/repository_locks.rs` | 13 | #42, #60, #63, #73, #75, #93, #95, #96, #98, #101, #103, #104, and 1 more |
| `src/tests/bud3_door.rs` | 13 | #45, #60, #77, #79, #80, #93, #95, #96, #98, #101, #104, #106, and 1 more |
| `docs/audit/FINDINGS.md` | 13 | #13, #36, #41, #44, #48, #77, #80, #88, #91, #93, #95, #96, and 1 more |
| `arena_claude.py` | 13 | #24, #34, #43, #60, #75, #77, #96, #100, #104, #106, #107, #109, and 1 more |
| `src/storage/bud3/dedup_index.rs` | 12 | #60, #77, #79, #80, #93, #95, #96, #98, #101, #104, #106, #107 |
| `src/execution/executor.rs` | 12 | #24, #39, #44, #49, #60, #62, #66, #67, #70, #75, #80, #91 |
| `src/crypto/domain_tags.rs` | 12 | #24, #36, #45, #52, #55, #60, #70, #75, #81, #89, #91, #107 |
| `src/core/account.rs` | 11 | #24, #36, #39, #59, #60, #66, #67, #70, #75, #80, #107 |
| `src/tests/mod.rs` | 10 | #52, #55, #56, #59, #60, #65, #75, #79, #80, #84 |
| `src/storage/bud3/registry.rs` | 10 | #45, #55, #59, #60, #65, #75, #93, #104, #106, #107 |
| `src/chain/blockchain.rs` | 10 | #40, #59, #60, #62, #66, #67, #70, #75, #80, #81 |
| `src/rpc/method_class.rs` | 9 | #41, #53, #60, #71, #72, #75, #83, #95, #96 |
| `src/rpc/mod.rs` | 8 | #53, #56, #72, #75, #83, #89, #95, #96 |
| `src/rpc/compat/mod.rs` | 8 | #51, #56, #72, #75, #83, #89, #95, #96 |
| `fuzz/Cargo.toml` | 8 | #29, #30, #36, #55, #61, #75, #104, #113 |
| `crates/uel-abi/tests/transport.rs` | 8 | #36, #38, #54, #64, #75, #77, #80, #113 |
| `crates/uel-abi/src/transport.rs` | 8 | #36, #38, #54, #64, #75, #77, #80, #113 |
| `.github/workflows/fuzz-nightly.yml` | 8 | #29, #30, #36, #55, #61, #75, #104, #113 |
| `src/rpc/tests.rs` | 7 | #41, #53, #71, #75, #89, #95, #96 |
| `src/rpc/api.rs` | 7 | #60, #71, #72, #75, #83, #95, #96 |
| `src/execution/coordinator/mod.rs` | 7 | #24, #36, #46, #51, #60, #75, #96 |

## The order the stack can land in

The directive's dependency table decides this, not the pull request numbering.
What follows is the order in which the open pull requests stop colliding.

1. **The tree boundary and the concept map, once.** One pull request removes the
   directive, writes `config/naming/concept_map.toml` verbatim, adds the
   baseline and the `concept-names` gate, and wires the two CI steps. Every
   other copy is then a rebase away rather than a second implementation.
2. **M0 and M1 slices that stand alone.** WP-01, WP-02, WP-03 and WP-04 are live
   verification and carry no code dependency. WP-05, WP-06, WP-07 and WP-09
   follow, and each of them lands from one pull request rather than from the
   stack that mentions it.
3. **The skeleton packages, one owner each.** WP-10 creates
   `src/storage/bud3/`; WP-26 creates `src/execution/coordinator/`. Until those
   two land, every package that adds files to them is blocked, and 25 open pull
   requests are adding files to the first one.
4. **The storage and execution packages, in dependency order.** WP-18, WP-19,
   WP-20, WP-24, WP-21, then WP-22, WP-23 and WP-25; then WP-27, WP-29, WP-31,
   WP-32 and WP-33 behind WP-26.
5. **The integration stacks last, reduced.** #24, #36, #43, #45, #75, #96 and
   #113 are rebased onto the landed slices and reduced to what is left.

The rule behind the order is the directive's: a package is ready when its
dependencies are on `main`, and a locked path is not modified by a second open
package. Nothing in this stack is blocked by choice; it is blocked by file.

