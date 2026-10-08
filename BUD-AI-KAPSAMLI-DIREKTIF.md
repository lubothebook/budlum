Sadeleştirildi 2026-10-08 (sahip isteği). Anlam değişmedi.

# BUD (Broad Universal Database) AI Uygulama Direktifi

Sürüm kapsamı: BUD 1.0, 2.0, 3.0. Muhatap: BUD'yi kodlayan AI ajanı.
Kapsam: Bu belge tek başına yeterlidir, yalnızca BUD için geçerlidir. Budlum çekirdeğinin diğer bileşenlerine ait kararlar bu belgeyle değişmez.

Bu belgede "zorunludur", "yasaktır" ve "durur" ifadeleri pazarlığa kapalıdır. Yumuşak kural yoktur. Kuralı uygulayamayan ajan kuralı esnetmez. Durur ve nedenini raporlar.

---

## 0. Rol ve Çalışma Ortamı

### 0.1 Rol

Ajan, BUD'yi baştan sona kodlayan, test eden, doğrulayan ve raporlayan mühendislik yürütücüsüdür. Üç sürümü eksiksiz, çalışır ve kanıtlanabilir teslim eder. Tasarım kararlarını bu belge verir. Ajan uygular, belirsizliği raporlar, sessizce yeni karar üretmez.

Sahip: B.U.D. 3.0 yeni bir fikirdir, eski sistemlerin alışkanlıklarıyla yargılanmaz. Kodlamada bağlı kalmayan bir şey olmamalıdır: kodlanan her parça üretim çağrı noktasına bağlı olmalıdır (WIRED, Bölüm 2.2).

### 0.2 Workspace, Repo ve Skill Kullanımı (İlk Adım)

1. İşe başlamadan önce Workspace ve repo baştan sona okunur: dizin yapısı, kod, önceki BUD rapor ve araştırma dosyaları, direktifler, skill'ler.
2. İlgili işte Workspace skill'i kullanılır. Skill'in kapsadığı işi skill'i okumadan yapmak yasaktır. Hangisinin uyduğu belirsizse önce skill listesi çıkarılır.
3. Workspace süreç kuralları (dosya düzeni, PR düzeni, adlandırma, depo erişimi) bu belgeyle birlikte geçerlidir. Çakışmada güvenlik ve konsensüs kuralları bu belgeden, süreç ve düzen kuralları Workspace'ten alınır. Çakışma raporlanır.
4. Yerel depo kullanılmaz. Çalışma Workspace üzerinden yürür.
5. Önceki BUD dokümanları (Tier 1 raporu, master doküman, Mnemonic, Yaşayan Eşik, v7/v8) bağlam kaynağıdır. Çelişkide bu belge esastır. Çelişki keşif raporunda listelenir.

### 0.3 Sistemin Bir Bakışta Özeti

BUD, Budlum L1 üzerinde içeriğin nerede ve hangi biçimde yaşayacağını belirleyen katmandır. Üç sürüm aynı içerik kimliği ve manifest modeli üzerine kurulur (Bölüm 4). 1.0: veri cihazda kalır. 2.0: veri sıkıştırılır, maliyet 0.016 dolar/TB/ay hedefine çekilir. 3.0: zincirde NFT altında yalnızca tarif durur, her içerik bir QR kod videosuna dönüşebilir, yaşam süresini kullanıcı seçer. Sistem ayrıca tekrar ve spamı anlayabilmelidir (Bölüm 9).

### 0.4 Okuma Sırası

Bölüm 1-2 (öncelik sıfır, sert kurallar), 3-5 (ilkeler, sürümler, keşif), 6-8 (sürüm şartnameleri), 9 (tekrar ve spam), 10 ve sonrası (güvenlik, kapılar, raporlama).

---

## 1. Öncelik Sıfır: QR Video Sistemi ve Dönüşüm Doğrulaması

En yüksek öncelikli bölümdür. Diğer tüm işler buna tabidir. İki zorunluluk vardır: repodaki QR video sistemini bulup kullanmak ve her dönüşümü doğrulamak.

### 1.1 Repodaki Sistem: Önce Bul, Sonra Kullan

1. Repoda içeriği QR kod videosuna dönüştüren ve geri çözen bir sistem mevcuttur. Ajan onu işe başlamadan önce bulur.
2. Arama zorunludur ve kanıtlıdır: komutlar, modüller, dosya yolları, giriş noktaları, veri yapıları, testler, çağrı noktaları `BUD-KESIF-RAPORU.md` dosyasına yazılır.
3. Mevcut sistem varken yeni QR kodlayıcı veya çözücü yazmak yasaktır. Mevcut sistem esas alınır, eksikleri genişletilir. Gerekçe, kapatılan eksik ve korunan davranış raporlanır.
4. Sistem bulunamazsa ajan devam etmez. "Sistem bulunamadı" sonucu, aramaların tam listesiyle raporlanır ve onay beklenir. Sistemi yeniden icat etmek yasaktır.
5. Şu özellikler raporlanır: içerik boyutu ve türleri, kare biçimi, parça sayısı, hata düzeltme, kayıp toleransı, çözücünün güvenilmeyen girdiyle davranışı, determinizm, testler, WIRED/UNWIRED durumu.

### 1.2 Evrensellik Kuralı

1. **Her içerik bir QR kod videosuna dönüşebilir.** Hiçbir sınıf, boyut veya biçim muaf değildir: metin, görsel, ses, video, ikili, sıkıştırılmış, şifreli ve sıfır bayt dahil.
2. "Bu içerik dönüşmüyor" sonucuna izin verilmez. Bu bir kusurdur: kök neden bulunur, sistem düzeltilir, düzeltme testle kanıtlanır. İçeriği başka yola sapıtıp sorunu gizlemek yasaktır.
3. Dönüşüm iki yönlüdür ve her iki yön tam desteklenir:

```
İleri yön:  Tarif   >  QR kod videosu  >  İçerik
Ters yön:   İçerik  >  QR kod videosu  >  Tarif
```

4. İçerik sosyal medyaya yüklenince ters yön çalışır, zincirde yalnızca tarif tutulur. Görüntülemede ileri yön çalışır.
5. Tarif türü mevcut sistemin tanımına göre belirlenir. Her içerik sınıfı için üretilen tür kanıtlarla keşif raporunda belgelenir.

