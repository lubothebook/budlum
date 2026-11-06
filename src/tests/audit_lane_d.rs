//! Live-verification regression locks for the lane D findings of the unified
//! findings ledger (directive section 14, package WP-04). Each lock names the
//! finding it keeps closed (`BLG-NNN` / archive id) and cites the evidence it
//! relies on. A lock that fails means the tree regressed below the recorded
//! state, not that the closing package did its work; the closing package moves
//! any lock it tightens into its own pull request.
//!
//! Statuses recorded in `docs/audit/live/lane-d.md`:
//! - `Closed (pre-fixed)` ... findings closed in this snapshot, held closed by
//!   these locks and by the older locks in `regression_ops_crypto.rs`.
//! - `Open` / `Confirmed` ... the finding still reproduces in the live tree.
//!   The red repro test for each is owed on the `audit/repro-d` branch and
//!   moves into the closing package's pull request (directive 14.1, S-3).
//!
//! The two ledger-integrity locks at the bottom tie the markdown ledger to the
//! tree: every `Closed (pre-fixed)` row must cite a lock that exists, and the
//! summary counts must match the rows. The ledger is English and ASCII.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_repo(rel: &str) -> String {
    std::fs::read_to_string(repo_root().join(rel))
        .unwrap_or_else(|e| panic!("cannot read {rel}: {e}"))
}

/// Source of a production file with its `#[cfg(test)]` tail cut off.
fn production_of(src: &str) -> &str {
    src.split("#[cfg(test)]").next().unwrap_or(src)
}

/// BLG-004: rewriting a key over an existing wide-mode file narrows it to
/// 0600. The new sibling is created with `create_new` and mode 0o600 and
/// atomically renamed, so the old file's mode is never kept.
#[cfg(unix)]
#[test]
fn blg_004_key_rewrite_narrows_permissions() {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("lane-d-key.bin");
    std::fs::write(&path, b"old-wide-mode").unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();
    let kp = crate::crypto::primitives::KeyPair::generate().unwrap();
    kp.save(&path).unwrap();
    let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
    assert_eq!(mode, 0o600, "a rewritten key file must be owner-only");
    // The atomic rename leaves no sibling behind.
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
}

/// BLG-022: the next-checkpoint query is a closed form. A huge interval and a
/// `u64::MAX` bound neither loop nor overflow.
#[test]
fn blg_022_next_checkpoint_closed_form_edge() {
    use crate::light_client::{LightClient, TrustedCheckpoint};
    let anchor = |height: u64| {
        LightClient::new(TrustedCheckpoint {
            height,
            block_hash: "0".repeat(64),
            set_hash: "0".repeat(64),
        })
        .unwrap()
    };
    assert_eq!(anchor(0).next_checkpoint_height(10, 50), Some(10));
    assert_eq!(
        anchor(0).next_checkpoint_height(u64::MAX, u64::MAX),
        Some(u64::MAX)
    );
    assert_eq!(
        anchor(u64::MAX - 1).next_checkpoint_height(u64::MAX, u64::MAX),
        Some(u64::MAX)
    );
    assert_eq!(anchor(u64::MAX).next_checkpoint_height(10, u64::MAX), None);
    assert_eq!(anchor(5).next_checkpoint_height(0, 100), None);
    assert_eq!(anchor(20).next_checkpoint_height(10, 25), None);
}

/// BLG-098: the clap-skipped RPC fields are restored to the secure defaults
/// before any file or environment overlay, so `#[arg(skip)]` cannot silently
/// turn authentication off or empty the IP allow-list.
#[test]
fn blg_098_secure_rpc_defaults_survive_clap() {
    let cfg = crate::cli::commands::NodeConfig::default();
    assert!(cfg.rpc_auth_required, "RPC auth must default on");
    assert_eq!(
        cfg.rpc_allowed_ips,
        vec!["127.0.0.1".to_string(), "::1".to_string()],
        "the allow-list must default to loopback"
    );
    let src = read_repo("src/cli/commands.rs");
    let load = src
        .split("pub fn load_with_file(&mut self) {")
        .nth(1)
        .expect("load_with_file must exist");
    let head = &load[..load.find("if let Some(path)").unwrap_or(load.len())];
    assert!(
        head.contains("self.apply_secure_rpc_defaults();"),
        "secure defaults must be applied before the file overlay"
    );
}

/// BLG-099: the API-key comparison walks the expected value in full and folds
/// the length difference into the accumulator. Equal keys match; any
/// difference, a short candidate, a long candidate and an empty candidate all
/// refuse. There is no self-comparison on the mismatch path.
#[test]
fn blg_099_constant_time_eq_no_length_leak() {
    use crate::rpc::server::constant_time_eq_str as eq;
    assert!(eq("secret-key", "secret-key"));
    assert!(!eq("secret-kex", "secret-key"));
    assert!(!eq("secret", "secret-key"));
    assert!(!eq("secret-key-and-more", "secret-key"));
    assert!(!eq("", "secret-key"));
    assert!(eq("", ""));
    let src = read_repo("src/rpc/server.rs");
    let body = src
        .split("pub fn constant_time_eq_str")
        .nth(1)
        .expect("constant_time_eq_str must exist");
    let body = &body[..body.find("\n}\n").unwrap_or(body.len())];
    assert!(!body.contains("a_b.ct_eq(a_b)"), "no self-comparison");
    assert!(!body.contains("b_b.ct_eq(b_b)"), "no self-comparison");
    assert!(body.contains("black_box"), "the accumulator must be opaque");
}

