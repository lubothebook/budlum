#[cfg(test)]
pub mod audit_lane_a;
#[cfg(test)]
pub mod bench_performance;
#[cfg(test)]
pub mod block_door_harness;
#[cfg(test)]
pub mod block_reward;
#[cfg(test)]
pub mod bridge_lifecycle;
#[cfg(test)]
pub mod conformance_locks;
#[cfg(test)]
pub mod global_header_sequence;
pub mod regression_canaries;
// State-machine sharding (Whitepaper v1.3): block-level shards commitment
// production/validation and atomic cross-shard transfers.
#[cfg(test)]
pub mod adversarial_p2p;
#[cfg(test)]
pub mod advisory_reachability;
#[cfg(test)]
pub mod ai_inference_runtime;
#[cfg(test)]
pub mod ai_verification_status_locks;
#[cfg(test)]
pub mod block_clock;
pub mod blockchair_fixture_locks;
#[cfg(test)]
pub mod bns;
#[cfg(test)]
pub mod bns_expanded;
#[cfg(test)]
pub mod bridge_negatives;
#[cfg(test)]
pub mod bud_e2e;
#[cfg(test)]
pub mod byzantine_settlement;
#[cfg(test)]
pub mod chain_id_registry_locks;
#[cfg(test)]
pub mod chaos;
pub mod consensus_bypass_locks;
pub mod consensus_digest;
#[cfg(test)]
pub mod consensus_expanded;
#[cfg(test)]
pub mod consensus_lock_order_loom;
#[cfg(test)]
pub mod consensus_round_boundaries;
#[cfg(test)]
pub mod consensus_rounds;
#[cfg(test)]
pub mod constitution_engine;
#[cfg(test)]
pub mod deed;
#[cfg(test)]
pub mod disaster_recovery;
#[cfg(test)]
pub mod distributed_settlement;
pub mod domain_edge_cases;
#[cfg(test)]
pub mod encryption_dao;
#[cfg(test)]
pub mod evm_finality_contract;
#[cfg(test)]
pub mod finality_adversarial;
#[cfg(test)]
pub mod finality_live_path;
#[cfg(test)]
pub mod fork_choice_locks;
#[cfg(test)]
pub mod genesis_repro;
#[cfg(test)]
pub mod hard_prune;
#[cfg(test)]
pub mod hardening;
#[cfg(test)]
pub mod hardening_h2_locks;
#[cfg(test)]
pub mod hardening_h4_locks;
#[cfg(test)]
pub mod hardening_h5_h7_locks;
#[cfg(test)]
pub mod hardening_locks;
#[cfg(test)]
pub mod hash_input_serialize_locks;
#[cfg(test)]
pub mod identity_tx_door;
#[cfg(test)]
pub mod integration;
#[cfg(test)]
pub mod liveness_consensus;
#[cfg(test)]
pub mod load_test;
#[cfg(test)]
pub mod manifest_commitment_locks;
pub mod mempool_dos_locks;
pub mod migration_v2;
#[cfg(test)]
pub mod multi_consensus_locks;
pub mod network_hardening_locks;
pub mod operator_kill_chaos;
#[cfg(test)]
pub mod permissionless;
#[cfg(test)]
pub mod permissionless_e2e;
#[cfg(test)]
pub mod persistence;
pub mod poa_isolation;
pub mod poa_onboarding_matrix;
#[cfg(test)]
pub mod poisoned_lock_locks;
#[cfg(test)]
pub mod pollen_ai_data_rights;
#[cfg(test)]
pub mod ported_slice_redteam;
#[cfg(test)]
pub mod pow_light_client;
pub mod privacy_ai_execution;
pub mod private_transfer_fee_market;
#[cfg(test)]
pub mod proptest_core;
#[cfg(test)]
pub mod protocol_guardrails;
#[cfg(test)]
pub mod prover;
#[cfg(test)]
pub mod qcblob_quorum;
#[cfg(test)]
pub mod real_chain_fixtures;
#[cfg(test)]
pub mod regression_lock;
#[cfg(test)]
pub mod relayer_e2e;
#[cfg(test)]
pub mod relayer_gates;
#[cfg(test)]
pub mod relayer_liveness;
#[cfg(test)]
pub mod relayer_worker_locks;
#[cfg(test)]
pub mod replay_audit;
#[cfg(test)]
pub mod security_auditor;
pub mod security_invariant_tests;
#[cfg(test)]
pub mod settlement_prod;
#[cfg(test)]
pub mod sharding_e2e;
pub mod slashing_matrix;
#[cfg(test)]
pub mod snapshot_chaos;
#[cfg(test)]
pub mod socialfi;
#[cfg(test)]
pub mod state_recovery;
#[cfg(test)]
pub mod storage_tx_door;
#[cfg(test)]
pub mod target_700;
#[cfg(test)]
pub mod tokenomics;
pub mod tokenomics_proptest;
#[cfg(test)]
pub mod vault_tx_door;
#[cfg(test)]
pub mod wall_clock_locks;
#[cfg(test)]
pub mod zkvm;
// Effect-asserting locks for the slashing matrix and the overflow rule.
#[cfg(test)]
pub mod refusal_locks;