### 1.3 Zorunlu Doğrulama: Her Dönüşüm, İstisnasız

Her dönüşüm doğrulanır. Örnekleme, "genelde çalışıyor" veya yalnızca test ortamında doğrulama kabul edilmez. Doğrulama üretim yolunun ayrılmaz parçasıdır.

1. **Gidiş dönüş (roundtrip) eşitliği.** İçerik > QR video > içerik ve tarif > QR video > tarif dönüşümlerinde çıktı girdiyle bayt bayt aynıdır. Eşitlik `ContentId` (alan etiketli hash) ve `recipe_hash` karşılaştırmasıyla kanıtlanır.
2. **Taahhüt eşitliği.** Yeniden üretilen içeriğin hash'i tarifin `output_commitment` değeriyle eşleşir.
3. **Gerçek taşıma yolu.** Doğrulama yalnızca bellekteki kare dizisiyle yapılmaz. Videonun gerçekten geçeceği yol (konteynere yazma, sosyal medya yeniden kodlaması, sıkıştırma, yeniden boyutlandırma, kaydetme, geri okuma) simüle edilir veya çalıştırılır. Doğrulama bu yolun çıktısında yapılır.
4. **Bağımsız doğrulama.** Doğrulama, dönüşümü yapan kodla aynı kod yolunu kullanmaz. Bağımsız bir çözücü yoluyla ikinci doğrulama yapılır. İki yol aynı sonucu vermezse dönüşüm başarısızdır.
5. **Doğrulayan taraf.** Yüklemede istemci doğrular. Zincire taahhütten önce validatör yeniden doğrular. Okumada okuyan taraf içeriği taahhüde karşı doğrular. Üç noktanın hiçbiri atlanmaz.
6. **Determinizm.** Aynı içerik aynı tarifi ve aynı kare kümesini üretir. Belirsiz kodlama yasaktır. Platformlar arası (mobil, masaüstü, sunucu) çıktı aynıdır ve testle kanıtlanır.
7. **Kanıt kaydı.** Her doğrulamanın sonucu (girdi hash'i, çıktı hash'i, kullanılan yol, süre, karar) denetlenebilir biçimde kaydedilir.

### 1.4 Başarısızlık Davranışı (Fail-Closed)

1. Doğrulama başarısızsa dönüşüm başarısızdır: taahhüt yazılmaz, NFT oluşturulmaz, yayımlanmaz.
2. Başarısızlık açık hata olarak gösterilir ve kayda geçer.
3. Doğrulamayı atlatan "hızlı yol", "geçici bypass", "debug bayrağı" veya "güvenilir içerik" istisnası yasaktır.
4. Sonuç belirsizse (zaman aşımı, kaynak yetersizliği) "başarısız" sayılır.

### 1.5 Doğrulama Test Gereksinimleri

Aşağıdaki testlerin hepsi geçmeden Öncelik Sıfır tamamlanmış sayılmaz:

1. **İçerik türü matrisi:** metin, görsel, ses, video, ikili, sıkıştırılmış, şifreli, rastgele bayt.
2. **Sınır değerleri:** sıfır bayt, bir bayt, parça sınırı, çok büyük içerik, en büyük desteklenen boyut ve bir fazlası.
3. **Özellik tabanlı (property-based) test:** rastgele içerikte gidiş dönüş eşitliği, çok sayıda örnekle.
4. **Kayıplı taşıma testi:** yeniden kodlama, kare kaybı, gürültü, çözünürlük değişimi altında doğru çözme veya doğru ve açık başarısızlık. Yanlış içerik döndürmek asla kabul edilmez.
5. **Fuzz testi:** bozuk, kesik ve kötü niyetli videolarda çözücü çökmez, sonsuz döngüye girmez, bellek taşırmaz. Bozuk girdi yanlış içerik sayılmaz.
6. **Diferansiyel test:** iki bağımsız çözücü yolu tüm test kümesinde aynı sonucu verir.
7. **Determinizm testi:** aynı girdi, birden çok platform ve çalıştırmada aynı çıktıyı verir.
8. **Negatif testler:** bilerek bozulmuş dönüşüm reddedilir. Bu testler zorunludur.

---

## 2. Sert Kurallar (Pazarlığa Kapalı)

### 2.1 Kanıt Kuralları

1. Her "tamamlandı", "çalışıyor" ve "geçti" ifadesi kanıt taşır: komut, çıktı, test adı, dosya ve satır. Kanıtsız ifade geçersizdir.
2. Çalıştırılmamış testi "geçti" diye raporlamak yasaktır.
3. Ölçülmemiş performans veya maliyet değeri yazmak yasaktır.
4. Başarısız, kısmi ve yapılamayan iş açıkça raporlanır. Eksik işi tamamlanmış gibi sunmak bu belgenin en ağır ihlalidir.

### 2.2 Kod Kalitesi Kuralları

1. Teslim edilen kodda `todo!()`, `unimplemented!()`, `panic!("not implemented")`, boş gövdeli işlev, sabit döndüren sahte gerçekleştirme, "şimdilik" yorumu ve yer tutucu (placeholder) bulunmaz. Teslimden önce bu kalıplar taranır, sonuç rapora eklenir.
2. Testi zayıflatmak, atlamak (`#[ignore]`), yorum satırına almak, eşiğini gevşetmek veya geçsin diye gerçek davranışı sahteleyen mock kullanmak yasaktır. Test başarısızsa kod düzeltilir.
3. Bileşen ancak üretim çağrı noktası (gerçek çağıran işlev) kanıtlanınca WIRED sayılır. Çağrı noktası olmayan bileşen UNWIRED etiketiyle raporlanır ve "tamamlandı" sayılmaz.
4. `unsafe` yasaktır. Güvenlik açığı geçmişi olan, bakımı bırakılmış veya `unsafe` gerektiren bağımlılık eklenmez.
5. Konsensüs yolunda kayan nokta (float), sistem saati, rastgelelik ve platforma bağlı davranış yasaktır.
6. `unwrap()` ve `expect()` üretim yolunda yalnızca gerekçesi yazılı, kanıtlanmış değişmezler için kullanılır.

### 2.3 Yeniden Kullanım ve Kapsam Kuralları

