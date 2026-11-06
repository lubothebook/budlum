//! WP-04 live verification locks for lane D.
//!
//! These tests cover fixes already present on the verification base. They do
//! not close a finding by themselves: the merge commit and a green CI run are
//! the closure evidence. Open findings and their owed reproductions are listed
//! in `docs/audit/live/lane-d.md`. No test here changes production behavior.

use crate::core::address::Address;
use crate::core::chain_config::Network;
use crate::light_client::{LightClient, TrustedCheckpoint};

fn read_repo(path: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()))
}

/// BLG-004: replacing a readable key file must replace its permissions too.
#[cfg(unix)]
#[test]
fn audit_lane_d_blg_004_key_rewrite_is_owner_only() {
    use std::os::unix::fs::PermissionsExt;

    let dir = tempfile::tempdir().expect("audit scratch directory");
    let path = dir.path().join("validator.key");
    std::fs::write(&path, b"old key material").expect("write old key");
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644))
        .expect("set the insecure predecessor mode");

    let key = crate::crypto::primitives::KeyPair::generate().expect("generate audit key");
    key.save(&path).expect("replace key atomically");

    let mode = std::fs::metadata(&path)
        .expect("read replacement metadata")
        .permissions()
        .mode()
        & 0o777;
    assert_eq!(mode, 0o600, "a replacement key must not retain mode 0644");
    assert!(!path.with_file_name("validator.key.tmp").exists());
    crate::crypto::primitives::KeyPair::load(&path).expect("replacement key must load");
}

/// BLG-022: checkpoint selection must handle the top of the height domain
/// without iterating through every intermediate height.
#[test]
fn audit_lane_d_blg_022_checkpoint_selection_handles_boundaries() {
    let client = LightClient::new(TrustedCheckpoint {
        height: u64::MAX - 2,
        block_hash: "ab".repeat(32),
        set_hash: "cd".repeat(32),
    })
    .expect("valid trust anchor");

    assert_eq!(client.next_checkpoint_height(0, u64::MAX), None);
    assert_eq!(
        client.next_checkpoint_height(2, u64::MAX),
        Some(u64::MAX - 1)
    );
    assert_eq!(
        client.next_checkpoint_height(3, u64::MAX),
        Some(u64::MAX)
    );
    assert_eq!(
        client.next_checkpoint_height(u64::MAX, u64::MAX),
        Some(u64::MAX)
    );
    assert_eq!(client.next_checkpoint_height(2, u64::MAX - 2), None);
}

/// BLG-030, testnet leg: the development credit doors are not the public
/// testnet faucet. Both must refuse without changing the account balance.
#[test]
fn audit_lane_d_blg_030_testnet_development_credit_is_closed() {
    use crate::chain::blockchain::Blockchain;
    use crate::consensus::pow::PoWEngine;
    use std::sync::Arc;

    let mut chain = Blockchain::new(
        Arc::new(PoWEngine::new(0)),
        None,
        Network::Testnet.chain_id().value(),
        None,
    );
    let target = Address::from([0x4d; 32]);
    let before = chain.state.get_balance(&target);

    let error = chain
        .fund_development_account(&target)
        .expect_err("testnet must refuse the development faucet");
    assert!(error.contains("not the devnet chain id"), "{error}");
    assert_eq!(chain.state.get_balance(&target), before);

    let error = chain
        .credit_development_account(&target, 1_000)
        .expect_err("testnet must refuse an arbitrary development credit");
    assert!(error.contains("devnet"), "{error}");
    assert_eq!(chain.state.get_balance(&target), before);
}

/// BLG-099: a length mismatch is a refusal, including empty inputs. This is
/// a correctness lock; the timing claim remains with the timing-safe CI job.
#[test]
fn audit_lane_d_blg_099_secret_comparison_refuses_length_mismatch() {
    use crate::rpc::server::constant_time_eq_str as equal;

    let expected = "a-secret-of-fixed-length";
    assert!(equal(expected, expected));
    for length in 0..expected.len() {
        assert!(!equal(&expected[..length], expected), "length {length}");
    }
    assert!(!equal(&format!("{expected}x"), expected));
    assert!(!equal("", expected));
    assert!(!equal(expected, ""));
    assert!(equal("", ""));
}

