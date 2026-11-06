//! The four delivery ledgers must exist and stay complete.
//!
//! The directive asks for four ledgers, and until this gate existed nothing
//! failed when one of them was missing or when it quietly lost rows:
//!
//! * `docs/audit/LIVE_STATUS.md` (section 14.1 item 4, milestone M0): the live
//!   state of every finding of the single finding ledger;
//! * `docs/work/PACKAGES.md` (section 15.5 item 7): one row per work package;
//! * `docs/audit/TLG_REPORT.md` (section 18): one row per launch gate item;
//! * `docs/audit/FINDINGS.md` (section 14.1 item 9): where BLG-268 and later
//!   are written.
//!
//! # Why the structure is gated and the numbers are not
//!
//! The three generated ledgers carry a block written by
//! `ops/scripts/live_audit.py`. The numbers in that block measure the tree, so
//! a Rust gate cannot recompute them without writing the measurement a second
//! time, and a second implementation of one measurement is the defect this
//! repository keeps finding (section 15.3 item 6: one owner per definition
//! path). The owner is the script, and the CI job `Live ledgers` runs it with
//! `--check`.
//!
//! What this gate owns is the part a reader relies on and a generator bug would
//! destroy quietly: the four files exist, they are not truncated, every
//! identifier of the directive's own tables is still in them, every Markdown
//! table is rectangular, and no placeholder or typographic dash slipped in. A
//! ledger that loses one package row still passes a byte count and still reads
//! as complete; it fails here.
//!
//! # Why the identifiers are counted rather than trusted
//!
//! Section 15.5 item 5 makes a package record the evidence that a package is
//! done, and section 18 makes an unproven item red. Both need the row to exist,
//! so the row set is checked against the directive's own numbering: 64
//! packages, 27 launch gate items and 311 finding records (267 in the ledger
//! plus 44 archive records).

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

const PACKAGES_DOC: &str = "docs/work/PACKAGES.md";
const LIVE_DOC: &str = "docs/audit/LIVE_STATUS.md";
const TLG_DOC: &str = "docs/audit/TLG_REPORT.md";
const FINDINGS_DOC: &str = "docs/audit/FINDINGS.md";
const GENERATOR: &str = "ops/scripts/live_audit.py";

const GENERATED_DOCS: [&str; 3] = [PACKAGES_DOC, LIVE_DOC, TLG_DOC];
const ALL_DOCS: [&str; 4] = [PACKAGES_DOC, LIVE_DOC, TLG_DOC, FINDINGS_DOC];

/// The markers `ops/scripts/live_audit.py` writes and `--check` compares.
const BEGIN: &str = "<!-- live-audit:begin";
const END: &str = "<!-- live-audit:end -->";

/// A ledger below this size is a stub, not a ledger. The smallest of the four is
/// the findings ledger, and the generated three are far above it.
const MIN_BYTES: usize = 2_000;

const PACKAGE_COUNT: usize = 64;
const TLG_COUNT: usize = 27;
const FINDING_COUNT: usize = 311;
const LEDGER_FINDING_COUNT: usize = 267;
const ARCHIVE_FINDING_COUNT: usize = 44;

/// The archive families, whose identifiers have no fixed width.
const ARCHIVE_PREFIXES: [&str; 3] = ["BULGU-", "TEST-GAP-", "DOC-GAP-"];

const PLACEHOLDERS: [&str; 5] = ["TODO", "TBD", "FIXME", "XXX", "???"];

/// The same thirteen characters `no_unicode_dashes` rejects.
const DASHES: [(char, &str); 13] = [
    ('\u{2010}', "U+2010 hyphen"),
    ('\u{2011}', "U+2011 non-breaking hyphen"),
    ('\u{2012}', "U+2012 figure dash"),
    ('\u{2013}', "U+2013 en dash"),
    ('\u{2014}', "U+2014 em dash"),
    ('\u{2015}', "U+2015 horizontal bar"),
    ('\u{2212}', "U+2212 minus sign"),
    ('\u{00ad}', "U+00AD soft hyphen"),
    ('\u{2e3a}', "U+2E3A two-em dash"),
    ('\u{2e3b}', "U+2E3B three-em dash"),
    ('\u{fe58}', "U+FE58 small em dash"),
    ('\u{fe63}', "U+FE63 small hyphen-minus"),
    ('\u{ff0d}', "U+FF0D fullwidth hyphen-minus"),
];

