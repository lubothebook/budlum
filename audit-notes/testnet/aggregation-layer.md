# Testnet Tasarımı: Agregasyon Katmanı ve Özyinelemeli STARK Kanıtları

## 1. Gereksinim
`THE_BUDLUM_AGREGASYON_DIREKTIF.md` uyarınca:
- Binlerce işlemi tek bir blokta toplayıp özyinelemeli (recursive) STARK ile tek bir kanıta indirgeme,
- `Claim::Prog`, gizli m-of-n çoklu imza (multisig), `Account.auth_program_hash`,
- `AggMerge` BudAir, kompakt `AggFoldAir` StarkBackend ve `ShadowAuditor` ile anında teyit,
- `ClaimedDomainFinalityAdapter` ile dış domain kanıtlarının birleştirilmesi.

## 2. Mevcut Durum ve Faz Dağılımı (Revizyon Bazında)

- **`main` Dalı (`1bd9356`)**:
  - Agregasyon modülü temel `src/aggregation/` veri tipleri ve Faz 0 arayüzleri ile yer almaktadır.
- **`testnet` Dalı (`ad88285c`)**:
  - **Faz 0**: `Claim` tipleri, blok kuralları, Merkle ağacı, yerel backend entegre (`ed5faed7`).
  - **Faz 1**: `Aggregator`, havuz, düğüm entegrasyonu, RPC kapıları entegre (`ed5faed7`).
- **Açık/Sıralı PR Dalları**:
  - **Faz 2**: Referans programlar, Keccak ve ML-DSA NTT STARK çipleri (`b034a9ee`, dal `agg/phase-2-sig-claims`, PR #192).
  - **Faz 3**: `AggMerge` BudAir, katlama backend'i, `ShadowAuditor` (dal `agg/phase-3-recursion-stark`, PR #193).
  - **Faz 4**: `Claim::Prog`, gizli m-of-n multisig, `ClaimedDomainFinalityAdapter` (dal `agg/phase-4-program-claims`, PR #195).
  - *Not*: `aggregation-phases.md` belgesinde belirtildiği üzere, `main` üzerinde Faz 3/4 henüz birleştirilmemiştir; testnet hazırlık dalında aşamalı olarak entegre edilecektir.

## 3. İlgili Dosyalar
- `src/aggregation/aggregator.rs`
- `src/aggregation/chain.rs`
- `src/aggregation/claim.rs`
- `src/aggregation/envelope.rs`
- `src/aggregation/frame.rs`
- `src/aggregation/node.rs`
- `src/aggregation/pool.rs`
- `src/aggregation/programs/`
- `src/aggregation/schemes.rs`

## 4. Doğrulama ve Testler
- `cargo test --lib aggregation::`
- `tests/aggregation/`
- `xtask/gates/src/gates/agg_native_backend.rs`
