# THE BUDLUM AGREGASYON DİREKTİF

Kuantum güvenli imzalar ve kanıtlar zincire girmez. Zincir yalnız iddiaları ve tek bir Ana Kanıtı saklar.

## 1. Sistem

Budlum ağır nesneleri zincire koymaz. Her nesne bir kez doğrulanır. Zincir sonucu öğrenir. Nesne bir daha görülmez.

Ağır nesneler üç türdür:

- Kuantum güvenli imza.
- STARK kanıtı.
- Polinom açılışı (PCS açılışı).

Kuantum güvenli olmayan imzalar bu sistemin dışında kalır.

Akış:

```text
Kullanıcı
   |  Zarf (nesne + işlem özeti)
   v
Nesne Havuzu (her düğüm)
   |  her tikte yeni nesneler
   v
Agregatör: doğrula -> iddia üret -> Tik Kanıtı üret
   |  yayın: iddialar + tek STARK kanıtı
   v
Agregatör: iki kanıtı birleştir (ikili ağaç)
   |
   v
Blok üreticisi: Ana Kanıtı seç, İddia Çerçevesini yaz
   |
   v
Blok: imzasız işlemler + İddia Çerçevesi
```

Bir iddia yaklaşık 64 ile 96 bayt tutar. Yazıdaki değerler: bir imza 1 ile 6 kB, bir STARK kanıtı yaklaşık 128 kB tutar. Blok imza veya kanıt gövdesi taşımaz. Blok iddiaları ve bir Ana Kanıt taşır.

## 2. Budlum ne yapabilir

Her yetenek bir iddia türüne bağlıdır.

| Yetenek | İddia | Açıklama |
|---|---|---|
| Kuantum güvenli hesaplar | `Sig` | Blok, imza yerine 64 baytlık bir iddia taşır. |
| Mantığı gizli çoklu imza | `Prog` | Yetki mantığı bir BudZKVM programıdır. Mantık gizli kalır. |
| Özel hesap soyutlaması | `Prog` | Yetki, durumdaki gizli bir değere bağlıdır. Tek anahtar birçok hesabı yönetir. Hesaplar arasında açık bağ oluşmaz. |
| Domain geçiş kanıtlarının toplanması | `Prog` | Domain durum geçiş kanıtları tek Ana Kanıta girer. Yerleşim katmanı tek kanıt doğrular. |
| Başka STARK sistemleri | `Poly` | Aynı PCS yapılandırmasını kullanan her STARK kanıtı doğrulanır. |
| Hafif istemci | hepsi | İstemci blok başına bir STARK doğrular. İmza indirmez. |

## 3. Terimler

| Terim | Kod | Anlam |
|---|---|---|
| Nesne | `Object` | Kullanıcının ürettiği ağır veri. |
| Zarf | `Envelope` | Nesne, işlem özeti ve domain. |
| İddia | `Claim` | Nesnenin doğru olduğunu söyleyen kısa ifade. İfade yalnız özet değerleri taşır. |
| Özet | `Hash` | H fonksiyonunun çıktısı. |
| Tik | `tick` | `tick_ms` uzunluğunda zaman dilimi. |
| Agregatör | `Aggregator` | Nesneleri iddiaya ve kanıta çeviren düğüm rolü. |
| Tik Kanıtı | `TickProof` | Bir agregatörün bir tikte gördüğü nesneleri kanıtlar. |
| Birleşik Kanıt | `MergeProof` | İki kanıtı tek kanıtta birleştirir. |
| Ana Kanıt | `MasterProof` | Kanıt ağacının kökü. Blokta durur. |
| Segment | `Segment` | Bir Tik Kanıtının iddia aralığı. |
| İddia Çerçevesi | `ClaimFrame` | Blok bölümü. Segmentleri, iddiaları ve kanıtı taşır. |
| Şema kaydı | `SchemeRegistry` | Her `SchemeId` için ad, yerel doğrulayıcı ve `max_object_bytes`. |

## 4. Veri yapıları

Depodaki eşdeğer tipleri kullan (`Hash`, `Address`, `DomainId`, `Signature`). Kodu yeni bir `aggregation` modülüne yaz.

