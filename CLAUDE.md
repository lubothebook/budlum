# BUDLUM KODLAMA DİREKTİFİ

Uygulayıcı: Claude (Claude Code)
Sahip ve karar mercii: Ayaz. MODEL_ROUTING dosyalarında "Japs" olarak geçer.
Sürüm: 1.3 (2026-10-07)
Durum: Bağlayıcı. Pazarlığa kapalı.

---

## 0. Direktif seti

| Dosya | Rol | Ne zaman okunur |
|---|---|---|
| CLAUDE.md | Amaç, Budlum tanımı, değişmez kurallar, çalışma ritmi | Her oturum |
| MODEL_ROUTING.md | Model, effort, ajan, okuma, maliyet ve onay kuralları | Her oturum (bu dosyanın sonundaki `@` satırı ile) |
| MODEL_ROUTING_KURULUM.md | Kurulum ve ek araçlar | Yalnızca kurulum oturumunda |
| BUD-AI-KAPSAMLI-DIREKTIF.md | BUD (B.U.D.) uygulama direktifi: sürümler, kapılar, öncelik sıfır | BUD işi yapılırken; her oturumda @ ile bağlanmaz |

- Çelişki kuralı: model, effort, ajan, okuma, maliyet ve onay konularında MODEL_ROUTING.md geçerlidir. Amaç ve çalışma ritmi konularında CLAUDE.md geçerlidir.
- Aktif iş STATUS.md dosyasında tutulur. Kalıcı kararlar memanto'da tutulur.
- Repo, dal ve görev yönlendirmesini Ayaz yapar.
- Bu dosyada değişiklik yalnızca Ayaz'ın onayı ile yapılır.

---

## 1. Amaç: Mainnet hazırlığı

- Tüm çalışmanın tek amacı Budlum'u mainnet'e hazırlamaktır.
- Her değişiklik şu soruya cevap verir: "Bu, Budlum'u mainnet'e bir adım yaklaştırır mı?"
- Mainnet ölçütü: üretim kodu çalışır, bağlıdır, test edilmiştir ve güvenlidir.
- Kodlu ama üretim çağrı noktasına bağlı olmayan parçalar mainnet'e hazır sayılmaz. Claude bunları bağlar.
- Deneysel, yarım veya kapalı kalan her parça açık bir mainnet engelidir. Claude bunları tespit eder ve kapatır.

---

## 2. Budlum nedir

- Budlum, Evrensel Mutabakat Katmanıdır (Universal Settlement Layer).
- Budlum, farklı konsensüs ağlarının (PoW, PoS, BFT ve izole PoA) doğrulanabilir şekilde birlikte çalışmasını sağlar.
- Budlum konsensüs paradigmalarını değiştirmez. Budlum bunların üstünde doğrulanabilir bir kesinlik (finality) ve mutabakat katmanı kurar.
- Temel yapı taşları: `DomainFinalityAdapter`, `GlobalBlockHeader`, `ReplayNonceStore`, `CrossDomainMessage`, `DomainStatus`.
- Odak: veri egemenliği ve kuantum direnci.
- Ad kökeni: "bud" (tomurcuk) + "lum" (ışık). Yeni adlandırmalar bu temadan beslenir.

| Bileşen | Kısa tanım |
|---|---|
| Çekirdek | Çok alanlı konsensüs ve mutabakat katmanı. Rust. |
| B.U.D. | İçerik adresli, erasure kodlu depolama katmanı. Kodlu ve testli parçaların bir kısmı henüz üretim çağrı noktasına bağlı değil. |
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

Z3. CI yumuşatma yasaktır. Yeni `#[allow(...)]`, yeni `#[ignore]`, eski commit sabitleme veya Clippy'yi geçmek için mantık zayıflatma bir ihlaldir. Bunlar düzeltme sayılmaz.

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

Her faz MODEL_ROUTING.md §2.4 tablosundaki ajanla yapılır. Her faz §2.5 beyan satırı ile başlar.

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
- Başarısız denemelerde MODEL_ROUTING.md §5 yükselme merdiveni uygulanır.
- PR birleştirme onayı Ayaz'ındır. Onay beklenmez. Çalışma devam eder.
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

Birden fazla karar varsa tek mesajda toplanır. Cevap beklerken bağımsız ADIM'lara devam edilir.

---

## 6. Analiz ve kanıt standardı

- Araştırma ve yorum ayrı aşamalardır.
- Aşama 1: Ham kanıt toplanır. Dosya:satır, komut çıktısı, SHA.
- Aşama 2: Sonuç yalnızca Aşama 1 kanıtına dayanır.
- Kanıtsız sonuç raporlanmaz. "Muhtemelen" ile karar verilmez.

---

## 7. Oturum başlangıcı

1. Kurulum yapılmadıysa: MODEL_ROUTING_KURULUM.md §9 sırasıyla kurulum yapılır. Kurulum bitmeden kod işine geçilmez.
2. MODEL_ROUTING.md §10 ortam kontrolü ve `memanto status` çalışır.
3. STATUS.md okunur. Canlı durum `git` ve `gh` ile karşılaştırılır. Çelişkide canlı durum esastır.
4. Ayaz'ın yönlendirmesi beklenir.
5. Yönlendirme gelince Bölüm 4 döngüsü aralıksız çalışır. Hedef mainnet'tir.

## Karpathy ilkeleri (kaynak: multica-ai/andrej-karpathy-skills)
Behavioral guidelines to reduce common LLM coding mistakes. Merge with project-specific instructions as needed.

**Tradeoff:** These guidelines bias toward caution over speed. For trivial tasks, use judgment.

## 1. Think Before Coding

**Don't assume. Don't hide confusion. Surface tradeoffs.**

Before implementing:
- State your assumptions explicitly. If uncertain, ask.
- If multiple interpretations exist, present them - don't pick silently.
- If a simpler approach exists, say so. Push back when warranted.
- If something is unclear, stop. Name what's confusing. Ask.

## 2. Simplicity First

**Minimum code that solves the problem. Nothing speculative.**

- No features beyond what was asked.
- No abstractions for single-use code.
- No "flexibility" or "configurability" that wasn't requested.
- No error handling for impossible scenarios.
- If you write 200 lines and it could be 50, rewrite it.

Ask yourself: "Would a senior engineer say this is overcomplicated?" If yes, simplify.

## 3. Surgical Changes

**Touch only what you must. Clean up only your own mess.**

When editing existing code:
- Don't "improve" adjacent code, comments, or formatting.
- Don't refactor things that aren't broken.
- Match existing style, even if you'd do it differently.
- If you notice unrelated dead code, mention it - don't delete it.

When your changes create orphans:
- Remove imports/variables/functions that YOUR changes made unused.
- Don't remove pre-existing dead code unless asked.

The test: Every changed line should trace directly to the user's request.

## 4. Goal-Driven Execution

**Define success criteria. Loop until verified.**

Transform tasks into verifiable goals:
- "Add validation" → "Write tests for invalid inputs, then make them pass"
- "Fix the bug" → "Write a test that reproduces it, then make it pass"
- "Refactor X" → "Ensure tests pass before and after"

For multi-step tasks, state a brief plan:
```
1. [Step] → verify: [check]
2. [Step] → verify: [check]
3. [Step] → verify: [check]
```

Strong success criteria let you loop independently. Weak criteria ("make it work") require constant clarification.

---

**These guidelines are working if:** fewer unnecessary changes in diffs, fewer rewrites due to overcomplication, and clarifying questions come before implementation rather than after mistakes.

@MODEL_ROUTING.md
