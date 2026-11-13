//! SocialFi module safety tests.

use crate::socialfi::NftRegistry;

#[test]
fn nft_registry_type_exists() {
    let _ = std::mem::size_of::<NftRegistry>();
}
