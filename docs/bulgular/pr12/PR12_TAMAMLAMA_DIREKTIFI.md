# PR12 Tamamlama Direktifi

Sahip: Ayaz. Durum: Bağlayıcı. Bu dosya PR12 kalan bulgu programının tek giriş noktasıdır.

## Neden bu dosya var
- Görev paketleri, kurallar ve ilerleme PR9'a dosya olarak yazılır. PR yorumu olarak yazılmaz.
- Amaç: yeni oturum fark çıkarma işini tekrar yapmadan, doğrudan kaldığı paketten devam eder.
- Amaç: token masrafı. 1099 bulguluk defter, PR12 commit'leri ve karar yorumları bir kez tarandı. Sonuç `paket-*.txt` dosyalarındadır. Yeni oturum bu taramayı tekrarlamaz.

## Kaynaklar
- PR9 (`consolidate/audit-62-mainnet`): direktif, defter (`docs/bulgular/BULGU_DEFTERI.md`), ham bulgular (`docs/audit/FINDINGS.md`, `notes/Pxx-scan.md`). Tek doğru kaynak.
- PR12 (`arena/6e08b7e6-budlum`): kod dalı. Commit hedefi yalnızca budur.

## Kural 1: Önce fark çıkar, sonra kodla
1. PR9 bulgularını kimlikleriyle listele.
2. PR9'da kodlanmış bulguları işaretle.
3. PR12'de kodlanmış bulguları işaretle.
4. Ana dalda ve birleşmiş PR'larda kodlanmış bulguları işaretle.
5. Kalan liste = tüm bulgular eksi işaretli bulgular. Yalnızca bunu kodla.

Fark çıkarma yapıldı (2026-10-18). Sonuç:
- Defter: 1099 bulgu. PR9'da kodlanmış (CI BEKLİYOR, KAPALI, YANLIŞ POZİTİF) ya da PR12 commit konusunda BLG veya Pxx-Fyy kimliğiyle geçenler çıkarıldı: kalan 610.
- Kararla kapananlar çıkarıldı (K5-C, K6-B, K7-B, K8-C, K9-C: P06-F09..F13, P12-F01..F07).
- Cevapsız karara bağlı olanlar çıkarıldı (K13-K26: P07-F01, P11-F06, P14-F04, P17-F01, P19-F03, P19-F04, P20-F06, P23-F03, P31-F01, P33-F01, P35-F04).
- Kalan: 595 bulgu, 11 pakette.

## Kural 2: Tekrar yazma yasak
- Kodlamadan önce ilgili kodu PR12 dalında ve ana dalda ara (`rg`, `git log -S`). Kod varsa atla.
- Başka PR'daki kodu PR12'ye kopyalama.

## Kural 3: Yalnızca çalışan kod
- Kaynak kod, test, gerekli yapılandırma commit edilir. Doküman, `.md`, rapor, yalnızca yorum değişikliği PR12'ye commit edilmez.

## Kural 4: Ajan yönetimi
- Ana oturum planlar ve dağıtır. Kodu Sonnet modelindeki alt ajanlar yazar.
- Ajan başına bir paket. Paket, aynı dosya alanındaki bulguların listesidir. Ajan listeyi tek tek işler: bir bulgu, bir commit.
- Neden paket: bulgu başına yeni ajan açmak her seferinde aynı bağlamı yeniden okutur. Paket bunu bir kez yapar.
- Aynı anda en fazla 3 ajan. Paketlerin dosya alanları çakışmaz. Kök crate (`budlum-core`, `src/**`) paketlerinden aynı anda yalnızca biri çalışır.
- Ajan yerelde `cargo test`, `cargo clippy`, `cargo build` çalıştırmaz (Ayaz kararı, 2026-10-18). Doğrulayıcı GitHub CI'dır. Yerelde yalnızca `cargo fmt` ve gerekirse `cargo check`.
- Ajan kuralları: `AJAN_KURALLARI.md`. Görev metni yalnızca paket kimliğini, dosya sahipliğini ve bu dosyanın yolunu verir.

## Kural 5: Commit
- Bulgu başına bir commit: `fix(<BLG-ID>): <kısa açıklama>`.
- Ajan kendi dalında (`pkg/<PAKET>`) commit atar, push etmez. Ana oturum `cherry-pick` ile PR12 dalına alır, `cargo fmt --check` ve `cargo check` yapar, push eder.
- Ana oturum CI kırmızısını PR12 dalında düzeltir.

## Kural 6: Çıktı
- Ayaz'a rapor, özet, açıklama yazılmaz. Tek çıktı: kalan bulgu ID listesi ve her biri için commit SHA. Liste boşalınca dur.

