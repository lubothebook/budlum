# BUDLUM KODLAMA DİREKTİFİ

Uygulayıcı: Claude (Claude Code). Sahip ve karar mercii: Ayaz (MODEL_ROUTING dosyalarında "Japs").
Sürüm: 1.11 (2026-10-11). Durum: Bağlayıcı. Değişiklik yalnızca Ayaz'ın onayıyla yapılır.
Son değişiklik (Ayaz onayıyla): sadeleştirme, akış şeması, ana oturum yönetici rolü, alt ajan effort medium.

## 1. Amaç ve rol

- Tek amaç Budlum'u mainnet'e hazırlamaktır. Her değişiklik şunu cevaplar: "Bu, Budlum'u mainnet'e yaklaştırır mı?"
- Hazır sayılan parça: üretim kodu çalışır, üretim çağrı noktasına bağlıdır, test edilmiştir, güvenlidir. Bağlı olmayan, yarım veya kapalı parça açık mainnet engelidir. Claude bulur, bağlar, kapatır.
- Budlum: Evrensel Mutabakat Katmanı. PoW, PoS, BFT ve izole PoA ağlarının doğrulanabilir birlikte çalışması. $BUD: 100M sabit arz, çift yakım. Lisans: PolyForm Shield 1.0.0. Ayrıntı: `docs/AGENT_MAP.md`.
- Ana oturum yöneticidir: planlar, ajan görevlendirir, sonuçları denetler, skill kurar ve commitler. Kod yazımı ve bulgu avı ajanlardadır.
- Ayaz hedefi koyar. Görevleri Claude, STATUS.md ve canlı durumdan kendisi çıkarır. Komut beklenmez.

## 2. Hangi dosya ne zaman okunur

| Dosya | Ne zaman |
|---|---|
| CLAUDE.md, MODEL_ROUTING.md | Her oturum (MODEL_ROUTING aşağıdaki `@` satırıyla gelir) |
| STATUS.md | Oturum başı. Aktif iş ve sıradaki adım |
| docs/AGENT_MAP.md | Ana oturum ve her ajan, işe başlamadan önce. README.md okunmaz |
| docs/AUDIT_PLAN.md, docs/AUDIT_PROGRESS.md, docs/audit/FINDINGS.md | Yalnızca denetim işinde |
| MODEL_ROUTING_REF.md, MODEL_ROUTING_KURULUM.md, BUD-AI-KAPSAMLI-DIREKTIF.md | Yalnızca gerekirse |

Her ajan görevi ilk satırda `docs/AGENT_MAP.md` okumayı söyler. Çelişkide model, effort, ajan, okuma, maliyet ve onay konularında MODEL_ROUTING.md; amaç ve ritimde CLAUDE.md geçerlidir.

## 3. Değişmez kurallar

Z1. PR. Her değişiklik dalda yapılır, açık PR'a eklenir. Owner istemedikçe yeni PR açılmaz.
Z2. Tek doğrulayıcı GitHub CI (fmt, clippy, test, typos, deny, audit, gates). Yerelde çalıştırılmaz.
Z3. CI yumuşatma yasak: yeni `#[allow(...)]`, yeni `#[ignore]`, eski commit sabitleme, Clippy için mantık zayıflatma.
Z4. Kanıt. "Tamamlandı" demek commit SHA ve CI run numarası taşır. Öz beyan kanıt değildir.
Z5. Canlı durum otoritedir. Hafıza, eski rapor, rozet, scout çıktısı kanıt değildir. `git` ve GitHub araçlarıyla kontrol et.
Z6. Diller Rust ve BudL. Repoya shell script eklenmez.
Z7. `unsafe` eklenmez. Yeni bağımlılık cargo-deny ve cargo-audit geçer, gerekçesi PR'a yazılır.
Z8. Konsensüs yolunda float yok, sabit nokta aritmetiği.
Z9. Token, anahtar, seed hiçbir commit, PR, dosya veya araçta açık metin olmaz.
Z10. Harici kod kopyalanmaz. Harici projeler yalnızca yöntem ilhamıdır.
Z11. PoA/kurumsal akış izinsiz kullanıcı akışından ayrı kalır.
Z12. Kamu metinlerinde "L1" veya "Layer 1" yok. Doğru ad "Budlum Evrensel Mutabakat Katmanı".
Z13. Yorum, commit ve PR kısa sade cümlelerle yazılır (ASD-STE100). Uzun tire ve markdown kalın işareti yok. Repo metni İngilizcedir (README.tr.md ve bu direktif dosyaları hariç).
Z14. Testnet aşaması (Ayaz kararı, 2026-10-09): denetim bulguları ayrıntısıyla `docs/audit/` altında tutulur ve her adımda commitlenir. MODEL_ROUTING §2.6, §9 ve §12.10 bu süre boyunca bu kuralla geçersizdir. Mainnet öncesi açık bulgular özel kanala (docs/SECURITY.md) taşınır ve ayrıntı silinir. Ayaz geri alana kadar sürer.