```rust
pub type SchemeId = u16;

pub enum ObjectKind { Signature, StarkProof, PcsOpening }

pub struct Envelope {
    pub kind: ObjectKind,
    pub scheme: SchemeId,       // yalnız Signature için
    pub payload: Vec<u8>,       // imza ve açık anahtar, kanıt veya açılış
    pub tx_hash: Hash,          // iddiayı kullanacak işlem
    pub domain: DomainId,
}

pub struct PoolItem {
    pub envelope: Envelope,
    pub claim: Claim,           // kabul sırasında türetilir
}

pub enum Claim {
    // signer, msg_hash özetli mesajı imzaladı
    Sig  { scheme: SchemeId, signer: Address, msg_hash: Hash },
    // program_hash programı input_hash girdisiyle çalıştı ve output_hash çıktısını verdi
    Prog { program_hash: Hash, input_hash: Hash, output_hash: Hash },
    // poly_hash taahhüdündeki polinom, point noktasında value değerini alır
    Poly { poly_hash: Hash, point: Ext, value: Ext },
}

pub struct Segment {
    pub claims_root: Hash,
    pub count: u32,
    pub aggregator: Address,
}

pub struct ClaimFrame {
    pub segments: Vec<Segment>,
    pub shape: Vec<u8>,         // sonek sırası: 0 = Tik Kanıtı, 1 = Birleştir
    pub claims: Vec<Claim>,     // segmentlerin sırasıyla birleşimi
    pub proof: Vec<u8>,         // Tik, Birleşik veya Ana Kanıt
}

pub enum TxAuth {
    Inline(Signature),          // mevcut yol
    Claimed(Hash),              // yetki iddiasının özeti
}

// Tx yapısına eklenen üye:
//   pub claim_refs: Vec<Hash>; // işlemin kullandığı diğer iddiaların özetleri
```

`ClaimFrame` her alt ağaç için de geçerlidir. Ağda dolaşan her kanıt bir `ClaimFrame`'dir. Blok çerçevesi ağacın köküdür.

### Özetler

```text
claim_digest = H("BDLM_CLAIM_V1"     || domain || tür_etiketi || iddia_değerleri)
claims_root  = Merkle(claim_digest_0 .. claim_digest_{n-1})
tick_pi      = H("BDLM_AGG_TICK_V1"  || domain || claims_root || count || aggregator)
merge_pi     = H("BDLM_AGG_MERGE_V1" || sol_pi || sağ_pi)
object_id    = H("BDLM_OBJECT_V1"    || zarf_baytları)
frame_hash   = H("BDLM_FRAME_V1"     || master_pi || H(proof))
AUTH_OK      = H("BDLM_AUTH_OK_V1")
```

Kurallar:

1. H, BudZKVM STARK yapılandırmasının hash fonksiyonudur. Düğümler H'yi yerel olarak da çalıştırır.
2. Tüm etiketler sabittir ve birbirinden farklıdır.
3. Kodlama kanoniktir. Aynı veri her zaman aynı baytları verir. Mevcut serileştirme düzenini kullan.
4. Merkle ağacı ikilidir. Yaprak: `H("BDLM_CLAIM_ROOT_V1" || 0x00 || özet)`. İç düğüm: `H("BDLM_CLAIM_ROOT_V1" || 0x01 || sol || sağ)`. Tek kalan düğüm bir üst seviyeye aynen çıkar. Boş liste geçersizdir.
5. `Ext`, BudZKVM STARK yapılandırmasının uzantı cisminin elemanıdır.

## 5. Blok kuralları

Hesap yetki kuralı:

```rust
// Şema hesabı
Claim::Sig  { scheme: hesap.scheme, signer: tx.from, msg_hash: tx.signing_hash() }

// Program hesabı (Faz 4)
Claim::Prog { program_hash: hesap.auth_program_hash, input_hash: tx.signing_hash(), output_hash: AUTH_OK }
```

`mode = On` iken kuantum güvenli şemalar yalnız `TxAuth::Claimed` kullanır. `TxAuth::Inline` kullanan kuantum güvenli işlem geçersizdir.

`mode = On` iken blok şu durumlarda geçersizdir:

1. `claims` tekrar içerir.
2. Segmentlerin `count` toplamı `claims.len()` değerine eşit değildir.
3. Bir segmentin `claims_root` değeri, kendi aralığındaki iddialardan hesaplanan köke eşit değildir.
4. `fold_shape` hata verir.
5. `proof`, `master_pi` için doğrulanmaz. `master_pi = fold_shape(blok.domain, segments, shape)` değeridir. `segments.len() == 1` ise program `AGG_TICK_PROGRAM_HASH` olur. Aksi halde program `AGG_MERGE_PROGRAM_HASH` olur.
6. `TxAuth::Claimed(d)` taşıyan işlemde `d`, hesap yetki kuralının iddia özetine eşit değildir.
7. Bir işlem, çerçevede olmayan bir iddiaya başvurur.
8. Çerçevedeki bir iddiaya hiçbir işlem başvurmaz.
9. `claims.len() > max_claims_per_block` veya `proof.len() > max_proof_bytes`.
10. Başlıktaki `frame_hash`, çerçeveden hesaplanan değere eşit değildir.

`fold_shape` sözde kodu:

```rust
fn fold_shape(domain: DomainId, segs: &[Segment], shape: &[u8]) -> Result<Hash> {
    let mut stack = Vec::new();
    let mut next = 0;
    for op in shape {
        match op {
            0 => {
                let s = segs.get(next).ok_or(BadShape)?;
                next += 1;
                stack.push(tick_pi(domain, s));
            }
            1 => {
                let r = stack.pop().ok_or(BadShape)?;
                let l = stack.pop().ok_or(BadShape)?;
                stack.push(merge_pi(l, r));
            }
            _ => return Err(BadShape),
        }
    }
    if next != segs.len() || stack.len() != 1 { return Err(BadShape); }
    Ok(stack.pop().unwrap())
}
```

## 6. Agregatör

### 6.1 Kabul kuralları

Agregatör bir zarfı şu koşullarda havuza alır:

1. `payload` boyutu sınırı aşmaz. İmza için sınır, şema kaydındaki `max_object_bytes` değeridir. Diğer türler için sınır `AggParams` içindedir.
2. `scheme` şema kaydında vardır.
3. `tx_hash`, havuzdaki geçerli bir işleme işaret eder. İşlem ücreti iddia ücretini kapsar. İşlem henüz bilinmiyorsa nesne en çok 2 tik bekler.
4. Yerel doğrulama geçer. Yerel doğrulama nesne türüne göre yapılır.
5. `object_id` daha önce görülmemiştir.
6. Gönderen, `sender_tick_limit` değerini aşmamıştır.
7. Havuz `pool_max_bytes` sınırının altındadır. Sınır dolunca ücret/bayt oranı en düşük nesne düşer.

Agregatör kabul sırasında iddiayı türetir ve `PoolItem` içine yazar. Kuralı bozan nesne düşer. Ağ katmanındaki mevcut ceza mekanizması gönderen eşe uygulanır.

Ağ konuları: `agg/object` (`Envelope`) ve `agg/frame` (`ClaimFrame`). Boyut sınırları mesaj katmanında uygulanır.

### 6.2 Tik döngüsü

```text
her tick_ms:
  items = havuz.yeni_öğeler(tick_max_objects)
  frame = backend.prove_tick(ctx, items)      // iddialar özete göre artan sıralı
  yayınla(frame)
  items içindeki payload baytlarını sil
  birleştir()
```

Kanıt üretimi hata verirse tiki atla ve hatayı kaydet. Bir nesne iki tik üst üste hata verirse nesneyi at.

### 6.3 Birleştirme

```text
birleştir():
  iddia kümeleri ayrık, geçerli iki çerçeve A ve B için:
    C = ClaimFrame::combine(A, B)
    yayınla(C)

combine(A, B):
  segments = A.segments ++ B.segments
  shape    = A.shape ++ B.shape ++ [1]
  claims   = A.claims ++ B.claims
  proof    = backend.prove_merge(A, B)
```

Agregatör gelen her çerçeveyi `backend.verify` ile doğrular. Geçersiz çerçeve düşer.

### 6.4 Blok üreticisi

1. Havuzdaki geçerli çerçeveleri topla.
2. İddia kümeleri ayrık olan çerçeveleri seç. Ücret/bayt oranı yüksek olandan başla. `max_claims_per_block` sınırını aşma.
3. Seçilen her çerçevenin iddialarına başvuran işlemleri `tx_hash` ile bul. İşlem eksikse çerçeveyi at.
4. Seçilen çerçeveleri `combine` ile tek çerçeveye indir.
5. Sonucu blok çerçevesi yap. `frame_hash` değerini başlığa yaz.
6. Blok süresi dolarsa hazır olan en büyük çerçeveyi kullan.

