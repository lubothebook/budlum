---
name: steward
description: Budlum PR'larını yeşile götürme rehberi. PR olayında (CI, inceleme, çatışma) okunur. CLAUDE.md ve MODEL_ROUTING.md ile birlikte geçerlidir.
---

# Steward: Budlum PR rehberi

Bu dosya kısadır. Çelişkide CLAUDE.md ve MODEL_ROUTING.md geçerlidir.

## Duruş

- Komut beklenmez. Kırmızı CI, çatışma ve açık inceleme yorumu iştir.
- Birleştirme ve onay Ayaz'ındır. PR birleştirilmez, onaylanmaz.
- Boş commit, PR'ı kapatıp açmak, testi atlamak veya `#[ignore]` koymak yok (Z3).

## Sıra

1. Birleştirme çatışması: base dalı PR dalına merge et. Rebase ve force push yok.
2. Kırmızı CI: önce nedeni oku, sonra düzelt.
3. İnceleme yorumu: kısa ve yerel istekleri uygula, büyük istekleri Ayaz'a sor.

## Kırmızı CI nasıl okunur

- `get_job_logs` ile en az 150 satır al. Son 30 satır çoğu zaman temizlik çıktısıdır, hata değildir.
- Gates işi tek gate adı verir (`FAIL [ad]`). Gate'i yerelde çalıştır:
  `cargo run --release --manifest-path xtask/gates/Cargo.toml -- <gate-adı>`
- Biçim: `cargo fmt --all -- --check`. `budzero/` ayrı çalışma alanıdır, orada `cd budzero` ile çalıştır.
- `protoc` yoksa `cargo test` ve clippy yerelde çalışmaz. Bunu PR açıklamasına yaz, CI'a bırak. Paket kurma, sor.
- Dependency Review "Dependency graph" hatası verirse bu repo ayarıdır, kod değildir. Ayaz'a bildir.
- Docker Trivy bulgusu: `ops/Dockerfile` içinde paketi açıkça listele (libpcre2-8-0 ve perl-base örneği). Genel `apt-get upgrade` yok (hadolint DL3005).

## Baseline dosyaları

- `.github/*-baseline.txt` dosyalarını gevşetme. Gevşetme için §8 onayı gerekir.
- Gate "bağlanmış veya silinmiş satır kaldı" derse yalnızca o satırı sil (sıkılaştırma). Araç izni engellerse atlatma, Ayaz'a bildir.

## R3 değişikliği

- R3 alanında (consensus, crypto, privacy, tokenomics, settlement, cross_domain, registry, chain, network/node.rs, budzero/bud-proof) push öncesi yeni bir `architect` çağrısı diff'i doğrular.
- Etiket baytları, imza girdileri ve kök hesapları değişmez. Değişiyorsa dur ve §8 ile sor.

## Yazım ve gizlilik

- Commit ve PR metni kısa sade cümleler. Uzun tire ve kalın işaret yok (Z13).
- Düzeltilmemiş güvenlik bulgusu PR, yorum, commit veya dosyaya yazılmaz. `docs/SECURITY.md` özel kanalı kullanılır.
- PR açıklaması `.github/PULL_REQUEST_TEMPLATE.md` başlıklarını izler. Test sayıları dosya başına yazılır.
