# MODEL YÖNLENDİRME (Budlum, Claude Code)

Sürüm: SERT-4, 9 Ekim 2026 (sadeleştirildi). Bu dosya her turda bağlama girer, bu yüzden kısadır. Tam metin ve gerekçe: `MODEL_ROUTING_REF.md` (bağlanmaz). Kurulum: `MODEL_ROUTING_KURULUM.md`.
Statü: Emirdir. Sapma yalnızca §8 şablonu ve Ayaz (Japs) yanıtıyla. Karşılığı olmayan durumda dur, §8 kullan, varsayma.

## 1. Ajanlar

| Ajan | Model | Effort | Görev | Yapmaz |
|---|---|---|---|---|
| scout | Haiku | yok | grep, sembol, dosya haritası, log kırpma | karar, kod, güvenlik yorumu |
| finder | Opus | xhigh | R3 bulgu avı | kod |
| architect | Opus | high | plan, handoff, R2 bulgu, bulgu ve diff doğrulama | kod, mekanik iş, keşif |
| finder-max | Opus | max | yalnızca §5 istisnası | rutin iş |
| coder | Sonnet | medium | handoff'a göre kod ve test; denetimde salt okuma | mimari karar |
| coder-deep | Sonnet | high | R3 kodu, "Karmaşıklık: yüksek"; R3 denetim okuması | R3 dışı kod |
| coder-lite | Sonnet | low | mekanik iş | davranış değişikliği, R3 |

- Mekanik iş: biçim, yeniden adlandırma, doküman, import düzeni, clippy'nin açıkça önerdiği düzeltme. Davranış değişmez. R3'te mekanik iş yoktur.
- "Karmaşıklık: yüksek" (architect işaretler): iki veya daha fazla modül, durum geçişi veya imza/doğrulama mantığı değişiyor, ya da değişmezler listesi 3 maddeyi aşıyor.
- scout yalnızca belirli sembol, çağrı yeri veya dosya bulmak içindir. Toplu Haiku taraması yapılmaz. Bulgu avı Opus ile yapılır.
- Opus ajanları dosya yazmaz ve ajan başlatmaz. Dosyaları ve ajan başlatmayı ana oturum yapar. Opus kod yazmaz.
- Ana oturum `opusplan`. Plan ve doğrulama `architect` ajanına devredilir.
- Aynı anda en çok 3 ajan çalışır.
- Fable ve "Requires usage credits" yazan model seçilmez.

## 2. Effort

2.1 Effort'u Claude kendisi seçer. Ayaz `/effort` kullanmaz. Gereken effort ana oturumunkinden farklıysa iş ilgili ajana devredilir.

2.4 Seçim tablosu.

| Faz | Kademe | Ajan |
|---|---|---|
| Keşif | R0 ile R3 | scout |
| Bulgu avı | R3 ilk tarama veya §2.6 yüzeyi değişti | finder |
| Bulgu avı | R3 delta tarama, R2 | architect |
| Plan, handoff | R3, R2 | architect |
| Kod | R3 normal | coder |
| Kod | R3 Karmaşıklık: yüksek | coder-deep |
| Kod | R2, R1 | coder |
| Mekanik | R2, R1, R0 | coder-lite |
| Doğrulama | R3 zorunlu, R2 (§3 koşulu) | architect, her seferinde yeni çağrı |
| İstisna | §5 koşulu | finder-max |

2.5 Faz beyanı. Her faz başlamadan önce tek satır yazılır:
`FAZ: <keşif|bulgu|plan|kod|mekanik|doğrulama> | KADEME: R<0-3> | AJAN: <ad> | EFFORT: <seviye> | NEDEN: <en fazla 8 kelime>`

2.6 Bulgu avı. finder: R3 modülünde ilk tarama, ya da önceki taramadan sonra imza/doğrulama, konsensüs durum makinesi, tokenomics parametresi veya kalıcılık formatı değiştiyse. architect: önceki tarama kaydı varsa ve yalnızca `git diff <son-taranan-commit>..HEAD` taranıyorsa; ve tüm R2 taramalarında. R1 ve R0'da bulgu avı yapılmaz. Claude bulgu avını Ayaz'dan istemeden başlatır: R3 modülleri sırayla taranır, her seferinde bir finder. Tam kod denetimi için `docs/AUDIT_PLAN.md` kullanılır. Bulgu raporları repoya yazılmaz (§12.10), scratchpad'te tutulur ve özel kanala (docs/SECURITY.md) bildirilir.

2.7 Handoff'taki "Ajan" alanı bağlayıcıdır. Boşsa Claude tablodan seçer ve beyan eder.

2.8 Opus çözüm verdikten veya doğruladıktan sonra uygulama her zaman Sonnet ajanına döner.

2.9 Bütünlük. Ajan transkriptindeki model ve effort, ajan dosyasıyla eşleşmelidir. Doğrulama: ilk kurulum, her `claude update`, her ajan dosyası değişikliği. Eşleşmezse dur ve §8 kullan.

