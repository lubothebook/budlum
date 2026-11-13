//! Prover module safety tests: ZK proof submission, program hash, and
//! proof claim registry.

use crate::prover::{is_empty_or_halt_only_program, zk_program_hash, ProofClaimRegistry};

#[test]
fn zk_program_hash_is_deterministic() {
    let program = vec![1u64, 2, 3, 4, 5];
    let h1 = zk_program_hash(&program);
    let h2 = zk_program_hash(&program);
    assert_eq!(h1, h2);
}

#[test]
fn different_programs_different_hash() {
    let h1 = zk_program_hash(&[1, 2, 3]);
    let h2 = zk_program_hash(&[4, 5, 6]);
    assert_ne!(h1, h2);
}

#[test]
fn empty_program_is_halt_only() {
    assert!(is_empty_or_halt_only_program(&[]));
}

#[test]
fn nonempty_program_is_not_halt_only() {
    assert!(!is_empty_or_halt_only_program(&[1, 2, 3, 4, 5]));
}

#[test]
fn proof_claim_registry_starts_empty() {
    let reg = ProofClaimRegistry::new();
    assert_eq!(reg.len(), 0);
}

#[test]
fn program_hash_output_is_32_bytes() {
    let h = zk_program_hash(&[42]);
    assert_eq!(h.len(), 32);
}

#[test]
fn program_hash_of_empty() {
    let h = zk_program_hash(&[]);
    assert_eq!(h.len(), 32);
}
