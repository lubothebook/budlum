# BUD (Broad Universal Database) AI Uygulama Direktifi

Sürüm kapsamı: BUD 1.0, BUD 2.0, BUD 3.0
Muhatap: BUD'yi kodlayan AI ajanı
Kapsam: Bu belge tek başına yeterlidir ve yalnızca BUD sistemi için geçerlidir. Budlum çekirdeğinin diğer bileşenlerine ait kararlar bu belgeyle değiştirilmez.

Bu belgede "zorunludur", "yasaktır" ve "durur" ifadeleri pazarlığa kapalıdır. "Yapılmalıdır" düzeyinde yumuşak kural yoktur. Bir kuralı uygulayamayan AI ajanı, kuralı sessizce esnetmez, çalışmayı durdurur ve nedenini raporlar.

---

## 0. Rol ve Çalışma Ortamı

### 0.1 Rol

AI ajanı, BUD sistemini baştan sona kodlayan, test eden, doğrulayan ve raporlayan mühendislik yürütücüsüdür. Görevi, aşağıda tanımlanan üç sürümü eksiksiz, çalışır ve kanıtlanabilir biçimde teslim etmektir. Tasarım kararlarını bu belge verir. AI ajanı bu kararları uygular, belirsiz noktaları raporlar ve sessizce yeni karar üretmez.

### 0.2 Workspace, Repo ve Skill Kullanımı (İlk Adım)

1. AI ajanı işe başlamadan önce Workspace'i ve repoyu baştan sona okur: dizin yapısı, mevcut kod, önceki BUD araştırma ve rapor dosyaları, direktifler ve kayıtlı skill'ler.
2. Workspace'te tanımlı skill'ler, ilgili oldukları her işte kullanılır. Bir skill'in kapsadığı işi skill'i okumadan yapmak yasaktır. Hangi skill'in hangi işe uyduğu belirsizse önce skill listesi çıkarılır, sonra seçim yapılır.
3. Workspace'teki süreç kuralları (dosya düzeni, PR düzeni, adlandırma, depo erişimi) bu belgedeki kurallarla birlikte geçerlidir. Çakışmada güvenlik ve konsensüs kuralları bu belgeden, süreç ve düzen kuralları Workspace'ten alınır. Çakışma raporlanır.
4. Yerel depo kullanılmaz. Çalışma Workspace üzerinden yürür.
5. Workspace'teki önceki BUD dokümanları (Tier 1 raporu, master doküman, Mnemonic, Yaşayan Eşik, v7/v8 çalışma dokümanları) bağlam kaynağıdır. Bu belgeyle çelişen nokta bulunursa bu belge esastır ve çelişki keşif raporunda listelenir.

### 0.3 Sistemin Bir Bakışta Özeti

BUD, Budlum L1 üzerinde içeriğin nerede ve hangi biçimde yaşayacağını belirleyen katmandır. Üç sürüm aynı içerik kimliği ve manifest modeli üzerine kurulur:

- **1.0:** Kullanıcının cihazı kendi verisinin sunucusudur. Veri zincire yüklenmez, cihazda kalır ve sosyal medyada görünür.
- **2.0:** Mevcut içerik sıkıştırılır ve depolama maliyeti 0.016 dolar/TB/ay hedefine çekilir. Bir AI denetim hattı hedefi doğrular ve sistemi mainnet'e hazırlar.
- **3.0:** İçerik zincirde bayt olarak durmaz. NFT'nin altında yalnızca tarif bulunur. Tarif, QR kod videosu ve içerik arasında iki yönlü dönüşüm vardır. Her içerik bir QR kod videosuna dönüşebilir. Yaşam süresini kullanıcı seçer ve ücret buna göre alınır.

Sistem ayrıca bir içeriğin başka bir içerikle aynı olup olmadığını ve spam amaçlı çoğaltılıp çoğaltılmadığını anlayabilmelidir (Bölüm 9).

### 0.4 Okuma Sırası

1. Bölüm 1 ve 2: Öncelik sıfır ve sert kurallar
2. Bölüm 3, 4, 5: İlkeler, sürüm haritası, keşif
3. Bölüm 6, 7, 8: Sürüm şartnameleri
4. Bölüm 9: Tekrar ve spam tespiti
5. Bölüm 10 ve sonrası: Güvenlik, kapılar, raporlama

---

## 1. Öncelik Sıfır: QR Video Sistemi ve Dönüşüm Doğrulaması

Bu bölüm belgenin en yüksek öncelikli bölümüdür. Diğer tüm işler bu bölümün kurallarına tabidir. İki zorunluluk vardır: repodaki mevcut QR video sistemini bulup kullanmak ve her dönüşümü doğrulamak.

### 1.1 Repodaki Sistem: Önce Bul, Sonra Kullan

1. Repoda içeriği QR kod videosuna dönüştüren ve geri çözen bir sistem mevcuttur. AI ajanı bu sistemi işe başlamadan önce bulur.
2. Arama zorunludur ve kanıtlıdır: arama komutları, bulunan modüller, dosya yolları, giriş noktaları, veri yapıları, testler ve çağrı noktaları `BUD-KESIF-RAPORU.md` dosyasına yazılır.
3. Mevcut sistem varken yeni bir QR kodlayıcı veya çözücü yazmak yasaktır. Mevcut sistem esas alınır, eksikleri genişletilir. Genişletme gerekçesi, hangi eksiğin neden kapatıldığı ve mevcut davranışın nasıl korunduğu raporlanır.
4. Sistem bulunamazsa AI ajanı çalışmaya devam etmez. "Sistem bulunamadı" sonucu, yapılan aramaların tam listesiyle birlikte raporlanır ve onay beklenir. Sistemi kendi başına yeniden icat etmek yasaktır.
5. Mevcut sistemin şu özellikleri raporlanır: desteklediği içerik boyutu ve türleri, kare biçimi, parça sayısı, hata düzeltme yöntemi, kayıp toleransı, çözücünün güvenilmeyen girdiyle davranışı, determinizmi, testleri, WIRED/UNWIRED durumu.

### 1.2 Evrensellik Kuralı

1. **Her içerik bir QR kod videosuna dönüşebilir.** Hiçbir içerik sınıfı, boyutu veya biçimi muaf değildir. Metin, görsel, ses, video, ikili dosya, sıkıştırılmış dosya, şifreli içerik ve sıfır bayt dahil.
2. "Bu içerik QR videosuna dönüşmüyor" sonucuna izin verilmez. Böyle bir durum bir kusur olarak ele alınır: kök neden bulunur, sistem düzeltilir ve düzeltme testle kanıtlanır. İçeriği başka bir yola sapıtarak sorunu gizlemek yasaktır.
3. Dönüşüm iki yönlüdür ve her iki yön de tam desteklenir:

