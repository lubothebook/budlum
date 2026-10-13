# Task template (common rules for sub-agents)

This file holds the common task rules of the PR9 finding program. Every task package points to this file. Language: the report is written in Turkish, code and comments in English (ASD-STE100, short sentences, no long dashes, no bold markers).

## 0. Start

1. First step: read `docs/AGENT_MAP.md`. Do not read the README. Do not read the whole repo.
2. Do not change any file outside the ownership list of the package. If another file must change, report the finding with the matching status line from section 4 (decision or dependency) and write no code.
3. Do not install skills or tools. No scripts, no hooks, no network calls. Do not write shell scripts (Z6). The code languages are Rust and BudL.

## 1. Z2: do not run CI tools locally

`cargo fmt`, `clippy`, `test`, `check`, `build`, `deny`, `audit`, typos, gates, miri and kani are NOT RUN. GitHub CI is the only judge. So read the code like a compiler:

- For every type, function and field you use, open its signature with `rg` and read it. No guessing.
- Check every `use` line, the visibility (`pub`, `pub(crate)`) and the `#[cfg(test)]` scope.
- rustfmt style: 4 spaces, 100 characters per line, common style. Copy the format of the neighbouring code.
- clippy `-D warnings`: avoid patterns such as `field_reassign_with_default`, `needless_borrow`, `too_many_arguments`, `large_enum_variant`, `manual_*`, `unwrap_used` and `expect_used` (deny on the production path).
- Exception: only lock file maintenance (`cargo update -p <crate> --precise <version>`) may be run. No build.

## 2. Flow for each finding

1. Find: the raw finding is in `docs/audit/FINDINGS.md` or `notes/Pxx-scan.md`. Read the code LIVE. Lines may have moved.
2. Verify: show that the finding is still valid with an `rg` command or a test. If it is not valid (the code changed, the claim is wrong), report `YP` (false positive) and write the proof: file:line and command. Write no code.
3. If a decision is needed (architecture or economics: protocol parameter, fee, tokenomics, new crate, new unsafe, backward-incompatible format change), report `KARAR`: write no code. The question text has: the finding ID, the evidence, the options (A, B, C) and your recommendation.
4. Code: first write a test that shows the finding and fails without the fix. Then write the fix. Change only the lines the finding needs. The test goes into the `#[cfg(test)]` module of the owned file. A new test file needs a `mod` entry in another file, so that file must be in the ownership list.
5. Commit: one finding, one commit. Add only the relevant files with `git add <file>` (no `git add -A`). Message format:

```
fix(P17/BLG-0042): <short description>

File: src/storage/erasure.rs:118
Evidence: <how to reproduce>
Test: <name of the added test>

Co-Authored-By: Claude Code <noreply@anthropic.com>
```

The branch you commit to is your worktree branch. DO NOT PUSH. The main session takes your commits and puts them on the PR9 branch.

## 3. Prohibitions (a breach counts as a violation)

- A new `#[allow(...)]`, a new `#[ignore]`, deleting a test or bending an expected value to fit an error, loosening a CI step or pinning it to an old commit, weakening logic for clippy.
- Touching the `.github/*-baseline.txt` files.
- New `unsafe` (Z7). A new external crate (needs a decision).
- `unwrap`, `expect`, `panic!` or unchecked indexing on the production path. Floats on the consensus path (Z8). `saturating_*` and `wrapping_*` on amounts, fees, balances and heights (without a written reason).
- "L1" or "Layer 1" in public text (Z12). Use the approved name given in rule Z12.
- Writing a token, key or seed (Z9).
- Dumping the finding into raw code: you write, read and review the code yourself.
- Adding a new public (pub) item without a production call site (the idle-code gates go red). If the fix needs a new public item that is not connected, wire it into the production path.

## 4. Delivery

Your final answer has ONLY this format, one line per finding, no extra comment:

```
PAKET: <id>   DAL: <git branch name>   TABAN: <sha>
BLG-xxxx | KOD | <sha> | <one-sentence result>
BLG-xxxx | YP | - | <evidence: file:line or command>
BLG-xxxx | KARAR | - | <question: options A/B/C, recommendation>
BLG-xxxx | DEPENDENT | - | <which file, who owns it>
BLG-xxxx | ENGELLI | - | <why, after 3 attempts>
YENI: <new finding outside the package scope: file:line and evidence>   (if any)
```

After three failed attempts, choose a different approach. If it still fails, write `ENGELLI` and move to the next finding.
