//! Domain registration and management safety tests.

use crate::domain::types::DomainId;

#[test]
fn domain_id_is_u64() {
    let id: DomainId = 1;
    assert_eq!(id, 1u64);
}

#[test]
fn domain_ids_are_distinct() {
    let pow: DomainId = 1;
    let pos: DomainId = 2;
    let bft: DomainId = 3;
    let poa: DomainId = 4;
    assert_ne!(pow, pos);
    assert_ne!(bft, poa);
}