/// BLG-200: `BlsKeypair::from_bytes` refuses the zero secret and a malformed
/// encoding, and `from_seed` refuses a seed that reduces to the zero scalar.
#[test]
fn blg_200_bls_zero_secret_and_identity_refused() {
    use crate::crypto::primitives::BlsKeypair;
    assert!(BlsKeypair::from_bytes(&[0u8; 32 + 96]).is_err());
    assert!(BlsKeypair::from_bytes(&[1u8; 5]).is_err());
    assert!(
        BlsKeypair::from_seed(&[0u8; 64]).is_err(),
        "the all-zero seed reduces to the zero scalar and must be refused"
    );
    let finality = read_repo("src/chain/finality.rs");
    assert!(
        finality.contains("BLS public key is identity or not in the prime-order subgroup"),
        "finality must refuse an identity public key"
    );
}

/// BULGU-012: an RPC result that cannot be serialized is an explicit `-32603`
/// error naming what failed, never a silent `null` (which reads as "absent").
#[test]
fn bulgu_012_rpc_serialization_error_is_not_null() {
    let src = read_repo("src/rpc/server.rs");
    let prod = production_of(&src);
    let body = prod
        .split("pub(crate) fn to_json")
        .nth(1)
        .expect("to_json must exist");
    let body = &body[..body.find("\n}\n").unwrap_or(body.len())];
    assert!(body.contains("-32603"), "the error code must be -32603");
    assert!(
        body.contains("serialization failed"),
        "the error must name what failed"
    );
    assert!(
        !body.contains("Value::Null"),
        "a serialization failure must not fold into null"
    );
    // A normal value still encodes.
    let v = crate::rpc::server::to_json("probe", &serde_json::json!({"a": 1})).unwrap();
    assert_eq!(v["a"], 1);
}

/// DOC-GAP-07: the public-service indexer is registered as an explicit `[[bin]]`
/// so a build or a package listing shows it instead of relying on discovery.
#[test]
fn doc_gap_07_indexer_binary_is_registered() {
    let cargo = read_repo("Cargo.toml");
    let at = cargo
        .find("name = \"budlum-indexer\"")
        .expect("budlum-indexer must be an explicit bin");
    // The registration sits under a [[bin]] table and names the source path.
    let before = &cargo[..at];
    assert!(
        before.rfind("[[bin]]").is_some(),
        "the indexer entry must be a [[bin]] table"
    );
    let window = &cargo[at..(at + 200).min(cargo.len())];
    assert!(
        window.contains("path = \"src/bin/budlum-indexer.rs\""),
        "the indexer bin must name its source path"
    );
}

/// Ledger integrity: every `Closed (pre-fixed)` row in `lane-d.md` cites a
/// regression lock that exists as a `fn <name>` in the tree's test sources.
/// This ties the markdown ledger to the code so a closed claim cannot drift.
#[test]
fn lane_d_ledger_closed_rows_cite_present_locks() {
    let ledger = include_str!("../../docs/audit/live/lane-d.md");
    let sources = [
        include_str!("audit_lane_d.rs"),
        include_str!("regression_ops_crypto.rs"),
        include_str!("../rpc/tests.rs"),
    ]
    .join("\n");
    let mut closed = 0usize;
    for line in ledger.lines() {
        if !(line.starts_with("| BLG-")
            || line.starts_with("| BULGU-")
            || line.starts_with("| TEST-GAP-")
            || line.starts_with("| DOC-GAP-"))
        {
            continue;
        }
        let cells: Vec<&str> = line.split('|').map(str::trim).collect();
        // BLG table: id, severity, status, evidence, lock, closing, note.
        // Archive table: id, severity, status, lock, closing, note.
        let (status, lock) = if line.starts_with("| BLG-") {
            (cells[3], cells[5])
        } else {
            (cells[3], cells[4])
        };
        if status != "Closed (pre-fixed)" {
            continue;
        }
        closed += 1;
        let name = lock.trim_matches('`');
        assert!(
            !name.is_empty() && !name.starts_with('('),
            "closed row must cite a real lock, got {name}"
        );
        let needle = format!("fn {name}");
        assert!(
            sources.contains(&needle),
            "closed row cites lock {name} but no `fn {name}` exists in the test sources"
        );
    }
    assert!(closed >= 20, "expected the closed lane D rows, found {closed}");
}

/// Ledger integrity: the summary counts match the rows, and the ledger is
/// English and ASCII (the `tree_is_english` gate holds the same rule).
#[test]
fn lane_d_ledger_is_english_and_counts_match() {
    let ledger = include_str!("../../docs/audit/live/lane-d.md");
    assert!(
        ledger.is_ascii(),
        "the lane D ledger must be ASCII English"
    );
    let mut blg = 0usize;
    let mut closed = 0usize;
    let mut confirmed = 0usize;
    let mut open = 0usize;
    for line in ledger.lines() {
        if !line.starts_with("| BLG-") {
            continue;
        }
        blg += 1;
        let cells: Vec<&str> = line.split('|').map(str::trim).collect();
        match cells[3] {
            "Closed (pre-fixed)" => closed += 1,
            "Confirmed" => confirmed += 1,
            "Open" => open += 1,
            other => panic!("unexpected status {other}"),
        }
    }
    assert_eq!(blg, 56, "lane D carries 56 BLG rows");
    assert_eq!(closed + confirmed + open, blg);
    assert!(
        ledger.contains(&format!("| Lane D BLG records | {blg} |")),
        "the summary BLG count must match the rows"
    );
    assert!(
        ledger.contains(&format!("| Closed (pre-fixed) | {closed} |")),
        "the summary closed count must match the rows"
    );
    assert!(
        ledger.contains(&format!("| Confirmed | {confirmed} |")),
        "the summary confirmed count must match the rows"
    );
    assert!(
        ledger.contains(&format!("| Open | {open} |")),
        "the summary open count must match the rows"
    );
}
