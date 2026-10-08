# MODEL YÖNLENDİRME VE MALİYET DİREKTİFİ (Budlum, Claude Pro, Claude Code)

Sürüm: SERT-2, 2026-10-08, sadeleştirildi (sahip isteği).
Bağlama: CLAUDE.md sonundaki `@MODEL_ROUTING.md` satırı. Kurulum ve ek araçlar `MODEL_ROUTING_KURULUM.md` dosyasındadır; bağlanmaz.
Amaç: Pro limitini en az harcayarak her fazda doğru model ve effort ile çalışmak.
Statü: Her cümle emirdir. Sapma yalnızca §8 şablonu ve Japs'ın yanıtıyla olur. Karşılığı olmayan durumda dur ve §8 şablonunu kullan; varsayma.

---

## 0. Özet

1. Opus bulur, planlar, doğrular; kod yazmaz. Sonnet kod yazar. Haiku yalnızca salt-okunur keşif yapar. Fable kullanılmaz.
2. Effort: bulgu avı en yüksek, plan ve doğrulama yüksek, kod orta, mekanik iş düşük.
3. Effort'u Claude ayarlar; Japs `/effort` veya `/model` kullanmaz (§2).
4. Gereken effort ana oturumunkinden farklıysa iş ilgili ajana devredilir.
5. 100 KB üstü dosya bütün okunmaz (§6).
6. Aynı hatada iki deneme sonrası bir üst kademeye çıkılır (§5).
7. Belirsiz veya geri dönüşsüz ekonomik karar seçenek olarak sunulur; Claude seçmez (§8).
8. Her oturumun ilk işi §10 ortam kontrolüdür.

---

## 1. Ajanlar

| Ajan | Model | Effort | Görev | Yapmaz |
|---|---|---|---|---|
| scout | Haiku | yok (alan yazılmaz) | grep, sembol bulma, dosya haritası, log kırpma, çağrı yeri listeleme | karar, kod, güvenlik yorumu |
| finder | Opus | xhigh | R3 bulgu avı | kod yazmak |
| architect | Opus | high | plan, tasarım kararı, R2 bulgu avı, diff doğrulama | kod, mekanik iş, keşif |
| finder-max | Opus | max | yalnızca §5 istisnası | rutin her şey |
| coder | Sonnet | medium | handoff'a göre kod, test, refactor | mimari karar, bulgu avı |
| coder-deep | Sonnet | high | R3 kodu, "Karmaşıklık: yüksek" | R3 dışı iş |
| coder-lite | Sonnet | low | mekanik iş | davranış değişikliği, R3 |

Mekanik iş: biçim, yeniden adlandırma, doküman, import düzeni, clippy'nin açıkça önerdiği düzeltme. Davranış değişmez. R3'te mekanik iş yoktur; R3 kodu coder veya coder-deep ile yazılır.

"Karmaşıklık: yüksek" (architect işaretler): iki veya daha fazla modüle yayılma, durum geçişi ya da imza/doğrulama mantığı değişimi, ya da üçten fazla değişmez.

Ana oturum: `opusplan`. Yürütmede Sonnet (medium), plan modunda Opus (high), `modelSettings` ile sabit (kurulum dosyası §2.2). Claude plan moduna güvenmez; plan ve doğrulamayı `architect`e devreder.

Scout (Haiku) çıktısı kanıt değil ipucudur. Sahibin isteğiyle scout bütün depoyu modül modül mekanik tarayabilir. Bulgu ancak Opus ajanı veya ana oturum satırı okuyunca bulgu sayılır.

---

## 2. Effort'u Claude kendisi ayarlar (ZORUNLU)

2.1 Yetki. Effort ayarının tamamı Claude'undur. Ana oturumun effort'u yetmiyorsa Japs'tan `/effort` istenmez; iş doğru ajana devredilir. Claude her fazda ajanı seçer, beyan eder ve §2.9'u uygular.