```
İleri yön:  Tarif   >  QR kod videosu  >  İçerik
Ters yön:   İçerik  >  QR kod videosu  >  Tarif
```

4. İçerik sosyal medyaya yüklendiğinde ters yön çalışır: içerik QR kod videosuna ve tarife dönüşür, zincirde yalnızca tarif tutulur, içerik bu yolla yayımlanmış olur. Görüntüleme sırasında ileri yön çalışır.
5. Tarif türü, repodaki mevcut sistemin tanımına göre belirlenir. AI ajanı sistemin her içerik sınıfı için hangi tarif türünü ürettiğini keşif raporunda belgeler ve bu belgeyi kanıtlarla destekler.

### 1.3 Zorunlu Doğrulama: Her Dönüşüm, İstisnasız

Her dönüşüm doğrulanır. Örnekleme, "genelde çalışıyor" veya yalnızca test ortamında doğrulama kabul edilmez. Doğrulama üretim yolunun ayrılmaz parçasıdır.

**Doğrulama kuralları:**

1. **Gidiş dönüş (roundtrip) eşitliği.** İçerik > QR video > içerik ve tarif > QR video > tarif dönüşümlerinde çıktı, girdiyle bayt bayt aynıdır. Eşitlik `ContentId` (alan etiketli hash) ve `recipe_hash` karşılaştırmasıyla kanıtlanır.
2. **Taahhüt eşitliği.** Yeniden üretilen içeriğin hash'i, tarifin taşıdığı `output_commitment` ile eşleşir.
3. **Gerçek taşıma yolunun doğrulanması.** Doğrulama yalnızca bellekteki kare dizisiyle yapılmaz. Videonun gerçekten geçeceği yol (video konteynerine yazma, sosyal medyanın yeniden kodlaması, sıkıştırma, yeniden boyutlandırma, kaydetme ve geri okuma) simüle edilir veya gerçekten çalıştırılır ve doğrulama bu yolun çıktısı üzerinde yapılır.
4. **Bağımsız doğrulama.** Doğrulama, dönüşümü yapan kod yoluyla aynı kod yolunu kullanmaz. Farklı bir çözücü yolu (bağımsız uygulama veya ayrı bir kod yolu) ile ikinci bir doğrulama yapılır. İki yol aynı sonucu vermezse dönüşüm başarısızdır.
5. **Doğrulayan taraf.** Yükleme sırasında istemci doğrular. Zincire taahhüt edilmeden önce validatör tarafı yeniden doğrular. Okuma sırasında okuyan taraf içeriği taahhüde karşı doğrular. Üç noktanın hiçbiri atlanmaz.
6. **Determinizm.** Aynı içerik aynı tarifi ve aynı kare kümesini üretir. Belirsiz (non-deterministic) kodlama yasaktır. Platformlar arası (mobil, masaüstü, sunucu) çıktı aynıdır ve bu, testle kanıtlanır.
7. **Kanıt kaydı.** Her doğrulamanın sonucu (girdi hash'i, çıktı hash'i, kullanılan yol, süre, karar) denetlenebilir biçimde kaydedilir.

### 1.4 Başarısızlık Davranışı (Fail-Closed)

1. Doğrulama başarısız olursa dönüşüm başarısızdır. İçerik için taahhüt yazılmaz, NFT oluşturulmaz, sosyal medyada yayımlanmaz.
2. Başarısızlık sessiz geçilmez, kullanıcıya açık hata olarak gösterilir ve kayda geçer.
3. Doğrulamayı atlatan bir "hızlı yol", "geçici bypass", "debug bayrağı" veya "güvenilir içerik" istisnası eklemek yasaktır.
4. Doğrulama sonucu belirsizse (zaman aşımı, kaynak yetersizliği) sonuç "başarısız" kabul edilir.

### 1.5 Doğrulama Test Gereksinimleri

AI ajanı aşağıdaki testleri yazar ve hepsi geçmeden Öncelik Sıfır tamamlanmış sayılmaz:

1. **İçerik türü matrisi:** metin, görsel, ses, video, ikili, sıkıştırılmış, şifreli, rastgele bayt.
2. **Sınır değerleri:** sıfır bayt, bir bayt, parça sınırında bayt sayıları, çok büyük içerik, en büyük desteklenen boyut ve bir fazlası.
3. **Özellik tabanlı (property-based) test:** rastgele içerik için gidiş dönüş eşitliği, çok sayıda üretilmiş örnekle.
4. **Kayıplı taşıma testi:** video yeniden kodlaması, kare kaybı, gürültü, çözünürlük değişimi altında doğru çözme veya doğru ve açık başarısızlık. Yanlış içerik döndürmek asla kabul edilmez.
5. **Fuzz testi:** bozuk, kesik ve kötü niyetli videolarla çözücü çökmez, sonsuz döngüye girmez, bellek taşırmaz. Bozuk girdi yanlış içerik olarak kabul edilmez.
6. **Diferansiyel test:** iki bağımsız çözücü yolunun tüm test kümesinde aynı sonucu verdiğinin kanıtı.
7. **Determinizm testi:** aynı girdi, birden çok platform ve çalıştırmada aynı çıktı.
8. **Negatif testler:** bilerek bozulmuş bir dönüşümün doğrulama tarafından reddedildiği kanıtı. Doğrulamanın gerçekten çalıştığını gösteren bu testler zorunludur.

---

## 2. Sert Kurallar (Pazarlığa Kapalı)

### 2.1 Kanıt Kuralları

1. Her "tamamlandı", "çalışıyor" ve "geçti" ifadesi kanıtla desteklenir: çalıştırılan komut, çıktısı, test adı, dosya ve satır referansı. Kanıtsız ifade geçersizdir.
2. Çalıştırılmamış bir testi "geçti" olarak raporlamak yasaktır.
3. Ölçülmemiş bir performans veya maliyet değeri yazmak yasaktır.
4. Başarısız sonuç, kısmi sonuç ve yapılamayan iş açıkça raporlanır. Eksik işi tamamlanmış gibi sunmak, bu belgenin ihlal edilebilecek en ağır kuralıdır.

### 2.2 Kod Kalitesi Kuralları

