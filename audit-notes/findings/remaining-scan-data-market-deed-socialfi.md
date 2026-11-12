# Findings: data market dual module, peripheral modules, final scan

## F-0113

- **ID**: F-0113
- **Location**: src/pollen/mod.rs and src/data_market/mod.rs
- **What is wrong**: Two separate modules implement marketplace/data market functionality. `src/pollen/mod.rs` (the old "Pollen" module) exports `MarketplaceRegistry`, `PollenPurchaseReceipt`, `DataOffer`, and `AccessGrant v2`. `src/data_market/mod.rs` (491 lines) implements the B.U.D. 3.0 Data Market (Section 5.6) with ML-DSA-87 buyer authorization and bud3 grants. The state field `state.marketplace` in `account.rs` points to the pollen module types. The data_market module operates through a different path.
- **Why**: The "Pollen" module is the legacy marketplace. The "data_market" module is the B.U.D. 3.0 replacement. Both exist and are referenced by different parts of the codebase.
- **What it causes**: 49 production references to `PollenPurchaseReceipt` or `MarketplaceRegistry` in non-test code. The metrics emit `pollen_*` names (F-0071). The data_market module uses the new naming (`BDLM_BUD3_DATA_MARKET_PURCHASE_V1`). The two modules may overlap in functionality, or the legacy module may handle the AccessGrant/offer part while data_market handles the B.U.D. 3.0 integration.
- **How to fix**: Clarify the relationship: either merge the two modules under `data_market` (renaming pollen types), or document which surfaces each module owns. The testnet index Section 14 lists "Data Market" as open at launch.

## F-0114

- **ID**: F-0114
- **Location**: src/deed/mod.rs (198 lines), src/education_domain/mod.rs (276 lines)
- **What is wrong**: The DeEd (Decentralized Education) module and the education_domain module exist in the tree. DeEd has permissionless role IDs (INDUSTRY_SPONSOR=10, RESEARCH_CONTRIBUTOR=11) and artifact categories. The education_domain has B.U.D. 3.0 education artifact registration. Neither module has unwired markers.
- **Why**: These are application-layer features that build on the core infrastructure. No errors found.
- **What it causes**: No error. The modules are wired and functional. The DeEd module preserves legacy state while education_domain adds B.U.D. 3.0 integration.
- **How to fix**: No fix needed.

## F-0115

- **ID**: F-0115
- **Location**: src/budlumxyz/mod.rs (699 lines)
- **What is wrong**: The BudlumXYZ app registry has `authorized_governors` as an `HashSet<Address>` that is always empty on a real network. The governance verification path (`mark_verified_by_governance`) refuses every caller because the set is empty. The alternative path (`mark_verified_by_proposal`) works through counted votes.
- **Why**: The governor set is populated by governance proposals, but no governance proposal type populates it yet.
- **What it causes**: No app can be governance-verified through the governor path. The proposal path works. This is a deliberate design: the governor path is reserved for future use.
- **How to fix**: No fix needed for testnet. The proposal path is the active verification mechanism.

## F-0116

- **ID**: F-0116
- **Location**: src/explorer_evidence/mod.rs (751 lines)
- **What is wrong**: The explorer evidence module exists with 751 lines. No unwired markers. This module provides evidence collection for the block explorer/indexer.
- **Why**: Part of the TLG-21 (explorer) infrastructure.
- **What it causes**: No error. Positive finding.
- **How to fix**: No fix needed.

## F-0117

- **ID**: F-0117
- **Location**: src/prover/mod.rs (523 lines)
- **What is wrong**: The prover module handles proof generation for the BudZero execution environment. No unwired markers. Clean module.
- **Why**: Part of the BudZero EE execution pipeline. The prover is wired into the execution path.
- **What it causes**: No error. Positive finding.
- **How to fix**: No fix needed.

## F-0118

- **ID**: F-0118
- **Location**: src/sdk/mod.rs (272 lines)
- **What is wrong**: The SDK module provides a programmatic interface for interacting with the Budlum node. No unwired markers. Clean module.
- **Why**: Part of the developer tooling.
- **What it causes**: No error.
- **How to fix**: No fix needed.

## F-0119

- **ID**: F-0119
- **Location**: src/cli/commands.rs (1984 lines), src/cli/bud3_commands.rs (534 lines)
- **What is wrong**: The interactive console CLI has 2518 lines across 2 files. No unwired markers. Commands include block inspection, transaction submission, account queries, and B.U.D. 3.0 content operations.
- **Why**: Part of the node's interactive console. Clean module.
- **What it causes**: No error.
- **How to fix**: No fix needed.

## F-0120

- **ID**: F-0120
- **Location**: Full codebase summary
- **What is wrong**: This is a summary finding. The codebase is approximately 470,000 lines of Rust across src/, budzero/, crates/, and mobile/. After scanning all top-level modules, the audit has identified: 78 WIRING: unwired markers, 50 TESTNET-DEFAULT markers, 3 large files exceeding 7000 lines (blockchain.rs 13063, storage_deal.rs 9864, account.rs 7222), deprecated concept names with 759 combined references (Pollen 455, Filiz 261, Mikoriza 43). The executor, blockchain, RPC server, governance, consensus, and tokenomics modules are clean (no unwired markers in production paths). The block validation path in blockchain.rs is thorough: it checks height continuity, chain ID, tx_root, witness_root, block hash, state_root, consensus, slashing evidence, and sharding commitment.
- **Why**: Summary for the TLG report.
- **What it causes**: The codebase is testnet-capable with the blockers listed in the PR description addressed. The unwired surfaces are mostly in optional or future features. The critical paths (block production, validation, finality, transaction execution) are wired.
- **How to fix**: Address the 8 key blockers in the PR description. The remaining findings are informational or post-testnet.
