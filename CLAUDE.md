# BUDLUM KODLAMA DİREKTİFİ

Uygulayıcı: Claude (Claude Code)
Sahip ve karar mercii: Ayaz. MODEL_ROUTING dosyalarında "Japs" olarak geçer.
Sürüm: 1.4, 2026-10-08, sadeleştirildi (sahip isteği)
Durum: Bağlayıcı. Pazarlığa kapalı.

---

## 0. Direktif seti

| Dosya | Rol | Ne zaman okunur |
|---|---|---|
| CLAUDE.md | Amaç, Budlum tanımı, değişmez kurallar, çalışma ritmi | Her oturum |
| MODEL_ROUTING.md | Model, effort, ajan, okuma, maliyet ve onay kuralları | Her oturum (sondaki `@` satırı ile) |
| MODEL_ROUTING_KURULUM.md | Kurulum ve ek araçlar | Yalnızca kurulum oturumu |
| BUD-AI-KAPSAMLI-DIREKTIF.md | BUD (B.U.D.) uygulama direktifi: sürümler, kapılar, öncelik sıfır | BUD işinde; @ ile bağlanmaz |

- Çelişki: model, effort, ajan, okuma, maliyet ve onay konularında MODEL_ROUTING.md geçerlidir. Amaç ve çalışma ritminde CLAUDE.md geçerlidir.
- Aktif iş STATUS.md dosyasındadır. Kalıcı kararlar memanto'dadır.
- Repo, dal ve görev yönlendirmesini Ayaz yapar.
- Bu dosyada değişiklik yalnızca Ayaz'ın onayı ile yapılır.

---

## 1. Amaç: Mainnet hazırlığı

- Tüm çalışmanın tek amacı Budlum'u mainnet'e hazırlamaktır. Her değişiklik şuna cevap verir: "Bu, Budlum'u mainnet'e bir adım yaklaştırır mı?"
- Mainnet ölçütü: üretim kodu çalışır, bağlıdır, test edilmiştir ve güvenlidir.
- Kodlu ama üretim çağrı noktasına bağlı olmayan parça mainnet'e hazır sayılmaz. Claude bunu bağlar.
- Deneysel, yarım veya kapalı kalan her parça açık bir mainnet engelidir. Claude bunu tespit eder ve kapatır.

---

## 2. Budlum nedir

- Budlum, Evrensel Mutabakat Katmanıdır (Universal Settlement Layer). Farklı konsensüs ağlarının (PoW, PoS, BFT ve izole PoA) doğrulanabilir şekilde birlikte çalışmasını sağlar.
- Budlum konsensüs paradigmalarını değiştirmez. Bunların üstünde doğrulanabilir bir kesinlik (finality) ve mutabakat katmanı kurar.
- Temel yapı taşları: `DomainFinalityAdapter`, `GlobalBlockHeader`, `ReplayNonceStore`, `CrossDomainMessage`, `DomainStatus`.
- Odak: veri egemenliği ve kuantum direnci.
- Ad: "bud" (tomurcuk) + "lum" (ışık). Yeni adlandırmalar bu temadan beslenir.

| Bileşen | Kısa tanım |
|---|---|
| Çekirdek | Çok alanlı konsensüs ve mutabakat katmanı. Rust. |
| B.U.D. | İçerik adresli, erasure kodlu depolama katmanı. Kodlu ve testli parçaların bir kısmı üretim çağrı noktasına bağlı değil. |
| BudZero / BudZKVM | Plonky3/STARK tabanlı ZK sanal makine. Alanların durum geçişlerini kanıtlar. VerifyMerkle 64 derinlik kapısı üretimde kapalı. |
| BudL | Budlum'un kendi programlama dili. |
| Cüzdan | Ortak Rust çekirdeği. Mobil ve tarayıcı eklentisi hedefleri. |
| Lubot | Cihaz içinde çalışan yapay zekâ bileşeni. |

- $BUD: 100M sabit arz, çift yakım mekanizması.
- Lisans: PolyForm Shield 1.0.0.

---

## 3. Değişmez kurallar

Z1. PR. Her değişiklik dal üzerinde yapılır ve PR ile açılır.

Z2. Tek doğrulayıcı. CI (`cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`) tek yetkili doğrulayıcıdır.

Z3. CI yumuşatma yasaktır. Yeni `#[allow(...)]`, yeni `#[ignore]`, eski commit sabitleme veya Clippy'yi geçmek için mantık zayıflatma ihlaldir. Bunlar düzeltme sayılmaz.

Z4. Kanıt. Her "tamamlandı" iddiası commit SHA ve CI run numarası taşır. Öz beyan kanıt değildir.

Z5. Canlı durum otoritedir. Hafıza, eski raporlar, README rozetleri ve scout çıktısı kanıt değildir. Durum `git` ve `gh` ile canlı kontrol edilir.

Z6. Diller. Kod dilleri Rust ve BudL'dir. Repoya shell script dosyası eklenmez.

Z7. Güvenli kod. `unsafe` kod eklenmez. Yeni bağımlılık `cargo-deny` ve `cargo-audit` denetiminden geçer. Gerekçesi PR açıklamasına yazılır.

Z8. Determinizm. Konsensüs yolunda kayan nokta (float) kullanılmaz. Sabit nokta aritmetiği kullanılır.

Z9. Gizli bilgiler. Token, anahtar ve seed hiçbir commit, PR açıklaması, dosya veya araçta açık metin olarak bulunmaz.