/// BULGU-012: an encoding failure must not become a successful null answer.
#[test]
fn audit_lane_d_bulgu_012_serialization_failure_is_explicit() {
    let mut value = std::collections::BTreeMap::new();
    value.insert((1u8, 2u8), 3u8);
    let error = crate::rpc::server::to_json("audit object", &value)
        .expect_err("a non-string map key cannot be a JSON object key");
    assert_eq!(error.code(), -32603);
    assert!(error.message().contains("audit object serialization failed"));
    assert_eq!(
        crate::rpc::server::to_json("audit object", &7u8).expect("integer encodes"),
        serde_json::json!(7)
    );
}

/// BLG-011: the shipped unit supplies the credential environment and disables
/// core dumps. The separate repository/HSM documentation finding stays open.
#[test]
fn audit_lane_d_blg_011_service_supplies_credentials_and_disables_dumps() {
    let unit = read_repo("ops/budlum-core.service");
    let settings: Vec<&str> = unit.lines().collect();
    let setting = |name: &str| {
        settings.iter()
            .find_map(|line| line.trim().strip_prefix(&format!("{name}=")))
            .unwrap_or_else(|| panic!("service has no {name} setting"))
    };
    assert_eq!(setting("LimitCORE"), "0");
    assert_eq!(setting("EnvironmentFile"), "/etc/budlum/rpc.env");
    // systemd permits repeated Environment settings. The first one carries
    // RUST_LOG, so select the credential declaration across all such lines.
    let key_env = settings
        .iter()
        .filter_map(|line| line.trim().strip_prefix("Environment="))
        .find_map(|value| value.strip_prefix("BUDLUM_RPC_API_KEY_ENV="));
    assert_eq!(key_env, Some("BUDLUM_RPC_API_KEY"));
}

/// DOC-GAP-07: an auto-discovered binary is not enough. Its explicit manifest
/// entry and its CI test command must both remain present.
#[test]
fn audit_lane_d_doc_gap_07_indexer_is_registered_and_tested() {
    let manifest = read_repo("Cargo.toml");
    let registration = manifest
        .split("[[bin]]")
        .find(|block| block.contains("name = \"budlum-indexer\""))
        .expect("explicit indexer binary registration");
    assert!(registration.contains("path = \"src/bin/budlum-indexer.rs\""));
    let ci = read_repo(".github/workflows/ci.yml");
    assert!(
        ci.lines().any(|line| {
            let line = line.trim_start();
            line.starts_with("cargo test ")
                && line.contains("--bin budlum-indexer")
                && !line.contains("--no-run")
        }),
        "the library test job must execute the indexer binary's tests"
    );
}

/// The ledger must cover every lane D identity exactly once. This is a
/// coverage lock for the record, not a claim that every finding is closed.
#[test]
fn audit_lane_d_ledger_covers_the_assigned_identities() {
    let ledger = read_repo("docs/audit/live/lane-d.md");
    let findings = ledger
        .split("## Findings\n")
        .nth(1)
        .and_then(|rest| rest.split("\n## ").next())
        .expect("findings section");
    let ids: Vec<&str> = findings
        .lines()
        .filter_map(|line| line.strip_prefix("| "))
        .filter_map(|rest| rest.split(" |").next())
        .filter(|id| {
            id.starts_with("BLG-") || id.starts_with("BULGU-")
                || id.starts_with("TEST-GAP-") || id.starts_with("DOC-GAP-")
        })
        .collect();
    assert_eq!(ids.len(), 70, "the assigned lane D record count changed");
    let unique: std::collections::BTreeSet<_> = ids.iter().copied().collect();
    assert_eq!(unique.len(), ids.len(), "a lane D identity is duplicated");
    for id in ["BLG-004", "BLG-265", "BULGU-012", "TEST-GAP-07", "DOC-GAP-09"] {
        assert!(unique.contains(id), "missing assigned identity {id}");
    }
}

/// The CI baseline must account for every workflow file in this snapshot.
/// Missing and cancelled runs are not silently counted as passes.
#[test]
fn audit_lane_d_ci_baseline_covers_every_workflow() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let baseline = read_repo("docs/audit/live/ci-baseline.md");
    let mut count = 0usize;
    let entries = std::fs::read_dir(root.join(".github/workflows"))
        .expect("workflow directory must exist");
    for entry in entries {
        let entry = entry.expect("workflow directory entry");
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.ends_with(".yml") || name.ends_with(".yaml") {
            count += 1;
            assert!(
                baseline.contains(&format!("`.github/workflows/{name}`")),
                "CI baseline omits {name}"
            );
        }
    }
    assert_eq!(count, 30, "workflow inventory changed; refresh the live baseline");
    assert!(baseline.contains("no run` is not a pass") || baseline.contains("no run. It is not a pass"));
    assert!(baseline.contains("42050659038"));
}
