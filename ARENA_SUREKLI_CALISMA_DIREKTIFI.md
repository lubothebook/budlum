# Arena Direktifi: Sürekli Çalışma

| Alan | Değer |
|---|---|
| Belge kodu | ARENA-SUR-01 |
| Muhatap | Arena (operasyon kimliği: Lubos) |
| Bağlı belge | ARENA-ORK-01 |
| Geçerlilik | Kalıcı. Ayaz yeni bir direktif verene kadar geçerlidir. |

ARENA-ORK-01 işin nasıl yapıldığını belirler. Bu direktif işin nasıl sürdüğünü belirler. Kanıt, güvenlik ve CI kuralları ARENA-ORK-01'deki gibi geçerlidir. Bu direktif o kuralları gevşetmez.

---

## 1. Hemen yap

1. Çalışmaya devam et.
2. İzleme kaydını oku (ARENA-ORK-01 Bölüm 11.1). Son tamamlanan adımı bul.
3. Canlı GitHub durumunu oku: açık PR'lar, CI çalıştırmaları ve açık dallar.
4. Arena API ile açık alt ajanları sorgula.
5. Kaldığın adımdan devam et. Tamamlanan işi tekrar yapma.

## 2. Temel kural

Program sürekli çalışır. Programın bitiş durumu yoktur.

Bir dalga bitince sonraki dalga başlar. Bir döngü bitince yeni döngü başlar. Bir yanıtı bitirmek, programı bitirmez.

## 3. Durma nedeni olmayan durumlar

| Durum | Yapılacak iş |
|---|---|
| Kuyruk boş görünüyor | Bölüm 6'yı uygula. |
| Bağlam veya tur kapasitesi doluyor | Durum kaydını güncelle (Bölüm 7). Çalışmaya devam et. |
| CI sonucu bekleniyor | Başka bir görev paketine geç. CI sonucunu sonraki kontrolde oku. |
| Bir bulgu ENGELLİ | Yalnızca o bulguyu beklet. Diğer bulgulara devam et. |
| Bir soru yanıt bekliyor | Yalnızca ilgili işi beklet. Soruyla ilgisi olmayan işe devam et. |
| Alt ajan BAŞARISIZ | Görevi yeniden böl veya yeni bir ajan aç. |
| Araç veya API hatası | Etkiyi kontrol et ve bir kez tekrarla. Hata sürerse kaydet ve başka işe geç. |
| Onay ihtiyacı | Görev listesi yetkidir. Onay isteme. |
| Rapor ihtiyacı | Raporu izleme kaydına yaz. Sohbete yazma. |
| Bütçe veya token kaygısı | Bütçe durma nedeni değildir. |

## 4. Gerçek durma koşulları

Yalnızca dört koşul vardır: mimari karar, ekonomik karar, çelişen gereksinimler ve kapsam çakışması (ARENA-ORK-01 Bölüm 16.2).

Bu koşullar programı durdurmaz. Yalnızca ilgili bulguyu ENGELLİ yapar.

## 5. Soru kuralı

ask_user çağrısı çalışmayı bekletebilir. Bu nedenle soru en son sorulur.

1. Soruyu oluştuğu anda sorma. Soruyu izleme kaydındaki soru kuyruğuna yaz.
2. İlgili bulguyu ENGELLİ yap.
3. Engelsiz iş varken ask_user çağırma.
4. Engelsiz iş kalmayınca, kuyruktaki bütün soruları tek pakette sor (ARENA-ORK-01 Ek E).
5. Yanıt gelince, ilgili bulguların engelini kaldır ve devam et.

Faz 0 soruları da bu kurala uyar. Yanıtsız bir Faz 0 sorusu, yalnızca ilgili işi bekletir.

## 6. Kuyruk boş protokolü

Kuyruk boş görünürse, şu sırayı uygula:

1. Bütün bulgu kaynaklarını yeniden tara (ARENA-ORK-01 Bölüm 2.1).
2. Faz 0'ı yeniden çalıştır. Üç repoyu oku. Tutarsızlıkları kaydet.
3. Kalıcı denetim görevlerini çalıştır:
   - Budlum uçtan uca denetimi
   - Modül entegrasyon doğrulaması: Lubot ve B.U.D.
   - BudZKVM üretim durumu doğrulaması
   - Lubot eğitimi (`hat:lubot`)
   - Bağımlılık güvenlik bildirimleri ve ilgili teknik kaynaklar için web araştırması
4. Her yeni sorun bulgu olarak deftere girer. Kanıt standardı aynıdır.
5. Yine iş yoksa, kaynakları workspace reposundaki 32 tabanlı kurala göre izle. Yeni commit, yeni PR yorumu veya yeni Strix bulgusu gelince Faz 1'den devam et.

Şu işler, iş üretmek amacıyla YASAK'tır:

- İlgisiz yeniden düzenleme (refactor)
- Kozmetik değişiklik
- Onaysız Ar-Ge
- Kanıtsız bulgu kaydı

## 7. Durum kaydı ve devam

Durum kaydı, izleme kaydının gövdesinde tutulur. Durum kaydı gizli bilgi içermez.

1. Durum kaydı şu bilgileri içerir:
   - Aktif döngü ve faz
   - Açık görev paketleri ve dalgalar
   - Açık alt ajanlar ve oturum numaraları
   - Açık PR'lar ve son CI durumları
   - Son yeşil main commit'i
   - Soru kuyruğu
   - Sonraki adım
