# MODEL YÖNLENDİRME (Budlum, Claude Code)

Sürüm: SERT-7, 11 Ekim 2026 (CLAUDE.md 1.12 ile uyumlu, sadeleştirildi). Bu dosya her turda bağlama girer, kısa tutulur. Tam metin: `MODEL_ROUTING_REF.md` (bağlanmaz).
Statü: Emirdir. Sapma yalnızca §8 şablonu ve Ayaz (Japs) yanıtıyla. Karşılığı olmayan durumda dur, §8 kullan, varsayma.

## 1. Ajanlar

| Ajan | Model | Effort | Görev | Yapmaz |
|---|---|---|---|---|
| scout | Haiku | yok | belirli sembol, çağrı yeri, dosya bulma, log kırpma | karar, kod, güvenlik yorumu |
| finder | Opus | medium | R3 bulgu avı | kod |
| architect | Opus | medium | plan, handoff, R2 bulgu, bulgu ve diff doğrulama | kod, mekanik iş, keşif |
| coder | Sonnet | medium | handoff'a göre kod ve test | mimari karar |
| coder-deep | Sonnet | medium | R3 kodu ("Karmaşıklık: yüksek"), R3 denetim taraması (salt okuma) | R3 dışı kod |
| coder-lite | Sonnet | low | mekanik iş | davranış değişikliği, R3 |
| finder-max | Opus | medium | kullanılmaz (max effort kapalı, Ayaz 2026-10-11) | hepsi |