1. Teslim edilen kodda `todo!()`, `unimplemented!()`, `panic!("not implemented")`, boş gövdeli işlev, sabit döndüren sahte gerçekleştirme, "şimdilik" yorumu ve yer tutucu (placeholder) bulunmaz. Teslimden önce bu kalıplar için tarama yapılır ve sonuç rapora eklenir.
2. Testleri zayıflatmak, atlamak (`#[ignore]`), yorum satırına almak, eşiklerini gevşetmek veya test geçsin diye gerçek davranışı sahteleyen mock kullanmak yasaktır. Test başarısızsa kod düzeltilir.
3. Bir bileşen ancak üretim çağrı noktası (gerçek çağıran işlev) kanıtlandığında WIRED sayılır. Çağrı noktası olmayan bileşen UNWIRED etiketiyle raporlanır ve "tamamlandı" sayılmaz.
4. `unsafe` yasaktır. Güvenlik açığı geçmişi olan, bakımı bırakılmış veya `unsafe` gerektiren bağımlılık eklenmez.
5. Konsensüs yolunda kayan nokta (float), sistem saati, rastgelelik ve platforma bağlı davranış yasaktır.
6. Hata yönetimi açıktır. `unwrap()` ve `expect()` üretim yolunda yalnızca gerekçesi yazılı, kanıtlanmış değişmezler için kullanılır.

### 2.3 Yeniden Kullanım ve Kapsam Kuralları

1. Yeni mekanizma yazmadan önce repoda ve Workspace'te aynı işi yapan mevcut mekanizma aranır. Aramanın kanıtı rapora eklenir. Mevcut mekanizma varken yenisini yazmak yasaktır.
2. Bu belgenin kapsamı dışındaki bileşenler (çekirdek konsensüs, cüzdan, BudZKVM çekirdeği) değiştirilmez. Değişiklik gerekiyorsa ayrı tasarım notu yazılır ve onay beklenir.
3. Konsensüs yüzeyine dokunan her değişiklik ayrı PR, ayrı tasarım notu ve açık onay gerektirir. Onaysız birleştirilmez.
4. Belgede olmayan özellik eklenmez. Belgede olan özellik atlanmaz.

### 2.4 Belirsizlik Kuralları

1. Belirsiz nokta bulunduğunda AI ajanı varsayımla ilerlemez. Sorusunu Açık Sorular listesine yazar, en güvenli (fail-closed) varsayılan davranışı uygular ve bunu raporda işaretler.
2. Güvenlik veya konsensüs açısından kritik bir belirsizlik varsa ilgili iş durur ve onay beklenir.
3. Bu belgeyle repo çelişirse bu belge esastır. Çelişki raporlanır.

### 2.5 Durma Koşulları

AI ajanı aşağıdaki durumlarda çalışmayı durdurur, durumu raporlar ve talimat bekler:

1. Repodaki QR video sistemi bulunamadığında (Bölüm 1.1).
2. Öncelik Sıfır doğrulaması geçmediğinde.
3. Bir kapının (Bölüm 11) kabul kriterleri karşılanamadığında.
4. Konsensüs yüzeyine dokunmak gerektiğinde ve onay yoksa.
5. Bu belgenin iki kuralı çeliştiğinde.
6. Güvenlik açısından kritik bir bulgu (anahtar sızıntısı, doğrulama atlatma, yetkisiz silme) tespit edildiğinde.

---

## 3. Değişmez İlkeler ve Mevcut Durum

### 3.1 Değişmez İlkeler

