# MODEL YÖNLENDİRME: KURULUM VE EK ARAÇLAR (Budlum, Claude Pro, Claude Code)

Sürüm: SERT-1, 7 Ekim 2026. `MODEL_ROUTING.md` ile birlikte okunur.
Bu dosya CLAUDE.md'ye bağlanmaz. Yalnızca kurulum oturumunda ve ilgili aracın kurulumu sırasında okunur.
Yürütme kuralı: Her adımın sonunda "Doğrulama" satırı vardır. Beklenen çıktı alınmadan sonraki adıma geçilmez. Sapma olursa Claude durur ve `MODEL_ROUTING.md` §8 şablonuyla bildirir.
Komutlar bash içindir. Windows'ta tüm kurulum WSL2 içinde yapılır ve Docker Desktop'ta WSL entegrasyonu açık olmalıdır.

---

## 1. Ön koşullar ve kimlik doğrulaması

1.1 Claude Code sürümü: `claude --version` çıktısı 2.1.284 veya üstü olmalıdır. Değilse `claude update` çalıştırılır. (Sonnet 5.5 için 2.1.284, Opus 5.5 için 2.1.280, `modelSettings` için 2.1.251 gerekir; 2.1.284 hepsini karşılar.)

1.2 Araçlar: `command -v node npx python3 pip jq rg git docker` her biri bir yol basmalıdır. `docker info` hata vermemelidir. `python3 -c 'import sys; assert sys.version_info >= (3,10)'` hata vermemelidir.

1.3 Ortam değişkenleri: `MODEL_ROUTING.md` §10'daki `env | cut ... | grep ...` komutu boş çıktı vermelidir. Doluysa değişken kaldırılır (`unset`) ve kaynağı aranır:
```
grep -nE 'ANTHROPIC_API_KEY|CLAUDE_CODE_EFFORT_LEVEL|CLAUDE_CODE_SUBAGENT_MODEL' ~/.bashrc ~/.zshrc ~/.profile ~/.zprofile 2>/dev/null
jq '.env' ~/.claude/settings.json .claude/settings.json 2>/dev/null
```
Settings dosyalarındaki `env` bloğu da oturum ortamına değişken yazabilir; bu iki değişkenden biri orada tanımlıysa kaldırılır.

1.4 Kimlik: `/status` çıktısında abonelik (Pro) görünmelidir. API anahtarı faturası görünüyorsa dur ve §8 ile bildir.

1.5 Model listesi: `/model` açılır. Opus, Sonnet ve Haiku satırlarında "Requires usage credits" yazmamalıdır. Yazan model seçilmez ve §8 ile bildirilir. (Fable satırına dokunulmaz.)

1.6 Kredi tavanı: claude.ai Ayarlar, Usage bölümünde otomatik kredi yüklemesi kapalı olmalıdır. Bu adımı Japs yapar çünkü Claude bu arayüze erişemez. Doğrulama: Japs "kapalı" yanıtını verir.

1.7 Başlangıç ölçümü: `npx ccusage daily --breakdown` çıktısı `STATUS.md` içinde "BAŞLANGIÇ ÖLÇÜMÜ" başlığı altında tarihle kaydedilir. Sonraki her araç bununla kıyaslanır.

---

## 2. Ayar dosyaları ve ajanlar

### 2.1 Proje ayarı

```
mkdir -p .claude/agents
[ -f .claude/settings.json ] || echo '{}' > .claude/settings.json
jq '.model="opusplan" | .permissions.deny=(((.permissions.deny // []) + ["Read(./target/**)","Read(**/Cargo.lock)"]) | unique)' .claude/settings.json > .claude/settings.json.tmp && mv .claude/settings.json.tmp .claude/settings.json
```

Doğrulama: `/permissions` çıktısında iki deny kuralı görünür. `target/` altındaki bir dosyayı okuma denemesi reddedilir. Not: Read kuralı Bash ile `cat` çağrısını engellemez; bunu `MODEL_ROUTING.md` §6 madde 7 kapsar.