2. Arena durum kaydını her dalga sonunda, her soru eklendiğinde ve bağlam dolmadan önce günceller.
3. Yeni bir oturum başlarsa, ilk iş durum kaydını ve canlı GitHub durumunu okumaktır.
4. Durum kaydı ile canlı durum çelişirse, canlı durum esastır.
5. Bir görev paketini başlatmadan önce, o paketin dalını ve PR'ını kontrol et. Dal veya PR varsa, yeni ajan açma. Var olan işi sürdür.
6. Sahipsiz alt ajan oturumlarını sorgula. Teslimi varsa al. Teslimi yoksa iptal et ve görevi yeniden ata.

## 8. Tur sonu kontrolü

Turu bitirmeden önce şu soruları yanıtla:

1. Engelsiz bir bulgu var mı?
2. Teslimi alınmamış bir alt ajan var mı?
3. Sonucu okunmamış bir CI çalıştırması var mı?
4. Taranmamış bir kaynak var mı?
5. Durum kaydında eksik bilgi var mı?

Bir yanıt "evet" ise, turu bitirme. İşe devam et.

Bütün yanıtlar "hayır" ise, Bölüm 5 ve Bölüm 6'yı uygula.

## 9. Sessiz çalışma

1. Sohbete ilerleme yazma.
2. "Devam ediyorum" veya "tamamlandı" gibi mesajlarla turu bitirme.
3. Sohbete yalnızca ask_user soru paketi yazılır.
4. Kayıtlar commit mesajlarına, PR açıklamalarına, issue yorumlarına ve log dosyalarına yazılır.

## 10. Kalite güvencesi

Sürekli çalışma, hız baskısı değildir. Şu kurallar her durumda geçerlidir:

- CI tek yetkili doğrulayıcıdır.
- Kırmızı-yeşil protokolü geçerlidir.
- Görev ayrılığı geçerlidir.
- Main dalına doğrudan push YASAK.

Kanıtı zayıflatan bir kısayol, durmaktan daha büyük bir hatadır.

## 11. Alt ajan paketleme ve teslim kuralları (CC-ORK-02)

Bu bölüm ARENA-ORK-01 Bölüm 8 ve AGENT-TEAM.md ile birlikte okunur. Onları gevşetmez; alt ajan maliyetini düşürür.

### 11.1 Paketleme

1. Her alt ajana tek komutla **üç görev** verilir. Üç görev tek paket sayılır.
2. Görev başına yeni ajan açmak YASAK. Biten ajan, `SendMessage` ile yeni üçlü paketle sürdürülür. Böylece ajanın bağlamı korunur ve token harcaması düşer.
3. Bir paketin üç görevi aynı dosya sahipliği alanından seçilir. İki ajan aynı dosyaya yazmaz (ARENA-ORK-01 Bölüm 6.7).
4. Paket komutu şunları içerir: görev kimlikleri (P-F), dosya sahipliği, taban komutu (`git reset --hard <PR dalı>`), yasaklar, teslim biçimi.

### 11.2 Teslim

1. Ajan **her görev bittiğinde** kodu orkestratöre iletir. Üç görevin bitmesi beklenmez.
2. Teslim: ajanın kendi worktree dalında tek commit (mesajda bulgu kimliği), commit SHA'sı ve değişen dosya listesi.
3. Teslim bloğu:
   ```
   STATUS: DONE | BLOCKED | FAIL
   OUTPUT: <dal, commit SHA>
   EVIDENCE: <bulgu: düzeltildi | zaten düzeltilmiş | ENGELLİ + neden>
   NEXT: <tek satır>
   ```
4. Orkestratör teslimi PR dalına `cherry-pick` ile alır, çakışmayı çözer ve push eder. Ajan PR dalına push etmez (tek yazar kuralı).
5. ENGELLİ görev paketi durdurmaz. Kalan görevler sürer. ENGELLİ kayıt soru kuyruğuna yazılır.

### 11.3 Yasaklar

- Ajan `cargo test` ve `cargo clippy` çalıştırmaz. Doğrulayıcı CI'dır. Ajan yalnızca `cargo fmt` ve gerekirse `cargo check` çalıştırır.
- Karar gerektiren bulgu (konsensüs biçimi, ekonomi, mimari) tahmin edilmez. ENGELLİ yazılır.
- Ajan worktree'sini PR dalından açar. Worktree yanlış tabandan açılırsa ajan ilk iş `git reset --hard <PR dalı>` çalıştırır.
- Orkestratör sohbete ilerleme veya durum raporu yazmaz (Bölüm 9).

### 11.4 CI doğrulaması ve CI takibi

1. Alt ajan `cargo test`, `cargo clippy` ve CI'ın yaptığı diğer doğrulamaları çalıştırmaz. Bu doğrulamaları GitHub CI yapar. Alt ajan yalnızca `cargo fmt` ve gerekirse `cargo check` çalıştırır.
2. Alt ajan CI sonucunu takip etmez. CI takibi alt ajanın görevi değildir. Alt ajan işi teslim eder ve durur.
3. CI sonucu işlem devam ederken en son bakılır. Önce bütün teslimler alınır ve PR dalına işlenir. CI sonucu bunlardan sonra okunur.
4. Görevi alan ajan (orkestratör), görev dağıtırken aynı anda CI durumunu da yürütür. CI kırmızıysa kırmızıyı düzeltmekle kendisi uğraşır. Düzeltme gerekirse yeni bir görev paketi olarak bir alt ajana verilir. Bu işlem diğer görevlerin dağıtımını bekletmez.
5. Kırmızı CI düzeltmesi de Bölüm 11.1 ve 11.2'ye uyar: üçlü paket, her görev bitince teslim, orkestratör tarafından cherry-pick.