// Regression locks for fixes already in the tree, one module per area.
#[cfg(test)]
pub mod regression_ai_storage;
#[cfg(test)]
pub mod regression_bridge;
#[cfg(test)]
pub mod regression_chain;
#[cfg(test)]
pub mod regression_ops_crypto;

// Validator classes and the single registration door.
#[cfg(test)]
pub mod validator_classes;

// The B.U.D. 3.0 storage door end to end.
#[cfg(test)]
pub mod bud3_door;

// Prometheus alert rules may only name metrics the node exports.
#[cfg(test)]
pub mod alert_rules_reference_existing_metrics;
#[cfg(test)]
pub mod alerts_have_runbook_entries;

// Derived object-layer measurement rings and counters.
#[cfg(test)]
pub mod object_layer_metrics;

// Readiness series and the operational surfaces that scrape and alert on them.
#[cfg(test)]
pub mod backup_covers_persisted_paths;
#[cfg(test)]
pub mod caddy_edges_match_the_services;
#[cfg(test)]
pub mod compose_topology_matches_services;
#[cfg(test)]
pub mod readiness_metrics;
#[cfg(test)]
pub mod scrape_topology_covers_the_rules;

// The persistence move is behaviour-neutral.
#[cfg(test)]
pub mod persistence_move;

// State transition, vesting, staking, governance, sharding, and snapshot hardening.
#[cfg(test)]
pub mod state_hardening;

// Domains, cross-domain bridge, settlement, and testnet feature-matrix locks.
#[cfg(test)]
pub mod domains_bridge_settlement;

// Instant Duty Handover and finality recovery.
#[cfg(test)]
pub mod handover_recovery;

// Ceremony rehearsal and protocol drills.
#[cfg(test)]
pub mod ceremony_and_drills;

// Security, conservation, and privilege-boundary regressions.
#[cfg(test)]
pub mod security_review;
// Object replacement: the second representation of one content id
// (section 6.14).
#[cfg(test)]
pub mod bud3_replace;

// Lane-D configuration, backup-drill and vote-domain regression locks.
#[cfg(test)]
pub mod audit_lane_d_locks;

// Testnet smoke tests: genesis configuration, chain IDs, feature matrix.
#[cfg(test)]
pub mod testnet_smoke;

// Instant Duty Handover mechanics: constants and math.
#[cfg(test)]
pub mod handover_mechanics;

// Tokenomics invariant tests: supply conservation, burn mechanics.
#[cfg(test)]
pub mod tokenomics_invariants;

// Cross-domain bridge safety checks and replay protection.
#[cfg(test)]
pub mod bridge_safety;

// Storage B.U.D. 3.0 content addressing and segment coding.
#[cfg(test)]
pub mod bud3_content_addressing;

// Execution environment isolation and coordinator tests.
#[cfg(test)]
pub mod execution_isolation;

