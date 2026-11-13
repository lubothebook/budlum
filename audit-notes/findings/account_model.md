# Account Model Findings

Files reviewed: src/core/account.rs, src/core/address.rs, src/core/transaction.rs

## F-0065: Multi-EE unit conversion is correct

- **Location**: `src/core/account.rs:77-141`
- **What is wrong**: Not a finding. The BudEnvUnit enum correctly maps EVM (Wei), Cosmos (Atom), Solana (Lamport), and Move (Octa) units to attos. The quantum_attos() method returns the correct conversion factor. The check_transfer_quantum() validates that transfers are aligned to the target EE's unit boundary.
- **Status**: closed (informational, positive assessment)

## F-0066: Unbonding period is 7 epochs

- **Location**: `src/core/account.rs:31`
- **What is wrong**: Not a finding. UNBONDING_EPOCHS = 7 provides sufficient time for slashing evidence to surface before stake is released.
- **Status**: closed (informational, positive assessment)

## F-0067: Account balance uses checked arithmetic

- **Location**: `src/core/account.rs:191-217`
- **What is wrong**: Not a finding. `checked_total_attos()` and `set_from_total_attos()` use checked arithmetic to prevent overflow when converting between the balance u64 and the total attos u128.
- **Status**: closed (informational, positive assessment)

## F-0068: Validator roles are additive

- **Location**: `src/core/account.rs:296-317`
- **What is wrong**: Not a finding. Validators can hold multiple roles (consensus validator, storage operator) simultaneously. Roles are added and removed individually.
- **Status**: closed (informational, positive assessment)
