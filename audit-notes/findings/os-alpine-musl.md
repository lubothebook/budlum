# İşletim Sistemi Denetimi: Alpine Linux 3.21 musl x86_64

## 1. Ortam Bilgileri
- **C Kütüphanesi**: `musl libc 1.2.5`
- **Rolü**: Hafif Docker imajları (`wolfi-base`, `alpine`) ve statik ikili dağıtımlar (`x86_64-unknown-linux-musl`).

## 2. Test ve Çalıştırma Sonuçları
- Statik derleme (`cargo build --target x86_64-unknown-linux-musl`).
- Yığın (stack) boyutu ve bellek ayırıcı (mimalloc) kontrolleri.

## 3. Bulgular ve Çözümler
- **B-OS-ALP-01 (musl Varsayılan Yığın Boyutu ve STARK Doğrulama Gereksinimi)**:
  - *Risk Analizi*: musl libc varsayılan thread stack boyutu (128 KB), derin özyinelemeli STARK FRI katlama veya büyük sıfır-bilgi kanıt doğrulamalarında yığın taşması (`SIGSEGV`) riski barındırır.
  - *Mevcut Durum*: `main` dalında Rayon/prover havuzları sistem varsayılan yığın boyutuyla başlatılmaktadır.
  - *Hedef Kuralı*: Testnet ve üretim ortamında Alpine/musl tabanlı doğrulayıcı çalıştırılırken `Rayon` ve arka plan kanıtlayıcı thread havuzlarının `.stack_size(8 * 1024 * 1024)` (8 MB) olarak açıkça yapılandırılması zorunlu kılınmıştır.
  - *Durum*: Mimari gereksinim olarak tanımlandı; ilgili thread havuzu başlatıcılarında kural olarak izlenmektedir.