Z10. Lisans. Harici kod Budlum kaynak koduna kopyalanmaz. Harici projeler yalnızca yöntem ilhamıdır.

Z11. İzolasyon. PoA/kurumsal akış izinsiz kullanıcı akışından ayrı kalır.

Z12. Kamu dili. Kamuya açık metinlerde "L1" veya "Layer 1" kullanılmaz. Doğru ad "Budlum Evrensel Mutabakat Katmanı"dır.

Z13. Yazım. Yorumlar, commit mesajları ve PR açıklamaları kısa ve sade cümlelerle yazılır (ASD-STE100). Uzun tire ve markdown kalın işareti kullanılmaz.

---

## 4. Aralıksız çalışma döngüsü

Her faz MODEL_ROUTING.md §2.4 tablosundaki ajanla yapılır ve §2.5 beyan satırı ile başlar.

1. Keşif. scout ile ilgili yerleri bul. MODEL_ROUTING.md §6 okuma kurallarına uy. Bütün repo okunmaz.
2. Bulgu ve plan. Kademeye göre finder veya architect çalışır (§2.6, §3). İş en fazla 64 ADIM'lık partilere bölünür. Her ADIM tek odaklıdır ve §7 handoff'u ile tanımlanır.
3. Kod. Handoff'taki ajan kodu yazar. Önce başarısız test yazılır.
4. Yerel doğrulama. Hedefli test, `cargo fmt --check` ve clippy çalışır. Bütün `cargo test` çalışmaz.
5. Opus doğrulaması. R3'te zorunlu, R2'de §3 koşulunda. Her seferinde yeni architect çağrısı.
6. PR. Dal push edilir, PR açılır. Test sayıları dosya başına PR açıklamasına yazılır.
7. Devam. CI sonucu beklenmez. Sonraki ADIM hemen başlar.

Ek kurallar:
- Açık PR'ların CI sonuçları ADIM'lar arasında kontrol edilir. Kırmızı CI, Z3 ihlal edilmeden düzeltilir.
- Bağımlı işler için yığılmış dallar kullanılır.
- Başarısız denemelerde MODEL_ROUTING.md §5 merdiveni uygulanır.
- PR birleştirme onayı Ayaz'ındır. Onay beklenmez, çalışma devam eder.
- Bağlam veya tur kapasitesi durma nedeni değildir. Bağlam yaklaşık yüzde 60 dolunca `/compact` yapılır. Ayaz istediği an araya girer.
- Bekleme için `sleep` döngüsü kullanılmaz.

---

## 5. Durma koşulları

Seviye 1: Yap ve devam et. Aşağıda olmayan her durum.

Seviye 2: Dur ve MODEL_ROUTING.md §8 şablonu ile sor. Yalnızca şu durumlarda:
1. MODEL_ROUTING.md §8 listesindeki durumlar.
2. Mimari karar.
3. Ekonomik veya geri dönüşsüz karar.
4. Çelişen gereksinimler veya somut kapsam çakışması.
5. Geri alınamaz işlem: dal silme, force push, üretim kapısı açma, geçmiş yeniden yazma.

- Birden fazla karar varsa tek mesajda toplanır. Cevap beklerken bağımsız ADIM'lara devam edilir.
- Ayaz'a sorular sade, teknik olmayan dille yazılır. Seçenekler A/B/C/D olur ve önerilen seçenek işaretlenir.

---

## 6. Analiz ve kanıt standardı

- Araştırma ve yorum ayrı aşamalardır.
- Aşama 1: Ham kanıt toplanır (dosya:satır, komut çıktısı, SHA).
- Aşama 2: Sonuç yalnızca Aşama 1 kanıtına dayanır.
- Kanıtsız sonuç raporlanmaz. "Muhtemelen" ile karar verilmez.

---

## 7. Oturum başlangıcı

1. Kurulum yapılmadıysa MODEL_ROUTING_KURULUM.md §9 sırasıyla yapılır. Kurulum bitmeden kod işine geçilmez.
2. MODEL_ROUTING.md §10 ortam kontrolü ve `memanto status` çalışır.
3. STATUS.md okunur ve canlı durumla (`git`, `gh`) karşılaştırılır. Çelişkide canlı durum esastır.
4. Ayaz'ın yönlendirmesi beklenir.
5. Yönlendirme gelince Bölüm 4 döngüsü aralıksız çalışır. Hedef mainnet'tir.

## Karpathy ilkeleri (kaynak: multica-ai/andrej-karpathy-skills)

Bu ilkeler hızdan çok dikkati öne alır. Önemsiz işte yargı kullan.

1. Kodlamadan önce düşün.
- Varsayımı açıkça yaz. Belirsizse sor. Birden fazla yorum varsa sessizce seçme, sun.
- Daha basit yol varsa söyle.

2. Önce sadelik.
- İstenenden fazlasını yazma. Tek kullanımlık kod için soyutlama kurma.
- 200 satır 50 olabiliyorsa yeniden yaz.

3. Cerrahi değişiklik.
- Yalnızca gerekeni değiştir. Komşu kodu, yorumu ve biçimi "iyileştirme".
- Değişen her satır isteğe bağlanabilmeli. Kendi yarattığın artığı temizle, eski ölü kodu yalnızca bildir.

4. Hedefe dayalı yürütme.
- Başarı ölçütünü tanımla ve doğrulanana kadar döngüde kal. Hata düzeltmede önce hatayı yeniden üreten test yaz.
- Çok adımlı işte kısa plan yaz: adım, doğrulama.

@MODEL_ROUTING.md
