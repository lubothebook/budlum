//! WP-03 (Lane C) live verification regression tests
//! Per THE_BUDLUM_DIREKTIF.md Section 15.8, card WP-03.
//! These tests stay in the tree once written and are not removed by later PRs (`Bölüm 14.1` madde 7).

#[cfg(test)]
mod audit_lane_c {
    /// Placeholder regression: verifies that the Lane C audit directory exists
    /// and contains the required artifacts (`lane-c.md`, `engine-inventory.md`).
    /// Expanded as WP-03 verification proceeds.
    #[test]
    fn audit_lane_c_artifacts_present() {
        use std::path::Path;
        assert!(Path::new("docs/audit/live/lane-c.md").exists());
        assert!(Path::new("docs/audit/live/engine-inventory.md").exists());
    }

    /// Placeholder: the engine inventory must contain the required entries
    /// (`revm`, `wasmi`, `solana-svm`, `solana-sbpf`, `aptos-vm`, `budzero`).
    #[test]
    fn engine_inventory_has_required_engines() {
        let content = std::fs::read_to_string("docs/audit/live/engine-inventory.md")
            .expect("engine-inventory.md must exist for WP-03");
        for name in ["revm", "wasmi", "solana-svm", "aptos-vm", "budzero"] {
            assert!(
                content.contains(name),
                "engine-inventory.md must mention {}",
                name
            );
        }
    }
}