## 7. Ücret

1. `claim_fee(claim) = claim_fee_per_byte * kodlanmış_bayt(claim)`.
2. İddia ücreti, iddiaya bloktaki sırayla ilk başvuran işlemin ücretinden kesilir.
3. İşlem ücreti ödeyemezse işlem başarısız olur. İddia çerçevede kalır.
4. Her segmentin iddia ücretlerinin `agg_share_bps / 10000` oranı `segment.aggregator` adresine gider.
5. Kalan iddia ücreti blok üreticisine gider.

Yazıdaki oran: 64 baytlık iddia için 4096 gas. Bu bayt başına 64 eder. Bu oranı Budlum ücret birimine çevir.

## 8. Saklama ve budama

1. `prove_tick` bitince `PoolItem` içindeki `payload` baytları silinir.
2. Çerçeve, bloğu kesinleşince havuzdan silinir.
3. Tam düğüm `proof` baytlarını `proof_retention_blocks` blok boyunca saklar. Arşiv düğümü süresiz saklar.
4. İddialar blok gövdesiyle birlikte kalır.

## 9. Parametreler

```rust
pub enum AggMode { Off, Shadow, On }

pub struct AggParams {
    pub mode: AggMode,                  // ana ağ varsayılanı: Off
    pub tick_ms: u64,                   // 1000
    pub tick_max_objects: usize,        // 256
    pub sender_tick_limit: usize,       // 8
    pub max_claims_per_block: usize,    // 8192
    pub max_proof_bytes: usize,         // 262_144
    pub max_stark_object_bytes: usize,  // 262_144
    pub max_pcs_object_bytes: usize,    // 131_072
    pub pool_max_bytes: usize,          // 268_435_456
    pub claim_fee_per_byte: u64,        // 64
    pub agg_share_bps: u16,             // 5000
    pub proof_retention_blocks: u64,    // 100_000
}
```

Bunlar başlangıç değerleridir. Faz 3'te ölçüme göre ayarla.

Konsensüs parametreleri: `AGG_TICK_PROGRAM_HASH`, `AGG_MERGE_PROGRAM_HASH`, `max_claims_per_block`, `max_proof_bytes`, `claim_fee_per_byte`, `agg_share_bps`. Kalan parametreler düğüm yapılandırmasıdır.

Modlar:

- `Off`: sistem kapalıdır. Davranış değişmez.
- `Shadow`: düğüm, bloklardaki satır içi imzaları nesne olarak işler. Tik Kanıtı üretir. Sonucu yerel doğrulamayla karşılaştırır. Çerçeve yayınlamaz. Konsensüs değişmez.
- `On`: bölüm 5 kuralları geçerlidir.

## 10. Yol

8288 yazısı üç yol verir:

- leanSPHINCS: tek bir imza şemasını toplar.
- leanSTARK: her programı toplar. Protokolün bir ISA (komut kümesi mimarisi) seçmesi gerekir.
- leanPCS: her programı toplar. Protokolün yalnız bir cisim seçmesi gerekir. PCS, polinom taahhüt şemasıdır.

Yazı leanPCS ile başlar. ISA seçilince leanSTARK'ı ekler.

Budlum'da ISA hazırdır: BudZKVM. Budlum'un asıl sorunu kuantum imza verisidir. Bu yüzden Budlum sırayı değiştirir: önce imza iddiaları, sonra kanıt ağacı, sonra program iddiaları, en son PCS iddiaları. Budlum'un PCS'i, BudZKVM'in mevcut PCS yapılandırmasıdır. Yeni cisim seçilmez.

### Faz 0. Ölçüm

1. Depodaki kuantum güvenli imza şemasını bul. Adını, imza baytını, açık anahtar baytını ve doğrulama süresini yaz.
2. Bir bloktaki imza payını 100 ve 1000 işlem için ölç.
3. Ölçümü bir benchmark olarak ekle: `benches/agg_baseline`.

Çıkış: Sonuç tablosu PR açıklamasındadır. Benchmark derlenir ve çalışır.

### Faz 1. İddia katmanı

