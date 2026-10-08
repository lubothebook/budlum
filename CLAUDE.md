# BUDLUM KODLAMA DİREKTİFİ

Uygulayıcı: Claude (Claude Code). Sahip ve karar mercii: Ayaz (MODEL_ROUTING dosyalarında "Japs").
Sürüm: 1.5 (2026-10-08). Durum: Bağlayıcı. Bu dosyada değişiklik yalnızca Ayaz'ın onayıyla yapılır.

## 0. Dosyalar

| Dosya | Ne zaman okunur |
|---|---|
| CLAUDE.md | Her oturum |
| MODEL_ROUTING.md | Her oturum (dosya sonundaki `@` satırı) |
| STATUS.md | Her oturum başı. Aktif iş burada. |
| MODEL_ROUTING_REF.md | Yalnızca ayrıntı ve gerekçe gerekirse (bağlanmaz) |
| MODEL_ROUTING_KURULUM.md | Yalnızca kurulum oturumunda (bağlanmaz) |
| BUD-AI-KAPSAMLI-DIREKTIF.md | Yalnızca BUD işinde (bağlanmaz) |

- Çelişki: model, effort, ajan, okuma, maliyet, onay konularında MODEL_ROUTING.md geçerlidir. Amaç ve çalışma ritminde CLAUDE.md geçerlidir.
- Kalıcı kararlar memanto'dadır. Ayaz hedefi koyar (mainnet). Görevleri Claude STATUS.md ve canlı durumdan kendisi çıkarır. Komut beklenmez.

## 1. Amaç

- Tek amaç Budlum'u mainnet'e hazırlamaktır. Her değişiklik şunu cevaplar: "Bu, Budlum'u mainnet'e yaklaştırır mı?"
- Mainnet ölçütü: üretim kodu çalışır, bağlıdır, test edilmiştir, güvenlidir.
- Kodlu ama üretim çağrı noktasına bağlı olmayan parça hazır sayılmaz. Claude bağlar.
- Deneysel, yarım veya kapalı parça açık bir mainnet engelidir. Claude bulur ve kapatır.

## 2. Budlum nedir

- Evrensel Mutabakat Katmanı: PoW, PoS, BFT ve izole PoA ağlarının doğrulanabilir birlikte çalışması. Konsensüs paradigmalarını değiştirmez, üstüne doğrulanabilir kesinlik ve mutabakat katmanı kurar.
- Yapı taşları: `DomainFinalityAdapter`, `GlobalBlockHeader`, `ReplayNonceStore`, `CrossDomainMessage`, `DomainStatus`. Odak: veri egemenliği, kuantum direnci. Ad: "bud" + "lum".
- Çekirdek (Rust). B.U.D.: içerik adresli, erasure kodlu depolama, bir kısmı henüz üretim çağrı noktasına bağlı değil. BudZero/BudZKVM: Plonky3/STARK tabanlı ZK VM, VerifyMerkle 64 derinlik kapısı üretimde kapalı. BudL: kendi dili. Cüzdan: ortak Rust çekirdeği. Lubot: cihaz içi yapay zekâ.
- $BUD: 100M sabit arz, çift yakım. Lisans: PolyForm Shield 1.0.0.

## 3. Değişmez kurallar

Z1. PR. Her değişiklik dalda yapılır ve PR ile açılır.
Z2. Tek doğrulayıcı. CI (`cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`).
Z3. CI yumuşatma yasak. Yeni `#[allow(...)]`, yeni `#[ignore]`, eski commit sabitleme, Clippy için mantık zayıflatma ihlaldir.
Z4. Kanıt. Her "tamamlandı" commit SHA ve CI run numarası taşır. Öz beyan kanıt değildir.
Z5. Canlı durum otoritedir. Hafıza, eski rapor, rozet, scout çıktısı kanıt değildir. `git` ve `gh` ile kontrol et.
Z6. Diller Rust ve BudL. Repoya shell script eklenmez.
Z7. `unsafe` eklenmez. Yeni bağımlılık `cargo-deny` ve `cargo-audit` geçer, gerekçesi PR'a yazılır.
Z8. Konsensüs yolunda float yok, sabit nokta aritmetiği.
Z9. Token, anahtar, seed hiçbir commit, PR, dosya veya araçta açık metin olmaz.
Z10. Harici kod kopyalanmaz. Harici projeler yalnızca yöntem ilhamıdır.
Z11. PoA/kurumsal akış izinsiz kullanıcı akışından ayrı kalır.
Z12. Kamu metinlerinde "L1" veya "Layer 1" yok. Doğru ad "Budlum Evrensel Mutabakat Katmanı".
Z13. Yorum, commit ve PR kısa sade cümlelerle yazılır (ASD-STE100). Uzun tire ve markdown kalın işareti yok.

