# Package rules (PR12 finding completion)

Repo: lubothebook/budlum. Finding source of truth: PR9 branch `origin/consolidate/audit-62-mainnet`.
Commit target: PR12 branch `arena/6e08b7e6-budlum`. You do NOT push. The orchestrator cherry-picks.

## Setup
- Your worktree: /home/user/wt-<PKG> on local branch pkg/<PKG> (already created from PR12 head). Work only there.
- Raw finding text: `git show origin/consolidate/audit-62-mainnet:docs/audit/FINDINGS.md` (P01-P26, P58-P62) or
  `git show origin/consolidate/audit-62-mainnet:notes/Pxx-scan.md` (P27-P57). Read only the lines for your finding
  (use grep -n on the Pxx-Fyy id). Do not read the whole repo, the README or docs.
- Decisions already made (PR9 comments): K1-A (all round-1 recommendations applied), K2-A, K3-A, K4-A, K5-C, K6-B,
  K7-B, K8-C (hybrid EVM path stays closed), K9-C (do not touch the AIR constraint set), K10-A, K11-A, K12-B
  (do not edit LICENSE.md or docs/NOTICE). Unanswered: K13-K26. A finding that needs a new consensus format,
  economic parameter, architecture change, new crate or new unsafe is SKIP-KARAR. Do not guess.

## Per finding loop (one finding at a time, in list order)
1. Read the raw finding. Read the live code at the named place (lines may have moved).
2. Rule 2 (no rewrite): search the worktree (PR12 head) and main for the fix: `rg`, `git log -S<symbol> --oneline
   origin/main pkg/<PKG>`. If the fix or an equivalent is already there: SKIP-VAR. Do not rewrite, do not re-commit,
   do not copy code from another PR branch.
3. If the claim is wrong for the live code: SKIP-YP (give file:line).
4. If the only fix is documentation, a .md file, a comment-only change or a report: SKIP-DOC. Never commit .md files,
   docs, notes or comment-only diffs.
5. Code: write a test that fails without the fix (put it in the `#[cfg(test)]` module of the file you change, or the
   existing test file of that area), then the minimal fix. Change only what the finding needs.
6. Verify: GitHub CI is the only judge. Do NOT run `cargo test`, `cargo clippy` or `cargo build`. Allowed locally:
   `cargo fmt` (only your files may change) and, when needed, `cargo check` on the crate you touched. Read code like a
   compiler: open every signature, `use` line and visibility you rely on with rg. Avoid clippy -D warnings patterns
   (too_many_arguments, field_reassign_with_default, needless_borrow, single_element_loop, doc_lazy_continuation,
   assertions_on_constants, unused imports or bindings).
7. Commit: one finding, one commit. `git add <files>` only (no -A). Message:
   ```
   fix(BLG-xxxx): <short description, English, plain short sentences>

   Co-Authored-By: Claude Sonnet <noreply@anthropic.com>
   ```
8. Append one line to docs/bulgular/pr12/sonuc-<PKG>.txt (orchestrator copies it to PR9):
   `BLG-xxxx | KOD | <sha>` or `BLG-xxxx | SKIP-VAR|SKIP-YP|SKIP-DOC|SKIP-KARAR|FAIL | <one short reason>`.
   Write the line right after each finding, so progress survives an interruption.

## Prohibitions
- New `#[allow(...)]`, new `#[ignore]`, deleting a test, bending an expected value, touching `.github/*-baseline.txt`,
  loosening a CI step. New `unsafe`. New external crate. Editing Cargo.lock (except as a build side effect you then
  revert). `unwrap`/`expect`/`panic!`/unchecked indexing on production paths. Floats on consensus paths.
  `saturating_*`/`wrapping_*` on amounts, fees, balances, heights.
- A new `pub` item without a production call site (idle-code gates go red). Wire it in or keep it private.
- Files outside your package ownership list. If a fix needs another file, SKIP-KARAR with "needs <file>".
- "L1" or "Layer 1" in any text. Code, comments and commit text are English (ASD-STE100, short sentences).
- No push, no PR, no skill or tool install, no shell scripts committed.

## Final answer (max 5 lines)
`PKG <id> done: <n> KOD, <n> SKIP, <n> FAIL. Branch pkg/<PKG> head <sha>.` The detail is in res-<PKG>.txt.

## Skip discipline
SKIP-KARAR is only for a real open owner choice (consensus/wire format, economics, new crate, new unsafe, K13-K26).
"Wiring is missing" or "needs a bigger change" is not a decision: if a minimal fail-closed or bounded fix exists
inside your files (refuse, cap, check, error instead of panic), code it. Prefer a small correct fix over a skip.
