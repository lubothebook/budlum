# Second pass alpha: part audit template

Repo: /var/tmp/budlum-gp/aud2 (git worktree, base 7c496cd). Use only this path.

## Rules

- First line of work: read docs/AGENT_MAP.md. Then read sections 5 and 6 of docs/AUDIT_PLAN.md.
- Read only. Never edit production code. No cargo, no builds, no fmt, no clippy. No skill or tool install.
- Bash only with: rg, sed -n 'a,bp', git log, git show, git diff, git ls-files, wc -l. The Read tool may use offset and limit.
- Read at most 1500 lines per call. Read every file range of your part in full. Outside the part: rg first, then at most 80 lines.
- Write exactly one file: notes/second-pass-alpha-<Pxx>.md. Create or edit no other file. Do not commit.
- English, ASCII only, short plain sentences. No bold, no long dashes. Do not name any AI, model or tool vendor in the text.
- Defensive wording: say what must be refused and what happens today. No exploit narration, no working exploit code.
- No secret values. For a possible secret write file:line and type only.
- Every claim has file:line from HEAD (7c496cd). No claim without evidence. Never write "probably".
- Scout or earlier-note output is a lead, not evidence. Read the cited lines yourself.

## Method

1. Read the first-pass findings of the part: `rg -n '<Pxx>-F' docs/audit/FINDINGS.md`, then read those sections. Also read notes/<Pxx>-scan.md if it exists. Also read any verification notes that cite the part.
2. Re-audit the listed ranges from scratch. The first pass used base 25ede0b; the code may have changed (check `git log --oneline 25ede0b..HEAD -- <path>` if the history is there). Say if a first-pass finding is already fixed at HEAD.
3. For each first-pass finding of the part, give a verdict: CONFIRMED (still true at HEAD), CHALLENGED (wrong or overstated, with evidence), FIXED (no longer true, with evidence), or UNCHECKED (reason). If severity should change, say so with one sentence.
4. Hunt for what was missed. Axes: correctness (overflow, `as` truncation, unsigned subtraction, HashMap order, half-finished state on error, panic on external input), security (bounds, skipped checks, replay, authority, DoS), integration (public entry with no production caller: `rg -n '<symbol>' src/ budzero/ crates/`), test coverage (negative test, empty assert, ignore), dead code (known baselines: .github/dead-pub-api-baseline.txt, .github/idle-code-baseline.txt), and rules Z3 Z7 Z8 Z9 and LABEL-12.9 (unwrap_or_default inside to_bytes: label only).
5. Part type extra checks are in section 5 of docs/AUDIT_PLAN.md. Apply the ones for your part.
6. Aim for real findings. Do not pad. A part with no new finding is allowed: list the axes checked and the rg commands used.

## Output file layout

```
# Pxx name
Base 7c496cd. Files read: <file: ranges>. Not read: <ranges and why, or none>.

## First-pass verdicts
- Pxx-Fnn: CONFIRMED|CHALLENGED|FIXED|UNCHECKED | file:line | one or two sentences | severity note if changed

## New findings
### SPA-Pxx-F01
File: path:lines
Severity: Critical | High | Medium | Low | Info
Axis: correctness | security | integration | test | dead code | invariant(Zn) | LABEL-12.9
Confidence: Certain | Likely (name the missing check)
Evidence: file:line, a short quote of at most 5 lines, or an rg command with its result
What must be refused and what happens today: 1 to 3 short sentences
Impact: who, with which precondition, loses what
Known: none | STATUS.md | baseline file

## Cross-part notes
Facts that matter for the invariants section: state root inputs, epoch handling, finality, serialization determinism, supply ceiling paths. One line each with file:line. Write none if none.

## Checks done
Axes and rg commands used (short).
```

Severity scale: Critical = fund mint or loss with no precondition, finality or consensus safety break, forged proof accepted, network halt. High = same class with a precondition, or remote single-node crash, or lasting data corruption. Medium = limited impact, wrong error handling, mainnet part not wired, missing negative test on an R3 path. Low = defense in depth, maintenance, dead code. Info = observation.

Keep the file under 350 lines. At most 3 lines of prose per finding beyond the fixed fields.

## Reply to the lead (at most 5 lines)

Line 1: DONE or BLOCKED. Line 2: the file path. Line 3: counts by severity for new findings. Line 4: first-pass verdict counts (confirmed, challenged, fixed). Line 5: one sentence on the most serious new finding. No code, no log, no diff.
