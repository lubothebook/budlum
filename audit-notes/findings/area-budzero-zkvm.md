# Denetim Bulguları: BudZero ve BudZKVM (Area BudZero/ZKVM)

Denetlenen Konumlar: `budzero/bud-vm/`, `budzero/bud-proof/`, `budzero/bud-compiler/`, `budzero/verifier-registry/`.

## 1. Kapsam ve İnceleme Metodolojisi

BudZero ZK rollup ve ZKVM yürütme katmanı:
- Trace matrisi doğrulaması ve AIR (Algebraic Intermediate Representation) kısıtları,
- Opcode davranışları ve sınır değer taşmaları,
- `VerifyMerkle` 64 derinlik kuralı: **Üretimde kapalı kalması kesin kuraldır** (`xtask/gates`).
- İkili ve çoklu kanıtlayıcı (multi-prover) determinizmi.

## 2. Bulgular

### B-ZKVM-01: VerifyMerkle 64-Depth Üretim Bayrağı
- **Önem**: Kritik (Critical) - Korunan Kapı
- **Dosya**: `budzero/bud-proof/src/predicates.rs`, `xtask/gates/src/gates/zkvm_immune_regeneration.rs`
- **Açıklama**: `VerifyMerkle` derinliği 64 olan kanıtlar aşırı hesaplama maliyeti ve DoS riski nedeniyle üretim zincirinde kesinlikle devre dışı kalmalıdır.
- **Kök Neden**: Test senaryolarında derin Merkle yollarının kullanılabilmesi.
- **Çözüm**: Gate kontrolü (`zkvm_immune_regeneration`) ile `VerifyMerkle 64` kodunun üretim bayraklarında açılamayacağı ve testnet/mainnet konfigürasyonlarında fail-closed olacağı garantilendi.
- **Durum**: Korunuyor ve Doğrulandı (`1bd9356` / `main`).

### B-ZKVM-02: AIR Kısıt Matrisinde Derece Aşımı (Degree Overflow)
- **Önem**: Yüksek (High)
- **Dosya**: `budzero/bud-vm/src/air.rs:310-345`
- **Açıklama**: Çoklu çarpım kısıtlarında AIR kısıt derecesi 3'ü aştığında STARK FRI katlama (folding) aşamasında seslilik (soundness) hatası oluşabiliyordu.
- **Kök Neden**: Ara değişkenlerin yardımcı polinomlar (auxiliary trace) yerine doğrudan tek bir kısıt içine gömülmesi.
- **Çözüm**: Yardımcı sütunlar eklenerek tüm kısıt dereceleri azami 3 ile sınırlandı.
- **Durum**: Çözüldü (`ed5faed7`).

### B-ZKVM-03: Güvenilmeyen Kanıt Türü ve ZKVM Kapı Doğrulaması
- **Önem**: Kritik (Critical)
- **Dosya**: `xtask/gates/src/gates/zkvm_refuses_untrusted_proofs.rs`, `src/domain/plugin.rs`
- **Açıklama**: ZK doğrulayıcı arayüzünde kriptografik kanıt taşımayan veya pini yapılmamış sahte kanıt yüklerinin kabul edilmesi, sahte durum geçişlerine zemin hazırlayabilir.
- **Kök Neden**: Test/mock ortamları için gevşetilmiş kanıt kabul kodlarının üretim kapılarına sızma riski.
- **Çözüm**: `xtask/gates/src/gates/zkvm_refuses_untrusted_proofs.rs` kapısı ile tüm ZK eklentilerinin sabitlenmiş (pinned) doğrulama anahtarları ile sıkı STARK/SNARK transkript doğrulaması yapması zorunlu kılındı; doğrulanmamış yükler fail-closed olarak reddedilir. PR #194 üzerinde ilgili eklenti doğrulamaları bu kapıya bağlanmıştır.
- **Durum**: Doğrulandı (`1bd9356` / `main` ve `cfc1fc7` / `PR #194`).
