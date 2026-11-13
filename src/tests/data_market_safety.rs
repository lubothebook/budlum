//! Data Market module safety tests.

use crate::data_market::{DataMarketListing, DataMarketRegistry};

#[test]
fn data_market_registry_type_exists() {
    let _ = std::mem::size_of::<DataMarketRegistry>();
}

#[test]
fn data_market_listing_type_exists() {
    let _ = std::mem::size_of::<DataMarketListing>();
}