2.2 İlke. Bulgu avı yanlış negatif riski taşıyan tek fazdır; en yüksek effort orada harcanır. Kod yazımı handoff ile daraltılmış ve testlidir; effort düşer. Güvence sonraki Opus doğrulamasıdır.

2.3 Mekanizma. Ana oturum: Opus high, Sonnet medium. Ajan dosyasındaki `effort` alanı, ajan çalıştığı sürece ana seviyeyi geçersiz kılar. Gereken effort farklıysa Claude işi satır içinde yapmaz, ajanı çağırır. Faz bitince effort eski seviyeye döner.

2.4 Seçim tablosu.

| Faz | Kademe | Ajan | Effort |
|---|---|---|---|
| Keşif | R0, R1, R2, R3 | scout | yok |
| Bulgu avı | R3, ilk tarama veya §2.6 yüzeyi değişti | finder | xhigh |
| Bulgu avı | R3 yeniden tarama (yalnızca delta), R2 | architect | high |
| Plan, tasarım, handoff | R3, R2 | architect | high |
| Kod yazımı | R3 normal | coder | medium |
| Kod yazımı | R3, Karmaşıklık: yüksek | coder-deep | high |
| Kod yazımı | R2, R1 | coder (R1'de ana oturum Sonnet satır içi de olur) | medium |
| Mekanik iş | R2, R1, R0 | coder-lite | low |
| Doğrulama | R3 (zorunlu), R2 (§3 koşulu) | architect, her seferinde yeni çağrı | high |
| İstisna | §5 koşulu | finder-max | max |

2.5 Faz beyanı. Her fazdan önce tek satır yazılır; yazılmadan faz başlamaz.

`FAZ: <keşif|bulgu|plan|kod|mekanik|doğrulama> | KADEME: R<0-3> | AJAN: <ad> | EFFORT: <seviye> | NEDEN: <en fazla 8 kelime>`

2.6 Bulgu avı seçimi. `finder` (xhigh): R3 modülünde ilk tarama; ya da önceki taramadan sonra imza/doğrulama, konsensüs durum makinesi, tokenomics parametresi veya kalıcılık formatı değişmişse. `architect` (high): önceki tarama kaydı STATUS.md veya memanto'da varsa ve yalnızca `git diff <son-taranan-commit>..HEAD` taranıyorsa; ve tüm R2 taramalarında. R1 ve R0'da bulgu avı yapılmaz.

2.7 Kod ajanı seçimi. Handoff'taki "Ajan" alanı bağlayıcıdır (architect doldurur). Boşsa Claude §2.4 ile seçer ve beyan eder.

2.8 Yükseltme ve düşürme. Yükseltme §5 merdivenine göre. Opus çözümü verdikten veya doğruladıktan sonra uygulama her zaman Sonnet ajanına döner.

2.9 Bütünlük kontrolü. Ajan dosyalarındaki effort değerinin uygulandığı, ajan transkriptindeki `effort` anahtarından doğrulanır (kurulum dosyası §2.4). Doğrulama ilk kurulumda, her `claude update` sonrası ve her ajan dosyası değişikliği sonrası yenilenir. Transkriptteki model veya effort ajan dosyasıyla eşleşmezse Claude durur ve §8 ile bildirir. Sessiz devam yoktur. Neden: `CLAUDE_CODE_EFFORT_LEVEL` tanımlıysa tüm ajan effort'larını uyarısız ezer.

2.10 Kayıt. Her finder-max çağrısı için STATUS.md "EFFORT LOG" başlığına tek satır yazılır: tarih, görev, ajan, effort, neden.

2.11 Yetki sınırı. Claude kendi başına yapar: ajan seçimi, §5 merdiveni, finder-max çağrısı. Claude §8 ile sorar: ajan dosyalarında effort değişikliği, R3'te coder-lite, finder'ı high altına indirmek, yeni ajan eklemek.

2.12 `ultrathink` bu düzenin parçası değildir; kullanılmaz.

---

## 3. Kademeler ve akış

Kademeler dizin adlarına göre öneridir. Yanlış alan görürsen Claude düzeltmeyi §8 ile önerir.

| Kademe | Alanlar | Akış |
|---|---|---|
| R3 kritik | `consensus`, `crypto`, `privacy`, `tokenomics`, `settlement`, `cross_domain`, `registry`, `chain` (snapshot ve kalıcılık), `network/node.rs`, `budzero/bud-proof` | scout, bulgu (finder veya architect), plan ve handoff (architect), kod (coder veya coder-deep), doğrulama (architect, zorunlu, yeni çağrı) |
| R2 orta | `execution`, `core`, `rpc`, `storage`, `domain`, `account_abstraction`, `ai`, `ai_inference` | scout, plan (architect, kısa), kod (coder), doğrulama (architect) yalnızca değişiklik 3 dosyadan fazlaysa veya durum geçişi değiştiyse |
| R1 düşük | `bns`, `socialfi`, `pollen`, `gateway`, `relayer`, `bin`, `docs`, `config`, ek testler | ana oturum Sonnet veya coder; plan modu yok |
| R0 mekanik | grep, liste, log kırpma, yeniden adlandırma | scout veya coder-lite |

`.github/*-baseline.txt` dosyalarına dokunulmaz. Gevşetme §8 onayı gerektirir.

---

## 4. Bulgu döngüsü (Opus bulur, Sonnet düzeltir)

```
1. finder veya architect (§2.6): tek modülde bulguları listeler (id, yol:satır, etki, kanıt)
2. architect: her bulgu için ayrı handoff yazar (§7)
3. coder, coder-deep veya coder-lite: yalnızca o bulguyu düzeltir
4. architect (yeni çağrı): düzeltme diff'ini doğrular
5. Sonraki bulgu
```

Bulan bağlam kendi düzeltmesini onaylamaz; doğrulama her seferinde yeni `architect` çağrısıyla yapılır. finder ve finder-max asla paralel çalışmaz.

---

## 5. Yükselme merdiveni ve max koşulu

```
coder, coder-deep veya coder-lite aynı hatada 2 kez başarısız  -> hata raporuyla architect (high); çözüm planı sonrası uygulama yine Sonnet'te
finder veya architect aynı problemi 2 ayrı denemede çözemedi,
  ya da R3 bulgusu doğrulamada "belirsiz" kaldı                  -> finder-max, tek görev, bulgu başına en fazla 1 çağrı
finder-max sonuçsuz                                               -> DUR, §8: A) kapsamı küçült  B) Japs karar verir  C) ertele
scout şüphe veya çelişki bildirdi                                 -> coder'a devret (scout alt ajan açmaz)
```

finder-max çağrısının ilk satırı `max: <neden>` olur ve §2.10'a göre kaydedilir. `max` rutin işte kullanılmaz; iş bitince alta inilir.

---

## 6. Budlum okuma kuralları (token yakan yerler)

Ağaç: yaklaşık 726 `.rs` dosyası, 13,7 MB. Tüm repo hiçbir zaman okunmaz veya özetlenmez.

Bütün okunmayacak dosyalar: `budzero/bud-proof/src/plonky3_prover.rs` (357 KB), `src/chain/blockchain.rs` (354 KB), `src/rpc/server.rs` (284 KB), `src/domain/storage_deal.rs` (284 KB), `src/network/node.rs` (202 KB), `src/chain/chain_actor.rs` (193 KB), `src/ai/mod.rs` (184 KB), `budzero/bud-proof/src/plonky3_air.rs` (150 KB), `src/core/account.rs` (149 KB), `src/execution/executor.rs` (141 KB), `docs/ARCHITECTURE.md` (82 bölüm).

Protokol:
1. `rg -n "<sembol>" src/` ile yeri bul, `sed -n 'a,bp' dosya` ile yalnızca gerekli aralığı oku.
2. `docs/ARCHITECTURE.md`: önce `rg -n '^## '`, sonra yalnızca ilgili bölüm.
3. `git diff --stat` önce, tam diff sonra, dosya dosya.
4. Çıktıları kırp: `cargo test <filtre> 2>&1 | tail -n 40`, `cargo clippy --message-format=short`.
5. `cargo test` bütün çalışmaz (yaklaşık 3229 lib testi). Hedefli: `cargo test -p <crate> <modül_filtresi>`. Tam koşu yalnızca kapanışta ve §8 onayıyla.
6. `rustfmt` ve `clippy` çıktısı tahmin edilmez; `rust-toolchain.toml` sürümüyle çalıştırılır.
7. `target/` ve `Cargo.lock` Read ile engellidir; Bash ile (`cat`, `sed`, `rg`) de okunmaz.
8. Scout çıktısı kanıt değildir; düzenlemeden önce satırlar bizzat okunur. Dış ajan ve özet raporlarda da aynı: özete değil, yamaya ve koda bak.

---

## 7. Handoff şablonu (architect yazar, kodlayan ajan yalnızca bunu okur)

architect handoff'u çıktı olarak verir, en fazla 40 satır, dosya yazmaz (§12 madde 4). Ana oturum handoff'u STATUS.md'ye yazar; dosya yoksa oluşturur.

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

---

## 8. Onay şablonu (seçenek sun, kendin seçme)

Şu durumlarda dur ve bu biçimde sor: 8 dosyadan fazla değişiklik, tokenomics veya protokol parametresi, `main`'e push, dosya silme, baseline gevşetme, tam güvenlik denetimi, memanto'yu Cloud'a geçirme veya §11 listesi dışında veri yazma, ajan dosyalarında effort değişikliği, R3'te coder-lite, "Requires usage credits" yazan modele geçiş, §10 ortam kontrolünde sapma, §2.9 eşleşmezliği.

Japs'a sorular sade, teknik olmayan dille yazılır. Seçenekler A/B/C/D olur, önerilen işaretlenir.

```
KARAR GEREKİYOR: <başlık>
Durum: <1-2 cümle>
A) <seçenek>, maliyet: ...
B) <seçenek>, maliyet: ...
C) Dur, ben karar vereyim
Önerim: <harf>, çünkü <tek cümle>
```

---

## 9. Raporlama biçimi

- Rapor PR açıklamasına yazılır; ayrı rapor dosyası açılmaz.
- Test sayıları dosya başına verilir, tek toplam yazılmaz: `src/registry/x.rs: 14 geçti / 0 kaldı`
- Her raporda: değişen dosyalar, çalıştırılan komutlar, çalıştırılamayanlar (neden), açık kararlar (§8 biçiminde).
- "Bitti" için hedefli test, `cargo fmt --check` ve clippy gerçekten çalışmış olmalıdır.

---

## 10. Maliyet disiplini ve oturum başı kontrol

Oturum başı kontrol (ilk iş, tek komut):

```
env | cut -d= -f1 | grep -E '^(ANTHROPIC_API_KEY|CLAUDE_CODE_EFFORT_LEVEL|CLAUDE_CODE_SUBAGENT_MODEL|CLAUDE_CODE_SUBAGENT_MODEL_FORCE)$'
```

Çıktı boş olmalıdır (yalnızca değişken adları basılır). Doluysa Claude durur ve §8 ile bildirir; bu değişkenler ajan modelini ve effort'unu ezer veya aboneliği API faturasına çevirir.

Sonra `memanto status` çalışır. Süre dolmuşsa `memanto agent activate budlum --hours 24`.

Kurallar:
- Görev değişince `/clear`. Bağlam yaklaşık yüzde 60 dolunca `/compact <odak cümlesi>`.
- Uzun açıklama `STATUS.md`'de tutulur, konuşmada değil.
- CLAUDE.md kısa kalır; ayrıntı ayrı dosyada tutulur.
- Model ping-pong yapılmaz: plan bitir, Sonnet'te uygula, gerekirse doğrulama için bir kez Opus.
- Paralel ajan ayrı bağlam demektir; yalnızca bağımsız ve küçük işte kullanılır.
- Pro'da pencere ve haftalık limit vardır: ağır R3 işleri (finder, doğrulama) taze pencerenin başında yapılır.
- Ölçüm: `npx ccusage daily --breakdown`, `npx ccusage session`, `npx ccusage blocks --live`. Dolar tutarı API eşdeğeri tahmindir; limit için `/usage` ve claude.ai Ayarlar, Usage esastır. Claude haftada bir model bazlı tüketimi tablo yapar ve öneriyi §8 ile sunar.

---

## 11. Memanto kullanım kuralları (On-Prem, `connect` kullanılmadan)

Kurulum `MODEL_ROUTING_KURULUM.md` §5'tedir. Memanto yalnızca Bash komutuyla kullanılır; CLAUDE.md'ye veya hook'lara bağlı değildir.

- R2 ve R3 işlerinde oturum başında bir kez: `memanto recall "<modül adı>" --limit 5` ve `memanto recall "değişmez" --type instruction --limit 10`. R1'de recall yapılmaz.
- Oturum sonunda en fazla bir kayıt: `memanto remember "<karar, en fazla 3 satır>" --type decision --confidence 0.95`. Tuzak: `--type error`. Değişmez: `--type instruction --confidence 1.0 --tags değişmez`.
- Saklanır: tasarım kararları, değişmezler, ADIM sonuçları, tekrar eden tuzaklar.
- Asla saklanmaz: özel anahtar, seed, imzalama anahtarı yolu, API anahtarı veya token, düzeltilmemiş güvenlik bulgusu ayrıntısı, kaynak kod blokları. Listenin dışı için önce §8.
- `memanto upload`, `answer`, `connect`, `serve`, `ui` çalıştırılmaz (izinlerde engelli). `forget`, `edit`, `memory`, `policy`, `schedule`, `conflicts`, `migrate`, `config` onay ister.
- Oturum hatasında `memanto agent activate budlum --hours 24` çalışır ve komut bir kez tekrarlanır.
- STATUS.md ile aynı bilgi tutulmaz: kalıcı karar memanto'da, aktif iş STATUS.md'de.
- Düzeltilmemiş güvenlik bulgusu raporları memanto'ya veya başka bir bulut belleğe yazılmaz.

---

## 12. Sert yasaklar

1. Fable seçilmez. `/model` listesinde "Requires usage credits" yazan hiçbir model seçilmez.
2. `ANTHROPIC_API_KEY`, `CLAUDE_CODE_EFFORT_LEVEL`, `CLAUDE_CODE_SUBAGENT_MODEL`, `CLAUDE_CODE_SUBAGENT_MODEL_FORCE` tanımlanmaz.
3. `max` yalnızca finder-max ile, yalnızca §5 koşulunda kullanılır.
4. Opus ajanları (finder, architect, finder-max) dosya yazmaz; Opus kod yazmaz.
5. Bütün repo okunmaz veya özetlenmez; `cargo test` bütün çalışmaz (kapanış hariç, onayla).
6. `.github/*-baseline.txt` dosyalarına dokunulmaz.
7. Özel anahtar, seed, token ve imzalama anahtarı yolu hiçbir araca (memanto dahil) yazılmaz.
8. Scout çıktısı kanıt sayılmaz.
9. `to_bytes()` içindeki `unwrap_or_default()` sessiz hata kalıbı odaklı ADIM'lara karıştırılmaz.
10. Düzeltilmemiş güvenlik bulguları halka açık issue veya PR'a yazılmaz; `docs/SECURITY.md` sürecine göre ele alınır.
