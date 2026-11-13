# Cryptography Findings

Files reviewed: src/crypto/primitives.rs, scheme.rs, domain_tags.rs, pkcs11.rs, hpke.rs, key_derivation.rs, composite.rs

## F-0059: PQ scheme is ML-DSA-65 (FIPS 204) for consensus, ML-DSA-87 for wallets

- **Location**: `src/crypto/primitives.rs:283`, `src/crypto/scheme.rs`
- **What is wrong**: Not a finding. The separation is correct: validators use ML-DSA-65 (1952-byte public keys) for consensus signing where message volume is high, while wallets use ML-DSA-87 (2592-byte public keys) for stronger long-term key security.
- **Status**: closed (informational, positive assessment)

## F-0060: BLS12-381 keypair generation and PoP verification exist

- **Location**: `src/crypto/primitives.rs:93-252`
- **What is wrong**: Not a finding. BLS keypairs support generation, byte serialization, Proof of Possession (PoP), and verification. The PoP is bound to chain_id and address, preventing cross-chain replay.
- **Status**: closed (informational, positive assessment)

## F-0061: Domain tags are inventoried and audited

- **Location**: `src/crypto/domain_tags.rs`
- **What is wrong**: Not a finding. The domain_tags module maintains DOMAIN_TAGS (all tags), CRITICAL_DOMAIN_TAGS (consensus-critical), and BUDLUM_PREFIXED_DOMAIN_TAGS. All tags use the BDLM_ prefix convention. This addresses K-3 (domain-separated hashing).
- **Status**: closed (informational, positive assessment)

## F-0062: PKCS#11 HSM integration exists

- **Location**: `src/crypto/pkcs11.rs`
- **What is wrong**: The PKCS#11 module supports SoftHSM for development and hardware HSMs for production. The testnet.toml defaults to softhsm backend.
- **Why it is wrong**: The PKCS#11 allowlist/denylist approach (F-0013) needs verification.
- **What it causes**: An overly permissive PKCS#11 policy could allow weak keys.
- **How to fix it**: Verify the mechanism list is an allowlist, not a denylist.
- **Status**: open (cross-reference F-0013)

## F-0063: HPKE module exists for encrypted communication

- **Location**: `src/crypto/hpke.rs`
- **What is wrong**: Not a finding. The HPKE module provides encrypted communication for validator-to-validator private channels.
- **Status**: closed (informational, positive assessment)

## F-0064: Composite signatures support hybrid classical+PQ

- **Location**: `src/crypto/composite.rs`
- **What is wrong**: Not a finding. The composite signature module combines Ed25519 and ML-DSA signatures for the hybrid trust mode. This provides security even if either primitive is broken.
- **Status**: closed (informational, positive assessment)