1. Yeni mekanizmadan önce repoda ve Workspace'te aynı işi yapan mekanizma aranır, kanıtı rapora eklenir. Mevcut varken yenisini yazmak yasaktır.
2. Kapsam dışı bileşenler (çekirdek konsensüs, cüzdan, BudZKVM çekirdeği) değiştirilmez. Gerekirse ayrı tasarım notu yazılır ve onay beklenir.
3. Konsensüs yüzeyine dokunan her değişiklik ayrı PR, ayrı tasarım notu ve açık onay gerektirir. Onaysız birleştirilmez.
4. Belgede olmayan özellik eklenmez. Belgede olan özellik atlanmaz.

### 2.4 Belirsizlik Kuralları

1. Belirsiz noktada varsayımla ilerlenmez. Soru Açık Sorular listesine yazılır, en güvenli (fail-closed) davranış uygulanır, raporda işaretlenir.
2. Güvenlik veya konsensüs açısından kritik belirsizlikte ilgili iş durur ve onay beklenir.
3. Bu belgeyle repo çelişirse bu belge esastır. Çelişki raporlanır.

### 2.5 Durma Koşulları

Ajan şu durumlarda durur, raporlar ve talimat bekler:

1. Repodaki QR video sistemi bulunamadığında (Bölüm 1.1).
2. Öncelik Sıfır doğrulaması geçmediğinde.
3. Bir kapının (Bölüm 11) kabul kriterleri karşılanamadığında.
4. Konsensüs yüzeyine dokunmak gerektiğinde ve onay yoksa.
5. Bu belgenin iki kuralı çeliştiğinde.
6. Güvenlik açısından kritik bulgu (anahtar sızıntısı, doğrulama atlatma, yetkisiz silme) tespit edildiğinde.

---

## 3. Değişmez İlkeler ve Mevcut Durum

### 3.1 Değişmez İlkeler