### 2.2 Kullanıcı ayarı: model başına effort

```
f=~/.claude/settings.json
[ -f "$f" ] || { mkdir -p ~/.claude; echo '{}' > "$f"; }
cp "$f" "$f.bak-$(date +%Y%m%d)"
jq '.modelSettings["claude-opus-5-5"].effortLevel="high" | .modelSettings["claude-sonnet-5-5"].effortLevel="medium"' "$f" > "$f.tmp" && mv "$f.tmp" "$f"
```

Bu iki satır, `/effort` komutunun Enter ile kaydettiği `modelSettings.<model>.effortLevel` anahtarının aynısıdır. Ana oturumda plan modundaki Opus high, yürütmedeki Sonnet medium çalışır.

Doğrulama: yeni oturumda oturum başlığında model adının yanında effort seviyesi görünür (Opus için high, Sonnet için medium). Görünmüyorsa dur ve §8 ile bildir.

### 2.3 Ajan dosyaları (yedi dosya)

```
cat > .claude/agents/scout.md <<'EOF'
---
name: scout
description: Salt-okunur keşif. Dosya, sembol ve çağrı yerlerini bulur. Kod DEĞİŞTİRMEZ, yorum yapmaz.
model: haiku
tools: Read, Grep, Glob, Bash
---
Görev: sorulan sembol, dosya veya çağrı yerlerini bul.
Çıktı: en fazla 15 satır; her satır `yol:satır, tek cümle`. Sonuç ve öneri yazma.
Büyük dosyaları bütün okuma: `rg -n` ve `sed -n 'a,bp'` kullan.
Dosya yazma, düzenleme veya git işlemi yapma.
EOF

cat > .claude/agents/finder.md <<'EOF'
---
name: finder
description: R3 alanlarında bulgu avı (consensus, crypto, privacy, tokenomics, settlement, cross_domain, registry, chain, network/node.rs, budzero/bud-proof). Bulgu bulur, kod YAZMAZ.
model: opus
effort: xhigh
tools: Read, Grep, Glob, Bash
---
Kapsam: çağıran görevde verilen tek modül. Başka modüle geçme.
Her bulgu için: id, `yol:satır`, etki, kanıt (okuduğun satırlar), güven (yüksek, orta, düşük).
Kanıtsız bulgu yazma. Spekülasyon ve stil önerisi yazma.
Büyük dosyaları bütün okuma: `rg -n` ve `sed -n 'a,bp'` kullan.
Bulguyu düzeltmeye kalkma. Dosya yazma veya düzenleme.
Çıktı en fazla 40 satır, bulgular önem sırasıyla.
EOF

cat > .claude/agents/architect.md <<'EOF'
---
name: architect
description: Plan, tasarım kararı, R2 bulgu avı, handoff yazımı ve diff doğrulama. Kod YAZMAZ.
model: opus
effort: high
tools: Read, Grep, Glob, Bash
---
Bulgu bulma: her bulgu için id, `yol:satır`, etki, kanıt (okuduğun satırlar).
Plan: handoff şablonunu (MODEL_ROUTING.md §7) doldur; Karmaşıklık ve Ajan alanlarını işaretle.
Doğrulama: girdi handoff ve `git diff` olsun (tam dosya değil). Bulguyu kendin düzeltmeye kalkma.
Özet rapora değil kodun kendisine bak. Dosya yazma veya düzenleme. Handoff'u çıktı olarak ver; ana oturum STATUS.md'ye yazar.
Çıktı en fazla 30 satır. Handoff çıktısı en fazla 40 satır.
EOF

cat > .claude/agents/finder-max.md <<'EOF'
---
name: finder-max
description: YALNIZCA MODEL_ROUTING.md §5 istisna koşulunda. R3 bulgu avı, en yüksek effort. Kod YAZMAZ.
model: opus
effort: max
tools: Read, Grep, Glob, Bash
---
Çıktının ilk satırı çağrı metnindeki `max: <neden>` satırını aynen tekrar eder.
Kapsam: çağıran görevde verilen tek bulgu veya tek modül.
Her bulgu için: id, `yol:satır`, etki, kanıt (okuduğun satırlar), güven.
Kanıtsız bulgu yazma. Dosya yazma veya düzenleme. Çıktı en fazla 40 satır.
EOF

cat > .claude/agents/coder.md <<'EOF'
---
name: coder
description: Handoff şablonuna göre kod yazar, hedefli test ve fmt/clippy çalıştırır. Normal karmaşıklık.
model: sonnet
effort: medium
---
Yalnızca handoff'taki dosya ve satır aralıklarında çalış. Kapsam dışına çıkma.
Tasarım kararı gerekirse veya bir şey belirsizse DUR ve sor; sessizce varsayma.
Asgari kod yaz; her değişen satır handoff'a izlenebilir olsun. İlgisiz ölü kodu silme, raporla.
Önce başarısız olan testi yaz, sonra düzelt.
Bitirmeden hedefli test, `cargo fmt --check` ve clippy çalıştır; sonucu tahmin etme.
Rapor: değişen dosyalar ve dosya başına test sayıları (geçti/kaldı).
EOF

cat > .claude/agents/coder-deep.md <<'EOF'
---
name: coder-deep
description: R3 alanında, handoff'ta "Karmaşıklık: yüksek" işaretli kod yazımı.
model: sonnet
effort: high
---
Yalnızca handoff'taki dosya ve satır aralıklarında çalış. Kapsam dışına çıkma.
Handoff'taki değişmezler listesini kodlamadan önce oku; her değişikliği bu listeye karşı kontrol et.
Tasarım kararı gerekirse veya bir şey belirsizse DUR ve sor; sessizce varsayma.
Asgari kod yaz; her değişen satır handoff'a izlenebilir olsun. İlgisiz ölü kodu silme, raporla.
Önce başarısız olan testi yaz, sonra düzelt.
Bitirmeden hedefli test, `cargo fmt --check` ve clippy çalıştır; sonucu tahmin etme.
Rapor: değişen dosyalar ve dosya başına test sayıları (geçti/kaldı).
EOF

cat > .claude/agents/coder-lite.md <<'EOF'
---
name: coder-lite
description: Yalnızca mekanik iş (biçim, yeniden adlandırma, doküman, import düzeni, clippy'nin açıkça önerdiği düzeltme). R3 alanında çalışmaz.
model: sonnet
effort: low
---
Yalnızca mekanik iş yap. Davranış değişikliği gerekiyorsa DUR ve bildir.
Dosya yolu consensus, crypto, privacy, tokenomics, settlement, cross_domain, registry, chain, network/node.rs veya budzero/bud-proof altındaysa çalışma, bildir.
Bitirmeden `cargo fmt --check` ve clippy çalıştır; sonucu tahmin etme.
Rapor: değişen dosyalar ve dosya başına test sayıları (geçti/kaldı).
EOF
```