1. Bölüm 4'teki tipleri ve özet kurallarını ekle. Her etiket için test yaz.
2. Şema kaydını yaz: her `SchemeId` için ad, yerel doğrulayıcı ve `max_object_bytes`.
3. `AggBackend` trait'ini yaz:

```rust
pub trait AggBackend {
    fn prove_tick(&self, ctx: &TickCtx, items: &[PoolItem]) -> Result<ClaimFrame>;
    fn prove_merge(&self, left: &ClaimFrame, right: &ClaimFrame) -> Result<Vec<u8>>;
    fn verify(&self, frame: &ClaimFrame, domain: DomainId) -> Result<()>;
}
```

4. `NativeBackend` yaz. Kanıt baytları, nesnelerin kodlanmış halidir. `verify` nesneleri yerel olarak doğrular. `NativeBackend` yalnız test ve devnet derlemesinde bulunur. Üretim derlemesinde derlenmez.
5. Nesne havuzunu, kabul kurallarını (6.1) ve tik döngüsünü (6.2) yaz.
6. Ağ konularını ekle: `agg/object` ve `agg/frame`.
7. İşlem biçimini değiştir: `TxAuth` ve `claim_refs`.
8. Blok biçimini değiştir: gövdede `claim_frame`, başlıkta `frame_hash`.
9. Blok kurallarını (bölüm 5) yaz.
10. Ücret (bölüm 7) ve budama (bölüm 8) kurallarını yaz.
11. `AggMode` ve `AggParams` ekle.

Çıkış: Devnet, `NativeBackend` ile blok üretir. Kuantum güvenli işlemlerde imza baytı yoktur. Bölüm 11'deki negatif testler geçer.

### Faz 2. İmza iddiaları (Tik Kanıtı)

1. Kayıtlı imza şemasının doğrulayıcısını bir BudZKVM programı olarak yaz: `verify_sig`. Önce referans programı yaz, sonra çipleri.
2. `AggTick` programını yaz. Girdi: `domain`, `aggregator`, nesneler. Program her nesne için şunu yapar:
   - İmzayı doğrular.
   - `address_of(pubkey) == signer` kontrol eder.
   - İddia özetini hesaplar.
3. `AggTick` programı `claims_root` ve `count` hesaplar. Program `tick_pi` değerini public değer olarak yazar.
4. Fark testi yaz. Yerel doğrulayıcı ve devre aynı kabul veya ret sonucunu verir.
5. 16, 64 ve 256 imza için kanıt süresini, kanıt boyutunu ve doğrulama süresini ölç.
6. Ölçümdeki sıcak noktaları (hash, Merkle yolu) özel çiplere taşı. Her taşımadan sonra yeniden ölç.

Kısıt: `VerifyMerkle` opcode'u üretimde kapalıdır. Devre bu opcode'a bağlıysa `mode` ana ağda `Off` kalır. Test ağında `On` olabilir.

Çıkış: 256 imzalık Tik Kanıtı doğrulanır. Bozuk bir imza kanıt üretimini durdurur. Ölçüm tablosu PR'dadır.

### Faz 3. Özyineleme ve Ana Kanıt

1. BudZKVM içinde BudZKVM kanıtı doğrulayan bir program yaz: `verify_vm_proof`. Önce referans programı yaz, sonra çipleri.
2. `AggMerge` programını yaz. Program iki alt kanıtı doğrular. Alt kanıtın program özeti `{AGG_TICK_PROGRAM_HASH, AGG_MERGE_PROGRAM_HASH}` kümesindedir. Program `merge_pi` değerini public değer olarak yazar.
3. `AggMerge` kendi program özetini girdi olarak alır. Son doğrulayıcı bu özeti sabit `AGG_MERGE_PROGRAM_HASH` ile karşılaştırır.
4. `StarkBackend` yaz. `prove_tick`, `prove_merge` ve `verify` bu programlarla çalışır.
5. Blok üreticisini (6.4) ve birleştirmeyi (6.3) `StarkBackend` ile çalıştır.
6. `StarkBackend`'i varsayılan yap.
7. Güvenlik düzeyini BudZKVM'in mevcut düzeyiyle aynı tut veya yükselt.
8. `claim_fee_per_byte` değerini ölçülen kanıt maliyetine göre ayarla. Bir iddianın ücreti, o iddianın kanıt maliyetini aşar.
9. Test ağında `Shadow` modunu aç. 10 000 blok boyunca yerel doğrulama ve devre sonucu aynıdır.