1. **İçerik adresleme.** ContentId, alan etiketli hash ile üretilir. Sahip ve boyut alanları kimliğe dahil edilmez, böylece aynı baytlar farklı yükleyiciler arasında tekilleşir.
2. **Konsensüs sınırı.** Konsensüse yalnızca taahhütler (manifest_id, tarif hash'i, durum geçişleri) girer. Depolama stratejisi geçişleri, yerel önbellek kararları ve sıkıştırma tercihleri konsensüs dışıdır.
3. **Determinizm.** Konsensüs yoluna giren her hesaplama sabit noktalı aritmetik kullanır.
4. **İki aktör modeli.** Sistemde yalnızca kullanıcı ve validatör vardır. Ayrı bir depolama, arşiv veya üçüncü taraf rolü tanımlanmaz. Bir kullanıcı istediği zaman validatör olabilir ve geri dönebilir.
5. **İstemci tarafı şifreleme.** Şifreleme istemci tarafında AEAD ile yapılır, anahtarlar cüzdan tohumundan türetilir. Zincir yalnızca Plaintext/ClientSide bayrağını manifeste bağlı olarak saklar.
6. **Dürüst durum etiketleme.** Kodlanmış ve test edilmiş ama üretim çağrı noktası olmayan bileşen `UNWIRED`, üretimde çalışan bileşen `WIRED` olarak etiketlenir. Etiket, kanıtla birlikte verilir.
7. **Fail-closed.** Doğrulanamayan içerik kabul edilmez.

### 3.2 Başlangıç Durumu (AI ajanının doğrulaması gereken varsayım)

Aşağıdaki durum son iç denetime dayanır. AI ajanı bunu Bölüm 5'te depo üzerinde yeniden doğrular ve farkları raporlar.

- Erasure coding (GF(2^8), Cauchy Reed-Solomon, şema (10,16)), shard yerleştirme (stake ağırlıklı rendezvous hashing), kodlama denetimi (coding audit) ve onarım tetikleyici kodlanmış ve testlidir, ancak üretim çağrı noktaları eksiktir (UNWIRED).
- Self-host politika kontrolü bağlanmış ilk parçadır.
- Gerçek kriptografik Proof-of-Storage, BudZKVM'deki VerifyMerkle işlevinin üretime çıkmasına bağlıdır.
- "Yaşayan Eşik" (living_threshold.rs) modülü kodlanmıştır. Erişim sayacının manifeste bağlanması yapılmamıştır ve konsensüs yüzeyine dokunur.
- QR kod videosu sistemi repoda mevcuttur (Bölüm 1.1). Durumu AI ajanı tarafından doğrulanır.

---

## 4. Sürüm Haritası

| Sürüm | Konu | Veri nerede durur | Ana çıktı |
|---|---|---|---|
| BUD 1.0 | Cihaz sunucu modeli | Kullanıcının kendi cihazında | Cihaz tabanlı depolama düğümü, sosyal medyada görünürlük |
| BUD 2.0 | Mevcut içeriğin sıkıştırılması | Cihaz ve ağ katmanı, sıkıştırılmış | Sıkıştırma hattı, 0.016 dolar/TB/ay maliyet hedefi, AI denetim ve mainnet hazırlık süreci |
| BUD 3.0 | Tarif tabanlı NFT içeriği | Zincirde, NFT altında, yalnızca tarif | Tarif standardı, QR video dönüşümü ve doğrulaması, yaşam süresi ücretlendirmesi, tekrar/spam tespiti |

Sürümler sıralı geliştirilir. Bir sürüm, kapısının (Bölüm 11) tüm kriterlerini geçmeden tamamlanmış sayılmaz. Ortak arayüzler (ContentId, manifest, erişim politikası) sürümler arasında geriye dönük uyumlu tasarlanır.

---

## 5. Hazırlık ve Depo Keşfi (Kodlamadan Önce Zorunlu)

AI ajanı aşağıdaki adımları sırasıyla tamamlar ve sonucu `BUD-KESIF-RAPORU.md` dosyasına yazar. Keşif raporu teslim edilmeden kodlamaya başlanmaz.

1. **QR video sistemi keşfi (en önce).** Bölüm 1.1'in tüm maddeleri.
2. Depo ağacı: `src/storage/` altındaki tüm modüller, çağrı grafı ve test kapsamı.
3. Her modül için WIRED/UNWIRED durumu, üretim çağrı noktası kanıtıyla (dosya, satır, çağıran işlev).
4. Mevcut manifest, ContentId, erasure coding, shard placement, audit, repair, living_threshold ve self-host politikası tiplerinin imzaları.
5. NFT modülünün durumu, NFT burn akışı ve DAO yetki noktaları.
6. Cüzdan ve uygulama katmanının (Budlum uygulaması, sosyal medya, galeri) depolama ile etkileşim noktaları.
7. Bu belgeyle çelişen veya belirsiz noktalar: "Açık Sorular" başlığı altında.
8. Önceki BUD dokümanlarıyla (Tier 1 raporu, master doküman, Mnemonic dokümanı) bu belge arasındaki farkların tablosu.

---

## 6. BUD 1.0: Cihaz Sunucu Modeli

### 6.1 Amaç

Kullanıcının cihazı, kendi verisi için bir sunucu (Personal Storage Node, PSN) olarak tanımlanır. Veri Budlum'a yüklenmez, her zaman kullanıcının cihazında kalır. Budlum ağı ve uygulaması yalnızca verinin varlığını, adresini ve erişim politikasını bilir. Kullanıcı içeriği sosyal medyada paylaşır ve içerik, cihaz depolamayı sahiplendiği sürece görünür kalır.

Benzetme: Bluesky'da herkesin kendi sunucusu vardır. BUD 1.0'da her cihaz bu rolü üstlenir. Cihaz validatör olmaz, yalnızca depolama görevini üstlenir.

### 6.2 Kapsam

**İçinde:** PSN çalışma zamanı (mobil, masaüstü, mini PC, düşük kaynak), cihaz sahiplik kaydı, yerel içerik deposu, doğrulanabilir okuma uç noktası, sosyal medya entegrasyonu, sahiplik yaşam döngüsü.

**Dışında:** Validatörlük görevleri (blok üretimi, konsensüs oylaması). Başkalarının içeriği için ücretli depolama pazarı.

### 6.3 Davranış Şartnamesi

**Sahiplenme.** Kullanıcı kurulumda depolamayı sahiplenir. Sahiplenme, cihaz anahtarının imzaladığı bir `StorageClaim` işlemiyle zincire taahhüt edilir. Claim şunları içerir: hesap kimliği, cihaz kimliği, kapasite beyanı, ağ erişim adresi, geçerlilik penceresi ve nonce.

**Görünürlük.** İçerik, claim aktif olduğu ve cihaz erişilebilir olduğu sürece sosyal medyada çözümlenir. Claim sonlandığında veya lease süresi dolduğunda içerik referansı "kullanılamıyor" durumuna geçer. Zincirde içerik baytı bulunmaz.

**Erişilebilirlik.**

1. Cihaz düzenli aralıklarla `Heartbeat` yayınlar. Sıklık ve tolerans yapılandırılabilir parametredir.
2. Uygulama, çevrimdışı cihaz içeriği için son bilinen önizlemeyi yerel önbellekten gösterebilir. Önizleme önbelleği konsensüs dışıdır ve otoriter değildir.
3. Çevrimdışı süre eşiği aştığında içerik "askıda" olur. Cihaz geri döndüğünde otomatik olarak "aktif" olur.

**Doğrulama.** Okuyan taraf aldığı baytları ContentId ile doğrular. Cihazın depolamayı gerçekten yaptığı, rastgele challenge'a verdiği yanıtla (retrieval challenge) kanıtlanır. Kriptografik Proof-of-Storage hazır olana kadar challenge örnekleme tabanlı ve olasılıksaldır. Bu sınırlama kodda ve raporda açıkça belirtilir.

**QR video entegrasyonu.** Sosyal medyaya paylaşılan her içerik Bölüm 1'deki dönüşüm ve doğrulama hattından geçer. Doğrulanmamış içerik paylaşılmaz.

**Gizlilik.** Özel içerik istemci tarafında şifrelenir. Cihaz şifreli baytları taşır, anahtarı bilmez. Açık içerik Plaintext bayrağıyla taşınır.

**Kendi içeriğini okuma.** Sahip olunan ve ağa taahhüt edilmiş içeriği kendi cihazından okumak ücretsizdir. Ağa hiç taahhüt edilmemiş içerik için kalıcılık garantisi verilmez. Bu ayrım arayüzde ve dokümantasyonda net tutulur.

**Kendi içeriğini sunma.** Kullanıcı kendi içeriğini sunarken ücret kazanmaz. Yalnızca başkalarının içeriğini sunarken ücret kazanır.

### 6.4 Kabul Kriterleri (BUD 1.0)

- PSN, referans mobil ve masaüstü hedeflerinde derlenir ve çalışır.
- Bir hesap cihazını sahiplenir, içerik yükler, içerik sosyal medyada görünür.
- Cihaz kapatıldığında içerik durumu tanımlanan sürede "askıda" olur, geri açıldığında "aktif" olur.
- Bozulmuş bayt ContentId doğrulamasında reddedilir.
- Retrieval challenge hem başarılı hem başarısız yanıt senaryolarında testlidir.
- Sosyal medyaya giden her içerik Bölüm 1'in doğrulamasından geçmiştir ve bunun kaydı vardır.
- Tüm yeni bileşenlerin üretim çağrı noktaları vardır (WIRED).

---

## 7. BUD 2.0: Sıkıştırma ve Maliyet Hedefi

### 7.1 Amaç

Mevcut içeriği sıkıştırmak ve depolama maliyetini **0.016 dolar/TB/ay** hedefine indirmek. Hedefin sağlandığını denetleyecek ve sistemi mainnet'e hazır hale getirecek bir yapay zeka denetim hattı bu sürümün parçasıdır.

> Not: 0.016 dolar/TB/ay hedefi, tek başına disk, erasure ve sıkıştırma ile ulaşılabilecek düzeyin altındadır (referans bileşik maliyet yaklaşık 0.18 dolar/TB/ay, ham disk maliyeti yaklaşık 0.29 dolar/TB/ay). Hedef bu nedenle içerik sınıfı stratejisiyle sağlanır: tariften yeniden üretilen içerik bayt olarak depolanmaz, bayt destekli içerik sıkıştırılır ve tekilleştirilir, erişim sıklığına göre strateji değişir (Yaşayan Eşik). AI ajanı hedefe hangi içerik karışımında ulaşıldığını ölçümle gösterir. Ulaşılamayan karışımları gizlemez, raporlar.

### 7.2 Sıkıştırma Hattı

1. **Sınıflandırma.** İçerik, deterministik yeniden üretilebilirliğe göre sınıflanır: Generated (tariften yeniden üretilebilir) ve Stored (bayt destekli). Sistem sınıf önerir, kullanıcı onaylar.
2. **Kayıpsız katman.** Organik içerik (fotoğraf, ses, video) için format bilinçli kayıpsız yeniden kodlama. Dropbox Lepton'un ölçülmüş oranı (yaklaşık yüzde 23) üretim tabanı referansıdır. Lepton kodu güvenlik açığı nedeniyle doğrudan kullanılmaz, bağımsız ve güvenli bir uygulama yazılır.
3. **Yaşayan Eşik.** `living_threshold.rs` hattın strateji seçici bileşeni olarak bağlanır. Strateji, azalan bir erişim sıklığı tahmini (720 epoch yarı ömür) ile yerel hesaplanan başa baş eşiği r* arasındaki ilişkiye göre seçilir. Geçişler çift yönlüdür. Geçiş kararı konsensüs dışıdır, yalnızca manifest_id konsensüs açısından bağlayıcıdır.
4. **Tekilleştirme.** Aynı baytlar aynı ContentId ile tek kopya olarak tutulur.
5. **Erasure coding.** Şema (10,16) ile başlar. Bir hafta sağlıklı çalışma süresinin ardından (20,26) şemasına iki fazlı geçiş yapılır. Eski shard'lar yalnızca yenileri doğrulandıktan sonra silinir.
6. **Format esnekliği.** İçeriğin çözünürlüğü korunur, format serbestçe değişebilir.
7. **Sıkıştırma doğrulaması.** Her sıkıştırma dönüşümü Bölüm 1.3'teki gidiş dönüş eşitliği kuralına tabidir. Açılan içerik özgün içerikle bayt bayt aynı olmadıkça sıkıştırma kabul edilmez.

### 7.3 Bağlama İşleri (UNWIRED bileşenlerin üretime alınması)

Aşağıdaki bileşenler gerçek çağrı noktalarıyla bağlanır ve her biri için entegrasyon testi yazılır:

- Erasure coding ve shard yerleştirme
- Coding audit (parite ilişkisi üzerinde örnekleme)
- Onarım tetikleyici
- Erişim sayacı: manifeste bağlama konsensüs yüzeyine dokunur. Ayrı tasarım notu ve ayrı PR olarak sunulur, onaysız birleştirilmez.

### 7.4 Maliyet Hedefinin Ölçümü

- Ölçüm birimi: orijinal (sıkıştırma öncesi) TB başına aylık toplam maliyet, dolar cinsinden. Maliyet; disk, CPU, erasure yükü, onarım trafiği ve doğrulama giderlerini içerir.
- Hedef: içerik karışımı için raporlanan bileşik değer 0.016 dolar/TB/ay veya altıdır.
- Yük testi: sentetik ve gerçekçi içerik karışımıyla, hedef hacme ulaşan tekrarlanabilir senaryo.
- Metrikler: sıkıştırma oranı, tekilleştirme oranı, erasure yükü, onarım trafiği, okuma gecikmesi, TB başına aylık maliyet.
- Referans maliyet modeli: orijinal bayt, yaklaşık 0.6x sıkıştırma ve tekilleştirme, 1.04 ila 1.17x erasure yükü, yaklaşık 0.29 dolar/TB/ay sahip olunan disk maliyeti. AI ajanı bunları kendi ölçümleriyle karşılaştırır ve sapmaları raporlar.
- Ölçüm yöntemi, girdi veri kümesi ve tekrar komutu rapora eklenir. Tekrarlanamayan ölçüm geçersizdir.

### 7.5 Ücret Modeli

Kullanıcı erişim başına öder (CPU, okuma, erasure onarımı, doğrulama). Ücret cüzdandan doğrudan hizmeti veren validatöre kesilir. İsteğe bağlı günlük harcama üst sınırı sunulur. Depolama fonu veya sübvansiyon yoktur. Okunmayan Stored içerik sürdürülmez (state-expiry). Bu kural, ödeme yapan okuyucusu olmayan içeriğin doğal olarak terk edilmesini sağlayacak biçimde uygulanır ve kullanıcıya önceden görünür kılınır.

### 7.6 AI Denetim ve Mainnet Hazırlık Hattı

Bu hat, BUD 2.0'ın hedeflerine ulaştığını denetleyen ve mainnet hazırlığını değerlendiren otomatik süreçtir.

**Sorumluluklar:**

1. Her PR ve sürüm adayı için otomatik denetim listesi çalıştırır: WIRED/UNWIRED doğrulaması, determinizm kontrolü (konsensüs yolunda float taraması), `unsafe` taraması, yer tutucu taraması (Bölüm 2.2), bağımlılık lisans ve güvenlik taraması.
2. Ölçülen maliyeti 0.016 dolar/TB/ay hedefiyle karşılaştırır ve geçti/kaldı kararını kanıtla verir.
3. Bölüm 1.5'teki dönüşüm doğrulama test kümesini her sürüm adayında yeniden çalıştırır ve sonucu rapora ekler.
4. Kaos senaryolarını çalıştırır: ani validatör kaybı, ağ bölünmesi, bozuk shard, gecikmeli onarım.
5. Mainnet hazırlık raporu üretir: açık riskler, çözülmemiş Tier 2/3 kararlar, güvenlik açısından kritik bağımlılıklar.
6. Bulgularını doğrulanabilir kanıtla sunar. Kanıtsız "geçti" kabul edilmez.

**Sınırlar:** AI denetçi karar verici değil, kanıt üreticidir. Konsensüs veya güvenlik açısından kritik değişikliklerin onayı insan sahibindedir.

### 7.7 Kabul Kriterleri (BUD 2.0)

- Sıkıştırma hattı, tanımlı içerik sınıflarında ölçülmüş oranlarla çalışır ve oranlar raporlanır.
- Her sıkıştırma dönüşümü gidiş dönüş eşitliğiyle doğrulanır.
- Bölüm 7.3'teki bileşenlerin tümü WIRED durumundadır.
- Maliyet ölçümü tekrarlanabilir biçimde 0.016 dolar/TB/ay hedefini karşılar (hangi içerik karışımında karşıladığı açıkça belirtilir).
- AI denetim hattı en az bir tam sürüm adayı üzerinde uçtan uca çalışmış ve rapor üretmiştir.
- Konsensüs yüzeyine dokunan tüm değişiklikler ayrı tasarım notu ve ayrı incelemeyle ilerlemiştir.

---

## 8. BUD 3.0: Tarif Tabanlı NFT İçeriği

### 8.1 Amaç

İçerik zincirde bayt olarak tutulmaz. Zincirde, bir NFT'nin altında yalnızca **tarif** bulunur. Tarif, içeriğin yeniden üretilmesi için gereken en küçük tanımdır. NFT silindiğinde içerik de silinmiş olur. İçeriğin yaşam süresi kullanıcı tercihidir ve ücretlendirme bu süreye göre yapılır. Bu sayede içerik Budlum'a çok düşük maliyetle yüklenmiş olur.

NFT'ler Budlum üzerindeki uygulamalarda görünür: açık (public) ise sosyal medyada, kapalı (private) ise galeride.

### 8.2 Akış

Dönüşüm iki yönlüdür ve her içerik için geçerlidir (Bölüm 1.2):

```
İleri yön:  Tarif   >  QR kod videosu  >  İçerik
Ters yön:   İçerik  >  QR kod videosu  >  Tarif
```

1. **İçerik > QR video > Tarif.** İçerik, repodaki mevcut sistemle QR kod videosuna ve tarife dönüştürülür. Bu yön, içerik sosyal medyaya yüklendiğinde çalışır.
2. **Tarif > NFT.** Tarif, NFT'nin altında zincire yazılır. Tarif ağda tek bir yerde tutulur.
3. **Tarif > QR video > İçerik.** Görüntüleme sırasında tarif QR kod videosuna ve oradan içeriğe dönüştürülür.
4. **Doğrulama.** Her adımda Bölüm 1.3 uygulanır. Doğrulanmamış dönüşüm zincire yazılmaz.

### 8.3 Tarif Standardı

Sürümlü ve kanonik bir tarif biçimi tanımlanır. Tarif türü ve içerik biçimi, repodaki mevcut QR video sistemiyle uyumlu olur (Bölüm 1.1).

**Alanlar (asgari):**

- `version`: tarif biçimi sürümü
- `kind`: tarif türü (mevcut sistemin tanımladığı türler)
- `generator_id`: kayıtlı üreteç kimliği ve sürümü (üreteçli tarifler için)
- `seed`, `params`: tohum ve parametre kümesi (kanonik sıralı)
- `output_spec`: çıktı türü, çözünürlük, süre, format
- `output_commitment`: yeniden üretilen çıktının içerik hash'i (ContentId ile uyumlu, alan etiketli)
- `visibility`: `public` veya `private`
- `lifetime`: talep edilen yaşam süresi (epoch cinsinden)

**Kanonikleştirme.** Tarif kanonik bir serileştirmeyle işlenir. Aynı mantıksal tarif her zaman aynı bayt dizisini ve aynı `recipe_hash` değerini üretir.

**Üreteç kaydı.** Üreteçler sürümlü ve değişmezdir. Yayınlanmış bir üreteç sürümü değiştirilemez, yeni davranış yeni sürüm olur. Üreteçler deterministiktir: kayan nokta farklılıkları, zaman, rastgelelik kaynağı ve platform bağımlılığı içeremez. Üreteç yürütmesi sınırlandırılmış kaynakla (adım, bellek, süre) kum havuzunda çalışır.

**Doğrulama modeli.** Tam ZK üretim kanıtı ekonomik olarak uygulanabilir değildir (ölçülmüş: örnek bir avatar için yaklaşık 3.4 sn kanıt süresi, çok yüksek eşdeğer depolama maliyeti). Doğrulama, çıktının yerel olarak yeniden üretilmesi ve `output_commitment` ile karşılaştırılması yoluyla yapılır. Bu karar kodda ve dokümantasyonda korunur.

### 8.4 NFT Bağlama, Yaşam Süresi ve Ücretlendirme

- **Yaşam süresi kullanıcı tercihidir.** Ücret süre ve tarif boyutuyla orantılıdır, ön ödemeli kira olarak tahsil edilir. Süre uzatma desteklenir.
- **Süre dolumu.** Süre dolduğunda tarif silme akışına girer (Bölüm 8.5). Kullanıcıya süre dolmadan önce uygulama içinde bildirim gösterilir.
- **Erişim.** NFT erişimi, NFT sahipliğinden ayrıdır. Erişim modları: private, cüzdan izni (wallet-grant), public. NFT devri yeni ödeyen tarafı oluşturur.
- **Görünürlük.** `public` NFT sosyal medyada listelenir. `private` NFT yalnızca sahibin galerisinde görünür ve sahibin verdiği izinlerle paylaşılır. Görünürlük sonradan değiştirilebilir.

### 8.5 Silme Yaşam Döngüsü

Bu bölüm onaylanmış silme direktifidir ve olduğu gibi uygulanır.

1. Tarif ağda **tek bir yerde** tutulur. Referans sayacı yoktur. Ağda görünmesi, orada kopya olarak durduğu anlamına gelir.
2. Silme yetkisi hem **NFT sahibinde** hem **DAO'da** vardır ve bu iki yetki birbirinden bağımsız çalışır.
3. Silme talebi tarifi değil **NFT'yi** hedefler. NFT sahibi veya DAO NFT'yi burn eder.
4. Tarif verisi, ayrı bir işlem gerekmeden bu burn ile birlikte silinir.
5. Validatör tarifi diskten **fiziksel olarak siler**.
6. Sonrasında bir **tombstone** kaydı tutulur.
7. Bu içerik sınıfında erasure coding yedekliliği yoktur. Silme **geri döndürülemez**.

Zorunlu testler: sahip burn'ü, DAO burn'ü, iki yetkinin birbirinden bağımsızlığı, fiziksel silme doğrulaması, tombstone varlığı, silme sonrası yeniden üretim denemesinin başarısız olması, geri alma denemesinin reddedilmesi, yetkisiz silme denemesinin reddedilmesi.

### 8.6 QR Kod Videosu Asgari Şartları

Repodaki mevcut sistem esastır (Bölüm 1.1). Aşağıdaki şartlar, mevcut sistemin karşılaması gereken asgari şartlardır. Karşılamayan noktalar raporlanır ve sistem genişletilerek kapatılır.

1. **Çerçeve başlığı.** Her çerçeve: biçim sürümü, tarif kimliği (recipe_hash kısa özeti), toplam parça sayısı, parça sırası ve çerçeve bütünlük özeti.
2. **Kayıp toleransı.** Çerçeve kaybına dayanıklılık için fountain kodu veya eşdeğer yapı kullanılır. Bağımlılık `unsafe` içeriyorsa kullanılmaz.
3. **Hata düzeltme.** QR hata düzeltme seviyesi, parça boyutu ve kare hızı yapılandırılabilir parametredir. Varsayılanlar ölçümle belirlenir (kamera ile okuma başarı oranı, kayıt sıkıştırması sonrası okunabilirlik).
4. **Çözücü güvenliği.** Çözücü güvenilmeyen girdi işler. Boyut, çerçeve sayısı ve kaynak sınırı uygular. Bozuk veya kötü niyetli videoda çökme, sonsuz döngü ve bellek taşması olmaz. Fuzz testiyle kanıtlanır.
5. **Yanlış içerik yasağı.** Çözücü ya doğru içeriği döndürür ya da açık bir hata verir. Sessizce yanlış veya eksik içerik döndürmez.

### 8.7 Kabul Kriterleri (BUD 3.0)

- Tarif standardı sürümlü, kanonik ve testlidir. Aynı tarif her zaman aynı `recipe_hash` üretir.
- İleri ve ters yön, tüm içerik türleri için uçtan uca çalışır. İstisna içerik sınıfı yoktur.
- Her dönüşüm Bölüm 1.3'e göre doğrulanmıştır ve doğrulama kayıtları vardır.
- Bölüm 1.5'teki tüm doğrulama testleri geçer.
- Yaşam süresi seçimi, ücretlendirme, uzatma ve süre dolumu uçtan uca çalışır.
- Silme yaşam döngüsü (Bölüm 8.5) eksiksiz uygulanmış ve testlidir.
- `public` NFT sosyal medyada, `private` NFT galeride görünür.
- Tüm bileşenler WIRED durumundadır.

---

## 9. Tekrar ve Spam Tespiti: Araştırma ve Mimari Görevi

### 9.1 Soru

Bir içeriğin başka bir içerikle aynı olduğunu, kopyalandığını veya spam amaçlı çoğaltıldığını nasıl anlarız? Bu, tarif üzerinden yapılabilir mi?

### 9.2 Başlangıç Çerçevesi (araştırmayla doğrulanacak hipotezler)

1. **Tam eşleşme (tarif uzayı).** Kanonik tarif, tam eşleşme tespiti için doğal bir parmak izidir: aynı `recipe_hash` aynı tarif demektir. Tarifin deterministik olması, aynı tarifin aynı çıktıyı üretmesini sağlar.
2. **Tarif uzayının sınırı.** Farklı tarifler aynı veya çok benzer çıktı üretebilir (farklı seed, eşdeğer parametreler, farklı üreteç sürümü). Tarif hash'i tek başına yeterli değildir.
3. **Çıktı uzayı parmak izi.** Çıktı yeniden üretildiğinde, çıktıdan algısal bir parmak izi hesaplanır. Görüntü, video ve ses için algısal hash ve gömme (embedding) tabanlı benzerlik yöntemleri incelenir.
4. **Yakın tekrar.** Benzerlik eşiği ve hızlı arama için yerelliğe duyarlı hash (LSH), MinHash/SimHash türevleri ve vektör indeksleri değerlendirilir.
5. **Ekonomik caydırma.** Tarif ücretli olduğundan spamın maliyeti vardır. Yaşam süresi ücreti, yükleme ücreti, stake şartı ve hız sınırları modellenir.
6. **Kriptografik katman.** Konsensüs yüzeyine yalnızca deterministik ve tekrarlanabilir kontroller girer. Yumuşak benzerlik kararları (eşik, model çıktısı) konsensüs dışı, uygulama ve moderasyon katmanında kalır.

### 9.3 Araştırma Görevleri

AI ajanı bu bölüm için yayın araştırması yapar ve sonucu **ayrı bir dosyaya** yazar: `BUD-TEKRAR-SPAM-ARASTIRMA.md`. Bu dosya keşifsel araştırma içeriğidir ve uygulama belgelerine karıştırılmaz.

**Konular:**

1. Algısal hashleme: pHash, dHash, PDQ, NeuralHash türevleri; saldırı direnci ve yanlış pozitif oranları.
2. Video yakın-tekrar tespiti: anahtar kare tabanlı, zamansal imza tabanlı ve öğrenilmiş gömme yöntemleri.
3. Ses parmak izi: Chromaprint ve benzeri yaklaşımlar.
4. Metin için MinHash, SimHash ve Broder shingling.
5. İçerik kaynak doğrulama standartları (ör. C2PA) ve zincir üstü kanıtlarla ilişkisi.
6. Üretken içerikte tohum ve parametre uzayı çakışması: farklı tarif, aynı çıktı sorunu.
7. NFT ekosistemlerindeki kopya, sahte koleksiyon ve spam örnekleri ve savunmalar.
8. Sybil ve spam ekonomisi: ücret, stake ve itibar tabanlı caydırıcılar.
9. Merkeziyetsiz sistemlerde tekilleştirme ve tekrar tespiti (IPFS, Arweave, Filecoin, Swarm).
10. Saldırı yüzeyi: algısal hash çarpıştırma, karşıt örnek (adversarial) manipülasyon, eşik sömürüsü.
11. QR video biçiminin tekrar tespitine etkisi: aynı içeriğin farklı kodlamalarının aynı parmak izine indirgenmesi.

**Kaynak kuralları:** Hakemli yayınlar, standart belgeleri ve üretim sistemlerinin teknik yazıları tercih edilir. Her iddia kaynağıyla verilir. Doğrulanamayan iddia yazılmaz. Kaynaklardan uzun alıntı yapılmaz, içerik kendi cümlelerle özetlenir.

**Araştırma çıktısı:**

- Yöntem karşılaştırma tablosu (doğruluk, maliyet, saldırı direnci, konsensüs uyumluluğu)
- Tarif uzayı ile çıktı uzayı tespitinin birleşik mimari önerisi
- Konsensüs içi ve dışı katmanların ayrımı
- Yanlış pozitif ve yanlış negatif bütçesi önerisi
- Açık sorunlar ve ölçülmesi gereken deneyler

### 9.4 Mimari Çıktı

Araştırmadan bağımsız olarak AI ajanı şu altyapıyı hazırlar:

1. **Parmak izi arayüzü.** Tarif hash'i, çıktı hash'i ve algısal parmak izi için ortak `Fingerprint` arayüzü ve sürümlü kayıt yapısı.
2. **Tekrar indeksi.** Yükleme sırasında parmak izine göre yakın-tekrar sorgusu yapan, konsensüs dışı indeks bileşeni.
3. **Politika katmanı.** Tespit sonucunda uygulanacak eylemler yapılandırılabilir: uyarı, ek ücret, etiketleme, sınırlama, moderasyon kuyruğu. Eylem seçimi konsensüs dışıdır.
4. **Ölçüm harnesi.** Yanlış pozitif ve yanlış negatif oranlarını gerçekçi veri kümeleriyle ölçen test düzeneği.
5. **Kayıt.** Tespit kararları denetlenebilir biçimde kaydedilir.

---

## 10. Güvenlik Gereksinimleri

- Cihaz anahtarı ve cüzdan tohumu: anahtar materyali günlüklere, hata mesajlarına ve telemetriye yazılmaz.
- Güvenilmeyen girdi işleyen tüm ayrıştırıcılar (QR video çözücü, tarif ayrıştırıcı, üreteç girdisi) sınır denetimi ve fuzz testi ile korunur.
- Üreteç kum havuzu: adım, bellek ve süre sınırı, dosya ve ağ erişimi yasağı.
- Silme akışı: yetkisiz burn ve yetkisiz fiziksel silme denemeleri reddedilir ve test edilir.
- Sahiplik iddiası: sahte StorageClaim ve replay saldırıları için nonce ve geçerlilik penceresi.
- Doğrulama atlatma: Bölüm 1.3'ün herhangi bir adımını atlatan yol güvenlik açığı sayılır.
- HSM/PKCS#11 güven sınırı, BudZero devre dışı opcode sınırları, BNS adlandırma kuralları, Recipe modunun ayrıntıları ve erişim sayacı gibi Tier 2/3 güvenlik kritik kararlar, kod yazılmadan önce netleştirilmesi gereken açık maddelerdir. Bunlara dokunan iş başlamadan ilgili soru raporlanır ve onay beklenir.

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
5. Rapor kanıtlarla birlikte yazılmıştır.
6. Açık sorular ve sapmalar raporlanmıştır.

**Teslim öncesi öz denetim listesi (her PR için):**

1. Bu PR belgede olmayan bir şey ekliyor mu?
2. Mevcut bir mekanizma varken yenisini yazıyor mu?
3. Doğrulamayı atlatan bir yol açıyor mu?
4. Test zayıflatıyor veya atlıyor mu?
5. Konsensüs yüzeyine dokunuyor mu ve onayı var mı?
6. Her iddiam kanıtlı mı?

Herhangi biri "evet" (1, 2, 3, 4) veya "hayır" (5, 6) ise PR teslim edilmez.

---

## 12. Ortak Mühendislik Kuralları

1. **Dil ve üslup.** Kod yorumları, dokümantasyon ve raporlar sade, profesyonel ve şimdiki zaman kipinde yazılır. Belgeler geçmişe atıf yapan değişiklik günlüğü üslubuyla değil, sistemin bugünkü halini anlatarak yazılır.
2. **Küçük ve incelenebilir PR'lar.** Her PR tek bir sorumluluğu taşır. Konsensüs yüzeyine dokunan PR'lar ayrı işaretlenir.
3. **Test önceliği.** Her yeni bileşen birim testi, entegrasyon testi ve gerektiğinde özellik tabanlı test ve fuzz testiyle gelir.
4. **Bağımlılık politikası.** Yeni bağımlılık eklenmeden önce bakım durumu, lisans ve `unsafe` kullanımı incelenir. Uygun bağımlılık yoksa yerinde uygulama tercih edilir.
5. **Güncel mekanizmalar.** Kademlia DHT ve proxy re-encryption gibi eski tasarım notları geçerli değildir. Güncel mekanizmalar: stake ağırlıklı rendezvous hashing ve istemci tarafı AEAD.
6. **Workspace ve skill'ler.** Bölüm 0.2 geçerlidir.

---

## 13. Raporlama

AI ajanı her aşamanın sonunda şu dosyaları üretir:

| Dosya | İçerik |
|---|---|
| `BUD-KESIF-RAPORU.md` | QR video sistemi keşfi, Bölüm 5 çıktısı, WIRED/UNWIRED tablosu, açık sorular |
| `BUD-DONUSUM-DOGRULAMA-RAPORU.md` | Öncelik Sıfır kanıtları: test matrisi sonuçları, fuzz, diferansiyel ve negatif test çıktıları |
| `BUD-1_0-TAMAMLANMA.md` | Kabul kriterleri kanıtları |
| `BUD-2_0-TAMAMLANMA.md` | Sıkıştırma ölçümleri, maliyet ölçüm sonuçları (0.016 dolar/TB/ay), AI denetim raporu |
| `BUD-3_0-TAMAMLANMA.md` | Tarif standardı, uçtan uca dönüşümler, silme testleri, QR fuzz sonuçları |
| `BUD-TEKRAR-SPAM-ARASTIRMA.md` | Bölüm 9 araştırması (ayrı, keşifsel dosya) |

**Rapor kuralları:**

- Her "tamamlandı" ifadesi kanıtla desteklenir (komut, çıktı, test adı, dosya ve satır).
- Yapılmayan veya kısmen yapılan iş açıkça yazılır.
- Ölçüm yapılmadan performans veya maliyet iddiası yazılmaz.
- Keşifsel araştırma içeriği uygulama belgelerinden ayrı dosyada tutulur.

---

## 14. Uygulama Sırası

1. Keşif (K0): QR video sistemi dahil
2. Öncelik Sıfır (K1): dönüşüm ve doğrulama hattı
3. BUD 1.0 (K2)
4. BUD 2.0 (K3)
5. BUD 3.0 (K4)
6. Tekrar/spam araştırması ve altyapısı (K5): BUD 1.0 ile paralel başlayabilir
7. AI denetim hattı ve mainnet hazırlık raporu (K6)

Her kapının sonunda ilgili rapor teslim edilir ve bir sonraki kapı için onay beklenir.

---

## 15. Açık Sorular (AI Ajanının Kesinleştirmesi Gereken Noktalar)

1. 0.016 dolar/TB/ay hedefinin hangi içerik karışımı için geçerli olduğu ve maliyet kalemlerinin kesin listesi.
2. Repodaki QR video sisteminin her içerik sınıfı için ürettiği tarif türü ve varsa kapsam boşlukları.
3. PSN'in çevrimdışı toleransı ve askı eşiklerinin varsayılan değerleri.
4. Yaşam süresi ücretinin fiyat eğrisi ve minimum/maksimum süre.
5. Erişim sayacının manifeste bağlanması için konsensüs tasarımı.
6. Tekrar tespitinde konsensüs içindeki tek kontrolün kapsamı (yalnızca tam `recipe_hash` eşleşmesi mi).

Bu noktalar çözülmeden ilgili kod sabitlenmez. Çözüm gelene kadar yapılandırılabilir parametre ve açık arayüz bırakılır.