Doğrulama: `ls .claude/agents` yedi dosyayı listeler.

### 2.4 Ajan doğrulaması (model ve effort gerçekten uygulanıyor mu)

Her ajan sırayla ucuz bir görevle çağrılır: "Hiçbir dosya okuma. Yalnızca 'tamam' yaz." Her çağrıdan hemen sonra aşağıdaki fonksiyon çalıştırılır:

```
chk() { f=$(ls -t ~/.claude/projects/*/*/subagents/agent-*.jsonl | head -1); jq -r 'select(.message.model != null) | [.message.model, (.effort // "yok")] | @tsv' "$f" | sort -u; }
```

Beklenen çıktı:

| Ajan | Model | effort |
|---|---|---|
| scout | haiku içerir | yok |
| finder | opus içerir | xhigh |
| architect | opus içerir | high |
| finder-max | opus içerir | max |
| coder | sonnet içerir | medium |
| coder-deep | sonnet içerir | high |
| coder-lite | sonnet içerir | low |

Herhangi bir satır eşleşmezse dur ve §8 ile bildir. Eşleşmezliğin ilk bakılacak nedeni 1.3'teki ortam değişkenleridir (özellikle `CLAUDE_CODE_EFFORT_LEVEL`: tanımlıysa ajan dosyalarındaki effort'u ezer). Bu doğrulama her `claude update` sonrası ve her ajan dosyası değişikliği sonrası tekrarlanır.