fn read(root: &Path, rel: &str) -> Result<String, String> {
    let path = root.join(rel);
    std::fs::read_to_string(&path).map_err(|e| format!("cannot read {rel}: {e}"))
}

/// The generated block of a document, if the document carries one.
fn generated_block(text: &str) -> Option<&str> {
    let start = text.find(BEGIN)?;
    let start = text[start..].find('\n').map(|n| start + n + 1)?;
    let end = text[start..].find(END).map(|n| start + n)?;
    Some(&text[start..end])
}

/// Split a Markdown table row into its cells. An escaped pipe is part of a cell
/// and does not split it.
fn cells(row: &str) -> Vec<String> {
    let trimmed = row.trim();
    let body = trimmed.strip_prefix('|').unwrap_or(trimmed);
    let body = body.strip_suffix('|').unwrap_or(body);
    let mut out = Vec::new();
    let mut current = String::new();
    let mut chars = body.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\\' && chars.peek() == Some(&'|') {
            current.push('|');
            let _ = chars.next();
        } else if ch == '|' {
            out.push(current.trim().to_string());
            current = String::new();
        } else {
            current.push(ch);
        }
    }
    out.push(current.trim().to_string());
    out
}

/// The first cell of every row in `block` whose first cell starts with `prefix`.
fn first_cells(block: &str, prefix: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in block.lines() {
        let line = line.trim();
        if !line.starts_with('|') {
            continue;
        }
        if let Some(first) = cells(line).into_iter().next() {
            if first.starts_with(prefix) {
                out.push(first);
            }
        }
    }
    out
}

/// Is this line a Markdown table separator (`|---|:--:|`)?
fn is_separator(line: &str) -> bool {
    let line = line.trim_start();
    line.starts_with('|') && line.chars().all(|c| matches!(c, '|' | '-' | ':' | ' '))
}

/// Every table in `block` must be rectangular: header, separator and every body
/// row must carry the same number of cells.
fn rectangle_problems(block: &str) -> Vec<String> {
    let mut faults = Vec::new();
    let lines: Vec<&str> = block.lines().map(str::trim_end).collect();
    let mut index = 0;
    while index + 1 < lines.len() {
        let is_table = lines[index].trim_start().starts_with('|')
            && is_separator(lines[index + 1]);
        if !is_table {
            index += 1;
            continue;
        }
        let width = cells(lines[index]).len();
        if cells(lines[index + 1]).len() != width {
            let line = index + 1;
            let got = cells(lines[index + 1]).len();
            faults.push(format!(
                "table header at block line {line} is {width} wide but its separator is {got}"
            ));
        }
        let mut row = index + 2;
        while row < lines.len() && lines[row].trim_start().starts_with('|') {
            let got = cells(lines[row]).len();
            if got != width {
                let line = row + 1;
                faults.push(format!(
                    "table row at block line {line} carries {got} cell(s) against a \
                     header of {width}"
                ));
            }
            row += 1;
        }
        index = row;
    }
    faults
}

fn numbered(prefix: &str, count: usize) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for number in 1..=count {
        out.insert(format!("{prefix}{number:02}"));
    }
    out
}

/// Compare a measured identifier list against a numbering.
fn numbering_problems(label: &str, found: &[String], expected: &BTreeSet<String>) -> Vec<String> {
    let mut faults = Vec::new();
    let seen: BTreeSet<String> = found.iter().cloned().collect();
    if found.len() != seen.len() {
        let rows = found.len();
        let distinct = seen.len();
        faults.push(format!(
            "{label}: {rows} row(s) carry only {distinct} distinct identifier(s)"
        ));
    }
    let missing: Vec<&String> = expected.difference(&seen).collect();
    if let Some(first) = missing.first() {
        let count = missing.len();
        faults.push(format!("{label}: {count} identifier(s) are missing, first {first}"));
    }
    let extra: Vec<&String> = seen.difference(expected).collect();
    if let Some(first) = extra.first() {
        let count = extra.len();
        faults.push(format!(
            "{label}: {count} identifier(s) are outside the numbering, first {first}"
        ));
    }
    faults
}

fn text_problems(rel: &str, text: &str) -> Vec<String> {
    let mut faults = Vec::new();
    for token in PLACEHOLDERS {
        if text.contains(token) {
            faults.push(format!("{rel} carries the placeholder {token}"));
        }
    }
    for (ch, name) in DASHES {
        if text.contains(ch) {
            faults.push(format!("{rel} carries {name}, which the dash gate rejects"));
        }
    }
    faults
}

