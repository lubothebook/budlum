# Wallet Findings

Files reviewed: src/wallet/mod.rs (611 lines), src/crypto/key_derivation.rs, src/crypto/key_file.rs

## F-0045: Wallet uses 24-word BIP39 mnemonic

- **Location**: `src/wallet/mod.rs:32`
- **What is wrong**: Not a finding. The wallet uses a 24-word (256-bit entropy) BIP39 mnemonic, which is the strongest standard option.
- **Status**: closed (informational, positive assessment)

## F-0046: KDF uses 600K iterations

- **Location**: `src/wallet/mod.rs:34`
- **What is wrong**: Not a finding. The keyfile uses 600,000 PBKDF2 iterations by default, which exceeds the OWASP recommendation of 600,000 for PBKDF2-SHA256.
- **Status**: closed (informational, positive assessment)

## F-0047: Wallet signing key is zeroed on drop

- **Location**: `src/crypto/primitives.rs:806`
- **What is wrong**: Not a finding. The WalletKeyPair implements Drop to overwrite the signing key with zeros, and derives ZeroizeOnDrop. This prevents key material from lingering in memory.
- **Status**: closed (informational, positive assessment)

## F-0048: Key derivation uses domain-separated seed

- **Location**: `src/wallet/mod.rs:172`
- **What is wrong**: Not a finding. `keypair_from_entropy` derives the ML-DSA-87 seed from entropy using SHA3-256 with the domain tag "BUDLUM_MLDSA87_SEED_V1". This prevents cross-protocol key reuse.
- **Status**: closed (informational, positive assessment)

## F-0049: V3 account derivation exists

- **Location**: `src/wallet/mod.rs:190`
- **What is wrong**: Not a finding. The V3 account derivation (`user_account_from_entropy`) supports hierarchical key derivation with account and index parameters, similar to BIP44.
- **Status**: closed (informational, positive assessment)