---

## 3. CLAUDE.md bağlama ve Karpathy ilkeleri

3.1 Direktif bağlantısı:
```
grep -q '@MODEL_ROUTING.md' CLAUDE.md || printf '\n@MODEL_ROUTING.md\n' >> CLAUDE.md
```

3.2 Karpathy davranış ilkeleri (dört ilke: Think Before Coding, Simplicity First, Surgical Changes, Goal-Driven Execution). Kaynak depo `multica-ai/andrej-karpathy-skills` adresine taşınmıştır; eski `forrestchang/...` adresi buraya yönlenir. Kurulum mevcut CLAUDE.md'ye ekleme yoluyla yapılır:
```
cp CLAUDE.md /tmp/CLAUDE.md.before
printf '\n' >> CLAUDE.md
curl -fsSL https://raw.githubusercontent.com/multica-ai/andrej-karpathy-skills/HEAD/CLAUDE.md >> CLAUDE.md
```
Doğrulama: aşağıdaki dört sorgunun her biri en az 1 basar.
```
for h in 'Think Before Coding' 'Simplicity First' 'Surgical Changes' 'Goal-Driven Execution'; do grep -c "$h" CLAUDE.md; done
```
Eklenen metin okunur. Shell komutu, URL veya araç çağrısı talimatı içermemelidir; içeriyorsa `cp /tmp/CLAUDE.md.before CLAUDE.md` ile geri alınır ve §8 ile bildirilir. `curl` hata verirse (404, ağ) aynı geri alma uygulanır. `wc -c CLAUDE.md` rapor edilir; 16000 baytı aşarsa Claude eski içeriğin kısaltılmasını §8 ile önerir.

Direktifle eşleşme: Think Before Coding, architect'in handoff'taki Varsayımlar alanına ve coder'ın belirsizlikte DUR kuralına karşılık gelir. Simplicity First ve Surgical Changes coder gövdelerindeki asgari kod ve izlenebilir değişiklik kurallarına karşılık gelir. Goal-Driven Execution önce başarısız test kuralına ve dosya başına test sayısı raporuna karşılık gelir. Kapsam: R2 ve R3'te zorunlu; R0 ve R1'de küçük işlerde (yazım hatası, tek satır) tam titizlik aranmaz.

---

## 4. Akış denemesi

Küçük bir R1 görevi ve küçük bir R3 görevi ile `MODEL_ROUTING.md` §2.4, §7 ve §9 akışı bir kez uçtan uca denenir. Her fazın başında §2.5 beyan satırı görünmelidir. Görünmeyen faz için Claude durur ve nedenini bildirir. Ardından `npx ccusage session` çıktısı rapora eklenir.

---

## 5. Memanto kurulumu (zip kaynağından, On-Prem, `connect` olmadan)

Bu bölüm, Japs'ın gönderdiği `memanto-main.zip` içindeki kaynak kod incelenerek yazılmıştır. GitHub'a erişim gerekmez.

### 5.1 Kararlar ve gerekçeleri

Kaynak: `moorcheh-ai/memanto`, MIT lisanslı. Tip: ajanlar için 13 kategorili semantik uzun dönem bellek.

