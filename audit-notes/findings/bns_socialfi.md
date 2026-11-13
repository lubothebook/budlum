# BNS and SocialFi Findings

Files reviewed: src/bns/registry.rs, src/bns/types.rs, src/socialfi/mod.rs, src/socialfi/vault.rs

## F-0073: BNS grace period prevents squatting

- **Location**: `src/bns/registry.rs:24`
- **What is wrong**: Not a finding. GRACE_PERIOD = 3000 epochs (~30 days) protects expired name owners from front-running. During the grace period, only the previous owner can renew.
- **Status**: closed (informational, positive assessment)

## F-0074: BNS name validation is strict

- **Location**: `src/bns/registry.rs:37`
- **What is wrong**: Not a finding. Name labels must be lowercase ASCII, digits, and hyphens only. No leading/trailing hyphens, no empty labels. This prevents homoglyph attacks and case-sensitivity issues.
- **Status**: closed (informational, positive assessment)

## F-0075: BNS registry is bounded at 100K names

- **Location**: `src/bns/registry.rs:28`
- **What is wrong**: Not a finding. MAX_NAMES = 100,000 bounds state growth to approximately 50MB. MAX_SUBDOMAINS_PER_NAME = 1,000 further limits per-name growth.
- **Status**: closed (informational, positive assessment)

## F-0076: NFT registry (vault) in SocialFi module

- **Location**: `src/socialfi/mod.rs:35`
- **What is wrong**: Not a finding. The NftRegistry handles NFT folder creation, transfer, and metadata. The vault (renamed from the original Turkish concept) provides folder-level organization.
- **Status**: closed (informational, positive assessment)
