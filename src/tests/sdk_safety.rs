//! SDK module safety tests.

use crate::sdk::BudlumToml;

#[test]
fn budlum_toml_type_exists() {
    let _ = std::mem::size_of::<BudlumToml>();
}

#[test]
fn budlum_toml_serde() {
    let toml_str = r#"
[project]
name = "test-app"
version = "0.1.0"
"#;
    let parsed: Result<BudlumToml, _> = toml::from_str(toml_str);
    assert!(parsed.is_ok(), "BudlumToml should parse valid TOML");
}