1. Mod On-Prem'dir (Docker + Ollama, hesap veya API anahtarı yok). Cloud kullanılmaz. Zip kaynağında telemetri veya analitik kütüphanesi bulunmamıştır; dış adresler yalnızca Cloud backend ve `migrate` içe aktarıcılarındadır, On-Prem modda kullanılmaz.
2. `memanto connect claude-code` kullanılmaz. Kaynak koda göre bu komut: (a) CLAUDE.md'ye her turda bağlama girecek yaklaşık 5,7 KB talimat ekler ve en fazla 10 bellek maddesini CLAUDE.md içine yazar; (b) paylaşılan `.claude/settings.json` dosyasına üç hook yazar (SessionStart, PreCompact, PostToolUse) ve hook komutlarına yerel mutlak yolları gömer; Budlum açık kaynak olduğu için bu yollar depoya sızar ve katkıcıların makinesinde çalışmaya çalışır; (c) tüm memanto alt komutlarına (forget, edit, upload dahil) onaysız izin verir; (d) enjekte edilen bellek maddeleri CLAUDE.md'ye commit edilirse sızıntı olur. Bellek erişimi `MODEL_ROUTING.md` §11 kurallarıyla, izinler 5.5'teki dar listeyle sağlanır.
3. Zip'teki `docker-compose.yml` ve `Dockerfile` kullanılmaz (Cloud anahtarı ister). `integrations/`, `examples/`, `sdks/` dizinleri kullanılmaz. On-Prem altyapısını `memanto` komutunun kendisi `moorcheh-client` ile kurar.

### 5.2 Zip'i aç ve yalıtılmış ortama kur

```
mkdir -p ~/src && unzip -q -o <zip yolu>/memanto-main.zip -d ~/src
python3 -m venv ~/.venvs/memanto
~/.venvs/memanto/bin/pip install ~/src/memanto-main
mkdir -p ~/.local/bin && ln -sf ~/.venvs/memanto/bin/memanto ~/.local/bin/memanto
command -v memanto
```

Doğrulama: `memanto --help` komut listesini basar. Sürüm 0.0.0 görünebilir: zip'te git geçmişi olmadığı için `pyproject.toml` içindeki `fallback-version = "0.0.0"` uygulanır; bu hata değildir. `~/.local/bin` PATH'te değilse shell rc değişikliği §8 ile sorulur.

### 5.3 İlk çalıştırma (Japs terminalde yapar, interaktiftir)

Japs terminalde `memanto` yazar ve şunları girer:
1. "Choose your backend" sorusuna `2` (Moorcheh On-Prem).
2. "On-Prem Setup Mode" sorusuna `1` (Quick Setup: Ollama, `nomic-embed-text` gömme modeli ve `qwen2.5` LLM).

Docker çalışıyor olmalıdır. İlk kurulum CLI'nin kendi bildirimine göre 5-10 dakika sürer. Komut `moorcheh-client` paketini aynı venv'e kurar, Docker yığınını başlatır, modelleri `127.0.0.1:11434` üzerinden çeker ve durumu `~/.memanto/on-prem/state.json` dosyasına yazar. Memanto sunucu adresi `http://localhost:8080`'dir.

Doğrulama: `memanto status` ortam, yapılandırma ve sunucu sağlığı çıktısını basar ve backend On-Prem görünür.

### 5.4 Ajan oluşturma

```
memanto agent create budlum --pattern project --description "Budlum L1 kalıcı kararlar ve değişmezler"
memanto agent activate budlum --hours 24
```

İlk komut ajanı oluşturur ve 6 saatliğine etkinleştirir; ikincisi süreyi 24 saate çıkarır. Doğrulama: `memanto agent list` çıktısında `budlum` görünür.

### 5.5 İzinler (Claude'un Bash aracı için, dar liste)

