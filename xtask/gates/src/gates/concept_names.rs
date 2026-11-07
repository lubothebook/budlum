//! Concept names: the retired metaphors are counted, and the count only falls.
//!
//! # The failure this closes
//!
//! Section 2 of the directive removes the metaphor names from the architecture.
//! It keeps two concept names, `BudZero` and `B.U.D.`, and it replaces the rest
//! with the professional term that states the function. A rename that is not
//! gated regresses the way a translation does: a new test name, a new error
//! message and a new doc heading are invisible to review because nothing fails,
//! and the old vocabulary creeps back one line at a time. Measured on this
//! tree, the retired names still appear in several thousand lines.
//!
//! So the map in [`MAP_PATH`] lists, for each retired name, the regular
//! expressions that find it, the paths where it may appear, and the work
//! package that owns the rename. This gate counts what is left and compares the
//! count with [`BASELINE_PATH`]. The count only goes down:
//!
//! * a concept that gains a file or a line fails,
//! * a concept whose count falls must have its baseline lowered in the same
//!   change, so progress is recorded rather than quietly banked,
//! * in `absolute` mode, which WP-43 sets at the end of T-Launch, any hit at all
//!   fails.
//!
//! # Why the patterns are read, not grepped
//!
//! A plain substring search would miss the case variants and the inflections
//! the map lists. The map's patterns are regular expressions in the syntax of
//! the `regex` crate, and this gate implements the subset that the map uses:
//! literal characters, an `(?i)` flag at the start of a pattern, `\b`, `^`,
//! `$`, character classes with ranges and negation, single-level groups with
//! alternation, and the `?` quantifier. Anything outside that subset is a
//! finding, not a silent pass: a gate that cannot read a pattern must say so
//! rather than report the tree clean. The `regex` crate is not a dependency of
//! this workspace, and adding one to a gate that runs inside the trust boundary
//! is a larger change than the matcher needs.
//!
//! # What counts
//!
//! Per concept, two numbers: how many files carry the name, and how many lines
//! carry it. A line that carries the name three times counts once, because the
//! work is to rewrite the line, not to count occurrences. Both numbers are
//! recorded, because a rename can shrink the first while leaving a long file
//! that is mostly the old vocabulary.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

/// The name map the directive specifies, written verbatim from its Appendix C.
const MAP_PATH: &str = "config/naming/concept_map.toml";

/// The recorded counts, which may only fall.
const BASELINE_PATH: &str = ".github/concept-names-baseline.txt";

/// The roots the gate scans when a concept names no paths of its own.
const SCAN_ROOTS: &[&str] = &[
    "src",
    "crates",
    "budzero",
    "execution",
    "mobile",
    "xtask",
    "docs",
    "config",
    "proto",
    "contracts",
];

/// Directories that hold no prose worth scanning.
const SKIP_DIRS: &[&str] = &[".git", "target", "node_modules"];

/// Build output that is not committed and not written by hand.
const SKIP_PREFIXES: &[&str] = &["contracts/out", "contracts/cache"];

/// A scan that walked fewer files than this is vacuous and must fail rather
/// than pass by looking at nothing.
const VACUITY_FLOOR: usize = 50;

/// How many findings are printed before the report is summarised.
const MAX_REPORTED: usize = 40;

/// The shortest literal a pattern must contain before it can be used to skip a
/// line without running the matcher. One character prunes almost nothing.
const MIN_NEEDLE: usize = 2;

/// Counts for one concept.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Counts {
    /// Files that carry the name at least once.
    files: usize,
    /// Lines that carry the name at least once.
    lines: usize,
}

/// How the counts are judged.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    /// A ratchet: the count may not rise, and a fall must be recorded.
    Count,
    /// No hit at all is acceptable.
    Absolute,
}

/// One item of a character class.
#[derive(Debug, Clone, PartialEq, Eq)]
enum ClassItem {
    /// A single character.
    One(char),
    /// An inclusive range, `a` to `z`.
    Range(char, char),
}

/// What a pattern node matches.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Kind {
    /// One literal character.
    Literal(char),
    /// A character class, possibly negated.
    Class {
        /// Whether the class is negated.
        negated: bool,
        /// The class members.
        items: Vec<ClassItem>,
    },
    /// A word boundary.
    Boundary,
    /// The start of the line.
    Start,
    /// The end of the line.
    End,
}

/// One node of a flattened pattern.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    /// What the node matches.
    kind: Kind,
    /// Whether the node may be skipped, which is how `?` is represented.
    optional: bool,
}

/// A literal that any match of one alternative must contain.
///
/// It exists to keep the matcher off lines that cannot match. The matcher is
/// quadratic in the line length and this tree is 866 text files, so a gate that
/// ran it on every line of every file would take minutes.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Needle {
    /// The literal, already folded when the pattern ignores case.
    text: String,
    /// Whether the pattern that owns it compares case-insensitively.
    ignore_case: bool,
}

/// A pattern from the map, compiled to one sequence per alternative.
///
/// A group with alternation is expanded into separate sequences at load time,
/// so matching never needs to backtrack across a group.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Compiled {
    /// One flat node sequence per alternative.
    alternatives: Vec<Vec<Node>>,
    /// Whether the pattern's `(?i)` flag was set.
    ignore_case: bool,
    /// One required literal per alternative, when every alternative has one.
    ///
    /// `None` means the pattern cannot be pre-filtered, and every line has to
    /// reach the matcher.
    needles: Option<Vec<Needle>>,
}