2.10 Her finder-max çağrısı STATUS.md "EFFORT LOG" başlığına tek satır yazılır: tarih, görev, ajan, effort, neden.

2.11 Claude kendi başına yapar: ajan seçimi, §5 merdiveni, finder-max. Claude §8 ile sorar: ajan dosyalarında effort değişikliği, R3'te coder-lite, finder'ı high altına indirmek, yeni ajan eklemek.

## 3. Kademeler

| Kademe | Alanlar | Akış |
|---|---|---|
| R3 | `consensus`, `crypto`, `privacy`, `tokenomics`, `settlement`, `cross_domain`, `registry`, `chain` (snapshot ve kalıcılık), `network/node.rs`, `budzero/bud-proof` | scout, bulgu, handoff (architect), kod (coder veya coder-deep), doğrulama (architect, zorunlu, yeni çağrı) |
| R2 | `execution`, `core`, `rpc`, `storage`, `domain`, `account_abstraction`, `ai`, `ai_inference` | scout, plan (architect, kısa), kod (coder), doğrulama (architect) yalnızca 3 dosyadan fazla veya durum geçişi değişiyorsa |
| R1 | `bns`, `socialfi`, `pollen`, `gateway`, `relayer`, `bin`, `docs`, `config`, ek testler | ana oturum Sonnet veya coder, plan modu yok |
| R0 | grep, liste, log kırpma, yeniden adlandırma | scout veya coder-lite |

Yanlış alan görürsen düzeltmeyi §8 ile öner. `.github/*-baseline.txt` dosyalarına dokunulmaz, gevşetme §8 onayı ister.

## 4. Bulgu döngüsü

1. finder veya architect tek modülde bulguları listeler (id, yol:satır, etki, kanıt).
2. architect her bulgu için ayrı handoff yazar.
3. coder, coder-deep veya coder-lite yalnızca o bulguyu düzeltir.
4. Yeni bir architect çağrısı diff'i doğrular. Bulan bağlam kendi düzeltmesini onaylamaz.
5. Sonraki bulgu.

finder ve finder-max asla paralel çalışmaz.

## 5. Yükselme merdiveni

```
coder, coder-deep veya coder-lite aynı hatada 2 kez başarısız -> hata raporuyla architect (high); uygulama yine Sonnet'te
finder veya architect 2 ayrı denemede çözemedi, ya da R3 bulgusu doğrulamada "belirsiz" kaldı -> finder-max, tek görev, bulgu başına en fazla 1 çağrı
finder-max sonuçsuz -> DUR, §8: A) kapsamı küçült  B) Japs karar verir  C) ertele
scout şüphe veya çelişki bildirdi -> coder'a devret
```

finder-max çağrı metninin ilk satırı `max: <neden>` olur ve §2.10'a göre kaydedilir. İş bitince alta inilir.

## 6. Okuma kuralları

Yaklaşık 747 `.rs` dosyası, 367 bin satır. Tüm repo okunmaz veya özetlenmez.

Bütün okunmayacak büyük dosyalar (100 KB üstü): `budzero/bud-proof/src/plonky3_prover.rs`, `src/chain/blockchain.rs`, `src/rpc/server.rs`, `src/domain/storage_deal.rs`, `src/network/node.rs`, `src/chain/chain_actor.rs`, `src/ai/mod.rs`, `budzero/bud-proof/src/plonky3_air.rs`, `src/core/account.rs`, `src/execution/executor.rs`, `docs/ARCHITECTURE.md`.

1. `rg -n "<sembol>" src/` ile bul, `sed -n 'a,bp'` ile yalnızca gerekli aralığı oku.
2. ARCHITECTURE.md: önce `rg -n '^## '`, sonra ilgili bölüm.
3. `git diff --stat` önce, tam diff dosya dosya.
4. Çıktıyı kırp: `cargo test <filtre> 2>&1 | tail -n 40`, `cargo clippy --message-format=short`.
5. `cargo test` bütün çalışmaz, hedefli çalışır. Tam koşu yalnızca kapanışta ve §8 onayıyla.
6. `rust-toolchain.toml` sürümüyle çalış, rustfmt ve clippy çıktısı tahmin edilmez.
7. `target/` ve `Cargo.lock` okunmaz.
8. Scout ve dış ajan çıktısı kanıt değildir. Düzenlemeden önce ilgili satırları bizzat oku.
9. Denetim ajanları parçadan büyük okuma yapmaz: çağrı başına en çok 1500 satır, parça dışında önce `rg`, en çok 80 satır.

## 7. Handoff şablonu (architect yazar, en fazla 40 satır, dosya yazmaz)

Ana oturum handoff'u STATUS.md'ye yazar.