- Effort kuralı: tüm ajanlar medium, coder-lite low. Yükseltme yalnızca Ayaz'ın izniyle (CLAUDE.md §4A).
- Her ajan işe `docs/AGENT_MAP.md` okuyarak başlar (görev metninin ilk satırı).
- Mekanik iş: biçim, yeniden adlandırma, doküman, import düzeni, clippy'nin açıkça önerdiği düzeltme. Davranış değişmez. R3'te mekanik iş yoktur.
- "Karmaşıklık: yüksek" (architect işaretler): iki veya daha fazla modül, durum geçişi veya imza/doğrulama mantığı değişiyor, ya da değişmezler listesi 3 maddeyi aşıyor.
- Opus ajanları dosya yazmaz, ajan başlatmaz, kod yazmaz. Uygulama her zaman Sonnet ajanına döner. Ana oturum yönetir (plan ve doğrulama architect'e devredilir).
- Aynı anda en çok 3 ajan. finder asla paralel çalışmaz. Fable ve "Requires usage credits" yazan model seçilmez.

## 2. Seçim

2.4 Seçim tablosu.

| Faz | Kademe | Ajan |
|---|---|---|
| Keşif | R0 ile R3 | scout |
| Bulgu avı | R3 ilk tarama veya §2.6 yüzeyi değişti | finder (kod okuma işi coder-deep'e verilebilir) |
| Bulgu avı | R3 delta tarama, R2 | architect |
| Plan, handoff | R3, R2 | architect |
| Kod | R3 normal, R2, R1 | coder |
| Kod | R3 Karmaşıklık: yüksek | coder-deep |
| Mekanik | R2, R1, R0 | coder-lite |
| Doğrulama | R3 zorunlu, R2 (§3 koşulu) | architect, her seferinde yeni çağrı |

2.5 Faz beyanı. Her faz başlamadan önce tek satır:
`FAZ: <keşif|bulgu|plan|kod|mekanik|doğrulama> | KADEME: R<0-3> | AJAN: <ad> | EFFORT: <seviye> | NEDEN: <en fazla 8 kelime>`

2.6 Bulgu avı. finder: R3 modülünde ilk tarama veya imza/doğrulama, konsensüs durum makinesi, tokenomics parametresi, kalıcılık formatı değiştiyse. architect: önceki tarama kaydı varsa ve yalnızca `git diff <son-taranan-commit>..HEAD` taranıyorsa, ve tüm R2 taramalarında. R1 ve R0'da bulgu avı yok. Claude bulgu avını Ayaz'dan istemeden başlatır. Tam kod denetimi: `docs/AUDIT_PLAN.md`. Testnet aşamasında bulgular `docs/audit/` altına yazılır (CLAUDE.md Z14).

2.7 Handoff'taki "Ajan" alanı bağlayıcıdır. Boşsa Claude tablodan seçer ve beyan eder.

2.9 Bütünlük. Ajan transkriptindeki model ve effort, ajan dosyasıyla eşleşmelidir (ilk kurulum, her `claude update`, her ajan dosyası değişikliği). Eşleşmezse dur, §8.

## 3. Kademeler

| Kademe | Alanlar | Akış |
|---|---|---|
| R3 | `consensus`, `crypto`, `privacy`, `tokenomics`, `settlement`, `cross_domain`, `registry`, `chain` (snapshot ve kalıcılık), `network/node.rs`, `budzero/bud-proof` | scout, bulgu, handoff (architect), kod (coder veya coder-deep), doğrulama (architect, zorunlu, yeni çağrı) |
| R2 | `execution`, `core`, `rpc`, `storage`, `domain`, `account_abstraction`, `ai`, `ai_inference` | scout, plan (architect, kısa), kod (coder), doğrulama (architect) yalnızca 3 dosyadan fazla veya durum geçişi varsa |
| R1 | `bns`, `socialfi`, `pollen`, `gateway`, `relayer`, `bin`, `docs`, `config`, ek testler | ana oturum veya coder, plan modu yok |
| R0 | grep, liste, log kırpma, yeniden adlandırma | scout veya coder-lite |

Yanlış alan görürsen düzeltmeyi §8 ile öner. `.github/*-baseline.txt` dosyalarına dokunulmaz, gevşetme §8 onayı ister.

## 4. Bulgu döngüsü

1. finder veya architect tek modülde bulguları listeler (id, yol:satır, etki, kanıt).
2. Critical, High, Medium bulgular yeni bir architect çağrısıyla doğrulanır (çağrı başına en çok 8). Low, Info, test ve ölü kod bulguları "ham" kalır; düzeltme adımı başlayınca tek toplu çağrıyla doğrulanır.
3. architect her doğrulanmış bulgu için ayrı handoff yazar.
4. coder yalnızca o bulguyu düzeltir ve push eder; CI sonucu GitHub'dan okunur.
5. R3 diff'i yeni bir architect çağrısıyla doğrulanır. Bulan bağlam kendi düzeltmesini onaylamaz.

## 5. Takılma

- Sonnet ajanı aynı hatada 2 kez başarısız: hata raporuyla architect, uygulama yine Sonnet'te.
- finder veya architect 2 denemede çözemedi, ya da bulgu "belirsiz" kaldı: DUR, §8 (A: kapsamı küçült, B: Japs karar verir, C: ertele). Effort yükseltilmez.
- scout şüphe veya çelişki bildirdi: coder'a devret.

## 6. Okuma kuralları

Tüm repo okunmaz veya özetlenmez. Büyük dosyalar (100 KB üstü) bütün okunmaz; liste `docs/AGENT_MAP.md` içinde.

1. `rg -n "<sembol>" src/` ile bul, `sed -n 'a,bp'` ile yalnızca gerekli aralığı oku. docs/ARCHITECTURE.md için önce `rg -n '^## '`.
2. `git diff --stat` önce, tam diff dosya dosya. `target/` ve `Cargo.lock` okunmaz.
3. CI'ın işi yerelde yapılmaz (`cargo fmt`, `clippy`, `test`, `check`, typos, deny, audit, gates, miri, kani, semgrep). Sonuç push sonrası GitHub araçlarıyla okunur, log yalnızca kırmızı adımdan çekilir ve kırpılır. rustfmt ve clippy çıktısı tahmin edilmez.
4. Scout ve dış ajan çıktısı kanıt değildir. Düzenlemeden önce ilgili satırları bizzat oku.
5. Denetim ajanları çağrı başına en çok 1500 satır okur; parça dışında önce `rg`, en çok 80 satır.

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
Test: <yazılacak başarısız test adı ve dosyası> (CI'da çalışır)
Karmaşıklık: normal | yüksek | mekanik
Ajan: coder | coder-deep | coder-lite
Bitti ölçütü: <gözlemlenebilir>
Opus doğrulaması gerekli mi: evet | hayır
```

## 8. Onay şablonu

Dur ve sor: 8 dosyadan fazla değişiklik, tokenomics veya protokol parametresi, `main`'e push, dosya silme, baseline gevşetme, memanto'yu Cloud'a geçirme, effort yükseltme, R3'te coder-lite, yeni ajan, "Requires usage credits" modeli, ortam kontrolünde sapma, §2.9 eşleşmezliği.
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

- Rapor PR açıklamasına yazılır, ayrı rapor dosyası açılmaz. Denetim bulguları `docs/audit/` altında (Z14).
- Her raporda: değişen dosyalar, kanıt olarak push edilen SHA ve CI sonucu, çalıştırılamayanlar (neden), açık kararlar (§8 biçimi). Test sayısını CI yazar.
- "Bitti" için ilgili CI işleri push edilen SHA üzerinde yeşil olmalıdır (Z4).

## 10. Oturum başı ve maliyet

İlk iş, çıktı boş olmalıdır. Doluysa dur ve §8 kullan:
```
env | cut -d= -f1 | grep -E '^(ANTHROPIC_API_KEY|CLAUDE_CODE_EFFORT_LEVEL|CLAUDE_CODE_SUBAGENT_MODEL|CLAUDE_CODE_SUBAGENT_MODEL_FORCE)$'
```
Sonra `memanto status` (komut yoksa atla ve §11'i kullanma).

Maliyet: token kuralları CLAUDE.md §4A'dadır. Ek olarak:
- Ajan prompt'u kısa: `docs/AUDIT_PLAN.md` §7 şablonları kullanılır.
- Görev değişince `/clear`, bağlam yaklaşık yüzde 60 dolunca `/compact <odak>`.
- Uzun açıklama konuşmada değil STATUS.md'de. STATUS.md her tur sonunda yeniden yazılır, 10 KB altında kalır.
- Model ping-pong yok: plan, Sonnet'te uygulama, gerekirse doğrulama için bir kez Opus.
- Limit bitince ajanlar durdurulur, yarım iş repoya yazılmaz.

## 11. Memanto (varsa; yalnızca Bash komutuyla, On-Prem, `connect` yok)

- R2 ve R3 oturum başında bir kez: `memanto recall "<modül>" --limit 5` ve `memanto recall "değişmez" --type instruction --limit 10`. R1'de recall yok.
- Oturum sonunda en fazla bir kayıt (3 satır): `memanto remember "..." --type decision --confidence 0.95`. Saklanır: tasarım kararı, değişmez, ADIM sonucu, tekrar eden tuzak. Saklanmaz: anahtar, seed, token, düzeltilmemiş bulgu ayrıntısı, kaynak kod.
- Çalıştırılmaz: `upload`, `answer`, `connect`, `serve`, `ui`. Onay ister: `forget`, `edit`, `memory`, `policy`, `schedule`, `conflicts`, `migrate`, `config`.

## 12. Sert yasaklar

1. Fable ve "Requires usage credits" modeli seçilmez.
2. `ANTHROPIC_API_KEY`, `CLAUDE_CODE_EFFORT_LEVEL`, `CLAUDE_CODE_SUBAGENT_MODEL`, `CLAUDE_CODE_SUBAGENT_MODEL_FORCE` tanımlanmaz.
3. Effort medium üstüne çıkarılmaz (`high`, `xhigh`, `max`), Ayaz izin vermedikçe.
4. Opus ajanları dosya yazmaz, Opus kod yazmaz.
5. Tüm repo okunmaz. Yerelde cargo ve diğer CI araçları çalıştırılmaz.
6. `.github/*-baseline.txt` dosyalarına dokunulmaz.
7. Özel anahtar, seed, token ve imzalama anahtarı yolu hiçbir araca (memanto dahil) yazılmaz.
8. Scout çıktısı kanıt sayılmaz.
9. `to_bytes()` içindeki `unwrap_or_default()` kalıbı odaklı ADIM'lara karıştırılmaz.
10. Düzeltilmemiş güvenlik bulguları memanto'ya ve halka açık issue veya PR'a yazılmaz. Testnet aşamasında repoda yalnızca `docs/audit/` altında tutulabilir (Z14). Mainnet öncesi `docs/SECURITY.md` özel kanalı.