/// The shape and the row set of the three generated ledgers.
fn generated_problems(
    root: &Path,
) -> (Vec<String>, Vec<String>, Vec<String>, Vec<String>) {
    let mut faults = Vec::new();
    let mut packages = Vec::new();
    let mut live = Vec::new();
    let mut tlg = Vec::new();
    for rel in GENERATED_DOCS {
        let text = match read(root, rel) {
            Ok(text) => text,
            Err(e) => {
                faults.push(e);
                continue;
            }
        };
        if text.len() < MIN_BYTES {
            let bytes = text.len();
            faults.push(format!("{rel} is {bytes} byte(s), below the {MIN_BYTES} floor"));
        }
        let Some(block) = generated_block(&text) else {
            faults.push(format!("{rel} carries no generated block markers"));
            continue;
        };
        if !block.contains("ops/scripts/live_audit.py") {
            faults.push(format!("{rel} does not say which script measured it"));
        }
        faults.extend(rectangle_problems(block));
        if rel == PACKAGES_DOC {
            packages = first_cells(block, "WP-");
        } else if rel == LIVE_DOC {
            live = first_cells(block, "BLG-");
            for prefix in ARCHIVE_PREFIXES {
                live.extend(first_cells(block, prefix));
            }
        } else {
            tlg = first_cells(block, "TLG-");
        }
    }
    (faults, packages, live, tlg)
}

/// The findings ledger: it exists, it is not a stub, and it carries BLG-268.
fn findings_ledger_problems(root: &Path) -> Vec<String> {
    let mut faults = Vec::new();
    match read(root, FINDINGS_DOC) {
        Ok(text) => {
            if text.len() < MIN_BYTES {
                let bytes = text.len();
                faults.push(format!(
                    "{FINDINGS_DOC} is {bytes} byte(s), below the {MIN_BYTES} floor"
                ));
            }
            if !text.contains("BLG-268") {
                faults.push(format!("{FINDINGS_DOC} does not carry BLG-268"));
            }
        }
        Err(e) => faults.push(e),
    }
    faults
}

fn finding_count_problems(live: &[String]) -> Vec<String> {
    let mut faults = Vec::new();
    let ledger = live.iter().filter(|id| id.starts_with("BLG-")).count();
    let archive = live.len().saturating_sub(ledger);
    if live.len() != FINDING_COUNT {
        let rows = live.len();
        faults.push(format!(
            "LIVE_STATUS.md carries {rows} finding row(s) against {FINDING_COUNT}"
        ));
    }
    if ledger != LEDGER_FINDING_COUNT {
        faults.push(format!(
            "LIVE_STATUS.md carries {ledger} BLG row(s) against {LEDGER_FINDING_COUNT}"
        ));
    }
    if archive != ARCHIVE_FINDING_COUNT {
        faults.push(format!(
            "LIVE_STATUS.md carries {archive} archive row(s) against \
             {ARCHIVE_FINDING_COUNT}"
        ));
    }
    faults
}

fn judge(root: &Path) -> Result<String, String> {
    let (mut faults, packages, live, tlg) = generated_problems(root);

    match read(root, GENERATOR) {
        Ok(text) if text.len() > 500 => {}
        Ok(_) => faults.push(format!("{GENERATOR} is too small to generate a ledger")),
        Err(e) => faults.push(e),
    }

    faults.extend(findings_ledger_problems(root));
    faults.extend(numbering_problems(
        "PACKAGES.md",
        &packages,
        &numbered("WP-", PACKAGE_COUNT),
    ));
    faults.extend(numbering_problems(
        "TLG_REPORT.md",
        &tlg,
        &numbered("TLG-", TLG_COUNT),
    ));
    faults.extend(finding_count_problems(&live));

    for rel in ALL_DOCS {
        if let Ok(text) = read(root, rel) {
            faults.extend(text_problems(rel, &text));
        }
    }

    if faults.is_empty() {
        return Ok(format!(
            "OK: four ledgers, {PACKAGE_COUNT} packages, {FINDING_COUNT} findings, \
             {TLG_COUNT} launch gate items, no placeholder and no typographic dash"
        ));
    }
    faults.dedup();
    let mut msg = String::new();
    for fault in &faults {
        let _ = writeln!(msg, "{fault}");
    }
    msg.push_str(
        "FAIL: a delivery ledger is missing, truncated or no longer numbered like the \
         directive",
    );
    Err(msg)
}