```
## ADIM <no>: <başlık>
Hedef: <1 cümle>
Bulgu (varsa): <id, yol:satır, etki>
Varsayımlar: <açıkça yaz>
Dosyalar: <yol:satır_aralığı>, ...
Değişmezler (bozulmayacak): <liste>
Yapılacak: 1) ... 2) ...
Yapılmayacak: <kapsam dışı>
Test komutu: <hedefli komut> (önce başarısız olan test)
Karmaşıklık: normal | yüksek | mekanik
Ajan: coder | coder-deep | coder-lite
Bitti ölçütü: <gözlemlenebilir>
Opus doğrulaması gerekli mi: evet | hayır
```

## 8. Onay şablonu

Dur ve sor: 8 dosyadan fazla değişiklik, tokenomics veya protokol parametresi, `main`'e push, dosya silme, baseline gevşetme, memanto'yu Cloud'a geçirme, ajan dosyasında effort değişikliği, R3'te coder-lite, "Requires usage credits" modeli, ortam kontrolünde sapma, §2.9 eşleşmezliği.
Tam kod denetimi için ayrı onay gerekmez, Ayaz'ın talimatı yeterlidir.

```
KARAR GEREKİYOR: <başlık>
Durum: <1-2 cümle>
A) <seçenek>, maliyet: ...
B) <seçenek>, maliyet: ...
C) Dur, ben karar vereyim
Önerim: <harf>, çünkü <tek cümle>
```

## 9. Raporlama

- Rapor PR açıklamasına yazılır, ayrı rapor dosyası açılmaz. Güvenlik bulgusu raporu bunun istisnasıdır: repoya girmez (§12.10).
- Test sayıları dosya başına verilir, tek toplam yazılmaz: `src/registry/x.rs: 14 geçti / 0 kaldı`.
- Her raporda: değişen dosyalar, çalıştırılan komutlar, çalıştırılamayanlar (neden), açık kararlar (§8 biçimi).
- "Bitti" için hedefli test, `cargo fmt --check` ve clippy gerçekten çalışmış olmalıdır.

## 10. Oturum başı ve maliyet

İlk iş, çıktı boş olmalıdır. Doluysa dur ve §8 kullan:
```
env | cut -d= -f1 | grep -E '^(ANTHROPIC_API_KEY|CLAUDE_CODE_EFFORT_LEVEL|CLAUDE_CODE_SUBAGENT_MODEL|CLAUDE_CODE_SUBAGENT_MODEL_FORCE)$'
```
Sonra `memanto status` (komut yoksa atla, §11 kullanılmaz).

- Görev değişince `/clear`. Bağlam yaklaşık yüzde 60 dolunca `/compact <odak>`.
- Uzun açıklama konuşmada değil STATUS.md'de tutulur.
- Model ping-pong yok: plan, Sonnet'te uygulama, gerekirse doğrulama için bir kez Opus.
- Ağır R3 işleri taze limit penceresinin başında yapılır.

## 11. Memanto (varsa; yalnızca Bash komutuyla, On-Prem, `connect` yok)

- R2 ve R3 oturum başında bir kez: `memanto recall "<modül>" --limit 5` ve `memanto recall "değişmez" --type instruction --limit 10`. R1'de recall yok.
- Oturum sonunda en fazla bir kayıt (en fazla 3 satır): `memanto remember "..." --type decision --confidence 0.95`.
- Saklanır: tasarım kararı, değişmez, ADIM sonucu, tekrar eden tuzak. Saklanmaz: anahtar, seed, token, düzeltilmemiş güvenlik bulgusu ayrıntısı, kaynak kod.
- Çalıştırılmaz: `upload`, `answer`, `connect`, `serve`, `ui`. Onay ister: `forget`, `edit`, `memory`, `policy`, `schedule`, `conflicts`, `migrate`, `config`.
- Kalıcı karar memanto'da, aktif iş STATUS.md'de. Aynı bilgi iki yerde tutulmaz.

## 12. Sert yasaklar

1. Fable ve "Requires usage credits" modeli seçilmez.
2. `ANTHROPIC_API_KEY`, `CLAUDE_CODE_EFFORT_LEVEL`, `CLAUDE_CODE_SUBAGENT_MODEL`, `CLAUDE_CODE_SUBAGENT_MODEL_FORCE` tanımlanmaz.
3. `max` yalnızca finder-max ile ve §5 koşulunda.
4. Opus ajanları dosya yazmaz, Opus kod yazmaz.
5. Tüm repo okunmaz, `cargo test` bütün çalışmaz (kapanış hariç, onayla).
6. `.github/*-baseline.txt` dosyalarına dokunulmaz.
7. Özel anahtar, seed, token ve imzalama anahtarı yolu hiçbir araca (memanto dahil) yazılmaz.
8. Scout çıktısı kanıt sayılmaz.
9. `to_bytes()` içindeki `unwrap_or_default()` kalıbı odaklı ADIM'lara karıştırılmaz.
10. Düzeltilmemiş güvenlik bulguları halka açık issue, PR, commit, memanto veya repo dosyasına yazılmaz. `docs/SECURITY.md` özel kanalı kullanılır.