/// One record of the map.
#[derive(Debug, Clone)]
struct Concept {
    /// The baseline key: the new name, folded to lowercase words.
    slug: String,
    /// The work package that owns the rename.
    owner: String,
    /// The retired names, for the report.
    old: Vec<String>,
    /// The name that replaces them.
    new_code: String,
    /// The compiled patterns.
    patterns: Vec<Compiled>,
    /// The paths this concept is scanned under, if any.
    paths: Vec<String>,
    /// The paths this concept is not scanned under, if any.
    allow_paths: Vec<String>,
    /// The literals that must be present before the matcher runs, when every
    /// pattern of the concept offers one.
    needles: Option<Vec<Needle>>,
}

/// The parsed map.
#[derive(Debug, Clone)]
struct Map {
    /// How the counts are judged.
    mode: Mode,
    /// Paths that no concept scans.
    global_allow_paths: Vec<String>,
    /// Every record, in file order.
    concepts: Vec<Concept>,
}

/// Cut a `#` comment, respecting quoted strings.
fn strip_comment(line: &str) -> &str {
    let mut quote: Option<char> = None;
    let mut escaped = false;
    for (idx, c) in line.char_indices() {
        match quote {
            Some(open) => {
                if escaped {
                    escaped = false;
                } else if c == '\\' && open == '"' {
                    escaped = true;
                } else if c == open {
                    quote = None;
                }
            }
            None => {
                if c == '"' || c == '\'' {
                    quote = Some(c);
                } else if c == '#' {
                    return line.get(..idx).unwrap_or("");
                }
            }
        }
    }
    line
}

/// The change in bracket depth a line makes, ignoring quoted text.
fn depth_delta(line: &str) -> i32 {
    let mut quote: Option<char> = None;
    let mut escaped = false;
    let mut delta = 0i32;
    for c in line.chars() {
        match quote {
            Some(open) => {
                if escaped {
                    escaped = false;
                } else if c == '\\' && open == '"' {
                    escaped = true;
                } else if c == open {
                    quote = None;
                }
            }
            None => {
                if c == '"' || c == '\'' {
                    quote = Some(c);
                } else if c == '[' {
                    delta += 1;
                } else if c == ']' {
                    delta -= 1;
                }
            }
        }
    }
    delta
}

/// Join a multi-line array into one line, so the parser sees one entry.
fn logical_lines(text: &str) -> Vec<(usize, String)> {
    let mut out: Vec<(usize, String)> = Vec::new();
    let mut open: Option<(usize, String)> = None;
    let mut depth = 0i32;
    for (idx, raw) in text.lines().enumerate() {
        let cleaned = strip_comment(raw);
        let trimmed = cleaned.trim();
        if open.is_none() && trimmed.is_empty() {
            continue;
        }
        let start = open.as_ref().map_or(idx, |(line, _)| *line);
        let mut buffer = open.as_ref().map_or_else(String::new, |(_, s)| s.clone());
        if !buffer.is_empty() {
            buffer.push(' ');
        }
        buffer.push_str(trimmed);
        depth += depth_delta(trimmed);
        if depth <= 0 {
            out.push((start, buffer));
            depth = 0;
            open = None;
        } else {
            open = Some((start, buffer));
        }
    }
    if let Some((start, buffer)) = open {
        out.push((start, buffer));
    }
    out
}

/// Split an array body on top-level commas.
fn split_top_level(inner: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut buffer = String::new();
    let mut quote: Option<char> = None;
    let mut escaped = false;
    for c in inner.chars() {
        match quote {
            Some(open) => {
                buffer.push(c);
                if escaped {
                    escaped = false;
                } else if c == '\\' && open == '"' {
                    escaped = true;
                } else if c == open {
                    quote = None;
                }
            }
            None => {
                if c == '"' || c == '\'' {
                    quote = Some(c);
                    buffer.push(c);
                } else if c == ',' {
                    out.push(std::mem::take(&mut buffer));
                } else {
                    buffer.push(c);
                }
            }
        }
    }
    if !buffer.trim().is_empty() {
        out.push(buffer);
    }
    out
}