/// # Errors
///
/// Every problem with the four ledgers, one per line.
pub fn run(root: &Path) -> Result<String, String> {
    judge(root)
}

// ---------------------------------------------------------------------------
// Self-test
// ---------------------------------------------------------------------------
fn scratch() -> Result<PathBuf, String> {
    crate::gates::rust_literals::exclusive_scratch_dir("budlum-gates-audit-ledgers")
}

fn write(root: &Path, rel: &str, body: &str) -> Result<(), String> {
    let path = root.join(rel);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("cannot create {rel}: {e}"))?;
    }
    std::fs::write(&path, body).map_err(|e| format!("cannot write {rel}: {e}"))
}

/// One two-column table with a fixed number of rows.
fn table(header: &str, rows: &[String]) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "| {header} |");
    let columns = header.matches('|').count() + 1;
    let mut separator = String::from("|");
    for _ in 0..columns {
        separator.push_str("---|");
    }
    let _ = writeln!(out, "{separator}");
    for row in rows {
        let _ = writeln!(out, "{row}");
    }
    out
}

fn generated_section(title: &str, filler: char, header: &str, rows: &[String]) -> String {
    let body: String = std::iter::repeat(filler).take(MIN_BYTES).collect();
    format!(
        "# {title}\n\nMeasured by `ops/scripts/live_audit.py`.\n\n{body}\n\n{BEGIN} (generated) \
         -->\n{}{END}\n",
        table(header, rows)
    )
}

/// One valid fixture tree, so a canary can break exactly one thing.
fn fixture(root: &Path) -> Result<(), String> {
    let script = format!("#!/usr/bin/env python3\n{}\n", "x".repeat(600));
    write(root, GENERATOR, &script)?;

    let mut package_rows = Vec::new();
    for number in 1..=PACKAGE_COUNT {
        package_rows.push(format!("| WP-{number:02} | measurement |"));
    }
    write(
        root,
        PACKAGES_DOC,
        &generated_section("Work packages", 'A', "Package | State", &package_rows),
    )?;

    let mut live_rows = Vec::new();
    for number in 1..=LEDGER_FINDING_COUNT {
        live_rows.push(format!("| BLG-{number:03} | Critical |"));
    }
    for number in 1..=11 {
        live_rows.push(format!("| BULGU-{number:03} | High |"));
    }
    for number in 1..=8 {
        live_rows.push(format!("| TEST-GAP-{number:02} | Medium |"));
    }
    for number in 1..=9 {
        live_rows.push(format!("| DOC-GAP-{number:02} | Critical |"));
    }
    write(
        root,
        LIVE_DOC,
        &generated_section("Live status", 'B', "Finding | Severity", &live_rows),
    )?;

    let mut tlg_rows = Vec::new();
    for number in 1..=TLG_COUNT {
        tlg_rows.push(format!("| TLG-{number:02} | red |"));
    }
    write(
        root,
        TLG_DOC,
        &generated_section("Launch gate", 'C', "Item | State", &tlg_rows),
    )?;

    let findings = format!(
        "# Findings ledger\n\n{}\n\n| Id | Subject |\n|---|---|\n| BLG-268 | one |\n",
        "D".repeat(MIN_BYTES)
    );
    write(root, FINDINGS_DOC, &findings)
}

fn canary_dir(base: &Path, name: &str) -> Result<PathBuf, String> {
    let dir = base.join(name);
    fixture(&dir)?;
    Ok(dir)
}

fn expect_fail(dir: &Path, what: &str) -> Result<(), String> {
    match judge(dir) {
        Ok(_) => Err(format!("{what}: the gate passed a tree it must refuse")),
        Err(_) => Ok(()),
    }
}

/// Drop the one row whose first cell equals `id` and rewrite the document.
fn without_row(dir: &Path, rel: &str, id: &str) -> Result<(), String> {
    let text = read(dir, rel)?;
    let prefix = format!("| {id} ");
    let kept: Vec<&str> = text.lines().filter(|line| !line.starts_with(&prefix)).collect();
    write(dir, rel, &format!("{}\n", kept.join("\n")))
}