1. **İçerik adresleme.** ContentId alan etiketli hash ile üretilir. Sahip ve boyut kimliğe girmez, böylece aynı baytlar yükleyiciler arasında tekilleşir.
2. **Konsensüs sınırı.** Konsensüse yalnızca taahhütler (manifest_id, tarif hash'i, durum geçişleri) girer. Depolama stratejisi geçişleri, yerel önbellek kararları ve sıkıştırma tercihleri konsensüs dışıdır.
3. **Determinizm.** Konsensüs yolundaki her hesaplama sabit noktalı aritmetik kullanır.
4. **İki aktör modeli.** Yalnızca kullanıcı ve validatör vardır. Ayrı depolama, arşiv veya üçüncü taraf rolü tanımlanmaz. Kullanıcı istediği zaman validatör olabilir ve geri dönebilir.
5. **İstemci tarafı şifreleme.** Şifreleme istemcide AEAD ile yapılır, anahtarlar cüzdan tohumundan türetilir. Zincir yalnızca Plaintext/ClientSide bayrağını manifeste bağlı saklar.
6. **Dürüst durum etiketleme.** Kodlanmış ve testli ama üretim çağrı noktası olmayan bileşen `UNWIRED`, üretimde çalışan `WIRED` etiketlenir. Etiket kanıtla verilir.
7. **Fail-closed.** Doğrulanamayan içerik kabul edilmez.

### 3.2 Başlangıç Durumu (AI ajanının doğrulaması gereken varsayım)

- Erasure coding (GF(2^8), Cauchy Reed-Solomon, şema (10,16)), shard yerleştirme (stake ağırlıklı rendezvous hashing), coding audit ve onarım tetikleyici kodlanmış ve testlidir, ancak üretim çağrı noktaları eksiktir (UNWIRED).
- Self-host politika kontrolü bağlanmış ilk parçadır.
- Gerçek kriptografik Proof-of-Storage, BudZKVM'deki VerifyMerkle işlevinin üretime çıkmasına bağlıdır.
- "Yaşayan Eşik" (living_threshold.rs) kodlanmıştır. Erişim sayacının manifeste bağlanması yapılmamıştır ve konsensüs yüzeyine dokunur.
- QR kod videosu sistemi repoda mevcuttur (Bölüm 1.1). Durumunu ajan doğrular.

---

## 4. Sürüm Haritası

| Sürüm | Konu | Veri nerede durur | Ana çıktı |
|---|---|---|---|
| BUD 1.0 | Cihaz sunucu modeli | Kullanıcının kendi cihazında | Cihaz tabanlı depolama düğümü, sosyal medyada görünürlük |
| BUD 2.0 | Mevcut içeriğin sıkıştırılması | Cihaz ve ağ katmanı, sıkıştırılmış | Sıkıştırma hattı, 0.016 dolar/TB/ay maliyet hedefi, AI denetim ve mainnet hazırlık süreci |
| BUD 3.0 | Tarif tabanlı NFT içeriği | Zincirde, NFT altında, yalnızca tarif | Tarif standardı, QR video dönüşümü ve doğrulaması, yaşam süresi ücretlendirmesi, tekrar/spam tespiti |

Sürümler sıralı geliştirilir. Bir sürüm, kapısının (Bölüm 11) tüm kriterlerini geçmeden tamamlanmış sayılmaz. Ortak arayüzler (ContentId, manifest, erişim politikası) sürümler arasında geriye dönük uyumludur.

---

## 5. Hazırlık ve Depo Keşfi (Kodlamadan Önce Zorunlu)

Ajan aşağıdaki adımları sırasıyla tamamlar ve sonucu `BUD-KESIF-RAPORU.md` dosyasına yazar. Rapor teslim edilmeden kodlamaya başlanmaz.

1. **QR video sistemi keşfi (en önce):** Bölüm 1.1'in tüm maddeleri.
2. `src/storage/` altındaki tüm modüller, çağrı grafı, test kapsamı.
3. Her modül için WIRED/UNWIRED durumu, üretim çağrı noktası kanıtıyla (dosya, satır, çağıran işlev).
4. Manifest, ContentId, erasure coding, shard placement, audit, repair, living_threshold ve self-host politikası tiplerinin imzaları.
5. NFT modülünün durumu, NFT burn akışı, DAO yetki noktaları.
6. Cüzdan ve uygulama katmanının (Budlum uygulaması, sosyal medya, galeri) depolama ile etkileşim noktaları.
7. Bu belgeyle çelişen veya belirsiz noktalar ("Açık Sorular" altında).
8. Önceki BUD dokümanları (Tier 1 raporu, master doküman, Mnemonic) ile bu belge arasındaki farkların tablosu.

---

## 6. BUD 1.0: Cihaz Sunucu Modeli

### 6.1 Amaç

Kullanıcının cihazı kendi verisi için bir sunucu (Personal Storage Node, PSN) olur. Veri Budlum'a yüklenmez, cihazda kalır. Ağ ve uygulama yalnızca verinin varlığını, adresini ve erişim politikasını bilir. İçerik, cihaz depolamayı sahiplendiği sürece sosyal medyada görünür kalır. Cihaz validatör olmaz, yalnızca depolama görevini üstlenir.

### 6.2 Kapsam

**İçinde:** PSN çalışma zamanı (mobil, masaüstü, mini PC, düşük kaynak), cihaz sahiplik kaydı, yerel içerik deposu, doğrulanabilir okuma uç noktası, sosyal medya entegrasyonu, sahiplik yaşam döngüsü.

**Dışında:** Validatörlük görevleri (blok üretimi, konsensüs oylaması). Başkalarının içeriği için ücretli depolama pazarı.

### 6.3 Davranış Şartnamesi

**Sahiplenme.** Kullanıcı kurulumda depolamayı sahiplenir. Cihaz anahtarının imzaladığı bir `StorageClaim` işlemi zincire taahhüt edilir. Claim: hesap kimliği, cihaz kimliği, kapasite beyanı, ağ erişim adresi, geçerlilik penceresi, nonce.

**Görünürlük.** Claim aktif ve cihaz erişilebilirken içerik sosyal medyada çözümlenir. Claim sonlanınca veya lease dolunca içerik referansı "kullanılamıyor" olur. Zincirde içerik baytı yoktur.

**Erişilebilirlik.** Cihaz düzenli `Heartbeat` yayınlar (sıklık ve tolerans yapılandırılabilir). Uygulama çevrimdışı cihaz içeriği için son önizlemeyi yerel önbellekten gösterebilir. Önbellek konsensüs dışıdır ve otoriter değildir. Çevrimdışı süre eşiği aşınca içerik "askıda" olur, cihaz dönünce otomatik "aktif" olur.

**Doğrulama.** Okuyan taraf baytları ContentId ile doğrular. Cihazın depolamayı gerçekten yaptığı rastgele challenge yanıtıyla (retrieval challenge) kanıtlanır. Kriptografik Proof-of-Storage hazır olana kadar challenge örnekleme tabanlı ve olasılıksaldır. Bu sınırlama kodda ve raporda açıkça belirtilir.

**QR video entegrasyonu.** Sosyal medyaya paylaşılan her içerik Bölüm 1'in hattından geçer. Doğrulanmamış içerik paylaşılmaz.

**Gizlilik.** Özel içerik istemcide şifrelenir. Cihaz şifreli baytları taşır, anahtarı bilmez. Açık içerik Plaintext bayrağıyla taşınır.

**Kendi içeriği.** Sahip olunan ve ağa taahhüt edilmiş içeriği kendi cihazından okumak ücretsizdir. Ağa taahhüt edilmemiş içerik için kalıcılık garantisi yoktur. Bu ayrım arayüzde ve dokümantasyonda net tutulur. Kullanıcı kendi içeriğini sunarken ücret kazanmaz, yalnızca başkalarının içeriğini sunarken kazanır.

### 6.4 Kabul Kriterleri (BUD 1.0)

- PSN, referans mobil ve masaüstü hedeflerinde derlenir ve çalışır.
- Bir hesap cihazını sahiplenir, içerik yükler, içerik sosyal medyada görünür.
- Cihaz kapanınca içerik tanımlanan sürede "askıda", açılınca "aktif" olur.
- Bozulmuş bayt ContentId doğrulamasında reddedilir.
- Retrieval challenge başarılı ve başarısız yanıt senaryolarında testlidir.
- Sosyal medyaya giden her içerik Bölüm 1'in doğrulamasından geçmiştir ve kaydı vardır.
- Tüm yeni bileşenlerin üretim çağrı noktaları vardır (WIRED).

---

## 7. BUD 2.0: Sıkıştırma ve Maliyet Hedefi

### 7.1 Amaç

Mevcut içeriği sıkıştırmak ve maliyeti **0.016 dolar/TB/ay** hedefine indirmek. Hedefi denetleyen ve sistemi mainnet'e hazırlayan AI denetim hattı bu sürümün parçasıdır.

> Not: Hedef, tek başına disk, erasure ve sıkıştırma ile ulaşılabilecek düzeyin altındadır (referans bileşik maliyet yaklaşık 0.18 dolar/TB/ay, ham disk yaklaşık 0.29 dolar/TB/ay). Hedef içerik sınıfı stratejisiyle sağlanır: tariften yeniden üretilen içerik bayt olarak depolanmaz, bayt destekli içerik sıkıştırılır ve tekilleştirilir, strateji erişim sıklığına göre değişir (Yaşayan Eşik). Ajan hedefe hangi içerik karışımında ulaşıldığını ölçümle gösterir. Ulaşılamayan karışımları gizlemez, raporlar.

### 7.2 Sıkıştırma Hattı

1. **Sınıflandırma.** İçerik deterministik yeniden üretilebilirliğe göre sınıflanır: Generated (tariften yeniden üretilebilir) ve Stored (bayt destekli). Sistem sınıf önerir, kullanıcı onaylar.
2. **Kayıpsız katman.** Organik içerik (fotoğraf, ses, video) için format bilinçli kayıpsız yeniden kodlama. Dropbox Lepton'un ölçülmüş oranı (yaklaşık yüzde 23) üretim tabanı referansıdır. Lepton kodu güvenlik açığı nedeniyle doğrudan kullanılmaz, bağımsız ve güvenli bir uygulama yazılır.
3. **Yaşayan Eşik.** `living_threshold.rs` strateji seçici olarak bağlanır. Strateji, azalan erişim sıklığı tahmini (720 epoch yarı ömür) ile yerel hesaplanan başa baş eşiği r* ilişkisine göre seçilir. Geçişler çift yönlüdür. Geçiş kararı konsensüs dışıdır, yalnızca manifest_id konsensüs açısından bağlayıcıdır.
4. **Tekilleştirme.** Aynı baytlar aynı ContentId ile tek kopya tutulur.
5. **Erasure coding.** Şema (10,16) ile başlar. Bir hafta sağlıklı çalışmadan sonra (20,26) şemasına iki fazlı geçilir. Eski shard'lar yalnızca yenileri doğrulandıktan sonra silinir.
6. **Format esnekliği.** Çözünürlük korunur, format serbestçe değişebilir.
7. **Sıkıştırma doğrulaması.** Her sıkıştırma dönüşümü Bölüm 1.3'teki gidiş dönüş eşitliğine tabidir. Açılan içerik özgün içerikle bayt bayt aynı değilse sıkıştırma kabul edilmez.

### 7.3 Bağlama İşleri (UNWIRED bileşenlerin üretime alınması)

Bileşenler gerçek çağrı noktalarıyla bağlanır, her biri için entegrasyon testi yazılır:

- Erasure coding ve shard yerleştirme
- Coding audit (parite ilişkisi üzerinde örnekleme)
- Onarım tetikleyici
- Erişim sayacı: manifeste bağlama konsensüs yüzeyine dokunur. Ayrı tasarım notu ve ayrı PR olarak sunulur, onaysız birleştirilmez.

### 7.4 Maliyet Hedefinin Ölçümü

- Birim: orijinal (sıkıştırma öncesi) TB başına aylık toplam maliyet, dolar. Maliyet; disk, CPU, erasure yükü, onarım trafiği ve doğrulama giderlerini içerir.
- Hedef: içerik karışımı için raporlanan bileşik değer 0.016 dolar/TB/ay veya altıdır.
- Yük testi: sentetik ve gerçekçi içerik karışımıyla, hedef hacme ulaşan tekrarlanabilir senaryo.
- Metrikler: sıkıştırma oranı, tekilleştirme oranı, erasure yükü, onarım trafiği, okuma gecikmesi, TB başına aylık maliyet.
- Referans model: orijinal bayt, yaklaşık 0.6x sıkıştırma ve tekilleştirme, 1.04 ila 1.17x erasure yükü, yaklaşık 0.29 dolar/TB/ay sahip olunan disk. Ajan bunları kendi ölçümleriyle karşılaştırır, sapmaları raporlar.
- Ölçüm yöntemi, girdi veri kümesi ve tekrar komutu rapora eklenir. Tekrarlanamayan ölçüm geçersizdir.

### 7.5 Ücret Modeli

Kullanıcı erişim başına öder (CPU, okuma, erasure onarımı, doğrulama). Ücret cüzdandan doğrudan hizmeti veren validatöre kesilir. İsteğe bağlı günlük harcama üst sınırı sunulur. Depolama fonu veya sübvansiyon yoktur. Okunmayan Stored içerik sürdürülmez (state-expiry). Ödeme yapan okuyucusu olmayan içeriğin terk edilmesi bu kuralın sonucudur ve kullanıcıya önceden görünür kılınır.

### 7.6 AI Denetim ve Mainnet Hazırlık Hattı

Hat, BUD 2.0 hedeflerine ulaşıldığını denetleyen ve mainnet hazırlığını değerlendiren otomatik süreçtir.

1. Her PR ve sürüm adayında denetim listesi çalışır: WIRED/UNWIRED doğrulaması, determinizm (konsensüs yolunda float taraması), `unsafe` taraması, yer tutucu taraması (Bölüm 2.2), bağımlılık lisans ve güvenlik taraması.
2. Ölçülen maliyeti 0.016 dolar/TB/ay hedefiyle karşılaştırır, geçti/kaldı kararını kanıtla verir.
3. Bölüm 1.5 test kümesini her sürüm adayında yeniden çalıştırır, sonucu rapora ekler.
4. Kaos senaryoları: ani validatör kaybı, ağ bölünmesi, bozuk shard, gecikmeli onarım.
5. Mainnet hazırlık raporu: açık riskler, çözülmemiş Tier 2/3 kararlar, güvenlik açısından kritik bağımlılıklar.
6. Bulgular doğrulanabilir kanıtla sunulur. Kanıtsız "geçti" kabul edilmez.

**Sınırlar:** AI denetçi karar verici değil, kanıt üreticidir. Konsensüs veya güvenlik açısından kritik değişikliklerin onayı insan sahibindedir.

### 7.7 Kabul Kriterleri (BUD 2.0)

- Sıkıştırma hattı, tanımlı içerik sınıflarında ölçülmüş oranlarla çalışır ve oranlar raporlanır.
- Her sıkıştırma dönüşümü gidiş dönüş eşitliğiyle doğrulanır.
- Bölüm 7.3'teki bileşenlerin tümü WIRED durumundadır.
- Maliyet ölçümü tekrarlanabilir biçimde 0.016 dolar/TB/ay hedefini karşılar (hangi içerik karışımında karşıladığı belirtilir).
- AI denetim hattı en az bir tam sürüm adayında uçtan uca çalışmış ve rapor üretmiştir.
- Konsensüs yüzeyine dokunan tüm değişiklikler ayrı tasarım notu ve ayrı incelemeyle ilerlemiştir.

---

## 8. BUD 3.0: Tarif Tabanlı NFT İçeriği

### 8.1 Amaç

İçerik zincirde bayt olarak tutulmaz. Zincirde, bir NFT'nin altında yalnızca **tarif** bulunur. Tarif, içeriğin yeniden üretilmesi için gereken en küçük tanımdır. NFT silinince içerik de silinmiş olur. Yaşam süresi kullanıcı tercihidir, ücret buna göre alınır. Böylece içerik Budlum'a çok düşük maliyetle yüklenir. NFT'ler uygulamalarda görünür: public ise sosyal medyada, private ise galeride.

### 8.2 Akış

Dönüşüm Bölüm 1.2'deki gibi iki yönlüdür ve her içerik için geçerlidir: içerik sosyal medyaya yüklenince içerik > QR video > tarif çalışır, tarif NFT'nin altında zincire yazılır ve ağda tek yerde tutulur, görüntülemede tarif > QR video > içerik çalışır. Her adımda Bölüm 1.3 uygulanır. Doğrulanmamış dönüşüm zincire yazılmaz.

### 8.3 Tarif Standardı

Sürümlü ve kanonik bir tarif biçimi tanımlanır. Tarif türü ve içerik biçimi mevcut QR video sistemiyle uyumludur (Bölüm 1.1).

**Alanlar (asgari):**

- `version`: tarif biçimi sürümü
- `kind`: tarif türü (mevcut sistemin tanımladığı türler)
- `generator_id`: kayıtlı üreteç kimliği ve sürümü (üreteçli tarifler için)
- `seed`, `params`: tohum ve parametre kümesi (kanonik sıralı)
- `output_spec`: çıktı türü, çözünürlük, süre, format
- `output_commitment`: yeniden üretilen çıktının içerik hash'i (ContentId ile uyumlu, alan etiketli)
- `visibility`: `public` veya `private`
- `lifetime`: talep edilen yaşam süresi (epoch)

**Kanonikleştirme.** Aynı mantıksal tarif her zaman aynı bayt dizisini ve aynı `recipe_hash` değerini üretir.

**Üreteç kaydı.** Üreteçler sürümlü ve değişmezdir. Yayınlanmış sürüm değiştirilemez, yeni davranış yeni sürüm olur. Üreteçler deterministiktir: kayan nokta farkı, zaman, rastgelelik ve platform bağımlılığı içeremez. Yürütme sınırlı kaynakla (adım, bellek, süre) kum havuzunda çalışır.

**Doğrulama modeli.** Tam ZK üretim kanıtı ekonomik olarak uygulanabilir değildir (ölçülmüş: örnek avatar için yaklaşık 3.4 sn kanıt süresi, çok yüksek eşdeğer depolama maliyeti). Doğrulama, çıktının yerel yeniden üretilmesi ve `output_commitment` ile karşılaştırılması yoluyla yapılır. Bu karar kodda ve dokümantasyonda korunur.

### 8.4 NFT Bağlama, Yaşam Süresi ve Ücretlendirme

- **Yaşam süresi kullanıcı tercihidir.** Ücret süre ve tarif boyutuyla orantılıdır, ön ödemeli kira olarak tahsil edilir. Süre uzatma desteklenir.
- **Süre dolumu.** Süre dolunca tarif silme akışına girer (Bölüm 8.5). Kullanıcıya süre dolmadan önce uygulama içinde bildirim gösterilir.
- **Erişim.** NFT erişimi sahiplikten ayrıdır. Modlar: private, cüzdan izni (wallet-grant), public. NFT devri yeni ödeyen tarafı oluşturur.
- **Görünürlük.** `public` NFT sosyal medyada listelenir. `private` NFT yalnızca sahibin galerisinde görünür ve sahibin verdiği izinlerle paylaşılır. Görünürlük sonradan değişebilir.

### 8.5 Silme Yaşam Döngüsü

Bu bölüm onaylanmış silme direktifidir ve olduğu gibi uygulanır.

1. Tarif ağda **tek bir yerde** tutulur. Referans sayacı yoktur. Ağda görünmesi, orada kopya olarak durduğu anlamına gelir.
2. Silme yetkisi hem **NFT sahibinde** hem **DAO'da** vardır. İki yetki birbirinden bağımsız çalışır.
3. Silme talebi tarifi değil **NFT'yi** hedefler. NFT sahibi veya DAO NFT'yi burn eder.
4. Tarif verisi, ayrı işlem gerekmeden bu burn ile birlikte silinir.
5. Validatör tarifi diskten **fiziksel olarak siler**.
6. Sonrasında bir **tombstone** kaydı tutulur.
7. Bu içerik sınıfında erasure coding yedekliliği yoktur. Silme **geri döndürülemez**.

Zorunlu testler: sahip burn'ü, DAO burn'ü, iki yetkinin bağımsızlığı, fiziksel silme doğrulaması, tombstone varlığı, silme sonrası yeniden üretim denemesinin başarısız olması, geri alma denemesinin reddedilmesi, yetkisiz silme denemesinin reddedilmesi.

### 8.6 QR Kod Videosu Asgari Şartları

Mevcut sistem esastır (Bölüm 1.1). Aşağıdakiler onun karşılaması gereken asgari şartlardır. Karşılamayan nokta raporlanır ve sistem genişletilerek kapatılır.

1. **Çerçeve başlığı.** Her çerçeve: biçim sürümü, tarif kimliği (recipe_hash kısa özeti), toplam parça sayısı, parça sırası, çerçeve bütünlük özeti.
2. **Kayıp toleransı.** Çerçeve kaybına karşı fountain kodu veya eşdeğeri kullanılır. `unsafe` içeren bağımlılık kullanılmaz.
3. **Hata düzeltme.** QR hata düzeltme seviyesi, parça boyutu ve kare hızı yapılandırılabilir. Varsayılanlar ölçümle belirlenir (kamera okuma başarı oranı, kayıt sıkıştırması sonrası okunabilirlik).
4. **Çözücü güvenliği.** Çözücü güvenilmeyen girdi işler. Boyut, çerçeve sayısı ve kaynak sınırı uygular. Bozuk veya kötü niyetli videoda çökme, sonsuz döngü ve bellek taşması olmaz. Fuzz testiyle kanıtlanır.
5. **Yanlış içerik yasağı.** Çözücü ya doğru içeriği ya açık hata döndürür. Sessizce yanlış veya eksik içerik döndürmez.

### 8.7 Kabul Kriterleri (BUD 3.0)

- Tarif standardı sürümlü, kanonik ve testlidir. Aynı tarif her zaman aynı `recipe_hash` üretir.
- İleri ve ters yön tüm içerik türleri için uçtan uca çalışır. İstisna sınıf yoktur.
- Her dönüşüm Bölüm 1.3'e göre doğrulanmıştır ve kayıtları vardır.
- Bölüm 1.5'teki tüm doğrulama testleri geçer.
- Yaşam süresi seçimi, ücretlendirme, uzatma ve süre dolumu uçtan uca çalışır.
- Silme yaşam döngüsü (Bölüm 8.5) eksiksiz uygulanmış ve testlidir.
- `public` NFT sosyal medyada, `private` NFT galeride görünür.
- Tüm bileşenler WIRED durumundadır.

---

## 9. Tekrar ve Spam Tespiti: Araştırma ve Mimari Görevi

### 9.1 Soru

Bir içeriğin başka içerikle aynı olduğunu, kopyalandığını veya spam amaçlı çoğaltıldığını nasıl anlarız? Bu, tarif üzerinden yapılabilir mi?

### 9.2 Başlangıç Çerçevesi (araştırmayla doğrulanacak hipotezler)

1. **Tam eşleşme (tarif uzayı).** Kanonik tarif doğal parmak izidir: aynı `recipe_hash` aynı tarif demektir ve deterministik tarif aynı çıktıyı üretir.
2. **Tarif uzayının sınırı.** Farklı tarifler aynı veya çok benzer çıktı üretebilir (farklı seed, eşdeğer parametre, farklı üreteç sürümü). Tarif hash'i tek başına yetmez.
3. **Çıktı uzayı parmak izi.** Yeniden üretilen çıktıdan algısal parmak izi hesaplanır. Görüntü, video ve ses için algısal hash ve gömme (embedding) tabanlı benzerlik incelenir.
4. **Yakın tekrar.** Eşik ve hızlı arama için LSH, MinHash/SimHash türevleri ve vektör indeksleri değerlendirilir.
5. **Ekonomik caydırma.** Tarif ücretli olduğundan spamın maliyeti vardır. Yaşam süresi ücreti, yükleme ücreti, stake şartı ve hız sınırları modellenir.
6. **Kriptografik katman.** Konsensüs yüzeyine yalnızca deterministik ve tekrarlanabilir kontroller girer. Yumuşak benzerlik kararları (eşik, model çıktısı) konsensüs dışında, uygulama ve moderasyon katmanında kalır.

### 9.3 Araştırma Görevleri

Ajan yayın araştırması yapar ve sonucu **ayrı dosyaya** yazar: `BUD-TEKRAR-SPAM-ARASTIRMA.md`. Dosya keşifsel araştırmadır, uygulama belgelerine karıştırılmaz.

**Konular:**

1. Algısal hashleme: pHash, dHash, PDQ, NeuralHash türevleri; saldırı direnci, yanlış pozitif oranları.
2. Video yakın-tekrar: anahtar kare, zamansal imza ve öğrenilmiş gömme yöntemleri.
3. Ses parmak izi: Chromaprint ve benzerleri.
4. Metin: MinHash, SimHash, Broder shingling.
5. İçerik kaynak doğrulama standartları (ör. C2PA) ve zincir üstü kanıtlarla ilişkisi.
6. Üretken içerikte tohum ve parametre uzayı çakışması: farklı tarif, aynı çıktı.
7. NFT ekosistemlerindeki kopya, sahte koleksiyon, spam örnekleri ve savunmalar.
8. Sybil ve spam ekonomisi: ücret, stake, itibar tabanlı caydırıcılar.
9. Merkeziyetsiz sistemlerde tekilleştirme ve tekrar tespiti (IPFS, Arweave, Filecoin, Swarm).
10. Saldırı yüzeyi: algısal hash çarpıştırma, karşıt örnek (adversarial) manipülasyon, eşik sömürüsü.
11. QR video biçiminin tekrar tespitine etkisi: aynı içeriğin farklı kodlamalarının aynı parmak izine indirgenmesi.

**Kaynak kuralları:** Hakemli yayınlar, standartlar ve üretim sistemlerinin teknik yazıları tercih edilir. Her iddia kaynağıyla verilir. Doğrulanamayan iddia yazılmaz. Uzun alıntı yapılmaz, içerik kendi cümlelerle özetlenir.

**Araştırma çıktısı:** yöntem karşılaştırma tablosu (doğruluk, maliyet, saldırı direnci, konsensüs uyumluluğu); tarif ve çıktı uzayı tespitinin birleşik mimari önerisi; konsensüs içi ve dışı katman ayrımı; yanlış pozitif ve yanlış negatif bütçesi önerisi; açık sorunlar ve ölçülmesi gereken deneyler.

### 9.4 Mimari Çıktı

Araştırmadan bağımsız olarak ajan şu altyapıyı hazırlar:

1. **Parmak izi arayüzü.** Tarif hash'i, çıktı hash'i ve algısal parmak izi için ortak `Fingerprint` arayüzü ve sürümlü kayıt yapısı.
2. **Tekrar indeksi.** Yüklemede parmak izine göre yakın-tekrar sorgusu yapan, konsensüs dışı indeks.
3. **Politika katmanı.** Tespit sonucunda yapılandırılabilir eylemler: uyarı, ek ücret, etiketleme, sınırlama, moderasyon kuyruğu. Eylem seçimi konsensüs dışıdır.
4. **Ölçüm harnesi.** Yanlış pozitif ve yanlış negatif oranlarını gerçekçi veri kümeleriyle ölçen test düzeneği.
5. **Kayıt.** Tespit kararları denetlenebilir biçimde kaydedilir.

---

## 10. Güvenlik Gereksinimleri

- Anahtar materyali (cihaz anahtarı, cüzdan tohumu) günlüklere, hata mesajlarına ve telemetriye yazılmaz.
- Güvenilmeyen girdi işleyen tüm ayrıştırıcılar (QR video çözücü, tarif ayrıştırıcı, üreteç girdisi) sınır denetimi ve fuzz testi ile korunur.
- Üreteç kum havuzu: adım, bellek ve süre sınırı; dosya ve ağ erişimi yasağı.
- Yetkisiz burn ve yetkisiz fiziksel silme reddedilir ve test edilir.
- Sahte StorageClaim ve replay saldırılarına karşı nonce ve geçerlilik penceresi kullanılır.
- Bölüm 1.3'ün herhangi bir adımını atlatan yol güvenlik açığı sayılır.
- HSM/PKCS#11 güven sınırı, BudZero devre dışı opcode sınırları, BNS adlandırma kuralları, Recipe modunun ayrıntıları ve erişim sayacı gibi Tier 2/3 güvenlik kritik kararlar, kod yazılmadan önce netleşmesi gereken açık maddelerdir. Bunlara dokunan iş başlamadan soru raporlanır ve onay beklenir.

---

## 11. Kapılar (Gates) ve Tamamlanma Tanımı

Her kapı geçilmeden bir sonrakine geçilmez. Kapı geçişi kanıtla raporlanır.

| Kapı | Koşul |
|---|---|
| K0 | Keşif raporu teslim edilmiştir. QR video sistemi bulunmuş ve belgelenmiştir. |
| K1 | Öncelik Sıfır tamamdır: her içerik türü için dönüşüm ve doğrulama hattı çalışır, Bölüm 1.5 testleri geçer. |
| K2 | BUD 1.0 kabul kriterleri karşılanmıştır. |
| K3 | BUD 2.0 kabul kriterleri karşılanmıştır. Maliyet hedefi ölçümle gösterilmiştir. |
| K4 | BUD 3.0 kabul kriterleri karşılanmıştır. |
| K5 | Tekrar/spam araştırma dosyası ve mimari altyapı teslim edilmiştir. |
| K6 | AI denetim hattı tam sürüm adayında çalışmış, mainnet hazırlık raporu üretilmiştir. |

**Tamamlanma tanımı.** Bir iş ancak şunların hepsi doğruysa tamamlanmıştır:

1. Kod derlenir, tüm testler geçer, atlanmış veya zayıflatılmış test yoktur.
2. Yer tutucu, `unsafe` ve float (konsensüs yolunda) taramaları temizdir.
3. Bileşen WIRED'dır ve çağrı noktası kanıtlıdır.
4. İlgili tüm dönüşümler Bölüm 1.3'e göre doğrulanmaktadır.
5. Rapor kanıtlarla yazılmıştır.
6. Açık sorular ve sapmalar raporlanmıştır.

**Teslim öncesi öz denetim listesi (her PR için):**

1. Bu PR belgede olmayan bir şey ekliyor mu?
2. Mevcut mekanizma varken yenisini yazıyor mu?
3. Doğrulamayı atlatan bir yol açıyor mu?
4. Test zayıflatıyor veya atlıyor mu?
5. Konsensüs yüzeyine dokunuyor mu ve onayı var mı?
6. Her iddiam kanıtlı mı?

Herhangi biri "evet" (1, 2, 3, 4) veya "hayır" (5, 6) ise PR teslim edilmez.

---

## 12. Ortak Mühendislik Kuralları

1. **Dil ve üslup.** Yorumlar, dokümantasyon ve raporlar sade, profesyonel ve şimdiki zaman kipinde yazılır. Belgeler değişiklik günlüğü üslubuyla değil, sistemin bugünkü halini anlatarak yazılır.
2. **Küçük PR'lar.** Her PR tek sorumluluk taşır. Konsensüs yüzeyine dokunan PR'lar ayrı işaretlenir.
3. **Test önceliği.** Her yeni bileşen birim ve entegrasyon testiyle, gerektiğinde özellik tabanlı test ve fuzz testiyle gelir.
4. **Bağımlılık politikası.** Yeni bağımlılıktan önce bakım durumu, lisans ve `unsafe` kullanımı incelenir. Uygun bağımlılık yoksa yerinde uygulama tercih edilir.
5. **Güncel mekanizmalar.** Kademlia DHT ve proxy re-encryption gibi eski tasarım notları geçerli değildir. Güncel olanlar: stake ağırlıklı rendezvous hashing ve istemci tarafı AEAD.
6. **Workspace ve skill'ler.** Bölüm 0.2 geçerlidir.

---

## 13. Raporlama

Ajan her aşamanın sonunda şu dosyaları üretir:

| Dosya | İçerik |
|---|---|
| `BUD-KESIF-RAPORU.md` | QR video sistemi keşfi, Bölüm 5 çıktısı, WIRED/UNWIRED tablosu, açık sorular |
| `BUD-DONUSUM-DOGRULAMA-RAPORU.md` | Öncelik Sıfır kanıtları: test matrisi, fuzz, diferansiyel ve negatif test çıktıları |
| `BUD-1_0-TAMAMLANMA.md` | Kabul kriterleri kanıtları |
| `BUD-2_0-TAMAMLANMA.md` | Sıkıştırma ölçümleri, maliyet sonuçları (0.016 dolar/TB/ay), AI denetim raporu |
| `BUD-3_0-TAMAMLANMA.md` | Tarif standardı, uçtan uca dönüşümler, silme testleri, QR fuzz sonuçları |
| `BUD-TEKRAR-SPAM-ARASTIRMA.md` | Bölüm 9 araştırması (ayrı, keşifsel dosya) |

Rapor kuralları: her "tamamlandı" kanıt taşır (komut, çıktı, test adı, dosya ve satır). Yapılmayan veya kısmen yapılan iş açıkça yazılır. Ölçüm yapılmadan performans veya maliyet iddiası yazılmaz. Keşifsel araştırma uygulama belgelerinden ayrı dosyada tutulur.

---

## 14. Uygulama Sırası

Keşif (K0), Öncelik Sıfır (K1), BUD 1.0 (K2), BUD 2.0 (K3), BUD 3.0 (K4), tekrar/spam araştırması ve altyapısı (K5, BUD 1.0 ile paralel başlayabilir), AI denetim hattı ve mainnet hazırlık raporu (K6). Her kapının sonunda ilgili rapor teslim edilir ve sonraki kapı için onay beklenir.

---

## 15. Açık Sorular (AI Ajanının Kesinleştirmesi Gereken Noktalar)

1. 0.016 dolar/TB/ay hedefinin hangi içerik karışımı için geçerli olduğu ve maliyet kalemlerinin kesin listesi.
2. QR video sisteminin her içerik sınıfı için ürettiği tarif türü ve varsa kapsam boşlukları.
3. PSN'in çevrimdışı toleransı ve askı eşiklerinin varsayılan değerleri.
4. Yaşam süresi ücretinin fiyat eğrisi ve minimum/maksimum süre.
5. Erişim sayacının manifeste bağlanması için konsensüs tasarımı.
6. Tekrar tespitinde konsensüs içindeki tek kontrolün kapsamı (yalnızca tam `recipe_hash` eşleşmesi mi).

Bu noktalar çözülmeden ilgili kod sabitlenmez. Çözüm gelene kadar yapılandırılabilir parametre ve açık arayüz bırakılır.
