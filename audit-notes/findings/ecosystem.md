# Ecosystem Modules Findings

Files reviewed: src/bns/, src/socialfi/, src/data_market/, src/pollen/, src/explorer_evidence/, src/app_registry/, src/education_domain/, src/budlumxyz/, src/ai/, src/ai_inference/

## F-0034: BNS module is minimal (7 lines in mod.rs)

- **Location**: `src/bns/mod.rs` (7 lines)
- **What is wrong**: The BNS (Budlum Name Service) module has only 7 lines in the module root, re-exporting registry.rs and types.rs. The feature matrix says BNS is "Open" at testnet launch (IP-23).
- **Why it is wrong**: The module may be sufficient if registry.rs and types.rs contain the full implementation. Size alone is not a measure of completeness.
- **What it causes**: Needs verification that the BNS registration, resolution, and renewal flows are complete.
- **How to fix it**: Verify the BNS test coverage in src/tests/bns.rs and src/tests/bns_expanded.rs.
- **Status**: open (verification needed)

## F-0035: SocialFi module includes vault (NFT folders)

- **Location**: `src/socialfi/mod.rs` (475 lines), `vault.rs`
- **What is wrong**: Not a finding. The SocialFi module includes the vault (NFT folders) implementation per the directive's rename table (Vault -> NFT folders -> nft_folders).
- **Status**: closed (informational, positive assessment)

## F-0036: Data Market module is substantial

- **Location**: `src/data_market/mod.rs` (545 lines)
- **What is wrong**: Not a finding. The data market (formerly Pollen) module is 545 lines. The directive renames Pollen to Data Market and requires it to be open on testnet.
- **Status**: closed (informational, positive assessment)

## F-0037: AI module is the largest non-test module

- **Location**: `src/ai/mod.rs` (5005 lines)
- **What is wrong**: The AI module at 5005 lines is the largest non-test module. It includes verification layers, registry, and execution submodules. The feature matrix says AI inference is "Conditional" on testnet.
- **Why it is wrong**: Not necessarily wrong, but the module's size suggests it may benefit from further decomposition. The AI verification layer (VL-0 through VL-3) is complex.
- **What it causes**: Maintenance complexity. The module should be verified for the VL-1 conditions specified in IP-45.
- **How to fix it**: Verify that the `ai_inference_prompt_is_true`, `ai_inference_reads`, and `ai_inference_zk` gates pass.
- **Status**: open (verification needed)

## F-0038: Explorer evidence module is well-structured

- **Location**: `src/explorer_evidence/mod.rs` (751 lines)
- **What is wrong**: Not a finding. The explorer evidence module (renamed from Atlas) provides the evidence model for the block explorer. It includes chain-specific proof verification.
- **Status**: closed (informational, positive assessment)

## F-0039: App registry module exists

- **Location**: `src/app_registry/mod.rs` (124 lines)
- **What is wrong**: The app registry (renamed from budlumxyz) is 124 lines. The budlumxyz module (699 lines) also exists. The relationship between these two needs verification.
- **Why it is wrong**: Both app_registry and budlumxyz exist in the codebase. The directive renames budlumxyz to app_registry.
- **What it causes**: Potential duplication or confusion about which is the canonical module.
- **How to fix it**: Verify that app_registry is the new canonical module and budlumxyz is either a re-export or scheduled for removal.
- **Status**: open

## F-0040: Education domain module exists

- **Location**: `src/education_domain/mod.rs` (276 lines)
- **What is wrong**: Not a finding. The education domain (renamed from DeEd) is 276 lines and implements the education domain features.
- **Status**: closed (informational, positive assessment)
