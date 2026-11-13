//! Sharding invariant tests: verify shard assignment, cross-shard detection,
//! and state commitment.

use crate::core::address::Address;
use crate::sharding::{shard_of, ShardId, ShardingConfig, MAX_SHARDS};

#[test]
fn max_shards_is_64() {
    assert_eq!(MAX_SHARDS, 64);
}

#[test]
fn shard_id_wraps_u16() {
    let s = ShardId(42);
    assert_eq!(s.0, 42);
}

#[test]
fn shard_of_zero_address_is_deterministic() {
    let addr = Address::from([0u8; 32]);
    let s1 = shard_of(&addr, 4);
    let s2 = shard_of(&addr, 4);
    assert_eq!(s1, s2);
}

#[test]
fn shard_of_is_bounded_by_num_shards() {
    for num_shards in [1u16, 2, 4, 8, 16, 32, 64] {
        for byte in 0..=255u8 {
            let addr = Address::from([byte; 32]);
            let s = shard_of(&addr, num_shards);
            assert!(
                s.0 < num_shards,
                "shard {} >= num_shards {} for byte {}",
                s.0,
                num_shards,
                byte
            );
        }
    }
}

#[test]
fn different_addresses_may_land_in_different_shards() {
    let mut shards_seen = std::collections::HashSet::new();
    for byte in 0..=255u8 {
        let addr = Address::from([byte; 32]);
        shards_seen.insert(shard_of(&addr, 64).0);
    }
    assert!(
        shards_seen.len() > 1,
        "all 256 addresses landed in the same shard"
    );
}

#[test]
fn sharding_config_inactive_before_activation() {
    let config = ShardingConfig {
        num_shards: 4,
        activation_height: 100,
    };
    assert!(!config.is_active_at(0));
    assert!(!config.is_active_at(99));
    assert!(config.is_active_at(100));
    assert!(config.is_active_at(1_000_000));
}