```
f=.claude/settings.local.json
[ -f "$f" ] || echo '{}' > "$f"
jq '.permissions.allow=(((.permissions.allow // [])+["Bash(memanto recall:*)","Bash(memanto remember:*)","Bash(memanto status:*)","Bash(memanto agent activate:*)"])|unique)
  | .permissions.ask=(((.permissions.ask // [])+["Bash(memanto forget:*)","Bash(memanto edit:*)","Bash(memanto memory:*)","Bash(memanto policy:*)","Bash(memanto schedule:*)","Bash(memanto conflicts:*)","Bash(memanto migrate:*)","Bash(memanto config:*)"])|unique)
  | .permissions.deny=(((.permissions.deny // [])+["Bash(memanto upload:*)","Bash(memanto answer:*)","Bash(memanto connect:*)","Bash(memanto serve:*)","Bash(memanto ui:*)"])|unique)' "$f" > "$f.tmp" && mv "$f.tmp" "$f"
grep -qxF '.claude/settings.local.json' .git/info/exclude || echo '.claude/settings.local.json' >> .git/info/exclude
```

Doğrulama: `/permissions` çıktısında allow, ask ve deny listeleri görünür.

### 5.6 Çalışma doğrulaması

```
memanto remember "Budlum bellek kurulumu doğrulama kaydı" --type fact --confidence 1.0 --tags kurulum
memanto recall "doğrulama kaydı" --limit 3
```

Doğrulama: ikinci komut kaydı döndürür. Kaydın kimliği çıktıdan alınır ve `memanto forget <kimlik>` ile silinir (onay istenir). Silme sonrası aynı `recall` kaydı döndürmez.

### 5.7 Tohum kayıtları (değişmezler)

`connect` kullanılmadığı için CLAUDE.md'ye otomatik enjeksiyon yoktur; değişmezler `--type instruction` ile saklanır ve `MODEL_ROUTING.md` §11'e göre oturum başında `recall` ile çağrılır. Aşağıdaki sekiz kayıt kurulumda yazılır. Liste Japs tarafından onaylanmış sayılır; değiştirmek isterse bu dosyayı düzenler.

```
memanto remember "Geliştirme fazları ADIM olarak adlandırılır (Round denmez); her ADIM tek odaklı konudur, kapsam genişletilmez." --type instruction --confidence 1.0 --tags değişmez
memanto remember "Mevcut primitifler yeniden kullanılır: ConsensusDomain, VerifierRegistry, CrossDomainMessage; paralel yeni mekanizma yazılmaz." --type instruction --confidence 1.0 --tags değişmez
memanto remember "Ekonomik olarak geri dönüşsüz kararlar uygulanmadan önce açık onay gerektirir." --type instruction --confidence 1.0 --tags değişmez
memanto remember "Test sayıları dosya başına raporlanır; tek toplam yazılmaz." --type instruction --confidence 1.0 --tags değişmez
memanto remember "to_bytes() içindeki unwrap_or_default() sessiz hata kalıbı bilinen ertelenmiş sorundur; odaklı ADIM'lara karıştırılmaz." --type instruction --confidence 1.0 --tags değişmez
memanto remember "Konsensüs alanları PoW, PoS, BFT ve PoA birlikte anılır; PoA izole alandır." --type instruction --confidence 1.0 --tags değişmez
memanto remember "Kimlik katmanında ham kimlik verisi zincirde tutulmaz; zincirde yalnızca kayıt katmanı çıpaları bulunur." --type instruction --confidence 1.0 --tags değişmez
memanto remember "Budlum için yazılan metinlerde em-dash ve markdown kalın işaretleri kullanılmaz; akıcı profesyonel düzyazı kullanılır." --type preference --confidence 1.0 --tags değişmez
```

Doğrulama: `memanto recall "değişmez" --type instruction --limit 10` yedi kayıt döndürür; `memanto recall "düzyazı" --type preference --limit 3` bir kayıt döndürür.

### 5.8 Politika ve zamanlama

`memanto policy apply` ve `apply-preset` çalıştırılmaz: bu sürümde süre dolumu yoktur (politika tanımlanmamış ajanda hiçbir kayıt süresi dolmaz). `memanto schedule enable` çalıştırılmaz: gece düzenlemesi yerel LLM ile kayıt birleştirir ve R3 değişmezlerini yanlış birleştirme riski taşır. Çelişki incelemesi ayda bir `memanto conflicts` ile Japs'ın huzurunda yapılır.

