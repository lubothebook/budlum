# Gorev sablonu (alt ajan ortak kurallari)

Bu dosya PR9 bulgu programinin ortak gorev kurallaridir. Her gorev paketi bu dosyaya yol verir. Dil: Turkce rapor, Ingilizce kod ve yorum (ASD-STE100, kisa cumleler, uzun tire ve kalin isaret yok).

## 0. Basla

1. Ilk is: `docs/AGENT_MAP.md` oku. README okunmaz. Tum repo okunmaz.
2. Paketin dosya sahipligi listesi disinda dosya degistirme. Baska dosyaya dokunmak gerekirse bulguyu `KARAR` veya `BAGIMLI` olarak raporla, kod yazma.
3. Skill veya arac kurma. Betik, hook, ag cagrisi yok. Shell betigi yazma (Z6). Kod dili Rust ve BudL.

## 1. Z2: yerelde CI araci calistirma

`cargo fmt`, `clippy`, `test`, `check`, `build`, `deny`, `audit`, typos, gates, miri, kani CALISTIRILMAZ. GitHub CI tek hakemdir. Bu nedenle kodu derleyici gibi oku:

- Her kullandigin tipin, fonksiyonun, alanin imzasini `rg` ile ac ve oku. Tahmin yok.
- Her `use` satirini, gorunurlugu (`pub`, `pub(crate)`) ve `#[cfg(test)]` kapsamini kontrol et.
- rustfmt bicimi: 4 bosluk, satir 100 karakter, yaygin stil. Komsu kodun bicimini kopyala.
- clippy `-D warnings`: `field_reassign_with_default`, `needless_borrow`, `too_many_arguments`, `large_enum_variant`, `manual_*`, `unwrap_used` ve `expect_used` (uretim yolunda deny) gibi kaliplardan kacin.
- Istisna: yalnizca kilit dosyasi bakimi (`cargo update -p <crate> --precise <surum>`) calistirilabilir. Derleme yok.

## 2. Her bulgu icin akis

1. Bul: ham bulgu `docs/audit/FINDINGS.md` veya `notes/Pxx-scan.md` icinde. Kodu CANLI oku. Satirlar kaymis olabilir.
2. Dogrula: bulgunun hala gecerli oldugunu `rg` komutu veya testle goster. Gecersizse (kod degismis, iddia yanlis) `YP` (yanlis pozitif), kanitini yaz: dosya:satir ve komut. Kod yazma.
3. Karar gerekirse (mimari veya ekonomik: protokol parametresi, ucret, tokenomics, yeni crate, yeni unsafe, gecmise donuk format degisikligi) `KARAR`: kod yazma. Soru metni: bulgu ID, kanit, secenekler (A, B, C) ve onerin.
4. Kodla: once bulguyu gosteren ve duzeltme olmadan basarisiz olacak bir test yaz. Sonra duzeltmeyi yaz. Yalnizca bulgunun gerektirdigi satirlari degistir. Test, sahip olunan dosyadaki `#[cfg(test)]` modulune girer. Yeni test dosyasi icin `mod` kaydi baska dosyada gerekiyorsa o dosya sahip listesinde olmalidir.
5. Commit: bir bulgu, bir commit. Yalnizca ilgili dosyalari `git add <dosya>` ile ekle (`git add -A` yok). Mesaj bicimi:

```
fix(P17/BLG-0042): <kisa aciklama>

Dosya: src/storage/erasure.rs:118
Kanit: <yeniden uretme yontemi>
Test: <eklenen testin adi>

Co-Authored-By: Claude Code <noreply@anthropic.com>
```

Commit atilan dal senin worktree dalindir. PUSH ETME. Ana oturum commitlerini alip PR9 dalina koyar.

## 3. Yasaklar (ihlal sayilir)

- Yeni `#[allow(...)]`, yeni `#[ignore]`, testi silmek veya beklenen degeri hataya uydurmak, CI adimini gevsetmek veya eski commite sabitlemek, clippy icin mantigi zayiflatmak.
- `.github/*-baseline.txt` dosyalarina dokunmak.
- Yeni `unsafe` (Z7). Yeni harici crate (karar gerektirir).
- Uretim yolunda `unwrap`, `expect`, `panic!`, denetimsiz indeksleme. Konsensus yolunda float (Z8). Tutar, ucret, bakiye, yukseklikte `saturating_*` ve `wrapping_*` (yazili gerekce olmadan).
- Kamu metinlerinde "L1" veya "Layer 1" (Z12). Dogru ad: Budlum Evrensel Mutabakat Katmani.
- Token, anahtar, seed yazmak (Z9).
- Bulguyu ham koda dokmek: kodu sen yaz, oku, gozden gecir.
- Yeni genel (pub) ogeyi uretim cagri noktasi olmadan eklemek (bos kod kapilari kirmizi olur). Duzeltme yeni bir bagli olmayan genel oge gerektiriyorsa uretim yoluna bagla.

## 4. Teslim

Son cevabin YALNIZCA su bicimde olur, bulgu basina bir satir, ek yorum yok:

```
PAKET: <kimlik>   DAL: <git dal adi>   TABAN: <sha>
BLG-xxxx | KOD | <sha> | <tek cumle sonuc>
BLG-xxxx | YP | - | <kanit: dosya:satir veya komut>
BLG-xxxx | KARAR | - | <soru: secenekler A/B/C, oneri>
BLG-xxxx | BAGIMLI | - | <hangi dosya, kimin sahipligi>
BLG-xxxx | ENGELLI | - | <3 deneme sonrasi neden>
YENI: <paket kapsami disi yeni bulgu: dosya:satir ve kanit>   (varsa)
```

Uc basarisiz denemede farkli yaklasim sec. Hala olmuyorsa `ENGELLI` yaz ve sonraki bulguya gec.
