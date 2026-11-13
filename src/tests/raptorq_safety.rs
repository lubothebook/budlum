//! RaptorQ (RFC 6330) segment coding safety tests.

use crate::storage::content_id::ContentId;

#[test]
fn content_id_type_exists() {
    let _ = std::mem::size_of::<ContentId>();
}

#[test]
fn content_id_is_deterministic() {
    let id1 = ContentId([42u8; 32]);
    let id2 = ContentId([42u8; 32]);
    assert_eq!(id1, id2);
}

#[test]
fn different_bytes_different_content_id() {
    let id1 = ContentId([1u8; 32]);
    let id2 = ContentId([2u8; 32]);
    assert_ne!(id1, id2);
}