/// One string value, with its quotes removed.
fn parse_string(raw: &str) -> Result<String, String> {
    let trimmed = raw.trim();
    if let Some(rest) = trimmed.strip_prefix('\'') {
        let body = rest
            .strip_suffix('\'')
            .ok_or_else(|| format!("a literal string is not closed: {trimmed}"))?;
        return Ok(body.to_string());
    }
    if let Some(rest) = trimmed.strip_prefix('"') {
        let body = rest
            .strip_suffix('"')
            .ok_or_else(|| format!("a string is not closed: {trimmed}"))?;
        let mut out = String::with_capacity(body.len());
        let mut escaped = false;
        for c in body.chars() {
            if escaped {
                let decoded = match c {
                    '\\' => '\\',
                    '"' => '"',
                    'n' => '\n',
                    't' => '\t',
                    _ => {
                        return Err(format!("an unsupported escape in a string: {trimmed}"));
                    }
                };
                out.push(decoded);
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else {
                out.push(c);
            }
        }
        return Ok(out);
    }
    Err(format!("a value is not a quoted string: {trimmed}"))
}

/// One array of strings.
fn parse_array(raw: &str) -> Result<Vec<String>, String> {
    let trimmed = raw.trim();
    let body = trimmed
        .strip_prefix('[')
        .and_then(|rest| rest.strip_suffix(']'))
        .ok_or_else(|| format!("a value is not an array: {trimmed}"))?;
    let mut out = Vec::new();
    for part in split_top_level(body) {
        if part.trim().is_empty() {
            continue;
        }
        out.push(parse_string(&part)?);
    }
    Ok(out)
}

/// The baseline key for the new name: lowercase, punctuation folded to hyphens.
fn slug_of(new_code: &str) -> String {
    let mut out = String::with_capacity(new_code.len());
    for c in new_code.chars() {
        let lowered = c.to_lowercase().next().unwrap_or(c);
        if lowered.is_ascii_lowercase() || lowered.is_ascii_digit() {
            out.push(lowered);
        } else if !out.ends_with('-') {
            out.push('-');
        }
    }
    out.trim_matches('-').to_string()
}

/// Parse a character class body.
fn parse_class(chars: &[char], cursor: &mut usize) -> Result<Kind, String> {
    *cursor += 1;
    let mut negated = false;
    if chars.get(*cursor) == Some(&'^') {
        negated = true;
        *cursor += 1;
    }
    let mut items: Vec<ClassItem> = Vec::new();
    loop {
        let Some(&c) = chars.get(*cursor) else {
            return Err(String::from(
                "a character class is not closed before the end of the pattern",
            ));
        };
        if c == ']' {
            if items.is_empty() {
                return Err(String::from("a character class has no members"));
            }
            *cursor += 1;
            return Ok(Kind::Class { negated, items });
        }
        if c == '\\' {
            return Err(String::from(
                "escapes inside a character class are outside the supported pattern subset",
            ));
        }
        if chars.get(*cursor + 1) == Some(&'-') {
            if let Some(&high) = chars.get(*cursor + 2) {
                if high != ']' {
                    items.push(ClassItem::Range(c, high));
                    *cursor += 3;
                    continue;
                }
            }
        }
        items.push(ClassItem::One(c));
        *cursor += 1;
    }
}

/// Parse one node, plus the `?` that may follow it.
fn parse_node(chars: &[char], cursor: &mut usize) -> Result<Node, String> {
    let c = *chars
        .get(*cursor)
        .ok_or_else(|| String::from("a pattern ends where a node was expected"))?;
    let kind = match c {
        '\\' => {
            *cursor += 1;
            let escaped = *chars
                .get(*cursor)
                .ok_or_else(|| String::from("a pattern ends with a backslash"))?;
            *cursor += 1;
            match escaped {
                'b' => Kind::Boundary,
                'd' | 'D' | 'w' | 'W' | 's' | 'S' | 'B' => {
                    return Err(format!(
                        "the class escape '\\{escaped}' is outside the supported pattern subset"
                    ));
                }
                literal => Kind::Literal(literal),
            }
        }
        '[' => parse_class(chars, cursor)?,
        '^' => {
            *cursor += 1;
            Kind::Start
        }
        '$' => {
            *cursor += 1;
            Kind::End
        }
        '?' | '*' | '+' => {
            return Err(format!(
                "the quantifier '{c}' is outside the supported pattern subset"
            ));
        }
        literal => {
            *cursor += 1;
            Kind::Literal(literal)
        }
    };
    let mut optional = false;
    if chars.get(*cursor) == Some(&'?') {
        optional = true;
        *cursor += 1;
    }
    Ok(Node { kind, optional })
}

/// Parse a single-level group, whose alternatives are flat node sequences.
fn parse_group(chars: &[char], cursor: &mut usize) -> Result<Vec<Vec<Node>>, String> {
    *cursor += 1;
    let mut alternatives: Vec<Vec<Node>> = Vec::new();
    let mut current: Vec<Node> = Vec::new();
    loop {
        let Some(&c) = chars.get(*cursor) else {
            return Err(String::from(
                "a group is not closed before the end of the pattern",
            ));
        };
        match c {
            ')' => {
                *cursor += 1;
                break;
            }
            '|' => {
                *cursor += 1;
                alternatives.push(std::mem::take(&mut current));
            }
            '(' => {
                return Err(String::from(
                    "nested groups are outside the supported pattern subset",
                ));
            }
            _ => current.push(parse_node(chars, cursor)?),
        }
    }
    alternatives.push(current);
    if chars.get(*cursor) == Some(&'?') {
        *cursor += 1;
        alternatives.push(Vec::new());
    }
    Ok(alternatives)
}

/// Fold one character the way the matcher compares characters.
fn fold(c: char) -> char {
    c.to_lowercase().next().unwrap_or(c)
}

/// Lowercase a line one character at a time, the way the matcher compares.
fn lowercase(text: &str) -> String {
    text.chars().map(fold).collect()
}

/// The longest run of consecutive, non-optional literals in one alternative.
fn longest_literal_run(nodes: &[Node], ignore_case: bool) -> Option<Needle> {
    let mut best = String::new();
    let mut current = String::new();
    for node in nodes {
        match (&node.kind, node.optional) {
            (Kind::Literal(c), false) => {
                current.push(if ignore_case { fold(*c) } else { *c });
                if current.chars().count() > best.chars().count() {
                    best = current.clone();
                }
            }
            _ => current.clear(),
        }
    }
    if best.chars().count() < MIN_NEEDLE {
        return None;
    }
    Some(Needle {
        text: best,
        ignore_case,
    })
}

/// One required literal per alternative, or `None` when an alternative has
/// none, in which case the pattern cannot be pre-filtered.
fn needles_for(alternatives: &[Vec<Node>], ignore_case: bool) -> Option<Vec<Needle>> {
    let mut out = Vec::new();
    for alternative in alternatives {
        out.push(longest_literal_run(alternative, ignore_case)?);
    }
    Some(out)
}

/// Is a needle present on a line?
fn needle_present(needle: &Needle, lower: &str, raw: &str) -> bool {
    if needle.ignore_case {
        lower.contains(needle.text.as_str())
    } else {
        raw.contains(needle.text.as_str())
    }
}

/// Parse a top-level pattern into alternatives of flat node sequences.
fn parse_pattern(src: &str) -> Result<Compiled, String> {
    let chars: Vec<char> = src.chars().collect();
    let mut cursor = 0usize;
    let mut ignore_case = false;
    if src.starts_with("(?i)") {
        ignore_case = true;
        cursor = 4;
    } else if src.contains("(?") {
        return Err(format!(
            "the inline flag group in '{src}' is outside the supported pattern subset"
        ));
    }
    let mut alternatives: Vec<Vec<Node>> = vec![Vec::new()];
    while cursor < chars.len() {
        let c = chars[cursor];
        if c == '(' {
            let group = parse_group(&chars, &mut cursor)?;
            let mut expanded: Vec<Vec<Node>> = Vec::new();
            for prefix in &alternatives {
                for alternative in &group {
                    let mut merged = prefix.clone();
                    merged.extend(alternative.iter().cloned());
                    expanded.push(merged);
                }
            }
            alternatives = expanded;
        } else if c == ')' || c == '|' {
            return Err(format!(
                "the stray '{c}' is outside the supported pattern subset"
            ));
        } else {
            let node = parse_node(&chars, &mut cursor)?;
            for alternative in &mut alternatives {
                alternative.push(node.clone());
            }
        }
    }
    if alternatives.iter().all(Vec::is_empty) {
        return Err(format!("the pattern '{src}' matches the empty string"));
    }
    let needles = needles_for(&alternatives, ignore_case);
    Ok(Compiled {
        alternatives,
        ignore_case,
        needles,
    })
}

/// Is this character part of a word, for a `\b` boundary.
fn is_word(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Compare two characters, honouring a pattern's `(?i)` flag.
fn same_char(a: char, b: char, ignore_case: bool) -> bool {
    if !ignore_case {
        return a == b;
    }
    a.to_lowercase().next().unwrap_or(a) == b.to_lowercase().next().unwrap_or(b)
}

/// Does one class item hold this character?
fn class_holds(item: &ClassItem, c: char, ignore_case: bool) -> bool {
    match item {
        ClassItem::One(one) => same_char(c, *one, ignore_case),
        ClassItem::Range(low, high) => {
            let (low, high, probe) = if ignore_case {
                (
                    low.to_lowercase().next().unwrap_or(*low),
                    high.to_lowercase().next().unwrap_or(*high),
                    c.to_lowercase().next().unwrap_or(c),
                )
            } else {
                (*low, *high, c)
            };
            low <= probe && probe <= high
        }
    }
}

/// Does a class hold this character?
fn class_matches(negated: bool, items: &[ClassItem], c: char, ignore_case: bool) -> bool {
    let held = items.iter().any(|item| class_holds(item, c, ignore_case));
    held != negated
}

/// Consume one node at a position, if it matches there.
fn step(node: &Node, chars: &[char], pos: usize, ignore_case: bool) -> Option<usize> {
    match &node.kind {
        Kind::Start => {
            if pos == 0 {
                Some(pos)
            } else {
                None
            }
        }
        Kind::End => {
            if pos == chars.len() {
                Some(pos)
            } else {
                None
            }
        }
        Kind::Boundary => {
            let before = pos > 0 && is_word(chars[pos - 1]);
            let after = pos < chars.len() && is_word(chars[pos]);
            if before == after {
                None
            } else {
                Some(pos)
            }
        }
        Kind::Literal(want) => {
            if pos < chars.len() && same_char(chars[pos], *want, ignore_case) {
                Some(pos + 1)
            } else {
                None
            }
        }
        Kind::Class { negated, items } => {
            if pos < chars.len() && class_matches(*negated, items, chars[pos], ignore_case) {
                Some(pos + 1)
            } else {
                None
            }
        }
    }
}

/// Match one node sequence from a node index and a character position.
fn match_seq(nodes: &[Node], chars: &[char], index: usize, pos: usize, ignore_case: bool) -> bool {
    let Some(node) = nodes.get(index) else {
        return true;
    };
    let stepped = step(node, chars, pos, ignore_case);
    if stepped.is_some_and(|next| match_seq(nodes, chars, index + 1, next, ignore_case)) {
        return true;
    }
    node.optional && match_seq(nodes, chars, index + 1, pos, ignore_case)
}

/// Does a compiled pattern match anywhere on a line?
fn pattern_matches(pattern: &Compiled, chars: &[char]) -> bool {
    for start in 0..=chars.len() {
        for alternative in &pattern.alternatives {
            if match_seq(alternative, chars, 0, start, pattern.ignore_case) {
                return true;
            }
        }
    }
    false
}

/// One required value out of a record.
fn require(record: &BTreeMap<String, String>, key: &str) -> Result<String, String> {
    let value = record
        .get(key)
        .ok_or_else(|| format!("{MAP_PATH}: a concept record has no {key}"))?;
    Ok(value.clone())
}

/// Build a concept record out of the keys one table carried.
fn build_concept(record: &BTreeMap<String, String>) -> Result<Concept, String> {
    let patterns_raw = require(record, "patterns")?;
    let new_code = parse_string(require(record, "new_code")?.as_str())?;
    let owner = parse_string(require(record, "owner")?.as_str())?;
    let old = parse_array(require(record, "old")?.as_str())?;
    if old.is_empty() {
        return Err(format!("{MAP_PATH}: '{new_code}' lists no old name"));
    }
    let mut patterns = Vec::new();
    for source in parse_array(patterns_raw.as_str())? {
        let compiled = parse_pattern(&source).map_err(|e| {
            format!("{MAP_PATH}: '{new_code}' has a pattern the gate cannot read: {e}")
        })?;
        patterns.push(compiled);
    }
    if patterns.is_empty() {
        return Err(format!("{MAP_PATH}: '{new_code}' lists no pattern"));
    }
    let paths = optional_array(record, "paths")?;
    let allow_paths = optional_array(record, "allow_paths")?;
    let mut needles = Vec::new();
    let mut every_pattern_has_needles = true;
    for pattern in &patterns {
        match &pattern.needles {
            Some(found) => needles.extend(found.iter().cloned()),
            None => every_pattern_has_needles = false,
        }
    }
    let needles = if every_pattern_has_needles {
        Some(needles)
    } else {
        None
    };
    Ok(Concept {
        slug: slug_of(&new_code),
        owner,
        old,
        new_code,
        patterns,
        paths,
        allow_paths,
        needles,
    })
}

/// One optional array key, empty when the record does not carry it.
fn optional_array(record: &BTreeMap<String, String>, key: &str) -> Result<Vec<String>, String> {
    let raw = record.get(key).cloned().unwrap_or_default();
    if raw.is_empty() {
        return Ok(Vec::new());
    }
    parse_array(raw.as_str())
}

/// Parse the whole map.
fn parse_map(text: &str) -> Result<Map, String> {
    let mut mode: Option<Mode> = None;
    let mut global_allow_paths: Vec<String> = Vec::new();
    let mut concepts: Vec<Concept> = Vec::new();
    let mut record: BTreeMap<String, String> = BTreeMap::new();
    for (line_number, line) in logical_lines(text) {
        if line == "[[concept]]" {
            if !record.is_empty() {
                concepts.push(build_concept(&record)?);
                record.clear();
            }
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            return Err(format!(
                "{MAP_PATH}: line {} is neither a table header nor a key = value pair",
                line_number + 1
            ));
        };
        let key = key.trim();
        let value = value.trim();
        match key {
            "schema" => {
                if value != "1" {
                    return Err(format!(
                        "{MAP_PATH}: schema {value} is not the version this gate reads"
                    ));
                }
            }
            "mode" => {
                mode = Some(match parse_string(value)?.as_str() {
                    "count" => Mode::Count,
                    "absolute" => Mode::Absolute,
                    other => {
                        return Err(format!("{MAP_PATH}: unknown mode '{other}'"));
                    }
                });
            }
            "global_allow_paths" => global_allow_paths = parse_array(value)?,
            _ => {
                record.insert(key.to_string(), value.to_string());
            }
        }
    }
    if !record.is_empty() {
        concepts.push(build_concept(&record)?);
    }
    if concepts.is_empty() {
        return Err(format!("{MAP_PATH}: the map has no concept record"));
    }
    let mut seen: BTreeMap<&str, &str> = BTreeMap::new();
    for concept in &concepts {
        if concept.slug.is_empty() {
            return Err(format!(
                "{MAP_PATH}: '{code}' has no usable baseline key",
                code = concept.new_code
            ));
        }
        if let Some(other) = seen.insert(concept.slug.as_str(), concept.new_code.as_str()) {
            return Err(format!(
                "{MAP_PATH}: '{}' and '{other}' share the baseline key '{}'",
                concept.new_code, concept.slug
            ));
        }
    }
    let mode = mode.ok_or_else(|| format!("{MAP_PATH}: the map has no mode"))?;
    Ok(Map {
        mode,
        global_allow_paths,
        concepts,
    })
}

/// Collect the files under the scan roots, as repository-relative paths.
fn collect_files(root: &Path) -> Vec<(String, PathBuf)> {
    let mut out: Vec<(String, PathBuf)> = Vec::new();
    for scan_root in SCAN_ROOTS {
        let base = root.join(scan_root);
        if base.is_dir() {
            walk_into(root, &base, &mut out);
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

/// Recursively collect the files of one directory.
fn walk_into(root: &Path, dir: &Path, out: &mut Vec<(String, PathBuf)>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    let mut sorted: Vec<fs::DirEntry> = entries.filter_map(Result::ok).collect();
    sorted.sort_by_key(std::fs::DirEntry::file_name);
    for entry in sorted {
        let Ok(kind) = entry.file_type() else {
            continue;
        };
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        if kind.is_dir() {
            if !SKIP_DIRS.contains(&name.as_str()) {
                walk_into(root, &path, out);
            }
            continue;
        }
        let relative = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        if !SKIP_PREFIXES.iter().any(|prefix| under_one(&relative, prefix)) {
            out.push((relative, path));
        }
    }
}

/// Does this relative path sit under this prefix, as a file or a directory?
fn under_one(relative: &str, prefix: &str) -> bool {
    let trimmed = prefix.trim_end_matches('/');
    relative == trimmed || relative.starts_with(&format!("{trimmed}/"))
}

/// Does a repository-relative path sit under one of these prefixes?
fn under_any(relative: &str, prefixes: &[String]) -> bool {
    prefixes.iter().any(|prefix| under_one(relative, prefix))
}

/// Measure every concept against the tree.
fn measure(root: &Path, map: &Map) -> Result<(BTreeMap<String, Counts>, usize), String> {
    let files = collect_files(root);
    let mut counts: BTreeMap<String, Counts> = BTreeMap::new();
    let mut scanned = 0usize;
    for (relative, path) in files {
        if under_any(&relative, &map.global_allow_paths) {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        scanned += 1;
        let raw_lines: Vec<&str> = text.split('\n').collect();
        let lower_lines: Vec<String> = raw_lines.iter().map(|line| lowercase(line)).collect();
        for concept in &map.concepts {
            if !concept.paths.is_empty() && !under_any(&relative, &concept.paths) {
                continue;
            }
            if under_any(&relative, &concept.allow_paths) {
                continue;
            }
            let mut hit_lines = 0usize;
            for (index, raw) in raw_lines.iter().enumerate() {
                if let Some(needles) = &concept.needles {
                    let present = needles.iter().any(|needle| {
                        needle_present(needle, lower_lines[index].as_str(), raw)
                    });
                    if !present {
                        continue;
                    }
                }
                let chars: Vec<char> = raw.chars().collect();
                if concept.patterns.iter().any(|pattern| pattern_matches(pattern, &chars)) {
                    hit_lines += 1;
                }
            }
            if hit_lines > 0 {
                let entry = counts.entry(concept.slug.clone()).or_default();
                entry.files += 1;
                entry.lines += hit_lines;
            }
        }
    }
    if scanned < VACUITY_FLOOR {
        return Err(format!(
            "only {scanned} text files scanned under {}; the gate would be vacuous",
            root.display()
        ));
    }
    Ok((counts, scanned))
}

/// Parse the recorded counts.
fn parse_baseline(text: &str) -> Result<BTreeMap<String, Counts>, String> {
    let mut out: BTreeMap<String, Counts> = BTreeMap::new();
    for (line_number, raw) in text.lines().enumerate() {
        let line = strip_comment(raw).trim();
        if line.is_empty() {
            continue;
        }
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() != 3 {
            return Err(format!(
                "{BASELINE_PATH}: line {} is not '<key> <files> <lines>'",
                line_number + 1
            ));
        }
        let files = fields[1].parse::<usize>().map_err(|_| {
            format!(
                "{BASELINE_PATH}: line {} has a file count that is not a number",
                line_number + 1
            )
        })?;
        let lines = fields[2].parse::<usize>().map_err(|_| {
            format!(
                "{BASELINE_PATH}: line {} has a line count that is not a number",
                line_number + 1
            )
        })?;
        let key = fields[0].to_string();
        if out.insert(key.clone(), Counts { files, lines }).is_some() {
            return Err(format!("{BASELINE_PATH}: '{key}' is recorded twice"));
        }
    }
    Ok(out)
}

/// Compare the measurement with the recorded counts.
fn judge_count(
    map: &Map,
    counts: &BTreeMap<String, Counts>,
    baseline: &BTreeMap<String, Counts>,
) -> Result<String, String> {
    let mut regressions: Vec<String> = Vec::new();
    let mut unrecorded: Vec<String> = Vec::new();
    let mut recorded_total = 0usize;
    let mut measured_total = 0usize;
    for concept in &map.concepts {
        let measured = counts.get(&concept.slug).copied().unwrap_or_default();
        measured_total += measured.lines;
        let recorded = baseline.get(&concept.slug).copied().ok_or_else(|| {
            format!(
                "{BASELINE_PATH}: '{}' has no record; add '{slug} {files} {lines}'",
                concept.new_code,
                slug = concept.slug,
                files = measured.files,
                lines = measured.lines
            )
        })?;
        recorded_total += recorded.lines;
        if measured.files > recorded.files || measured.lines > recorded.lines {
            regressions.push(format!(
                "  '{}' (owner {}) is at {} file(s)/{} line(s); recorded {} file(s)/{} line(s)",
                concept.old[0],
                concept.owner,
                measured.files,
                measured.lines,
                recorded.files,
                recorded.lines
            ));
        } else if measured.files < recorded.files || measured.lines < recorded.lines {
            unrecorded.push(format!(
                "  '{slug}' fell to {files} file(s)/{lines} line(s); record it",
                slug = concept.slug,
                files = measured.files,
                lines = measured.lines
            ));
        }
    }
    for key in baseline.keys() {
        if !map.concepts.iter().any(|concept| concept.slug == *key) {
            return Err(format!(
                "{BASELINE_PATH}: '{key}' is recorded but no concept in {MAP_PATH} owns it"
            ));
        }
    }
    if !regressions.is_empty() {
        let mut msg = format!(
            "{} retired concept(s) gained the name they are renaming away from:\n",
            regressions.len()
        );
        for finding in regressions.iter().take(MAX_REPORTED) {
            let _ = writeln!(msg, "{finding}");
        }
        msg.push_str(
            "  Rename the new occurrences with the term in the map; the count only falls.",
        );
        return Err(msg);
    }
    if !unrecorded.is_empty() {
        let mut msg = format!(
            "{} concept(s) fell below the recorded count; lower {BASELINE_PATH} in this change:\n",
            unrecorded.len()
        );
        for finding in unrecorded.iter().take(MAX_REPORTED) {
            let _ = writeln!(msg, "{finding}");
        }
        return Err(msg);
    }
    Ok(format!(
        "Retired concept names are at the recorded level: {measured_total} line(s) across {} record(s), against a recorded {recorded_total}.",
        map.concepts.len()
    ))
}

/// Judge a map in `absolute` mode, where no hit is acceptable.
fn judge_absolute(map: &Map, counts: &BTreeMap<String, Counts>) -> Result<String, String> {
    let hits: Vec<String> = map
        .concepts
        .iter()
        .filter_map(|concept| {
            let measured = counts.get(&concept.slug).copied().unwrap_or_default();
            if measured.lines == 0 {
                return None;
            }
            Some(format!(
                "  '{}' (owner {}) is in {} file(s)/{} line(s)",
                concept.old[0], concept.owner, measured.files, measured.lines
            ))
        })
        .collect();
    if hits.is_empty() {
        return Ok(String::from(
            "No retired concept name appears anywhere in the scanned roots.",
        ));
    }
    let mut msg = format!(
        "the map is in absolute mode and {} retired concept(s) still appear:\n",
        hits.len()
    );
    for finding in hits.iter().take(MAX_REPORTED) {
        let _ = writeln!(msg, "{finding}");
    }
    Err(msg)
}

/// Run the gate against a named map and a named baseline.
fn run_with(root: &Path, map_path: &str, baseline_path: &str) -> Result<String, String> {
    let map_text = fs::read_to_string(root.join(map_path))
        .map_err(|e| format!("cannot read {map_path}: {e}"))?;
    let map = parse_map(&map_text)?;
    let (counts, scanned) = measure(root, &map)?;
    let verdict = match map.mode {
        Mode::Absolute => judge_absolute(&map, &counts)?,
        Mode::Count => {
            let baseline_text = fs::read_to_string(root.join(baseline_path))
                .map_err(|e| format!("cannot read {baseline_path}: {e}"))?;
            let baseline = parse_baseline(&baseline_text)?;
            judge_count(&map, &counts, &baseline)?
        }
    };
    Ok(format!("{verdict} ({scanned} files scanned)"))
}

/// # Errors
///
/// Returns a finding when a retired name rose above its recorded count, when a
/// fall is not recorded, when a pattern is outside the subset the gate reads,
/// or when the scan would be vacuous.
pub fn run(root: &Path) -> Result<String, String> {
    run_with(root, MAP_PATH, BASELINE_PATH)
}

/// Stage a fixture tree with sixty files, so the vacuity floor is not in the way.
fn build_tree(root: &Path) -> std::io::Result<()> {
    let src = root.join("src");
    fs::create_dir_all(&src)?;
    for i in 1..=59 {
        fs::write(
            src.join(format!("file{i}.rs")),
            format!("fn f{i}() -> u8 {{ {i} }}\n"),
        )?;
    }
    fs::write(src.join("lib.rs"), "pub fn nothing() {}\n")?;
    Ok(())
}

/// The map a canary runs against, with one path-restricted concept.
const FIXTURE_MAP: &str = "schema = 1\nmode = \"count\"\nglobal_allow_paths = [\"allowed/\"]\n\n\
    [[concept]]\nold = [\"fixture\"]\npatterns = ['(?i)fixture']\npaths = [\"src/\"]\n\
    new_code = \"fixture_term\"\nterm_en = \"fixture\"\nterm_tr = \"fixture\"\n\
    area = \"test\"\nowner = \"WP-05\"\nreason = \"A canary.\"\n";

/// Write the canary map and an empty baseline into a staged tree.
fn stage(canary: &Path) -> Result<(), String> {
    build_tree(canary).map_err(|e| format!("cannot build the fixture tree: {e}"))?;
    fs::write(canary.join("map.toml"), FIXTURE_MAP)
        .map_err(|e| format!("cannot write the fixture map: {e}"))?;
    fs::write(canary.join("baseline.txt"), "fixture-term 0 0\n")
        .map_err(|e| format!("cannot write the fixture baseline: {e}"))?;
    Ok(())
}

/// The page a canary writes into the staged tree.
fn page(canary: &Path, body: &str) -> Result<(), String> {
    fs::write(canary.join("src/file1.rs"), body).map_err(|e| format!("cannot write a fixture: {e}"))
}

/// # Errors
///
/// Returns the first canary that misbehaves: a clean tree that is refused, an
/// injected old name that passes, a fall that is not demanded, a hit that
/// absolute mode accepts, a nested group that is read as a pattern, or a
/// near-empty tree that passes.
pub fn self_test() -> Result<String, String> {
    let scratch = crate::gates::rust_literals::exclusive_scratch_dir("budlum-gates-concept-names")?;
    let clean = scratch.join("clean");
    stage(&clean)?;
    if let Err(msg) = run_with(&clean, "map.toml", "baseline.txt") {
        let _ = fs::remove_dir_all(&scratch);
        return Err(format!("canary: a clean tree was refused: {msg}"));
    }
    page(&clean, "fn f1() { let fixture = 1; }\n")?;
    if run_with(&clean, "map.toml", "baseline.txt").is_ok() {
        let _ = fs::remove_dir_all(&scratch);
        return Err(String::from("canary: an injected old name was not detected"));
    }
    page(&clean, "fn f1() {}\n")?;
    fs::write(clean.join("baseline.txt"), "fixture-term 1 1\n")
        .map_err(|e| format!("cannot raise the fixture baseline: {e}"))?;
    if run_with(&clean, "map.toml", "baseline.txt").is_ok() {
        let _ = fs::remove_dir_all(&scratch);
        return Err(String::from(
            "canary: a fall below the recorded count was accepted",
        ));
    }
    fs::write(clean.join("baseline.txt"), "fixture-term 0 0\n")
        .map_err(|e| format!("cannot reset the fixture baseline: {e}"))?;
    let absolute = FIXTURE_MAP.replace("mode = \"count\"", "mode = \"absolute\"");
    fs::write(clean.join("absolute.toml"), absolute)
        .map_err(|e| format!("cannot write the absolute fixture: {e}"))?;
    page(&clean, "fn f1() { let fixture = 1; }\n")?;
    if run_with(&clean, "absolute.toml", "baseline.txt").is_ok() {
        let _ = fs::remove_dir_all(&scratch);
        return Err(String::from("canary: absolute mode accepted a hit"));
    }
    page(&clean, "fn f1() {}\n")?;
    let broken = FIXTURE_MAP.replace("'(?i)fixture'", "'(?i)(fixture|nest(ed))'");
    fs::write(clean.join("broken.toml"), broken)
        .map_err(|e| format!("cannot write the broken fixture: {e}"))?;
    if run_with(&clean, "broken.toml", "baseline.txt").is_ok() {
        let _ = fs::remove_dir_all(&scratch);
        return Err(String::from(
            "canary: a nested group was accepted as a pattern",
        ));
    }
    let empty = scratch.join("empty");
    fs::create_dir_all(&empty).map_err(|e| format!("cannot create the empty tree: {e}"))?;
    fs::write(empty.join("map.toml"), FIXTURE_MAP)
        .map_err(|e| format!("cannot write the empty fixture map: {e}"))?;
    fs::write(empty.join("baseline.txt"), "fixture-term 0 0\n")
        .map_err(|e| format!("cannot write the empty fixture baseline: {e}"))?;
    if run_with(&empty, "map.toml", "baseline.txt").is_ok() {
        let _ = fs::remove_dir_all(&scratch);
        return Err(String::from(
            "canary: a near-empty tree passed; the vacuity floor is not working",
        ));
    }
    let _ = fs::remove_dir_all(&scratch);
    Ok(String::from(
        "Self-test OK: a clean tree passes, an injected name fails, a fall must be recorded, absolute mode refuses every hit, a nested group is refused, and the vacuity floor fires.",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The repository root, for the canaries that read the real map.
    fn repo() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
    }

    /// A scratch directory for one test.
    fn scratch() -> PathBuf {
        crate::gates::rust_literals::exclusive_scratch_dir("budlum-gates-concept-names-test")
            .expect("a scratch dir")
    }

    #[test]
    fn concept_names_gate_catches_injected_name() {
        let dir = scratch();
        build_tree(&dir).expect("the fixture tree");
        fs::write(dir.join("map.toml"), FIXTURE_MAP).expect("the fixture map");
        fs::write(dir.join("baseline.txt"), "fixture-term 0 0\n").expect("the fixture baseline");
        assert!(run_with(&dir, "map.toml", "baseline.txt").is_ok());
        fs::write(dir.join("src/lib.rs"), "pub fn fixture() {}\n").expect("the injected name");
        let finding = run_with(&dir, "map.toml", "baseline.txt").expect_err("the name is caught");
        assert!(finding.contains("fixture"), "{finding}");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn concept_names_baseline_only_decreases() {
        let dir = scratch();
        build_tree(&dir).expect("the fixture tree");
        fs::write(dir.join("map.toml"), FIXTURE_MAP).expect("the fixture map");
        fs::write(dir.join("src/lib.rs"), "pub fn fixture() {}\n").expect("the hit");
        fs::write(dir.join("baseline.txt"), "fixture-term 1 1\n").expect("the fixture baseline");
        assert!(
            run_with(&dir, "map.toml", "baseline.txt").is_ok(),
            "a count at the recorded level must pass"
        );
        fs::write(dir.join("src/lib.rs"), "pub fn clear() {}\n").expect("the cleared hit");
        let finding = run_with(&dir, "map.toml", "baseline.txt").expect_err("the fall is demanded");
        assert!(finding.contains("fell"), "{finding}");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn concept_map_owner_is_a_known_package() {
        let text = fs::read_to_string(repo().join(MAP_PATH)).expect("the map is in the tree");
        let map = parse_map(&text).expect("the map parses");
        assert!(map.concepts.len() > 20, "the map has {}", map.concepts.len());
        for concept in &map.concepts {
            let number = concept.owner.strip_prefix("WP-").unwrap_or_else(|| {
                panic!("'{}' has no owner package", concept.new_code)
            });
            let parsed = number.parse::<u8>().unwrap_or_else(|_| {
                panic!("'{}' has a malformed owner", concept.new_code)
            });
            assert!(
                (1..=64).contains(&parsed),
                "'{}' is owned by {}",
                concept.new_code,
                concept.owner
            );
        }
    }

    #[test]
    fn the_matcher_reads_the_map_subset() {
        let cases: [(&str, &str, bool); 8] = [
            ("(?i)fixture", "a FIXTURE word", true),
            ("(?i)fixture", "unrelated", false),
            (r"\bDon:", "Don: x", true),
            (r"\bDon:", "xDon: y", false),
            ("Atlas[A-Z]", "AtlasX", true),
            ("Atlas[A-Z]", "atlasX", false),
            ("(?i)(^|[^a-z])probe($|[^a-z])", "a PROBE b", true),
            ("(?i)(^|[^a-z])probe($|[^a-z])", "aprobeb", false),
        ];
        for (pattern, line, expected) in cases {
            let compiled = parse_pattern(pattern).expect("a supported pattern");
            let chars: Vec<char> = line.chars().collect();
            assert_eq!(
                pattern_matches(&compiled, &chars),
                expected,
                "{pattern} against {line}"
            );
        }
    }

    #[test]
    fn the_slug_folds_to_lowercase_words() {
        assert_eq!(slug_of("hash_chain_election"), "hash-chain-election");
        assert_eq!(slug_of("(removed)"), "removed");
        assert_eq!(slug_of("Shared::Security"), "shared-security");
    }
}
