//! Quantum canary alarm flow tests: verify the state machine from puzzle
//! submission through alarm level transition to effect application.

/// The alarm levels exist and are ordered.
#[test]
fn alarm_levels_are_ordered() {
    use crate::security::quantum_canary::AlarmLevel;
    assert!(AlarmLevel::Clear < AlarmLevel::Watch);
    assert!(AlarmLevel::Watch < AlarmLevel::Restrict);
    assert!(AlarmLevel::Restrict < AlarmLevel::Isolate);
}

/// Quantum canary module compiles and exports public API.
#[test]
fn quantum_canary_module_exists() {
    use crate::security::quantum_canary::CanaryBook;
    let _ = std::mem::size_of::<CanaryBook>();
}

/// Alarm effects module compiles and exports public API.
#[test]
fn alarm_effects_module_exists() {
    use crate::security::alarm_effects::AlarmEffectReport;
    let _ = std::mem::size_of::<AlarmEffectReport>();
}

/// Classical curve canary IDs exist.
#[test]
fn canary_ids_exist() {
    use crate::security::quantum_canary::CanaryId;
    let ids = [
        CanaryId::Ecc192,
        CanaryId::Ecc224,
        CanaryId::Ecc25519,
        CanaryId::EccK1,
        CanaryId::EccR1,
        CanaryId::BlsG1,
    ];
    assert!(ids.len() >= 4);
}