## 4. Çalışma döngüsü

Her faz MODEL_ROUTING §2.4 ajanıyla yapılır ve §2.5 beyan satırıyla başlar.

1. Keşif: scout ile ilgili yerler. Bütün repo okunmaz (§6).
2. Bulgu ve plan: finder veya architect (§2.6, §3). İş en fazla 64 ADIM'lık partilere bölünür, her ADIM tek odaklıdır ve §7 handoff'u taşır.
3. Kod: handoff'taki ajan, önce başarısız test.
4. Yerel doğrulama: hedefli test, `cargo fmt --check`, clippy. Bütün `cargo test` çalışmaz.
5. Opus doğrulaması: R3'te zorunlu, R2'de §3 koşulunda, her seferinde yeni architect çağrısı.
6. PR: dal push edilir, PR açılır. Test sayıları dosya başına yazılır.
7. Devam: CI beklenmez, sonraki ADIM başlar.

- Açık PR'ların CI'ı ADIM'lar arasında kontrol edilir. Kırmızı CI Z3 ihlal edilmeden düzeltilir.
- Bağımlı işte yığılmış dal. Başarısızlıkta §5 merdiveni. PR birleştirme onayı Ayaz'ındır, beklenmez.
- Bağlam yaklaşık yüzde 60 dolunca `/compact`. Bağlam kapasitesi durma nedeni değildir. Bekleme için `sleep` döngüsü yok.

## 5. Durma koşulları

Seviye 1: Yap ve devam et. Aşağıda olmayan her durum.
Seviye 2: Dur, MODEL_ROUTING §8 şablonuyla sor, yalnızca: §8 listesi, mimari karar, ekonomik veya geri dönüşsüz karar, çelişen gereksinim, geri alınamaz işlem (dal silme, force push, üretim kapısı açma, geçmiş yeniden yazma). Birden fazla karar tek mesajda toplanır. Cevap beklerken bağımsız ADIM'lara devam edilir.

## 6. Kanıt standardı

Önce ham kanıt (dosya:satır, komut çıktısı, SHA), sonra yorum. Sonuç yalnızca kanıta dayanır. "Muhtemelen" ile karar verilmez.

## 7. Oturum başı

1. Kurulum yapılmadıysa MODEL_ROUTING_KURULUM.md §9 sırasıyla kurulum, bitmeden kod işi yok.
2. MODEL_ROUTING §10 ortam kontrolü ve `memanto status`.
3. STATUS.md oku, `git` ve `gh` ile karşılaştır. Çelişkide canlı durum esastır.
4. Yönlendirme beklenmez. STATUS.md sonraki adımından §4 döngüsü kesintisiz çalışır. Ayaz araya girerse onun yönü esastır.
5. Önce kırmızı CI, sonra bulgu avı (MODEL_ROUTING §2.6), sonra STATUS.md kuyrukları. Bulgu bulunmazsa sıradaki R3 modülü taranır.

## 8. Kodlama ilkeleri (Karpathy, kısa)

- Varsayımı yaz, belirsizse sor, birden fazla yorum varsa sun. Daha basit yol varsa söyle.
- En az kod. İstenmeyen özellik, tek kullanımlık soyutlama, imkânsız durum için hata işleme yok.
- Cerrahi değişiklik: yalnızca gerekeni değiştir, komşu kodu "iyileştirme", mevcut stile uy. Ölü kod görürsen söyle, silme. Kendi değişikliğinin yetim bıraktığını temizle.
- Hedef odaklı: görevi doğrulanabilir hedefe çevir (hata için önce başarısız test), doğrulanana kadar döngü kur. Çok adımda kısa plan ve her adımda doğrulama yaz.

@MODEL_ROUTING.md