Çıkış: 1024 iddialık bir blok tek Ana Kanıtla doğrulanır. `proof.len() <= max_proof_bytes`. 10 000 blokluk `Shadow` çalışmasında sıfır fark vardır. Ölçüm tablosu (üretim süresi, kanıt boyutu, blok doğrulama süresi) PR'dadır. Sonra test ağı `mode = On` olur.

### Faz 4. Program iddiaları

1. `AggTick` programına `StarkProof` nesne türünü ekle. Program BudZKVM kanıtını `verify_vm_proof` ile doğrular. Program `Prog { program_hash, input_hash, output_hash }` iddiasını üretir.
2. Domain durum geçiş kanıtlarını bu yola bağla. Her geçiş kanıtı, kendi programının `Prog` iddiasına dönüşür. `DomainFinalityAdapter` geçişi bu iddiadan okur.
3. Hesaba `auth_program_hash` üyesini ekle. Bu hesabın işlemi için `Claimed(d)`, bölüm 5'teki program hesabı iddiasına eşittir.
4. Mantığı gizli çoklu imza için örnek bir program yaz: m-of-n, anahtar kümesi gizli. İddia yalnız `(program_hash, msg_hash, AUTH_OK)` değerlerini taşır.

Çıkış: Bir domain geçiş kanıtı Ana Kanıt üzerinden kesinleşir. Gizli çoklu imza hesabı bir işlem gönderir. Yetki mantığı zincirde görünmez.

### Faz 5. PCS iddiaları

1. BudZKVM STARK doğrulayıcısını iki parçaya ayır: `verify_pcs_openings` ve `verify_algebra`.
2. `Claim::Poly` türünü ve `PcsOpening` nesne türünü ekle. `AggTick` programı açılışı doğrular ve `Poly` iddiaları üretir.
3. `VerifyStark` işlem türünü ekle. İşlem public değerleri, taahhütleri ve `Poly` iddialarına başvuruları taşır. Zincir `verify_algebra` çalıştırır.
4. Zincir, açılış noktasını (`zeta`) taahhütlerden ve public değerlerden yeniden türetir. `Poly.point` bu değere eşittir.
5. `AirRegistry` ekle. Kayıt, bir AIR özetini yerel kısıt değerlendiricisine bağlar. BudZKVM AIR'i yerleşik girdidir.
6. Fark testi yaz. İki aşamalı doğrulama ile tam doğrulama aynı kabul veya ret sonucunu verir. Bozuk kanıtlar dahil 10 000 durum çalışır.
7. BudZKVM dışındaki bir AIR'in kanıtını (aynı PCS yapılandırması) kabul eden bir test yaz.
8. Zincir tarafı doğrulama maliyetini tam doğrulamayla karşılaştır.

Çıkış: Fark testi geçer. Maliyet tablosu PR'dadır.

## 11. Testler

Her faz ilgili testleri çalıştırır:

1. Fark testi: yerel doğrulama ve kanıt doğrulaması aynı sonucu verir.
2. Negatif testler. Her biri geçersiz blok veya reddedilen kanıt üretir:
   - Yanlış imza, yanlış mesaj, yanlış şema, yanlış adres.
   - Tekrarlı iddia, yeniden sıralanmış iddia.
   - Değiştirilmiş `count`, `claims_root`, `aggregator`, `shape`.
   - Yer değiştirmiş segment.
   - Başka domain'in iddiası.
   - Başka parametre kümesinin kanıtı.
   - Kısaltılmış ve aşırı büyük kanıt.
   - İşlemsiz iddia, iddiasız `Claimed` işlem.
   - Ücreti ödenemeyen işlem.
3. Özellik testleri: `fold_shape`, `Merkle`, `combine`.
4. Fuzz hedefleri: `ClaimFrame` ve `Envelope` çözümleme.
5. Etiket testi: tüm `BDLM_*` etiketleri farklıdır. Hiçbiri diğerinin öneki değildir.
6. Derleme testi: üretim profilinde `NativeBackend` yoktur.
7. Yük testi: 4 agregatör ve 1 blok üreticisi. Bir blok en az 1000 iddia taşır.

## 12. Teslim

1. Her faz ayrı bir PR'dır.
2. Çıkış ölçütü PR açıklamasında ölçümle gösterilir.
