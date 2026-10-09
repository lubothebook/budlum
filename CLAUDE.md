# BUDLUM KODLAMA DİREKTİFİ

Uygulayıcı: Claude (Claude Code). Sahip ve karar mercii: Ayaz (MODEL_ROUTING dosyalarında "Japs").
Sürüm: 1.10 (2026-10-09). Durum: Bağlayıcı. Bu dosyada değişiklik yalnızca Ayaz'ın onayıyla yapılır.
Son değişiklik (Ayaz onayıyla): §9 Mainnet'e giden yol eklendi. Önceki: CI kontrollerini GitHub yapar; ajanlar README yerine `docs/AGENT_MAP.md` okur.

## 0. Dosyalar

| Dosya | Kim, ne zaman okur |
|---|---|
| CLAUDE.md, MODEL_ROUTING.md | Ana oturum, her oturum (MODEL_ROUTING dosyanın sonundaki `@` satırıyla gelir) |
| STATUS.md | Ana oturum, oturum başı. Aktif iş ve sıradaki adım burada. |
| docs/AGENT_MAP.md | Ana oturum ve görev aktarılan her ajan, işe başlamadan önce. Depo haritası, kademeler, CI kuralı. README.md okunmaz, yapıyı baştan çıkarmaya çalışılmaz. |
| docs/AUDIT_PLAN.md, docs/AUDIT_PROGRESS.md, docs/audit/FINDINGS.md | Yalnızca kod denetimi işinde |
| MODEL_ROUTING_REF.md, MODEL_ROUTING_KURULUM.md, BUD-AI-KAPSAMLI-DIREKTIF.md | Yalnızca gerekirse |

- Her ajan çağrısının görev metni ilk satırda `docs/AGENT_MAP.md` okunmasını söyler. Haritada eksik bir şey varsa ana oturum haritaya ekler.
- Çelişki: model, effort, ajan, okuma, maliyet, onay konularında MODEL_ROUTING.md geçerlidir. Amaç ve çalışma ritminde CLAUDE.md geçerlidir.
- Ayaz hedefi koyar (mainnet). Görevleri Claude STATUS.md ve canlı durumdan kendisi çıkarır. Komut beklenmez.

## 1. Amaç

- Tek amaç Budlum'u mainnet'e hazırlamaktır. Her değişiklik şunu cevaplar: "Bu, Budlum'u mainnet'e yaklaştırır mı?"
- Mainnet ölçütü: üretim kodu çalışır, bağlıdır, test edilmiştir, güvenlidir.
- Kodlu ama üretim çağrı noktasına bağlı olmayan parça hazır sayılmaz. Claude bağlar.
- Deneysel, yarım veya kapalı parça açık bir mainnet engelidir. Claude bulur ve kapatır.

## 2. Budlum nedir

Budlum Evrensel Mutabakat Katmanı: PoW, PoS, BFT ve izole PoA ağlarının doğrulanabilir birlikte çalışması. Konsensüs paradigmalarını değiştirmez, üstüne doğrulanabilir kesinlik katmanı kurar. Ayrıntı ve depo haritası: `docs/AGENT_MAP.md`.
$BUD: 100M sabit arz, çift yakım. Lisans: PolyForm Shield 1.0.0.

## 3. Değişmez kurallar

