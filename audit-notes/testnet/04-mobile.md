# How to build: Mobile validator

Testnet index Section 4. Work packages IP-11, IP-36 through IP-38. TLG gates TLG-10, TLG-11.

---

## Current state

The mobile core library (`mobile/core/`) exists with UniFFI bindings (`mobile/ffi/`, UniFFI 0.32.2). The share store is in-memory only (F-0044). `mobile/core/DB_CHOICE.md` documents that no production phone build ships until a crash-safe backend is chosen. Candidates: redb, flat file + append log. sled is excluded (KR-37).

The Android app (`mobile/android/`) uses Kotlin and Jetpack Compose. The iOS app (`mobile/ios/`) has Swift sources including Turkish localization (F-0047).

The mobile core uses sha3 0.10 while the main crate uses sha3 0.12 (F-0045).

## What must be built

1. **Persistent share store** (TLG-10, TLG-11). Choose and implement the crash-safe storage backend for `mobile/core/`. The candidates are redb (embedded key-value store) or flat file + append log. Measure each on target phones. Replace the in-memory `ShareStore` with the chosen backend. Without this, shares are lost on app kill, blocking TLG-10 and TLG-11.

2. **Unify sha3 version** (F-0045). Move `mobile/core/Cargo.toml` from sha3 0.10 to sha3 0.12 to match the main crate. SHA-3 output is identical across versions (standard algorithm), but the version split doubles code size and complicates auditing.

3. **UniFFI version consistency** (F-0046). Verify that UniFFI 0.32.2 in `mobile/ffi/Cargo.toml` matches the Gradle plugin version in `mobile/android/` and the SPM plugin version in `mobile/ios/`. Add a CI check.

4. **Android field test** (TLG-10). At least 20 real Android phones validate on the rehearsal network. Each phone must: store custody shares, respond to audits, serve content, and earn rewards. The test runs for 24 hours (TLG-11).

5. **iOS beta** (IP-38). The iOS app is beta and does not block testnet launch. Verify that it builds and the opportunistic validator path works in simulation.

6. **Turkish localization audit** (F-0047). Verify that Turkish strings in `mobile/ios/Sources/BudlumValidator/TurkishLocalization.swift` are in localization resources only, not in code identifiers or API surfaces.

## Key files

- `mobile/core/`: Rust mobile core library
- `mobile/core/DB_CHOICE.md`: storage backend decision document
- `mobile/ffi/`: UniFFI FFI bindings
- `mobile/android/`: Kotlin Android app
- `mobile/ios/`: Swift iOS app
- `src/validators/mod.rs`: Mobile/MobileOpportunistic validator classes

## Blocking findings

- F-0044: mobile share store in-memory (TESTNET BLOCKER)
- F-0045: sha3 version split
- F-0046: UniFFI version consistency
- F-0047: Turkish localization audit
