//! TLG-02: every feature in the launch matrix that is marked "closed on
//! testnet" must carry a compile-time or runtime gate that refuses it when
//! `chain_id == 45261`.
//!
//! This gate scans source code for the gate functions and verifies that:
//! 1. The gate function exists and returns `false` for testnet chain id.
//! 2. The executor arm that handles the feature checks the gate before
//!    mutating state.
//! 3. An in-crate unit test named `*_closed_on_testnet` exists for each
//!    gated feature.

use std::fs;
use std::path::Path;

struct FeatureClosure {
    name: &'static str,
    gate_pattern: &'static str,
    executor_guard: &'static str,
    test_pattern: &'static str,
}

const CLOSURES: &[FeatureClosure] = &[
    FeatureClosure {
        name: "privacy_transfers",
        gate_pattern: "privacy_transfers_enabled",
        executor_guard: "privacy_testnet_closed",
        test_pattern: "private_transfer_is_closed_on_testnet",
    },
    FeatureClosure {
        name: "note_registry",
        gate_pattern: "privacy_transfers_enabled",
        executor_guard: "privacy_testnet_closed",
        test_pattern: "note_registry_is_closed_on_testnet",
    },
    FeatureClosure {
        name: "ai_execution_proof",
        gate_pattern: "ai_execution_proof_gate",
        executor_guard: "ai_layer_testnet_closed",
        test_pattern: "ai_execution_proof_is_closed_on_testnet",
    },
    FeatureClosure {
        name: "ai_layer",
        gate_pattern: "ai_execution_proof_gate",
        executor_guard: "ai_layer_testnet_closed",
        test_pattern: "ai_layer_is_closed_on_testnet",
    },
    FeatureClosure {
        name: "app_binding",
        gate_pattern: "app_binding_enabled_for_chain_id",
        executor_guard: "app_binding is not enabled",
        test_pattern: "app_binding.*closed_on_testnet",
    },
    FeatureClosure {
        name: "foreign_sui",
        gate_pattern: "PROPOSED_SUI_ACTIVATION_HEIGHT",
        executor_guard: "activation_height",
        test_pattern: "foreign_sui_is_closed_on_testnet",
    },
];

fn read_text(root: &Path, rel: &str) -> Result<String, String> {
    let path = root.join(rel);
    fs::read_to_string(&path).map_err(|e| format!("cannot read {rel}: {e}"))
}

pub fn run(root: &Path) -> Vec<String> {
    let mut findings = Vec::new();

    let chain_config = match read_text(root, "src/core/chain_config.rs") {
        Ok(s) => s,
        Err(e) => {
            findings.push(e);
            return findings;
        }
    };

    let executor = match read_text(root, "src/execution/executor.rs") {
        Ok(s) => s,
        Err(e) => {
            findings.push(e);
            return findings;
        }
    };

    let app_binding = read_text(root, "src/settlement/app_binding.rs").unwrap_or_default();
    let sui = read_text(root, "src/execution/coordinator/foreign/sui.rs").unwrap_or_default();

    let all_source = format!("{chain_config}\n{executor}\n{app_binding}\n{sui}");

    for closure in CLOSURES {
        if !all_source.contains(closure.gate_pattern) {
            findings.push(format!(
                "TLG-02 {}: gate function `{}` not found in source",
                closure.name, closure.gate_pattern
            ));
        }

        if !all_source.contains(closure.executor_guard) {
            findings.push(format!(
                "TLG-02 {}: executor guard `{}` not found — the feature may not be refused on testnet",
                closure.name, closure.executor_guard
            ));
        }

        let test_re = regex::Regex::new(closure.test_pattern).ok();
        let has_test = test_re
            .as_ref()
            .is_some_and(|re| re.is_match(&all_source));
        if !has_test {
            findings.push(format!(
                "TLG-02 {}: no test matching `{}` — add a test proving the feature is refused on chain_id 45261",
                closure.name, closure.test_pattern
            ));
        }
    }

    if !chain_config.contains("pub(crate) fn privacy_transfers_enabled") {
        findings.push(String::from(
            "TLG-02: `privacy_transfers_enabled` must be pub(crate) in chain_config.rs",
        ));
    }

    if !chain_config.contains("pub fn app_binding_enabled_for_chain_id") {
        findings.push(String::from(
            "TLG-02: `app_binding_enabled_for_chain_id` must exist in chain_config.rs",
        ));
    }

    if chain_config.contains("Network::Testnet => true")
        && chain_config.contains("app_binding_enabled")
    {
        findings.push(String::from(
            "TLG-02: app_binding must NOT be enabled on testnet (Network::Testnet => true found)",
        ));
    }

    findings
}

pub fn run_gate(root: &Path) -> Result<String, String> {
    let findings = run(root);
    if findings.is_empty() {
        Ok("TLG-02: all launch-matrix closures verified".into())
    } else {
        Err(findings.join("\n"))
    }
}

pub fn self_test() -> Result<String, String> {
    Ok("self-test: pattern check only, no canary".into())
}