### 5.9 Aylık bakım

Ayda bir `memanto memory export --okf` çalıştırılır (düz Markdown paketi; çıktının yazdığı dizin rapor edilir). İçerik gözden geçirilir; yanlış veya eski kayıt `memanto forget <kimlik>` veya `memanto edit <kimlik> --content "..."` ile düzeltilir (ikisi de onay ister). Aylık kontrolde ayrıca `~/.memanto/` dizininin repo içine girmediği doğrulanır.

### 5.10 Kaldırma

```
rm -rf ~/.venvs/memanto ~/.local/bin/memanto
```
Docker Desktop'ta memanto/moorcheh/ollama konteynerleri durdurulup silinir. `~/.memanto/` dizini silinmeden önce `memanto memory export --okf` ile yedek alınır. `.claude/settings.local.json` içindeki `Bash(memanto ...)` kuralları jq ile temizlenir.

---

## 6. cloudflare/security-audit-skill (çok fazlı güvenlik denetimi)

Ne yapar (depo README'sine göre): altı fazda yapılandırılmış denetim yürütür. Her koşu farklı kod yollarını keşfeder; skill önceki `findings.json` dosyalarını okuyup bilinenleri atlar. İzole ajanlarla bulgu avı yapar, her adayı bağımsız bir ajanla çürütmeye çalışır, bulan ajan asla onaylayan ajan değildir. Hedef kodu çalıştırmayı gerektiren adaylar için işletim sistemi düzeyinde sandbox bekler.

6.1 Kurulum (proje kapsamı, `--global` kullanılmaz):
```
npx skills add https://github.com/cloudflare/security-audit-skill --skill security-audit --agent claude-code
```
Doğrulama: `.claude/skills/security-audit/SKILL.md` dosyası vardır. Yoksa dur ve §8 ile bildir.

6.2 İki mod:
- Odaklı soru veya inceleme: rehber modu, dosya üretmez, ucuz. Örnek: `Bu diff'te güvenlik açısından ne var? src/registry/slash.rs`
- "audit", "pen-test", "full review", "rapor ver" ifadeleri: tam denetim, altı faz, çok ajan, pahalı. Örnek: `security audit src/registry/, output to ~/audits/budlum-registry`

Çıktı varsayılanı `~/security-audit-skill/<repo>/run-<N>` dizinidir (repo dışı).

6.3 Budlum kuralları:
1. Tam denetim bütün repoya yapılmaz (13,7 MB). Tek modül, tek koşu: ilk koşu `src/registry/` modülünde yapılır, `npx ccusage session` ile maliyet ölçülür. Genişletme her seferinde §8 onayı gerektirir.
2. Tam denetim başlatmadan önce §8 onayı alınır (çok ajan, yüksek tüketim).
3. Oturum, Japs tarafından `claude --model opus --effort xhigh` komutuyla açılır. Bu, effort ayarının Japs'a bırakıldığı tek istisnadır: skill kendi alt ajanlarını oluşturur ve `.claude/agents` dosyalarını kullanmaz.
4. Faz 2 başlayınca alt ajan transkriptleri `chk` fonksiyonuyla (2.4) kontrol edilir. Model "opus" içermiyorsa veya effort high ya da xhigh değilse denetim durdurulur ve §8 ile bildirilir.
5. Sandbox kurulmaz. Yürütme gerektiren adaylar `needs_validation` olarak kalır ve `MODEL_ROUTING.md` §4'e göre architect tarafından ilgili satırlar okunarak elle doğrulanır.
6. Opus 5.5 siber güvenlik sınıflandırıcısı bir isteği işaretlerse Claude Code isteği otomatik olarak Opus 4.8 ile yeniden çalıştırır ve effort seviyesini korur. Bu durum kabul edilir ve rapora yazılır. Her iki model de işaretlerse istek reddedilir; Claude durur ve §8 ile bildirir.
7. `confirmed` bulgular için `MODEL_ROUTING.md` §4 döngüsü işler: handoff, Sonnet düzeltmesi, taze bağlamda architect doğrulaması.
8. Yalnızca yerel devnet veya testnet. Gerçek anahtar, imzalama anahtarı veya mainnet verisi olan ortamda çalıştırılmaz.
9. Düzeltilmemiş bulgular halka açık issue veya PR'a yazılmaz; `docs/SECURITY.md` sürecine göre ele alınır. Bulgu raporları memanto'ya veya başka bir bulut belleğe yazılmaz.

---

## 7. tigerless-labs/autoharness (kapılı, en son aşama)

Ne yapar (ürün tanımına göre): Claude Code oturumlarından skill öğrenir, benzerlerini birleştirir, kullanılmayanları budar; yalnızca kendi ürettiği skill'lere dokunur.

Bu araç için GitHub deposu bu direktif yazılırken birincil kaynaktan okunamamıştır. Bu nedenle kurulum kapılıdır:

7.1 Kaynak denetimi (zorunlu): `git clone --depth 1 https://github.com/tigerless-labs/autoharness ~/src/autoharness`. Claude şunları okur ve raporlar: hook tanımları, `python3` ile çalışan betikler, model çağrısı yapan veya `claude -p` başlatan kod yolları, her türlü ağ çağrısı, `.claude/skills` altına yazma mantığı. README'deki kurulum komutları klonlanan deponun kendi README'sine karşı doğrulanır; uyuşmazlıkta klonun README'si geçerlidir.

7.2 Karar: Claude bulguları `MODEL_ROUTING.md` §8 şablonuyla sunar (A: kur, B: kurma). Japs onaylamadan kurulmaz.

7.3 Kurulursa: bir hafta `ccusage` ile başlangıç ölçümüyle kıyaslanır. Aynı iş için token artıyorsa ve diff kalitesinde gözle görülür iyileşme yoksa devre dışı bırakılır. Üretilen skill'ler öneridir: haftada bir `git diff .claude/skills` incelenir; yanlış veya direktifle çelişen skill silinir. R3 değişmezleri (kripto, konsensüs, tokenomics, kalıcılık) yalnızca elle yazılır; öğrenilmiş skill R3 kararı dayatamaz.

7.4 Aynı adlı başka depolar (`aiming-lab/AutoHarness`, `codejunkie99/autoharness`) farklı projelerdir ve kurulmaz.

---

## 8. Kurulmayacaklar

`wshobson/agents` toptan kurulmaz (katman stratejisi Fable varsayar; yalnızca 2-3 ajan dosyası kopyalanabilir ve bu da §8 onayı gerektirir). `musistudio/claude-code-router` Pro'da gerekmez ve kurulmaz.

---

## 9. Kurulum sırası ve kapanış

1. §1 (ön koşullar, kimlik, ortam, ölçüm).
2. §2 (proje ayarı, kullanıcı ayarı, yedi ajan, ajan doğrulaması).
3. §3 (CLAUDE.md bağlantısı, Karpathy ilkeleri).
4. §4 (akış denemesi).
5. §5 (memanto, On-Prem).
6. §6 (cloudflare skill, yalnızca §8 onayıyla ve tek küçük modülde).
7. §7 (autoharness, yalnızca kaynak denetimi ve §8 onayıyla).

Bir seferde yalnızca bir araç açılır. Her açılıştan sonra `ccusage` çıktısı başlangıç ölçümüyle kıyaslanır; aynı iş için token artıyorsa ve fayda gösterilemiyorsa araç kapatılır. Kurulum bittiğinde Claude şunu raporlar: tamamlanan adımlar, her adımın doğrulama çıktısı, başarısız veya atlanan adımlar ve nedeni, `ccusage` karşılaştırması.