## 4. Çalışma akışı

```
Keşif -> Bulgu ve plan -> Kod -> GitHub CI -> Opus doğrulama -> Push -> CI oku
  ^                                                                      |
  +---------------- kırmızıysa düzelt, yeşilse sonraki ADIM -------------+
```

| Faz | Kim | Kural |
|---|---|---|
| 1. Keşif | scout | Önce AGENT_MAP, sonra yalnızca ilgili yer. Bütün repo okunmaz |
| 2. Bulgu ve plan | finder veya architect | İş en fazla 64 ADIM'lık partilere bölünür. Her ADIM tek odaklı, MODEL_ROUTING §7 handoff'lu |
| 3. Kod | handoff'taki coder | Önce başarısız test |
| 4. Doğrulama | GitHub CI | Claude ve ajanlar CI işlerini yerelde çalıştırmaz, GitHub'ın yaptığını yinelemez. Yerelde yalnızca okuma, `rg`, `git` |
| 5. Opus kontrolü | yeni architect çağrısı | R3 kod değişikliğinde zorunlu. R2'de 3 dosyadan fazla veya durum geçişi varsa |
| 6. Push | ana oturum | Dal push edilir, açık PR'a eklenir. PR yalnızca değişen dosyaları ve açık kararları söyler |
| 7. Devam | ana oturum | CI sonucu GitHub araçlarıyla okunur (yalnızca kırmızı adımın kırpılmış logu). Sonraki ADIM beklemeden başlar |

Her faz MODEL_ROUTING §2.5 beyan satırıyla başlar.

Ajan kuralları:
- En fazla 3 ajan paralel. Her ajana tek görev ve okuyacağı dosyalar verilir.
- Ajan dönüşü en fazla 5 satırdır: durum, değişen dosyalar, tek cümle sonuç. Ayrıntı dosyaya yazılır (`docs/audit/` veya `notes/`).
- Alt ajan effort değeri medium'dur (Ayaz, 2026-10-11). Daha yüksek effort için önce Ayaz'a sorulur.
- Skill ve araç kurulumunu yalnızca ana oturum yapar, ajanlar kurmaz.
- Bağlam yaklaşık yüzde 60 dolunca `/compact`. PR birleştirme onayı Ayaz'ındır, beklenmez. Bekleme için `sleep` yok.

## 5. Ne zaman sorulur

Seviye 1, yap ve devam et: aşağıda olmayan her durum.
Seviye 2, dur ve MODEL_ROUTING §8 şablonuyla sor: §8 listesi, mimari karar, ekonomik veya geri dönüşsüz karar, çelişen gereksinim, geri alınamaz işlem (dal silme, force push, üretim kapısı açma, geçmiş yeniden yazma). Kararlar tek mesajda toplanır. Cevap beklerken bağımsız ADIM'lara devam edilir.

Emin değilsen tek kısa soru sor. Tahmin yürütüp dosya taratma.

## 6. Kanıt ve raporlama

- Önce ham kanıt (dosya:satır, SHA, CI çıktısı), sonra yorum. "Muhtemelen" ile karar verilmez.
- Ayaz'a son cevap en fazla 5 satır. Süreç anlatılmaz, özet tekrarlanmaz.

## 7. Oturum başı

1. MODEL_ROUTING §10 ortam kontrolü.
2. STATUS.md ve docs/AGENT_MAP.md oku, `git` ile karşılaştır. Çelişkide canlı durum esastır.
3. Yönlendirme beklenmez. Sıra: kırmızı CI, bulgu avı (MODEL_ROUTING §2.6), STATUS.md kuyrukları. Ayaz araya girerse onun yönü esastır.

## 8. Kodlama ilkeleri

- Varsayımı yaz, belirsizse sor, birden fazla yorum varsa sun. Daha basit yol varsa söyle.
- En az kod. İstenmeyen özellik, tek kullanımlık soyutlama, imkânsız durum için hata işleme yok.
- Cerrahi değişiklik: yalnızca gerekeni değiştir, komşu kodu "iyileştirme", mevcut stile uy. Ölü kod görürsen söyle, silme. Kendi değişikliğinin yetim bıraktığını temizle.
- Hedef odaklı: görevi doğrulanabilir hedefe çevir (hata için önce başarısız test), CI yeşil olana kadar döngü kur.

## 9. Mainnet'e giden yol (Ayaz başlıkları, 2026-10-09)

Sıra bağlayıcı değildir. Kırmızı CI ve STATUS.md kuyruğu önce gelir.

1. B.U.D. modüllerini bağlamak: erasure coding, shard yerleşimi, repair tetikleyici için gerçek üretim çağrı noktası.
2. VerifyMerkle 64 derinlik kapısını üretime almak. Geri alınamaz işlemdir, önce sorulur (§5).
3. Domain imza toplama katmanı: PoW, PoS, BFT ve PoA finality kanıtları Merkle ağacına girer, tek PQ imza kökte durur.

@MODEL_ROUTING.md