## Paketler

Dosyalar bu dizindedir. `paket-<ID>.txt`: satır başına `BLG | Pxx-Fyy | durum | yer | eski sha | kanıt`. `sonuc-<ID>.txt`: satır başına `BLG | KOD <sha>` ya da `BLG | SKIP-VAR/YP/DOC/KARAR/FAIL | neden`. Sonuçtaki SHA ajan dalındadır; PR12'deki karşılığı aynı commit konusuyla bulunur (`git log --oneline origin/main..origin/arena/6e08b7e6-budlum | grep <BLG>`).

| Paket | Parçalar | Bulgu | Dosya sahipliği | Durum (2026-10-18) |
|---|---|---|---|---|
| X1 | P50-P53 | 84 | xtask/gates/** | durduruldu (kısmi), PR12'de; kalan KALANLAR.txt |
| X2 | P54-P57 | 67 | xtask/gates/**, xtask/tools/** | başlamadı (X1 bitince) |
| B1 | P37-P38 | 48 | bud/** | bitti, PR12'de |
| B2 | P39-P40 | 39 | bud/** | bitti, PR12'de |
| Z | P14, P16, P17, P35 | 36 | budzero/**, crates/wallet-core/**, crates/note-packing/**, src/account_abstraction/** | bitti, PR12'de |
| ST | P23, P24, P26-P30 | 65 | src/** (kök crate) | bitti, PR12'de |
| CR | P02, P07, P10, P18-P22, P25, P36 | 36 | src/** (src/account_abstraction ve src/tests hariç), crates/bpqs/** | durduruldu (kısmi), PR12'de; kalan KALANLAR.txt |
| AI | P31-P34 | 48 | src/ai/**, src/ai_inference/**, crates/ai-inference/** | başlamadı (kök crate, CR bitince) |
| AP | P41-P43 | 51 | budzero/bud-node, budzero/bud-cli, src/bin, src/relayer, src/gateway, src/bns, src/socialfi, src/budlumxyz, src/deed, src/developer_os.rs, src/pollen, crates/budscan | başlamadı (kök crate) |
| T | P44-P49 | 93 | src/tests/**, benches/**, fuzz/**, kani/**, bud/tests/**, bud/fuzz/**, bud/kani/** | başlamadı (kök crate) |
| O | P58-P62 | 28 | .github/** (baseline dosyaları hariç), config/**, ops/**, contracts/**, proto/**, Cargo.toml dosyaları | başlamadı (en son, tek başına) |

## Durum (2026-10-18, oturum kapanışı)
- Oturum token bitimi nedeniyle durduruldu. PR12 ucu: `39b9617`.
- 595 bulgudan 159 KOD (PR12'de), 140 SKIP (VAR, YP, DOC, KARAR, FAIL), 296 işlenmedi.
- Kalan liste: `KALANLAR.txt` (satır başına `paket | BLG | durum`). `ISLENMEDI` satırları sıradaki iştir. SKIP-KARAR satırları karar bekler.
- X1 ve CR durduruldu: işlenmemiş satırları aynı paket kimliğiyle sürdür. Commit edilmemiş iş atıldı.
- PR12 CI son durumu okunmadı. Yeni oturum önce CI kırmızılarını düzeltir.

## Yeni oturum nasıl devam eder
1. Bu dosyayı ve `AJAN_KURALLARI.md` dosyasını oku. Fark çıkarmayı tekrarlama.
2. PR12 dalını çek. Her paket için `sonuc-<ID>.txt` dosyasında olmayan BLG satırları kalan iştir.
3. "Başlamadı" paketleri sırayla başlat: aynı anda en fazla 3 ajan, kök crate paketlerinden yalnızca biri.
4. Ajana worktree aç: `git worktree add /home/user/wt-<ID> -b pkg/<ID> origin/arena/6e08b7e6-budlum`. Görev metni: paket kimliği, `paket-<ID>.txt` yolu, sahiplik, `AJAN_KURALLARI.md` yolu.
5. Ajan bitince commit'leri PR12'ye `cherry-pick` et, push et, `sonuc-<ID>.txt` dosyasını buraya kopyala ve PR9'a commit et.

## Kararlar (PR9 yorumları)
- Cevaplı: K1-A, K2-A, K3-A, K4-A, K5-C, K6-B, K7-B, K8-C, K9-C, K10-A, K11-A, K12-B.
- Cevapsız: K13-K26 (tur 3). Bu kararlara bağlı bulgular SKIP-KARAR kalır.
