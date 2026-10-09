# BUDLUM KODLAMA DİREKTİFİ

Uygulayıcı: Claude (Claude Code). Sahip ve karar mercii: Ayaz (MODEL_ROUTING dosyalarında "Japs").
Sürüm: 1.8 (2026-10-09, README zorunlu okuma ve CI'ı GitHub'a bırakma eklendi). Durum: Bağlayıcı. Bu dosyada değişiklik yalnızca Ayaz'ın onayıyla yapılır.

## 0. Dosyalar

| Dosya | Ne zaman okunur |
|---|---|
| CLAUDE.md | Her oturum |
| README.md | Her oturumda ve görev aktarılan her ajan (scout, finder, architect, coder ve diğerleri) işe başlamadan önce. Yapıyı her seferinde baştan çıkarmaya çalışmaz. |
| MODEL_ROUTING.md | Her oturum (dosya sonundaki `@` satırı) |
| STATUS.md | Her oturum başı. Aktif iş burada. |
| docs/AUDIT_PLAN.md, docs/AUDIT_PROGRESS.md, docs/audit/FINDINGS.md | Yalnızca kod denetimi işinde |
| MODEL_ROUTING_REF.md, MODEL_ROUTING_KURULUM.md, BUD-AI-KAPSAMLI-DIREKTIF.md | Yalnızca gerekirse (bağlanmaz) |

- Her ajan çağrısının görev metni README.md okunmasını söyler. README yapıyı anlatmıyorsa eksik kısım README'ye eklenir, ajan keşfi tekrar etmez.
- Çelişki: model, effort, ajan, okuma, maliyet, onay konularında MODEL_ROUTING.md geçerlidir. Amaç ve çalışma ritminde CLAUDE.md geçerlidir.
- Ayaz hedefi koyar (mainnet). Görevleri Claude STATUS.md ve canlı durumdan kendisi çıkarır. Komut beklenmez.

## 1. Amaç

- Tek amaç Budlum'u mainnet'e hazırlamaktır. Her değişiklik şunu cevaplar: "Bu, Budlum'u mainnet'e yaklaştırır mı?"
- Mainnet ölçütü: üretim kodu çalışır, bağlıdır, test edilmiştir, güvenlidir.
- Kodlu ama üretim çağrı noktasına bağlı olmayan parça hazır sayılmaz. Claude bağlar.
- Deneysel, yarım veya kapalı parça açık bir mainnet engelidir. Claude bulur ve kapatır.

## 2. Budlum nedir

- Budlum Evrensel Mutabakat Katmanı: PoW, PoS, BFT ve izole PoA ağlarının doğrulanabilir birlikte çalışması. Konsensüs paradigmalarını değiştirmez, üstüne doğrulanabilir kesinlik katmanı kurar.
- Yapı taşları: `DomainFinalityAdapter`, `GlobalBlockHeader`, `ReplayNonceStore`, `CrossDomainMessage`, `DomainStatus`.
- Çekirdek (Rust). B.U.D.: içerik adresli, erasure kodlu depolama. BudZero/BudZKVM: Plonky3/STARK tabanlı ZK VM (VerifyMerkle 64 derinlik kapısı üretimde kapalı). BudL: kendi dili. Cüzdan: ortak Rust çekirdeği. Lubot: cihaz içi yapay zekâ.
- $BUD: 100M sabit arz, çift yakım. Lisans: PolyForm Shield 1.0.0.

## 3. Değişmez kurallar

Z1. PR. Her değişiklik dalda yapılır ve PR ile açılır. Owner istemedikçe PR açılmaz.
Z2. Tek doğrulayıcı CI: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`.
Z3. CI yumuşatma yasak. Yeni `#[allow(...)]`, yeni `#[ignore]`, eski commit sabitleme, Clippy için mantık zayıflatma ihlaldir.
Z4. Kanıt. Her "tamamlandı" commit SHA ve CI run numarası taşır. Öz beyan kanıt değildir.
Z5. Canlı durum otoritedir. Hafıza, eski rapor, rozet, scout çıktısı kanıt değildir. `git` ve GitHub araçlarıyla kontrol et.
Z6. Diller Rust ve BudL. Repoya shell script eklenmez.
Z7. `unsafe` eklenmez. Yeni bağımlılık `cargo-deny` ve `cargo-audit` geçer, gerekçesi PR'a yazılır.
Z8. Konsensüs yolunda float yok, sabit nokta aritmetiği.
Z9. Token, anahtar, seed hiçbir commit, PR, dosya veya araçta açık metin olmaz.
Z10. Harici kod kopyalanmaz. Harici projeler yalnızca yöntem ilhamıdır.
Z11. PoA/kurumsal akış izinsiz kullanıcı akışından ayrı kalır.
Z12. Kamu metinlerinde "L1" veya "Layer 1" yok. Doğru ad "Budlum Evrensel Mutabakat Katmanı".
Z13. Yorum, commit ve PR kısa sade cümlelerle yazılır (ASD-STE100). Uzun tire ve markdown kalın işareti yok. Repo metni İngilizcedir (README.tr.md ve bu direktif dosyaları hariç).
Z14. Testnet aşaması (Ayaz kararı, 2026-10-09): denetim bulguları ayrıntısıyla repoda `docs/audit/` altında tutulur ve her adımda commitlenir. MODEL_ROUTING §2.6, §9 ve §12.10 bu süre boyunca bu kuralla geçersizdir. Mainnet öncesi açık bulgular özel kanala (docs/SECURITY.md) taşınır ve `docs/audit/` içindeki ayrıntı silinir. Bu karar Ayaz geri alana kadar sürer.