fn canaries_structural(base: &Path) -> Result<u32, String> {
    let mut done = 0u32;

    let dir = canary_dir(base, "missing-row")?;
    without_row(&dir, PACKAGES_DOC, "WP-31")?;
    expect_fail(&dir, "canary 2 (a package row is missing)")?;
    done += 1;

    let dir = canary_dir(base, "duplicate-row")?;
    let text = read(&dir, PACKAGES_DOC)?;
    write(
        &dir,
        PACKAGES_DOC,
        &text.replace("| WP-31 | measurement |", "| WP-30 | measurement |"),
    )?;
    expect_fail(&dir, "canary 3 (a row was duplicated over another)")?;
    done += 1;

    let dir = canary_dir(base, "ragged")?;
    let text = read(&dir, TLG_DOC)?;
    write(&dir, TLG_DOC, &text.replace("| TLG-14 | red |", "| TLG-14 |"))?;
    expect_fail(&dir, "canary 4 (a table row lost a cell)")?;
    done += 1;

    let dir = canary_dir(base, "markers")?;
    let text = read(&dir, PACKAGES_DOC)?;
    write(&dir, PACKAGES_DOC, &text.replace(BEGIN, "\n"))?;
    expect_fail(&dir, "canary 5 (the generated block markers are gone)")?;
    done += 1;

    let dir = canary_dir(base, "tlg-missing")?;
    without_row(&dir, TLG_DOC, "TLG-27")?;
    expect_fail(&dir, "canary 6 (a launch gate item is missing)")?;
    done += 1;

    Ok(done)
}

fn canaries_text(base: &Path) -> Result<u32, String> {
    let mut done = 0u32;

    let dir = canary_dir(base, "dash")?;
    let text = read(&dir, LIVE_DOC)?;
    write(&dir, LIVE_DOC, &text.replace("Measured by", "Measured\u{2014}by"))?;
    expect_fail(&dir, "canary 7 (an em dash)")?;
    done += 1;

    let dir = canary_dir(base, "placeholder")?;
    let text = read(&dir, FINDINGS_DOC)?;
    write(&dir, FINDINGS_DOC, &format!("{text}\n| BLG-269 | TBD |\n"))?;
    expect_fail(&dir, "canary 8 (a placeholder)")?;
    done += 1;

    let dir = canary_dir(base, "no-generator")?;
    std::fs::remove_file(dir.join(GENERATOR)).map_err(|e| e.to_string())?;
    expect_fail(&dir, "canary 9 (the generator is gone)")?;
    done += 1;

    let dir = canary_dir(base, "no-findings-ledger")?;
    std::fs::remove_file(dir.join(FINDINGS_DOC)).map_err(|e| e.to_string())?;
    expect_fail(&dir, "canary 10 (the findings ledger is gone)")?;
    done += 1;

    Ok(done)
}

/// # Errors
///
/// The first canary that misbehaves.
pub fn self_test() -> Result<String, String> {
    let base = scratch()?;

    // Canary 1: the valid fixture passes. Without this the refuses below prove
    // nothing.
    let good = canary_dir(&base, "good")?;
    if let Err(e) = judge(&good) {
        let _ = std::fs::remove_dir_all(&base);
        return Err(format!("canary 1: the valid fixture was refused: {e}"));
    }
    let mut canaries = 1u32;

    match canaries_structural(&base) {
        Ok(count) => canaries += count,
        Err(e) => {
            let _ = std::fs::remove_dir_all(&base);
            return Err(e);
        }
    }
    match canaries_text(&base) {
        Ok(count) => canaries += count,
        Err(e) => {
            let _ = std::fs::remove_dir_all(&base);
            return Err(e);
        }
    }

    let _ = std::fs::remove_dir_all(&base);
    Ok(format!("audit ledger gate self-test OK: {canaries} canaries"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escaped_pipes_do_not_split_a_cell() {
        assert_eq!(
            cells("| a \\| b | c |"),
            vec!["a | b".to_string(), "c".to_string()]
        );
    }

    #[test]
    fn a_rectangle_table_passes_and_a_ragged_row_fails() {
        assert!(rectangle_problems("| A | B |\n|---|---|\n| 1 | 2 |\n").is_empty());
        assert_eq!(rectangle_problems("| A | B |\n|---|---|\n| 1 |\n").len(), 1);
    }

    #[test]
    fn the_numbering_helper_builds_a_two_digit_set() {
        let set = numbered("WP-", PACKAGE_COUNT);
        assert!(set.contains("WP-01"));
        assert!(set.contains("WP-64"));
        assert!(!set.contains("WP-65"));
    }
}