// Quantum canary alarm flow end-to-end.
#[cfg(test)]
pub mod quantum_alarm_flow;

// RPC compatibility gateway tests: Ethereum, Cosmos.
#[cfg(test)]
pub mod rpc_compat;

// Validator registration door and class enforcement.
#[cfg(test)]
pub mod validator_registration_door;

// Network peer management and gossip dedup.
#[cfg(test)]
pub mod network_peer_safety;

// Consensus safety: constants, bounds, and internal consistency.
#[cfg(test)]
pub mod consensus_safety;

// Sharding invariants: shard assignment, bounds, cross-shard detection.
#[cfg(test)]
pub mod sharding_invariants;

// Wallet safety: mnemonic, keyfile, entropy.
#[cfg(test)]
pub mod wallet_safety;

// Light client safety: checkpoint, trust modes, sync plan.
#[cfg(test)]
pub mod light_client_safety;

// Post-quantum cryptography: ML-DSA, BLS, scheme consistency.
#[cfg(test)]
pub mod crypto_pq_safety;

// Domain tag inventory: prefix, uniqueness, ASCII.
#[cfg(test)]
pub mod domain_tags_safety;

// Settlement and commitment tree: merkle root determinism.
#[cfg(test)]
pub mod settlement_safety;

// Gas schedule: ordering, bounds, network consistency.
#[cfg(test)]
pub mod gas_schedule_safety;

// Genesis ceremony: validation gates, build, serde.
#[cfg(test)]
pub mod genesis_ceremony_safety;

// Cross-domain nonce store extended: prune depth, root, edge cases.
#[cfg(test)]
pub mod cross_domain_nonce_extended;

// Account model invariants: balance, fees, units.
#[cfg(test)]
pub mod account_invariants;

// Hash function determinism and domain separation.
#[cfg(test)]
pub mod hash_determinism;

// Validator stake bounds and class constraints.
#[cfg(test)]
pub mod validator_stake_bounds;

// Prover module: ZK proof submission, program hash.
#[cfg(test)]
pub mod prover_safety;

// Address type: construction, display, serde.
#[cfg(test)]
pub mod address_safety;

// Block structure: genesis fields, hash determinism.
#[cfg(test)]
pub mod block_structure_safety;

// Config consistency: TOML/JSON alignment.
#[cfg(test)]
pub mod config_consistency;

// BNS safety: registry, grace period, bounds.
#[cfg(test)]
pub mod bns_safety;

// SocialFi safety: NFT registry.
#[cfg(test)]
pub mod socialfi_safety;

// Data Market safety: registry, listings.
#[cfg(test)]
pub mod data_market_safety;

// Mempool module exists.
#[cfg(test)]
pub mod mempool_safety;

// SDK module: BudlumToml.
#[cfg(test)]
pub mod sdk_safety;

// Object layer (B.U.D. 3.0 storage).
#[cfg(test)]
pub mod object_layer_safety;

// Relayer module exists.
#[cfg(test)]
pub mod relayer_safety;

// Execution coordinator: UEL types, BudZero limits.
#[cfg(test)]
pub mod execution_coordinator_safety;

// Privacy module: note registry, private I/O bounds.
#[cfg(test)]
pub mod privacy_safety;

// Executor: transaction/block application.
#[cfg(test)]
pub mod executor_safety;

// Settlement anchor ledger.
#[cfg(test)]
pub mod anchor_safety;

// Secp256k1 recovery for Ethereum compatibility.
#[cfg(test)]
pub mod secp256k1_recovery_safety;

// Domain registration and management.
#[cfg(test)]
pub mod domain_registration_safety;

// HPKE encrypted communication.
#[cfg(test)]
pub mod hpke_safety;

// TLS module.
#[cfg(test)]
pub mod tls_safety;

// Education domain (DeEd).
#[cfg(test)]
pub mod education_domain_safety;

// App registry (budlumxyz).
#[cfg(test)]
pub mod app_registry_safety;

// AI verification layer.
#[cfg(test)]
pub mod ai_verification_safety;