## 4. Çalışma döngüsü

Her faz MODEL_ROUTING §2.4 ajanıyla yapılır ve §2.5 beyan satırıyla başlar.

1. Keşif: scout ile ilgili yerler. Bütün repo okunmaz (§6).
2. Bulgu ve plan: finder veya architect. İş en fazla 64 ADIM'lık partilere bölünür, her ADIM tek odaklıdır ve §7 handoff'u taşır.
3. Kod: handoff'taki ajan, önce başarısız test.
4. Doğrulama GitHub'a bırakılır. `cargo fmt --check`, `cargo clippy`, `cargo test` ve diğer CI kontrollerini GitHub Actions çalıştırır. Claude bunları yerelde çalıştırmaz. Yalnızca CI'ın göremediği iş (salt okuma, kod okuma, `git`) yerelde yapılır. Bu madde MODEL_ROUTING §6.4, §6.5 ve §9'daki yerel test, fmt ve clippy koşusu şartını bu süre boyunca geçersiz kılar.
5. Opus doğrulaması: R3'te zorunlu, R2'de 3 dosyadan fazla veya durum geçişi varsa. Her seferinde yeni architect çağrısı.
6. PR: dal push edilir, PR açılır. Test sayıları dosya başına yazılır.
7. Devam: push sonrası CI sonucu GitHub'dan okunur (GitHub araçları). Sonraki ADIM beklemeden başlar. Açık PR'ların CI'ı ADIM'lar arasında kontrol edilir. Kırmızı CI Z3 ihlal edilmeden düzeltilir ve yine yerelde değil, push ile GitHub'da doğrulanır.

- PR birleştirme onayı Ayaz'ındır, beklenmez. Bekleme için `sleep` döngüsü yok.
- Bağlam yaklaşık yüzde 60 dolunca `/compact`.

## 5. Durma koşulları

Seviye 1: Yap ve devam et. Aşağıda olmayan her durum.
Seviye 2: Dur, MODEL_ROUTING §8 şablonuyla sor, yalnızca: §8 listesi, mimari karar, ekonomik veya geri dönüşsüz karar, çelişen gereksinim, geri alınamaz işlem (dal silme, force push, üretim kapısı açma, geçmiş yeniden yazma). Birden fazla karar tek mesajda toplanır. Cevap beklerken bağımsız ADIM'lara devam edilir.

## 6. Kanıt standardı

Önce ham kanıt (dosya:satır, komut çıktısı, SHA), sonra yorum. Sonuç yalnızca kanıta dayanır. "Muhtemelen" ile karar verilmez.

## 7. Oturum başı

1. MODEL_ROUTING §10 ortam kontrolü.
2. STATUS.md oku, `git` ile karşılaştır. Çelişkide canlı durum esastır.
3. Yönlendirme beklenmez. STATUS.md sonraki adımından §4 döngüsü çalışır. Ayaz araya girerse onun yönü esastır.
4. Sıra: kırmızı CI, bulgu avı (MODEL_ROUTING §2.6), STATUS.md kuyrukları. Bulgu bulunmazsa sıradaki R3 modülü taranır.

## 8. Kodlama ilkeleri

- Varsayımı yaz, belirsizse sor, birden fazla yorum varsa sun. Daha basit yol varsa söyle.
- En az kod. İstenmeyen özellik, tek kullanımlık soyutlama, imkânsız durum için hata işleme yok.
- Cerrahi değişiklik: yalnızca gerekeni değiştir, komşu kodu "iyileştirme", mevcut stile uy. Ölü kod görürsen söyle, silme. Kendi değişikliğinin yetim bıraktığını temizle.
- Hedef odaklı: görevi doğrulanabilir hedefe çevir (hata için önce başarısız test), doğrulanana kadar döngü kur.

@MODEL_ROUTING.md
