//! Privacy module safety tests.

use crate::privacy::{L1NoteRegistry, MAX_PRIVATE_IO};

#[test]
fn max_private_io_is_bounded() {
    assert!(MAX_PRIVATE_IO >= 2, "need at least 2 I/O for a transfer");
    assert!(MAX_PRIVATE_IO <= 256, "too many I/O per transaction");
}

#[test]
fn l1_note_registry_type_exists() {
    let _ = std::mem::size_of::<L1NoteRegistry>();
}