Z1. PR. Her değişiklik dalda yapılır ve PR ile açılır. Owner istemedikçe yeni PR açılmaz.
Z2. Doğrulayıcı tek: GitHub CI (`cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, ek kapılar). Yerelde çalıştırılmaz (§4).
Z3. CI yumuşatma yasak. Yeni `#[allow(...)]`, yeni `#[ignore]`, eski commit sabitleme, Clippy için mantık zayıflatma ihlaldir.
Z4. Kanıt. Her "tamamlandı" commit SHA ve CI run numarası taşır. Öz beyan kanıt değildir.
Z5. Canlı durum otoritedir. Hafıza, eski rapor, rozet, scout çıktısı kanıt değildir. `git` ve GitHub araçlarıyla kontrol et.
Z6. Diller Rust ve BudL. Repoya shell script eklenmez.
Z7. `unsafe` eklenmez. Yeni bağımlılık `cargo-deny` ve `cargo-audit` geçer (CI), gerekçesi PR'a yazılır.
Z8. Konsensüs yolunda float yok, sabit nokta aritmetiği.
Z9. Token, anahtar, seed hiçbir commit, PR, dosya veya araçta açık metin olmaz.
Z10. Harici kod kopyalanmaz. Harici projeler yalnızca yöntem ilhamıdır.
Z11. PoA/kurumsal akış izinsiz kullanıcı akışından ayrı kalır.
Z12. Kamu metinlerinde "L1" veya "Layer 1" yok. Doğru ad "Budlum Evrensel Mutabakat Katmanı".
Z13. Yorum, commit ve PR kısa sade cümlelerle yazılır (ASD-STE100). Uzun tire ve markdown kalın işareti yok. Repo metni İngilizcedir (README.tr.md ve bu direktif dosyaları hariç).
Z14. Testnet aşaması (Ayaz kararı, 2026-10-09): denetim bulguları ayrıntısıyla repoda `docs/audit/` altında tutulur ve her adımda commitlenir. MODEL_ROUTING §2.6, §9 ve §12.10 bu süre boyunca bu kuralla geçersizdir. Mainnet öncesi açık bulgular özel kanala (docs/SECURITY.md) taşınır ve `docs/audit/` içindeki ayrıntı silinir. Karar Ayaz geri alana kadar sürer.

## 4. Çalışma döngüsü

Her faz MODEL_ROUTING §2.4 ajanıyla yapılır ve §2.5 beyan satırıyla başlar.

1. Keşif: önce `docs/AGENT_MAP.md`, sonra scout ile ilgili yerler. Bütün repo okunmaz.
2. Bulgu ve plan: finder veya architect. İş en fazla 64 ADIM'lık partilere bölünür, her ADIM tek odaklıdır ve MODEL_ROUTING §7 handoff'unu taşır.
3. Kod: handoff'taki ajan, önce başarısız test.
4. Doğrulama GitHub'a bırakılır. `cargo fmt`, `clippy`, `test`, `check`, typos, deny, audit, gates ve diğer CI işlerini GitHub Actions çalıştırır. Claude ve ajanlar bunları yerelde çalıştırmaz ve GitHub'ın zaten yaptığı hiçbir denetimi yinelemez. Yerelde yalnızca CI'ın göremediği iş yapılır: okuma, `rg`, `git`.
5. Opus doğrulaması: R3 kod değişikliğinde zorunlu, R2'de 3 dosyadan fazla veya durum geçişi varsa. Her seferinde yeni architect çağrısı.
6. Push ve PR: dal push edilir. Yeni PR yalnızca owner isterse açılır, aksi halde açık PR'a eklenir. Test sayıları PR'da dosya başına yazılmaz (CI yazar), PR yalnızca değişen dosyaları ve açık kararları söyler.
7. Devam: push sonrası CI sonucu GitHub araçlarıyla okunur (log yalnızca kırmızı adımdan, kırpılmış). Sonraki ADIM beklemeden başlar. Kırmızı CI Z3 ihlal edilmeden düzeltilir ve push ile GitHub'da doğrulanır.

- PR birleştirme onayı Ayaz'ındır, beklenmez. Bekleme için `sleep` döngüsü yok.
- Bağlam yaklaşık yüzde 60 dolunca `/compact`.

## 5. Durma koşulları

Seviye 1: Yap ve devam et. Aşağıda olmayan her durum.
Seviye 2: Dur, MODEL_ROUTING §8 şablonuyla sor, yalnızca: §8 listesi, mimari karar, ekonomik veya geri dönüşsüz karar, çelişen gereksinim, geri alınamaz işlem (dal silme, force push, üretim kapısı açma, geçmiş yeniden yazma). Birden fazla karar tek mesajda toplanır. Cevap beklerken bağımsız ADIM'lara devam edilir.

## 6. Kanıt standardı

Önce ham kanıt (dosya:satır, SHA, CI çıktısı), sonra yorum. Sonuç yalnızca kanıta dayanır. "Muhtemelen" ile karar verilmez.

## 7. Oturum başı

1. MODEL_ROUTING §10 ortam kontrolü.
2. STATUS.md ve docs/AGENT_MAP.md oku, `git` ile karşılaştır. Çelişkide canlı durum esastır.
3. Yönlendirme beklenmez. STATUS.md sonraki adımından §4 döngüsü çalışır. Ayaz araya girerse onun yönü esastır.
4. Sıra: kırmızı CI, bulgu avı (MODEL_ROUTING §2.6), STATUS.md kuyrukları.

## 8. Kodlama ilkeleri

- Varsayımı yaz, belirsizse sor, birden fazla yorum varsa sun. Daha basit yol varsa söyle.
- En az kod. İstenmeyen özellik, tek kullanımlık soyutlama, imkânsız durum için hata işleme yok.
- Cerrahi değişiklik: yalnızca gerekeni değiştir, komşu kodu "iyileştirme", mevcut stile uy. Ölü kod görürsen söyle, silme. Kendi değişikliğinin yetim bıraktığını temizle.
- Hedef odaklı: görevi doğrulanabilir hedefe çevir (hata için önce başarısız test), CI yeşil olana kadar döngü kur.

## 9. Mainnet'e giden yol (Ayaz başlıkları, 2026-10-09)

Sıra bağlayıcı değildir. STATUS.md kuyruğu ve kırmızı CI önce gelir (§7). Bu başlıklar §1'deki hedefin somut işleridir.

1. B.U.D. modüllerini bağlamak. Erasure coding, shard yerleşimi ve repair tetikleyici hazır ama gerçek üretim çağrı noktası yok. İlk somut iş bu olabilir.
2. VerifyMerkle 64 derinlik kapısını üretime almak. Gerçek Proof-of-Storage ve PQ toplama katmanı buna bağlıdır. Tek iş iki engeli birden açar. Kapıyı açmak geri alınamaz işlemdir, §5 gereği önce sorulur.
3. Domain imza toplama katmanını yazmak. PoW, PoS, BFT ve PoA finality kanıtları Merkle ağacına girer. Tek PQ imza kök üzerinde durur.

@MODEL_ROUTING.md
