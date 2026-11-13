//! Settlement and commitment tree safety tests.

use crate::settlement::commitment_tree::merkle_root;

#[test]
fn merkle_root_of_empty_is_zero() {
    let root = merkle_root(&[]);
    assert_eq!(root, [0u8; 32]);
}

#[test]
fn merkle_root_of_one_leaf() {
    let leaf = [42u8; 32];
    let root = merkle_root(&[leaf]);
    assert_ne!(root, [0u8; 32], "single-leaf root must not be zero");
}

#[test]
fn merkle_root_is_deterministic() {
    let leaves = vec![[1u8; 32], [2u8; 32], [3u8; 32]];
    let r1 = merkle_root(&leaves);
    let r2 = merkle_root(&leaves);
    assert_eq!(r1, r2);
}

#[test]
fn merkle_root_changes_with_order() {
    let a = [1u8; 32];
    let b = [2u8; 32];
    let r1 = merkle_root(&[a, b]);
    let r2 = merkle_root(&[b, a]);
    assert_ne!(r1, r2, "swapping leaves must change the root");
}

#[test]
fn merkle_root_changes_with_extra_leaf() {
    let a = [1u8; 32];
    let b = [2u8; 32];
    let r1 = merkle_root(&[a]);
    let r2 = merkle_root(&[a, b]);
    assert_ne!(r1, r2, "adding a leaf must change the root");
}
