# Bulgu Defteri

Kural: CLAUDE.md (CC-ORK-01) ve PR9 direktifi. Durum değerleri: `AÇIK`, `KODLANIYOR`, `CI BEKLİYOR`, `KAPALI`, `YANLIŞ POZİTİF`, `KARAR BEKLİYOR`. Bir bulgu yalnızca commit SHA, yeşil CI run numarası ve yeniden üretme yönteminin artık göstermemesi ile `KAPALI` olur.

Kaynak: ham bulgular `docs/audit/FINDINGS.md` ve `notes/Pxx-scan.md` içindedir. `Kanıt` sütunu ham bulgunun yerini ve varsa doğrulama notunu verir. Yeniden üretme kanıtı, bulgu işlenirken bu sütuna yazılır. P0 parçası parça numarası olmayan program ve CI bulgularını taşır (varsayım: direktifteki 63 parça P0 ile P62 arasıdır).

| Parça | ID | Dosya:Satır | Kanıt | Durum | Commit SHA | CI run |
|---|---|---|---|---|---|---|
| P0 | BLG-0001 | .gitleaks.toml | CI #122 Secret Scan: `[[allowlists]] must contain at least one check`; düzeltme b35dd86 (parmak izi .gitleaksignore dosyasına taşındı); yeşil kanıt bekleniyor | CI BEKLİYOR | b35dd86 |  |
| P0 | BLG-0002 | src/network/regen_health_tick.rs:337 | CI #124 Clippy: `field_reassign_with_default` (-D warnings), iş 115030778899 ve 115030778747 / gp/P0-A:fd859e7 | KODLANIYOR |  |  |
| P0 | BLG-0003 | Cargo.lock; budzero/Cargo.lock | CI #124 `cargo audit`: iş 115030778600: 3 zafiyet ve 9 reddedilen uyarı, ikinci kilit dosyasında 5 zafiyet ve 5 reddedilen uyarı | AÇIK |  |  |
| P0 | BLG-0004 | .github/workflows/ci.yml (Repo Lint, Rust gates) | CI #124 iş 115030778593: `Rust gates` adımı "One house rule OK" satırından sonra exit 1, hata satırı yok / umbrella: kapanışı BLG-1095..1098 ile | KODLANIYOR |  |  |
| P0 | BLG-0005 | PR9 mergeable_state | PR9 `dirty` idi, son iki commit için `pull_request` CI hiç çalışmadı; 030cc60 merge commit ile giderildi | CI BEKLİYOR | 030cc60 |  |
| P01 | BLG-0006 | - | P01-F01: FINDINGS.md:11; ham önem Critical | AÇIK |  |  |
| P01 | BLG-0007 | - | P01-F02: FINDINGS.md:18; ham önem High | AÇIK |  |  |
| P01 | BLG-0008 | - | P01-F03: FINDINGS.md:24; ham önem High | AÇIK |  |  |
| P01 | BLG-0009 | - | P01-F04: FINDINGS.md:29; ham önem Critical | AÇIK |  |  |
| P01 | BLG-0010 | - | P01-F05: FINDINGS.md:35; ham önem High | AÇIK |  |  |
| P01 | BLG-0011 | - | P01-F06: FINDINGS.md:41; ham önem High | AÇIK |  |  |
| P01 | BLG-0012 | - | P01-F07: FINDINGS.md:46; ham önem High | AÇIK |  |  |
| P01 | BLG-0013 | - | P01-F08: FINDINGS.md:52; ham önem Medium | AÇIK |  |  |
| P01 | BLG-0014 | src/consensus/poa.rs:353-417 | P01-F09: FINDINGS.md:70; ham önem Medium; PARTLY | AÇIK |  |  |
| P01 | BLG-0015 | - | P01-F09b: FINDINGS.md:78; ham önem Low | AÇIK |  |  |
| P01 | BLG-0016 | - | P01-F09c: FINDINGS.md:78; ham önem Low | AÇIK |  |  |
| P01 | BLG-0017 | - | P01-F10: FINDINGS.md:71; ham önem Medium; CONFIRMED | AÇIK |  |  |
| P01 | BLG-0018 | - | P01-F11: FINDINGS.md:72; ham önem Medium; CONFIRMED | AÇIK |  |  |
| P01 | BLG-0019 | - | P01-F12: FINDINGS.md:73; ham önem Medium; CONFIRMED | AÇIK |  |  |
| P01 | BLG-0020 | - | P01-F13: FINDINGS.md:74; ham önem Medium; CONFIRMED | AÇIK |  |  |
| P01 | BLG-0021 | storage/db.rs:1454-1500 | P01-F14: FINDINGS.md:75; ham önem Medium; CONFIRMED | AÇIK |  |  |
| P01 | BLG-0022 | - | P01-F14b: FINDINGS.md:78; ham önem Low | AÇIK |  |  |
| P01 | BLG-0023 | - | P01-F15: FINDINGS.md:76; ham önem Medium; CONFIRMED | AÇIK |  |  |
| P01 | BLG-0024 | - | P01-F16: FINDINGS.md:77; ham önem Medium; CONFIRMED | AÇIK |  |  |
| P01 | BLG-0025 | - | P01-F16b: FINDINGS.md:78; ham önem Low | AÇIK |  |  |
| P01 | BLG-0026 | - | P01-F17: FINDINGS.md:79; ham önem Medium | AÇIK |  |  |
| P01 | BLG-0027 | src/tests/integration.rs:171-196 | P01-F18: FINDINGS.md:80; ham önem Medium | AÇIK |  |  |
| P01 | BLG-0028 | - | P01-F19: FINDINGS.md:81; ham önem Medium | AÇIK |  |  |
| P01 | BLG-0029 | consensus/mod.rs:136-172 | P01-F20: FINDINGS.md:82; ham önem Low | AÇIK |  |  |
| P01 | BLG-0030 | - | P01-F21: FINDINGS.md:83; ham önem Low | AÇIK |  |  |
| P01 | BLG-0031 | - | P01-F22: FINDINGS.md:84; ham önem Low | AÇIK |  |  |
| P01 | BLG-0032 | - | P01-F23: FINDINGS.md:85; ham önem Low | AÇIK |  |  |
| P01 | BLG-0033 | - | P01-F24: FINDINGS.md:86; ham önem Low | AÇIK |  |  |
| P01 | BLG-0034 | consensus/mod.rs:88 | P01-F25: FINDINGS.md:87; ham önem Low | AÇIK |  |  |
| P01 | BLG-0035 | - | P01-F26: FINDINGS.md:88; ham önem Info | AÇIK |  |  |
| P01 | BLG-0036 | - | P01-F27: FINDINGS.md:89; ham önem Info | AÇIK |  |  |
| P02 | BLG-0037 | src/crypto/pkcs11.rs:116-134, 417-418, 554-566 | P02-F01: FINDINGS.md:95; ham önem Medium; CONFIRMED | AÇIK |  |  |
| P02 | BLG-0038 | - | P02-F02: FINDINGS.md:96; ham önem Low; CONFIRMED | AÇIK |  |  |
| P02 | BLG-0039 | - | P02-F03: FINDINGS.md:97; ham önem Medium; CONFIRMED | AÇIK |  |  |
| P02 | BLG-0040 | .github/workflows/ci.yml | P02-F04: FINDINGS.md:98; ham önem Medium; PARTLY | AÇIK |  |  |
| P02 | BLG-0041 | settlement/mod.rs:27-28 | P02-F05: FINDINGS.md:99; ham önem Low; CONFIRMED | AÇIK |  |  |
| P02 | BLG-0042 | crates/bpqs/src/wots.rs:26-32, 63-69, 137-148 | P02-F06: FINDINGS.md:100; ham önem Low; CONFIRMED | AÇIK |  |  |
| P02 | BLG-0043 | src/crypto/primitives.rs:505-518, 677-689 | P02-F07: FINDINGS.md:101; ham önem Low; CONFIRMED | AÇIK |  |  |
| P02 | BLG-0044 | src/crypto/mainnet_policy.rs:119-124, 90-136 | P02-F08: FINDINGS.md:102; ham önem Medium | AÇIK |  |  |
| P02 | BLG-0045 | - | P02-F09: FINDINGS.md:103; ham önem Medium | AÇIK |  |  |
| P02 | BLG-0046 | - | P02-F10: FINDINGS.md:104; ham önem Low | AÇIK |  |  |
| P02 | BLG-0047 | - | P02-F11: FINDINGS.md:105; ham önem Low | AÇIK |  |  |
| P02 | BLG-0048 | - | P02-F12: FINDINGS.md:106; ham önem Low | AÇIK |  |  |
| P02 | BLG-0049 | src/crypto/signer.rs:10-15 | P02-F13: FINDINGS.md:107; ham önem Low | AÇIK |  |  |
| P02 | BLG-0050 | - | P02-F14: FINDINGS.md:108; ham önem Low | AÇIK |  |  |
| P02 | BLG-0051 | - | P02-F15: FINDINGS.md:109; ham önem Low | AÇIK |  |  |
| P02 | BLG-0052 | - | P02-F16: FINDINGS.md:110; ham önem Low | AÇIK |  |  |
| P02 | BLG-0053 | - | P02-F17: FINDINGS.md:111; ham önem Low | AÇIK |  |  |
| P02 | BLG-0054 | - | P02-F18: FINDINGS.md:112; ham önem Info | AÇIK |  |  |
| P02 | BLG-0055 | - | P02-F19: FINDINGS.md:113; ham önem Info | AÇIK |  |  |
| P02 | BLG-0056 | - | P02-F20: FINDINGS.md:114; ham önem Info | AÇIK |  |  |
| P02 | BLG-0057 | - | P02-F21: FINDINGS.md:115; ham önem Info | AÇIK |  |  |
| P03 | BLG-0058 | src/privacy/note_registry.rs:104-111, 163-173 | P03-F01: FINDINGS.md:121; ham önem Medium; CONFIRMED / soru PR9 yorumunda | KARAR BEKLİYOR |  |  |
| P03 | BLG-0059 | core/account.rs:2456-2458 | P03-F02: FINDINGS.md:122; ham önem Medium; CONFIRMED / gp/P03-A:7fbcd05 | KODLANIYOR |  |  |
| P03 | BLG-0060 | - | P03-F03: FINDINGS.md:123; ham önem Low / gp/P03-A:509fe1e | KODLANIYOR |  |  |
| P03 | BLG-0061 | - | P03-F04: FINDINGS.md:124; ham önem Low / gp/P03-A:059a7c5 | KODLANIYOR |  |  |
| P03 | BLG-0062 | src/settlement/proof_market.rs:296-316, 634-650, 695-697 | P03-F05: FINDINGS.md:125; ham önem Low; CONFIRMED / gp/P03-A:3e30204 kısmi | KODLANIYOR |  |  |
| P03 | BLG-0063 | - | P03-F06: FINDINGS.md:127; ham önem Low / gp/P03-A:99ed814 | KODLANIYOR |  |  |
| P03 | BLG-0064 | core/account.rs:1570 | P03-F07: FINDINGS.md:128; ham önem Medium / BAĞIMLI: executor.rs, account.rs; önce escrow kararı | AÇIK |  |  |
| P03 | BLG-0065 | - | P03-F08: FINDINGS.md:129; ham önem Low / gp/P03-A:5683bd8 | KODLANIYOR |  |  |
| P03 | BLG-0066 | - | P03-F09: FINDINGS.md:130; ham önem Low | AÇIK |  |  |
| P03 | BLG-0067 | - | P03-F10: FINDINGS.md:131; ham önem Low | AÇIK |  |  |
| P03 | BLG-0068 | src/settlement/cold_wallet.rs:498-513, 614-619, 635-653 | P03-F11: FINDINGS.md:132; ham önem Medium; CONFIRMED | AÇIK |  |  |
| P03 | BLG-0069 | src/settlement/pq_anchor.rs:639-668 | P03-F12: FINDINGS.md:133; ham önem Low; CONFIRMED | AÇIK |  |  |
| P03 | BLG-0070 | - | P03-F13: FINDINGS.md:134; ham önem Medium; CONFIRMED | AÇIK |  |  |
| P03 | BLG-0071 | - | P03-F14: FINDINGS.md:135; ham önem Low | AÇIK |  |  |
| P03 | BLG-0072 | - | P03-F15: FINDINGS.md:136; ham önem Low | AÇIK |  |  |
| P03 | BLG-0073 | src/settlement/cold_quorum.rs:282-355 | P03-F16: FINDINGS.md:137; ham önem Low | AÇIK |  |  |
| P03 | BLG-0074 | settlement/mod.rs:9-12 | P03-F17: FINDINGS.md:138; ham önem Low | AÇIK |  |  |
| P03 | BLG-0075 | - | P03-F18: FINDINGS.md:139; ham önem Low | AÇIK |  |  |
| P03 | BLG-0076 | - | P03-F19: FINDINGS.md:140; ham önem Medium | AÇIK |  |  |
| P03 | BLG-0077 | src/tokenomics/mod.rs:279-290 | P03-F20: FINDINGS.md:141; ham önem Medium; CONFIRMED | AÇIK |  |  |
| P03 | BLG-0078 | tokenomics/mod.rs:243-277 | P03-F21: FINDINGS.md:142; ham önem Low | AÇIK |  |  |
| P03 | BLG-0079 | - | P03-F22: FINDINGS.md:143; ham önem Info | AÇIK |  |  |
| P03 | BLG-0080 | src/tests/tokenomics_proptest.rs:27-122 | P03-F23: FINDINGS.md:144; ham önem Low | AÇIK |  |  |
| P03 | BLG-0081 | src/settlement/proof_verifier.rs:67-117 | P03-F24: FINDINGS.md:145; ham önem Low | AÇIK |  |  |
| P03 | BLG-0082 | - | P03-F25: FINDINGS.md:146; ham önem Info | AÇIK |  |  |
| P04 | BLG-0083 | src/registry/invalid_vote.rs:66-107 | P04-F01: FINDINGS.md:152; ham önem Critical; CONFIRMED / soru PR9 yorumunda | KARAR BEKLİYOR |  |  |
| P04 | BLG-0084 | src/registry/permissionless.rs:480-515, 690-737 | P04-F02: FINDINGS.md:154; ham önem High; CONFIRMED / gp/P04-A:1bbd9f7 kısmi: registry yarısı; kalan yarı account.rs/blockchain.rs/executor.rs | KODLANIYOR |  |  |
| P04 | BLG-0085 | - | P04-F03: FINDINGS.md:155; ham önem High; CONFIRMED / BAĞIMLI: sahip olunmayan dosya (account.rs/blockchain.rs/executor.rs/snapshot.rs) | AÇIK |  |  |
| P04 | BLG-0086 | rpc/tests.rs:1146 | P04-F04: FINDINGS.md:157; ham önem Medium; PARTLY / BAĞIMLI: sahip olunmayan dosya (account.rs/blockchain.rs/executor.rs/snapshot.rs) | AÇIK |  |  |
| P04 | BLG-0087 | - | P04-F05: FINDINGS.md:160; ham önem Medium / soru PR9 yorumunda | KARAR BEKLİYOR |  |  |
| P04 | BLG-0088 | - | P04-F06: FINDINGS.md:161; ham önem Medium / BAĞIMLI: sahip olunmayan dosya (account.rs/blockchain.rs/executor.rs/snapshot.rs) | AÇIK |  |  |
| P04 | BLG-0089 | consensus/poa.rs:149-158 | P04-F07: FINDINGS.md:162; ham önem Medium / BAĞIMLI: sahip olunmayan dosya (account.rs/blockchain.rs/executor.rs/snapshot.rs) | AÇIK |  |  |
| P04 | BLG-0090 | core/account.rs:2256-2532 | P04-F08: FINDINGS.md:163; ham önem Medium / BAĞIMLI: sahip olunmayan dosya (account.rs/blockchain.rs/executor.rs/snapshot.rs) | AÇIK |  |  |
| P04 | BLG-0091 | registry/poa_compliance.rs:114-264 | P04-F09: FINDINGS.md:164; ham önem Medium | AÇIK |  |  |
| P04 | BLG-0092 | - | P04-F10: FINDINGS.md:165; ham önem Medium | AÇIK |  |  |
| P04 | BLG-0093 | - | P04-F11: FINDINGS.md:166; ham önem Medium | AÇIK |  |  |
| P04 | BLG-0094 | - | P04-F12: FINDINGS.md:167; ham önem Low | AÇIK |  |  |
| P04 | BLG-0095 | registry/identity.rs:571-622 | P04-F13: FINDINGS.md:168; ham önem Medium | AÇIK |  |  |
| P04 | BLG-0096 | - | P04-F14: FINDINGS.md:169; ham önem Medium | AÇIK |  |  |
| P04 | BLG-0097 | registry/identity_fill.rs:333-381, 508-551 | P04-F15: FINDINGS.md:170; ham önem Medium | AÇIK |  |  |
| P04 | BLG-0098 | - | P04-F16: FINDINGS.md:171; ham önem Medium | AÇIK |  |  |
| P04 | BLG-0099 | src/tests/identity_tx_door.rs:167-215 | P04-F17: FINDINGS.md:172; ham önem Medium | AÇIK |  |  |
| P04 | BLG-0100 | registry/evidence.rs:122-135, 228-251, 431-486 | P04-F18: FINDINGS.md:173; ham önem Low | AÇIK |  |  |
| P04 | BLG-0101 | registry/params.rs:242-287 | P04-F19: FINDINGS.md:174; ham önem Low | AÇIK |  |  |
| P04 | BLG-0102 | - | P04-F20: FINDINGS.md:175; ham önem Low | AÇIK |  |  |
| P04 | BLG-0103 | - | P04-F21: FINDINGS.md:176; ham önem Low | AÇIK |  |  |
| P04 | BLG-0104 | - | P04-F22: FINDINGS.md:177; ham önem Low | AÇIK |  |  |
| P04 | BLG-0105 | registry/quarantine_ledger.rs:53-116 | P04-F23: FINDINGS.md:178; ham önem Low | AÇIK |  |  |
| P04 | BLG-0106 | registry/evidence.rs:23-48 | P04-F24: FINDINGS.md:179; ham önem Info | AÇIK |  |  |
| P04 | BLG-0107 | - | P04-F25: FINDINGS.md:180; ham önem Info | AÇIK |  |  |
| P05 | BLG-0108 | src/tests/bridge_lifecycle.rs:80 | P05-F01: FINDINGS.md:199; ham önem High | AÇIK |  |  |
| P05 | BLG-0109 | - | P05-F02: FINDINGS.md:200; ham önem High | AÇIK |  |  |
| P05 | BLG-0110 | evm/mpt.rs:181-194 | P05-F03: FINDINGS.md:201; ham önem Medium | AÇIK |  |  |
| P05 | BLG-0111 | evm/header.rs:117-158 | P05-F04: FINDINGS.md:202; ham önem Medium | AÇIK |  |  |
| P05 | BLG-0112 | - | P05-F05: FINDINGS.md:203; ham önem Medium | AÇIK |  |  |
| P05 | BLG-0113 | - | P05-F06: FINDINGS.md:204; ham önem Medium | AÇIK |  |  |
| P05 | BLG-0114 | - | P05-F07: FINDINGS.md:205; ham önem Medium | AÇIK |  |  |
| P05 | BLG-0115 | - | P05-F08: FINDINGS.md:206; ham önem Medium | AÇIK |  |  |
| P05 | BLG-0116 | evm/bud_to_eth.rs:25-137 | P05-F09: FINDINGS.md:207; ham önem Medium | AÇIK |  |  |
| P05 | BLG-0117 | - | P05-F10: FINDINGS.md:208; ham önem Medium | AÇIK |  |  |
| P05 | BLG-0118 | - | P05-F11: FINDINGS.md:209; ham önem Low | AÇIK |  |  |
| P05 | BLG-0119 | - | P05-F12: FINDINGS.md:210; ham önem Low | AÇIK |  |  |
| P05 | BLG-0120 | - | P05-F13: FINDINGS.md:211; ham önem Low | AÇIK |  |  |
| P05 | BLG-0121 | core/hash.rs:22-28 | P05-F14: FINDINGS.md:212; ham önem Low | AÇIK |  |  |
| P05 | BLG-0122 | evm/rlp.rs:162-166, 258-282 | P05-F15: FINDINGS.md:213; ham önem Low | AÇIK |  |  |
| P05 | BLG-0123 | evm/rlp.rs:138-236 | P05-F16: FINDINGS.md:214; ham önem Low | AÇIK |  |  |
| P05 | BLG-0124 | evm/mpt.rs:150-171, 249-268, 27-47 | P05-F17: FINDINGS.md:215; ham önem Low | AÇIK |  |  |
| P05 | BLG-0125 | evm/receipt.rs:70-76, 97-104, 148-155 | P05-F18: FINDINGS.md:216; ham önem Low | AÇIK |  |  |
| P05 | BLG-0126 | - | P05-F19: FINDINGS.md:217; ham önem Low | AÇIK |  |  |
| P05 | BLG-0127 | - | P05-F20: FINDINGS.md:218; ham önem Low | AÇIK |  |  |
| P05 | BLG-0128 | - | P05-F21: FINDINGS.md:219; ham önem Info | AÇIK |  |  |
| P05 | BLG-0129 | - | P05-F22: FINDINGS.md:220; ham önem Info | AÇIK |  |  |
| P06 | BLG-0130 | src/cross_domain/external/ethereum.rs:393-406 | P06-F01: FINDINGS.md:250; ham önem High / soru PR9 yorumunda | KARAR BEKLİYOR |  |  |
| P06 | BLG-0131 | src/cross_domain/external/ethereum.rs:255-257, 299-306, 344-376, 399-425 | P06-F02: FINDINGS.md:266; ham önem High / soru PR9 yorumunda | KARAR BEKLİYOR |  |  |
| P06 | BLG-0132 | src/cross_domain/external/ethereum.rs:361-425 | P06-F03: FINDINGS.md:281; ham önem Medium / gp/P06-A:afa7834 kısmi: politika yarısı | KODLANIYOR |  |  |
| P06 | BLG-0133 | src/cross_domain/external/domain_bridge.rs:108-160 | P06-F04: FINDINGS.md:291; ham önem Medium / BAĞIMLI: spec.rs, blockchain.rs | AÇIK |  |  |
| P06 | BLG-0134 | src/cross_domain/external/zkvm_proof.rs:202-263 | P06-F05: FINDINGS.md:302; ham önem Medium / soru PR9 yorumunda | KARAR BEKLİYOR |  |  |
| P06 | BLG-0135 | src/cross_domain/evm/adapter.rs:153-172, 328-357, 221-272 | P06-F06: FINDINGS.md:312; ham önem Medium / soru PR9 yorumunda | KARAR BEKLİYOR |  |  |
| P06 | BLG-0136 | src/cross_domain/evm/adapter.rs:244-254 | P06-F07: FINDINGS.md:322; ham önem Medium / soru PR9 yorumunda | KARAR BEKLİYOR |  |  |
| P06 | BLG-0137 | src/cross_domain/evm/sync_committee.rs:20-23, 410-439, 441-446 | P06-F08: FINDINGS.md:333; ham önem Medium / soru PR9 yorumunda | KARAR BEKLİYOR |  |  |
| P06 | BLG-0138 | src/cross_domain/evm/sync_committee.rs:355-358, 430-436, 281-292 | P06-F09: FINDINGS.md:343; ham önem Low | AÇIK |  |  |
| P06 | BLG-0139 | src/cross_domain/evm/verify.rs:245-314 | P06-F10: FINDINGS.md:353; ham önem Medium | AÇIK |  |  |
| P06 | BLG-0140 | src/cross_domain/external/evm_hybrid.rs:183-202, 210-250 | P06-F11: FINDINGS.md:363; ham önem Medium | AÇIK |  |  |
| P06 | BLG-0141 | src/cross_domain/external/prover.rs | P06-F12: FINDINGS.md:373; ham önem Medium | AÇIK |  |  |
| P06 | BLG-0142 | src/cross_domain/external/prover.rs:92-111, 185-192, 227-249 | P06-F13: FINDINGS.md:383; ham önem Low | AÇIK |  |  |
| P06 | BLG-0143 | src/cross_domain/external/zkvm_proof.rs:86-108 | P06-F14: FINDINGS.md:393; ham önem Low | AÇIK |  |  |
| P06 | BLG-0144 | src/cross_domain/evm/adapter.rs:1 | P06-F15: FINDINGS.md:403; ham önem Low | AÇIK |  |  |
| P06 | BLG-0145 | src/cross_domain/evm/adapter.rs:343-357 | P06-F16: FINDINGS.md:413; ham önem Info | AÇIK |  |  |
| P07 | BLG-0146 | src/rpc/server.rs:1712-1722 | P07-F01: FINDINGS.md:432; ham önem High | AÇIK |  |  |
| P07 | BLG-0147 | src/cross_domain/external/intake.rs:64-131 | P07-F02: FINDINGS.md:443; ham önem High | AÇIK |  |  |
| P07 | BLG-0148 | src/cross_domain/external/registry.rs:452-471 | P07-F03: FINDINGS.md:454; ham önem High | AÇIK |  |  |
| P07 | BLG-0149 | src/cross_domain/external/registry.rs:51-52, 638-754 | P07-F04: FINDINGS.md:466; ham önem Medium | AÇIK |  |  |
| P07 | BLG-0150 | src/cross_domain/external/registry.rs:337-371 | P07-F05: FINDINGS.md:477; ham önem Medium | AÇIK |  |  |
| P07 | BLG-0151 | src/cross_domain/external/intake.rs:521-545 | P07-F06: FINDINGS.md:487; ham önem Medium | AÇIK |  |  |
| P07 | BLG-0152 | src/cross_domain/external/intake_quorum.rs:251-271, 357-363 | P07-F07: FINDINGS.md:497; ham önem Medium | AÇIK |  |  |
| P07 | BLG-0153 | src/cross_domain/external/intake.rs:391-403 | P07-F08: FINDINGS.md:508; ham önem Medium | AÇIK |  |  |
| P07 | BLG-0154 | src/cross_domain/external/intake.rs:240-246 | P07-F09: FINDINGS.md:518; ham önem Medium | AÇIK |  |  |
| P07 | BLG-0155 | src/cross_domain/external/registry.rs:173-293 | P07-F10: FINDINGS.md:528; ham önem Medium | AÇIK |  |  |
| P07 | BLG-0156 | src/cross_domain/external/registry.rs:764-803 | P07-F11: FINDINGS.md:538; ham önem Low | AÇIK |  |  |
| P07 | BLG-0157 | src/cross_domain/external/registry.rs | P07-F12: FINDINGS.md:548; ham önem Medium | AÇIK |  |  |
| P07 | BLG-0158 | src/cross_domain/external/intake.rs:1122-1161 | P07-F13: FINDINGS.md:558; ham önem Low | AÇIK |  |  |
| P07 | BLG-0159 | src/cross_domain/external/mod.rs:381-392, 665 | P07-F14: FINDINGS.md:568; ham önem Low | AÇIK |  |  |
| P07 | BLG-0160 | src/cross_domain/external/intake.rs:441-489 | P07-F15: FINDINGS.md:578; ham önem Low | AÇIK |  |  |
| P07 | BLG-0161 | src/cross_domain/external/versioning.rs:138-200, 244-265 | P07-F16: FINDINGS.md:588; ham önem Low | AÇIK |  |  |
| P07 | BLG-0162 | src/cross_domain/external/selftest.rs:214-222, 326-330 | P07-F17: FINDINGS.md:598; ham önem Low | AÇIK |  |  |
| P07 | BLG-0163 | src/cross_domain/external/quorum.rs:112-129, 245-265, 305-314 | P07-F18: FINDINGS.md:608; ham önem Low | AÇIK |  |  |
| P07 | BLG-0164 | src/cross_domain/external/intake_quorum.rs:357-363 | P07-F19: FINDINGS.md:618; ham önem Low | AÇIK |  |  |
| P07 | BLG-0165 | src/cross_domain/external/registry.rs:295-337 | P07-F20: FINDINGS.md:628; ham önem Info | AÇIK |  |  |
| P07 | BLG-0166 | src/cross_domain/external/mod.rs:39-60 | P07-F21: FINDINGS.md:638; ham önem Info | AÇIK |  |  |
| P08 | BLG-0167 | - | P08-F01: FINDINGS.md:707; ham önem High | AÇIK |  |  |
| P08 | BLG-0168 | - | P08-F02: FINDINGS.md:708; ham önem High | AÇIK |  |  |
| P08 | BLG-0169 | - | P08-F03: FINDINGS.md:709; ham önem High | AÇIK |  |  |
| P08 | BLG-0170 | - | P08-F04: FINDINGS.md:710; ham önem Medium | AÇIK |  |  |
| P08 | BLG-0171 | - | P08-F05: FINDINGS.md:711; ham önem Medium | AÇIK |  |  |
| P08 | BLG-0172 | - | P08-F06: FINDINGS.md:712; ham önem Medium | AÇIK |  |  |
| P08 | BLG-0173 | - | P08-F07: FINDINGS.md:713; ham önem Low | AÇIK |  |  |
| P08 | BLG-0174 | - | P08-F08: FINDINGS.md:714; ham önem Low | AÇIK |  |  |
| P08 | BLG-0175 | - | P08-F09: FINDINGS.md:715; ham önem Low | AÇIK |  |  |
| P08 | BLG-0176 | - | P08-F10: FINDINGS.md:716; ham önem Low | AÇIK |  |  |
| P08 | BLG-0177 | - | P08-F11: FINDINGS.md:717; ham önem Low | AÇIK |  |  |
| P08 | BLG-0178 | - | P08-F12: FINDINGS.md:718; ham önem Info | AÇIK |  |  |
| P08 | BLG-0179 | - | P08-F13: FINDINGS.md:719; ham önem Info | AÇIK |  |  |
| P09 | BLG-0180 | - | P09-F01: FINDINGS.md:725; ham önem High | AÇIK |  |  |
| P09 | BLG-0181 | - | P09-F02: FINDINGS.md:726; ham önem Medium | AÇIK |  |  |
| P09 | BLG-0182 | - | P09-F03: FINDINGS.md:727; ham önem Medium | AÇIK |  |  |
| P09 | BLG-0183 | - | P09-F04: FINDINGS.md:728; ham önem Medium | AÇIK |  |  |
| P09 | BLG-0184 | - | P09-F05: FINDINGS.md:729; ham önem Medium | AÇIK |  |  |
| P09 | BLG-0185 | - | P09-F06: FINDINGS.md:730; ham önem Low | AÇIK |  |  |
| P09 | BLG-0186 | - | P09-F07: FINDINGS.md:731; ham önem Low | AÇIK |  |  |
| P09 | BLG-0187 | - | P09-F08: FINDINGS.md:732; ham önem Low | AÇIK |  |  |
| P09 | BLG-0188 | - | P09-F09: FINDINGS.md:733; ham önem Low | AÇIK |  |  |
| P09 | BLG-0189 | - | P09-F10: FINDINGS.md:734; ham önem Low | AÇIK |  |  |
| P09 | BLG-0190 | - | P09-F11: FINDINGS.md:735; ham önem Low | AÇIK |  |  |
| P10 | BLG-0191 | - | P10-F01: FINDINGS.md:762; ham önem High | AÇIK |  |  |
| P10 | BLG-0192 | ai/registry.rs:1921-1923, 1952-1953 | P10-F02: FINDINGS.md:763; ham önem High | AÇIK |  |  |
| P10 | BLG-0193 | - | P10-F03: FINDINGS.md:764; ham önem High | AÇIK |  |  |
| P10 | BLG-0194 | - | P10-F04: FINDINGS.md:765; ham önem Medium | AÇIK |  |  |
| P10 | BLG-0195 | - | P10-F05: FINDINGS.md:766; ham önem Low | AÇIK |  |  |
| P10 | BLG-0196 | - | P10-F06: FINDINGS.md:767; ham önem Low | AÇIK |  |  |
| P10 | BLG-0197 | - | P10-F07: FINDINGS.md:768; ham önem Low | AÇIK |  |  |
| P10 | BLG-0198 | - | P10-F08: FINDINGS.md:769; ham önem Low | AÇIK |  |  |
| P10 | BLG-0199 | - | P10-F09: FINDINGS.md:770; ham önem Low | AÇIK |  |  |
| P10 | BLG-0200 | - | P10-F10: FINDINGS.md:771; ham önem Low | AÇIK |  |  |
| P10 | BLG-0201 | - | P10-F11: FINDINGS.md:772; ham önem Info | AÇIK |  |  |
| P11 | BLG-0202 | - | P11-F01: FINDINGS.md:790; ham önem Critical; CONFIRMED | AÇIK |  |  |
| P11 | BLG-0203 | - | P11-F02: FINDINGS.md:791; ham önem Medium | AÇIK |  |  |
| P11 | BLG-0204 | - | P11-F03: FINDINGS.md:792; ham önem Medium | AÇIK |  |  |
| P11 | BLG-0205 | - | P11-F04: FINDINGS.md:793; ham önem Medium | AÇIK |  |  |
| P11 | BLG-0206 | - | P11-F05: FINDINGS.md:794; ham önem Low | AÇIK |  |  |
| P11 | BLG-0207 | - | P11-F06: FINDINGS.md:795; ham önem Low | AÇIK |  |  |
| P11 | BLG-0208 | - | P11-F07: FINDINGS.md:796; ham önem Low | AÇIK |  |  |
| P11 | BLG-0209 | - | P11-F08: FINDINGS.md:797; ham önem Low | AÇIK |  |  |
| P11 | BLG-0210 | - | P11-F09: FINDINGS.md:798; ham önem Low | AÇIK |  |  |
| P11 | BLG-0211 | - | P11-F10: FINDINGS.md:799; ham önem Low | AÇIK |  |  |
| P11 | BLG-0212 | gateway/service.rs:272 | P11-F11: FINDINGS.md:800; ham önem Low | AÇIK |  |  |
| P11 | BLG-0213 | - | P11-F12: FINDINGS.md:801; ham önem Low | AÇIK |  |  |
| P12 | BLG-0214 | - | P12-F01: FINDINGS.md:807; ham önem High | AÇIK |  |  |
| P12 | BLG-0215 | - | P12-F02: FINDINGS.md:808; ham önem Medium | AÇIK |  |  |
| P12 | BLG-0216 | - | P12-F03: FINDINGS.md:809; ham önem Medium | AÇIK |  |  |
| P12 | BLG-0217 | - | P12-F04: FINDINGS.md:810; ham önem Low | AÇIK |  |  |
| P12 | BLG-0218 | - | P12-F05: FINDINGS.md:811; ham önem Low | AÇIK |  |  |
| P12 | BLG-0219 | - | P12-F06: FINDINGS.md:812; ham önem Low | AÇIK |  |  |
| P12 | BLG-0220 | - | P12-F07: FINDINGS.md:813; ham önem Low | AÇIK |  |  |
| P13 | BLG-0221 | - | P13-F01: FINDINGS.md:875; ham önem High | AÇIK |  |  |
| P13 | BLG-0222 | - | P13-F02: FINDINGS.md:876; ham önem Medium | AÇIK |  |  |
| P13 | BLG-0223 | - | P13-F03: FINDINGS.md:877; ham önem Medium | AÇIK |  |  |
| P13 | BLG-0224 | - | P13-F04: FINDINGS.md:878; ham önem Medium | AÇIK |  |  |
| P13 | BLG-0225 | tests/fiat_shamir_binding.rs:170-260 | P13-F05: FINDINGS.md:879; ham önem Medium | AÇIK |  |  |
| P13 | BLG-0226 | - | P13-F06: FINDINGS.md:880; ham önem Medium | AÇIK |  |  |
| P13 | BLG-0227 | tests/soundness_negatives.rs:138-165 | P13-F07: FINDINGS.md:881; ham önem Low | AÇIK |  |  |
| P13 | BLG-0228 | tests/compiled_branching.rs:49-93 | P13-F08: FINDINGS.md:882; ham önem Low | AÇIK |  |  |
| P13 | BLG-0229 | - | P13-F09: FINDINGS.md:883; ham önem Low | AÇIK |  |  |
| P13 | BLG-0230 | - | P13-F10: FINDINGS.md:884; ham önem Info | AÇIK |  |  |
| P14 | BLG-0231 | - | P14-F01: FINDINGS.md:862; ham önem Medium | AÇIK |  |  |
| P14 | BLG-0232 | - | P14-F02: FINDINGS.md:863; ham önem Medium | AÇIK |  |  |
| P14 | BLG-0233 | - | P14-F03: FINDINGS.md:864; ham önem Low | AÇIK |  |  |
| P14 | BLG-0234 | - | P14-F04: FINDINGS.md:865; ham önem Low | AÇIK |  |  |
| P14 | BLG-0235 | bud_stark/prover.rs:44-47 | P14-F05: FINDINGS.md:866; ham önem Low | AÇIK |  |  |
| P14 | BLG-0236 | - | P14-F06: FINDINGS.md:867; ham önem Low | AÇIK |  |  |
| P14 | BLG-0237 | - | P14-F07: FINDINGS.md:868; ham önem Info | AÇIK |  |  |
| P14 | BLG-0238 | benches/canonical_programs.rs:59-155 | P14-F08: FINDINGS.md:869; ham önem Low | AÇIK |  |  |
| P15 | BLG-0239 | - | P15-F01: FINDINGS.md:837; ham önem Critical | AÇIK |  |  |
| P15 | BLG-0240 | bud-proof/Cargo.toml:30 | P15-F02: FINDINGS.md:838; ham önem Low | AÇIK |  |  |
| P15 | BLG-0241 | - | P15-F03: FINDINGS.md:839; ham önem Medium | AÇIK |  |  |
| P15 | BLG-0242 | - | P15-F04: FINDINGS.md:840; ham önem Low | AÇIK |  |  |
| P15 | BLG-0243 | - | P15-F05: FINDINGS.md:841; ham önem Low | AÇIK |  |  |
| P16 | BLG-0244 | bud-vm/src/lib.rs:538-547, 363-367, 453-487 | P16-F01: FINDINGS.md:891; ham önem Medium | AÇIK |  |  |
| P16 | BLG-0245 | bud-vm/src/lib.rs:1034-1046, 581-615 | P16-F02: FINDINGS.md:892; ham önem Medium | AÇIK |  |  |
| P16 | BLG-0246 | - | P16-F03: FINDINGS.md:893; ham önem Low | AÇIK |  |  |
| P16 | BLG-0247 | - | P16-F04: FINDINGS.md:894; ham önem Low | AÇIK |  |  |
| P16 | BLG-0248 | - | P16-F05: FINDINGS.md:895; ham önem Low | AÇIK |  |  |
| P16 | BLG-0249 | bud-isa/src/lib.rs:215-268 | P16-F06: FINDINGS.md:896; ham önem Info | AÇIK |  |  |
| P16 | BLG-0250 | bud-isa/src/lib.rs:59-61, 271-309 | P16-F07: FINDINGS.md:897; ham önem Low | AÇIK |  |  |
| P16 | BLG-0251 | verifier-registry/src/evidence.rs:140-216, 64-71 | P16-F08: FINDINGS.md:898; ham önem Medium | AÇIK |  |  |
| P16 | BLG-0252 | - | P16-F09: FINDINGS.md:899; ham önem Low | AÇIK |  |  |
| P16 | BLG-0253 | src/registry/permissionless.rs | P16-F10: FINDINGS.md:900; ham önem Medium | AÇIK |  |  |
| P16 | BLG-0254 | src/core/account.rs:672-771 | P16-F11: FINDINGS.md:901; ham önem Medium | AÇIK |  |  |
| P16 | BLG-0255 | - | P16-F12: FINDINGS.md:902; ham önem Low | AÇIK |  |  |
| P16 | BLG-0256 | - | P16-F13: FINDINGS.md:903; ham önem Low | AÇIK |  |  |
| P16 | BLG-0257 | - | P16-F14: FINDINGS.md:904; ham önem Low | AÇIK |  |  |
| P16 | BLG-0258 | - | P16-F15: FINDINGS.md:905; ham önem Low | AÇIK |  |  |
| P16 | BLG-0259 | bud-state/src/lib.rs:242-254 | P16-F16: FINDINGS.md:906; ham önem Low | AÇIK |  |  |
| P16 | BLG-0260 | bud-state/src/lib.rs:158-193 | P16-F17: FINDINGS.md:907; ham önem Low | AÇIK |  |  |
| P16 | BLG-0261 | bud-state/src/note.rs:85-112 | P16-F18: FINDINGS.md:908; ham önem Low | AÇIK |  |  |
| P16 | BLG-0262 | - | P16-F19: FINDINGS.md:909; ham önem Low | AÇIK |  |  |
| P16 | BLG-0263 | - | P16-F20: FINDINGS.md:910; ham önem Low | AÇIK |  |  |
| P16 | BLG-0264 | - | P16-F21: FINDINGS.md:911; ham önem Low | AÇIK |  |  |
| P16 | BLG-0265 | - | P16-F22: FINDINGS.md:912; ham önem Info | AÇIK |  |  |
| P16 | BLG-0266 | bud-isa/src/lib.rs:595-616 | P16-F23: FINDINGS.md:913; ham önem Info | AÇIK |  |  |
| P17 | BLG-0267 | wallet-core/src/privacy_crypto.rs:81-122 | P17-F01: FINDINGS.md:949; ham önem High | AÇIK |  |  |
| P17 | BLG-0268 | account_abstraction/registry.rs:128-149, 225-261 | P17-F02: FINDINGS.md:950; ham önem Medium | AÇIK |  |  |
| P17 | BLG-0269 | - | P17-F03: FINDINGS.md:951; ham önem Medium | AÇIK |  |  |
| P17 | BLG-0270 | - | P17-F04: FINDINGS.md:952; ham önem Medium | AÇIK |  |  |
| P17 | BLG-0271 | - | P17-F05: FINDINGS.md:953; ham önem Medium | AÇIK |  |  |
| P17 | BLG-0272 | - | P17-F06: FINDINGS.md:954; ham önem Medium | AÇIK |  |  |
| P17 | BLG-0273 | - | P17-F07: FINDINGS.md:955; ham önem Medium | AÇIK |  |  |
| P17 | BLG-0274 | - | P17-F08: FINDINGS.md:956; ham önem Medium | AÇIK |  |  |
| P17 | BLG-0275 | - | P17-F09: FINDINGS.md:957; ham önem Medium | AÇIK |  |  |
| P17 | BLG-0276 | - | P17-F10: FINDINGS.md:958; ham önem Medium | AÇIK |  |  |
| P17 | BLG-0277 | settlement/pq_anchor.rs:14 | P17-F11: FINDINGS.md:959; ham önem Medium | AÇIK |  |  |
| P17 | BLG-0278 | - | P17-F12: FINDINGS.md:960; ham önem Low | AÇIK |  |  |
| P17 | BLG-0279 | - | P17-F13: FINDINGS.md:960; ham önem Low | AÇIK |  |  |
| P17 | BLG-0280 | - | P17-F14: FINDINGS.md:960; ham önem Low | AÇIK |  |  |
| P17 | BLG-0281 | - | P17-F15: FINDINGS.md:960; ham önem Low | AÇIK |  |  |
| P17 | BLG-0282 | - | P17-F16: FINDINGS.md:960; ham önem Low | AÇIK |  |  |
| P17 | BLG-0283 | - | P17-F17: FINDINGS.md:960; ham önem Low | AÇIK |  |  |
| P17 | BLG-0284 | note-packing/src/lib.rs:87-107 | P17-F18: FINDINGS.md:960; ham önem Low | AÇIK |  |  |
| P17 | BLG-0285 | - | P17-F19: FINDINGS.md:960; ham önem Low | AÇIK |  |  |
| P18 | BLG-0286 | - | P18-F01: FINDINGS.md:966; ham önem High | AÇIK |  |  |
| P18 | BLG-0287 | prover/mod.rs:197-199 | P18-F02: FINDINGS.md:967; ham önem High | AÇIK |  |  |
| P18 | BLG-0288 | - | P18-F03: FINDINGS.md:968; ham önem High | AÇIK |  |  |
| P18 | BLG-0289 | ai/registry.rs:878 | P18-F04: FINDINGS.md:969; ham önem Medium | AÇIK |  |  |
| P18 | BLG-0290 | - | P18-F05: FINDINGS.md:970; ham önem Medium | AÇIK |  |  |
| P18 | BLG-0291 | mempool/pool.rs:173-185 | P18-F06: FINDINGS.md:971; ham önem Medium | AÇIK |  |  |
| P18 | BLG-0292 | light_client/mod.rs:223-270, 285-356, 431-450 | P18-F07: FINDINGS.md:972; ham önem Medium | AÇIK |  |  |
| P18 | BLG-0293 | light_client/mod.rs:459-470 | P18-F08: FINDINGS.md:973; ham önem Low | AÇIK |  |  |
| P18 | BLG-0294 | prover/mod.rs:96-110, 203-344 | P18-F09: FINDINGS.md:973; ham önem Low | AÇIK |  |  |
| P18 | BLG-0295 | prover/mod.rs:157-158 | P18-F10: FINDINGS.md:973; ham önem Low | AÇIK |  |  |
| P18 | BLG-0296 | - | P18-F11: FINDINGS.md:973; ham önem Low | AÇIK |  |  |
| P18 | BLG-0297 | - | P18-F12: FINDINGS.md:973; ham önem Low | AÇIK |  |  |
| P18 | BLG-0298 | - | P18-F13: FINDINGS.md:973; ham önem Low | AÇIK |  |  |
| P18 | BLG-0299 | - | P18-F14: FINDINGS.md:973; ham önem Low | AÇIK |  |  |
| P19 | BLG-0300 | - | P19-F01: FINDINGS.md:1007; ham önem High | AÇIK |  |  |
| P19 | BLG-0301 | - | P19-F02: FINDINGS.md:1008; ham önem Medium | AÇIK |  |  |
| P19 | BLG-0302 | tokenomics/mod.rs:385 | P19-F03: FINDINGS.md:1009; ham önem Medium | AÇIK |  |  |
| P19 | BLG-0303 | - | P19-F04: FINDINGS.md:1010; ham önem Low | AÇIK |  |  |
| P19 | BLG-0304 | - | P19-F05: FINDINGS.md:1010; ham önem Low | AÇIK |  |  |
| P19 | BLG-0305 | - | P19-F06: FINDINGS.md:1010; ham önem Low | AÇIK |  |  |
| P19 | BLG-0306 | - | P19-F07: FINDINGS.md:1010; ham önem Low | AÇIK |  |  |
| P19 | BLG-0307 | - | P19-F08: FINDINGS.md:1010; ham önem Low | AÇIK |  |  |
| P19 | BLG-0308 | - | P19-F09: FINDINGS.md:1010; ham önem Low | AÇIK |  |  |
| P19 | BLG-0309 | - | P19-F10: FINDINGS.md:1010; ham önem Low | AÇIK |  |  |
| P19 | BLG-0310 | - | P19-F11: FINDINGS.md:1010; ham önem Low | AÇIK |  |  |
| P19 | BLG-0311 | - | P19-F12: FINDINGS.md:1010; ham önem Low | AÇIK |  |  |
| P20 | BLG-0312 | - | P20-F01: FINDINGS.md:1016; ham önem Medium | AÇIK |  |  |
| P20 | BLG-0313 | - | P20-F02: FINDINGS.md:1017; ham önem Medium | AÇIK |  |  |
| P20 | BLG-0314 | sharding/mod.rs | P20-F03: FINDINGS.md:1018; ham önem Medium | AÇIK |  |  |
| P20 | BLG-0315 | - | P20-F05: FINDINGS.md:1019; ham önem Medium | AÇIK |  |  |
| P20 | BLG-0316 | - | P20-F06: FINDINGS.md:1020; ham önem Medium | AÇIK |  |  |
| P20 | BLG-0317 | - | P20-F07: FINDINGS.md:1021; ham önem Low | AÇIK |  |  |
| P20 | BLG-0318 | - | P20-F08: FINDINGS.md:1021; ham önem Low | AÇIK |  |  |
| P20 | BLG-0319 | - | P20-F09: FINDINGS.md:1021; ham önem Low | AÇIK |  |  |
| P20 | BLG-0320 | sharding/mod.rs:154-158, 184-188 | P20-F10: FINDINGS.md:1021; ham önem Low | AÇIK |  |  |
| P20 | BLG-0321 | - | P20-F11: FINDINGS.md:1021; ham önem Low | AÇIK |  |  |
| P20 | BLG-0322 | - | P20-F12: FINDINGS.md:1021; ham önem Low | AÇIK |  |  |
| P20 | BLG-0323 | core/encoding.rs | P20-F13: FINDINGS.md:1021; ham önem Low | AÇIK |  |  |
| P21 | BLG-0324 | cli/commands.rs:388 | P21-F01: FINDINGS.md:1048; ham önem Medium | AÇIK |  |  |
| P21 | BLG-0325 | - | P21-F02: FINDINGS.md:1049; ham önem Medium | AÇIK |  |  |
| P21 | BLG-0326 | - | P21-F03: FINDINGS.md:1050; ham önem Medium | AÇIK |  |  |
| P21 | BLG-0327 | - | P21-F04: FINDINGS.md:1051; ham önem Medium | AÇIK |  |  |
| P21 | BLG-0328 | - | P21-F05: FINDINGS.md:1052; ham önem Low | AÇIK |  |  |
| P21 | BLG-0329 | - | P21-F06: FINDINGS.md:1052; ham önem Low | AÇIK |  |  |
| P21 | BLG-0330 | - | P21-F07: FINDINGS.md:1052; ham önem Low | AÇIK |  |  |
| P21 | BLG-0331 | - | P21-F08: FINDINGS.md:1052; ham önem Low | AÇIK |  |  |
| P22 | BLG-0332 | cli/commands.rs:116-123, 246-250 | P22-F01: FINDINGS.md:1038; ham önem Medium | AÇIK |  |  |
| P22 | BLG-0333 | cli/commands.rs:1180-1290 | P22-F02: FINDINGS.md:1039; ham önem Medium | AÇIK |  |  |
| P22 | BLG-0334 | - | P22-F03: FINDINGS.md:1040; ham önem Medium | AÇIK |  |  |
| P22 | BLG-0335 | rpc/tests.rs | P22-F04: FINDINGS.md:1041; ham önem Medium | AÇIK |  |  |
| P22 | BLG-0336 | rpc/server.rs:3273 | P22-F05: FINDINGS.md:1042; ham önem Low | AÇIK |  |  |
| P22 | BLG-0337 | cli/commands.rs:605 | P22-F06: FINDINGS.md:1042; ham önem Low | AÇIK |  |  |
| P22 | BLG-0338 | - | P22-F07: FINDINGS.md:1042; ham önem Low | AÇIK |  |  |
| P22 | BLG-0339 | - | P22-F08: FINDINGS.md:1042; ham önem Low | AÇIK |  |  |
| P22 | BLG-0340 | - | P22-F09: FINDINGS.md:1042; ham önem Low | AÇIK |  |  |
| P22 | BLG-0341 | - | P22-F10: FINDINGS.md:1042; ham önem Low | AÇIK |  |  |
| P22 | BLG-0342 | - | P22-F11: FINDINGS.md:1042; ham önem Low | AÇIK |  |  |
| P23 | BLG-0343 | - | P23-F01: FINDINGS.md:1058; ham önem High | AÇIK |  |  |
| P23 | BLG-0344 | - | P23-F02: FINDINGS.md:1059; ham önem Medium | AÇIK |  |  |
| P23 | BLG-0345 | - | P23-F03: FINDINGS.md:1060; ham önem Medium | AÇIK |  |  |
| P23 | BLG-0346 | - | P23-F04: FINDINGS.md:1061; ham önem Medium | AÇIK |  |  |
| P23 | BLG-0347 | - | P23-F05: FINDINGS.md:1062; ham önem Medium | AÇIK |  |  |
| P23 | BLG-0348 | - | P23-F06: FINDINGS.md:1063; ham önem Medium | AÇIK |  |  |
| P23 | BLG-0349 | - | P23-F07: FINDINGS.md:1064; ham önem Medium | AÇIK |  |  |
| P23 | BLG-0350 | - | P23-F08: FINDINGS.md:1065; ham önem Medium | AÇIK |  |  |
| P23 | BLG-0351 | - | P23-F09: FINDINGS.md:1066; ham önem Medium | AÇIK |  |  |
| P23 | BLG-0352 | - | P23-F10: FINDINGS.md:1067; ham önem Medium | AÇIK |  |  |
| P23 | BLG-0353 | - | P23-F11: FINDINGS.md:1068; ham önem Low | AÇIK |  |  |
| P23 | BLG-0354 | - | P23-F12: FINDINGS.md:1068; ham önem Low | AÇIK |  |  |
| P23 | BLG-0355 | - | P23-F13: FINDINGS.md:1068; ham önem Low | AÇIK |  |  |
| P23 | BLG-0356 | storage/db.rs:1136 | P23-F14: FINDINGS.md:1068; ham önem Low | AÇIK |  |  |
| P23 | BLG-0357 | - | P23-F15: FINDINGS.md:1068; ham önem Low | AÇIK |  |  |
| P23 | BLG-0358 | - | P23-F16: FINDINGS.md:1068; ham önem Low | AÇIK |  |  |
| P23 | BLG-0359 | - | P23-F17: FINDINGS.md:1068; ham önem Low | AÇIK |  |  |
| P24 | BLG-0360 | - | P24-F01: FINDINGS.md:1074; ham önem Medium | AÇIK |  |  |
| P24 | BLG-0361 | - | P24-F02: FINDINGS.md:1075; ham önem Medium | AÇIK |  |  |
| P24 | BLG-0362 | domain/mod.rs:32 | P24-F03: FINDINGS.md:1076; ham önem Medium | AÇIK |  |  |
| P24 | BLG-0363 | - | P24-F04: FINDINGS.md:1077; ham önem Medium | AÇIK |  |  |
| P24 | BLG-0364 | core/account.rs:2462 | P24-F05: FINDINGS.md:1078; ham önem Medium | AÇIK |  |  |
| P24 | BLG-0365 | - | P24-F06: FINDINGS.md:1079; ham önem Low | AÇIK |  |  |
| P24 | BLG-0366 | - | P24-F07: FINDINGS.md:1079; ham önem Low | AÇIK |  |  |
| P24 | BLG-0367 | - | P24-F08: FINDINGS.md:1079; ham önem Low | AÇIK |  |  |
| P25 | BLG-0368 | - | P25-F01: FINDINGS.md:1085; ham önem High | AÇIK |  |  |
| P25 | BLG-0369 | - | P25-F02: FINDINGS.md:1086; ham önem Medium | AÇIK |  |  |
| P25 | BLG-0370 | - | P25-F03: FINDINGS.md:1087; ham önem Medium | AÇIK |  |  |
| P25 | BLG-0371 | rpc/server.rs:2568-2575 | P25-F04: FINDINGS.md:1088; ham önem Medium | AÇIK |  |  |
| P25 | BLG-0372 | - | P25-F05: FINDINGS.md:1089; ham önem Medium | AÇIK |  |  |
| P25 | BLG-0373 | - | P25-F06: FINDINGS.md:1090; ham önem Medium | AÇIK |  |  |
| P25 | BLG-0374 | - | P25-F07: FINDINGS.md:1091; ham önem Low | AÇIK |  |  |
| P25 | BLG-0375 | - | P25-F08: FINDINGS.md:1091; ham önem Low | AÇIK |  |  |
| P25 | BLG-0376 | - | P25-F09: FINDINGS.md:1091; ham önem Low | AÇIK |  |  |
| P25 | BLG-0377 | - | P25-F10: FINDINGS.md:1091; ham önem Low | AÇIK |  |  |
| P25 | BLG-0378 | - | P25-F11: FINDINGS.md:1091; ham önem Low | AÇIK |  |  |
| P26 | BLG-0379 | - | P26-F01: FINDINGS.md:1097; ham önem Medium | AÇIK |  |  |
| P26 | BLG-0380 | - | P26-F02: FINDINGS.md:1098; ham önem Medium | AÇIK |  |  |
| P26 | BLG-0381 | - | P26-F03: FINDINGS.md:1099; ham önem Medium | AÇIK |  |  |
| P26 | BLG-0382 | - | P26-F04: FINDINGS.md:1100; ham önem Medium | AÇIK |  |  |
| P26 | BLG-0383 | - | P26-F05: FINDINGS.md:1101; ham önem Medium | AÇIK |  |  |
| P26 | BLG-0384 | - | P26-F06: FINDINGS.md:1102; ham önem Low | AÇIK |  |  |
| P26 | BLG-0385 | - | P26-F07: FINDINGS.md:1102; ham önem Low | AÇIK |  |  |
| P26 | BLG-0386 | - | P26-F08: FINDINGS.md:1102; ham önem Low | AÇIK |  |  |
| P26 | BLG-0387 | - | P26-F09: FINDINGS.md:1102; ham önem Low | AÇIK |  |  |
| P26 | BLG-0388 | - | P26-F10: FINDINGS.md:1102; ham önem Low | AÇIK |  |  |
| P26 | BLG-0389 | gateway/service.rs:39 | P26-F11: FINDINGS.md:1102; ham önem Low | AÇIK |  |  |
| P26 | BLG-0390 | - | P26-F12: FINDINGS.md:1102; ham önem Low | AÇIK |  |  |
| P26 | BLG-0391 | - | P26-F13: FINDINGS.md:1102; ham önem Low | AÇIK |  |  |
| P27 | BLG-0392 | src/storage/living_threshold.rs:293-306, 346-372 | P27-F01: notes/P27-scan.md:36; ham önem Medium | AÇIK |  |  |
| P27 | BLG-0393 | src/storage/living_threshold.rs:346-357 | P27-F02: notes/P27-scan.md:55; ham önem Low | AÇIK |  |  |
| P27 | BLG-0394 | src/storage/dictionary.rs:242-271, 348-394 | P27-F03: notes/P27-scan.md:70; ham önem Medium | AÇIK |  |  |
| P27 | BLG-0395 | src/storage/erasure.rs:783-866 | P27-F04: notes/P27-scan.md:87; ham önem Medium | AÇIK |  |  |
| P27 | BLG-0396 | src/storage/lrc.rs | P27-F05: notes/P27-scan.md:100; ham önem Medium | AÇIK |  |  |
| P27 | BLG-0397 | src/storage/erasure.rs:783-866 | P27-F06: notes/P27-scan.md:118; ham önem Low | AÇIK |  |  |
| P27 | BLG-0398 | src/storage/erasure.rs:818-866 | P27-F07: notes/P27-scan.md:132; ham önem Low | AÇIK |  |  |
| P27 | BLG-0399 | src/storage/payload_crypt.rs:40-48, 146-167 | P27-F08: notes/P27-scan.md:149; ham önem Low | AÇIK |  |  |
| P27 | BLG-0400 | src/storage/pact_binding.rs:38, 85, 107 | P27-F09: notes/P27-scan.md:164; ham önem Low | AÇIK |  |  |
| P27 | BLG-0401 | src/storage/pact_binding.rs:20-30, 115-118 | P27-F10: notes/P27-scan.md:179; ham önem Low | AÇIK |  |  |
| P27 | BLG-0402 | src/storage/dictionary.rs:217-221, 242-271, 364-394 | P27-F11: notes/P27-scan.md:189; ham önem Low | AÇIK |  |  |
| P27 | BLG-0403 | src/storage/mobile_self.rs:59-76, 158-181, 275-283, 335-381 | P27-F12: notes/P27-scan.md:203; ham önem Low | AÇIK |  |  |
| P27 | BLG-0404 | src/storage/server_admission.rs:58-75 | P27-F13: notes/P27-scan.md:220; ham önem Low | AÇIK |  |  |
| P27 | BLG-0405 | src/domain/storage_deal.rs:2101-2108 | P27-F14: notes/P27-scan.md:234; ham önem Low | AÇIK |  |  |
| P27 | BLG-0406 | src/storage/msr.rs:50-56, 71-101, 112-131 | P27-F15: notes/P27-scan.md:249; ham önem Info | AÇIK |  |  |
| P27 | BLG-0407 | src/storage/erasure.rs:683-696, 744-769 | P27-F16: notes/P27-scan.md:264; ham önem Info | AÇIK |  |  |
| P28 | BLG-0408 | src/storage/emit.rs:1395-1429 | P28-F01: notes/P28-scan.md:32; ham önem Medium | AÇIK |  |  |
| P28 | BLG-0409 | src/storage/emit.rs:803, 843-892, 1184-1186 | P28-F02: notes/P28-scan.md:48; ham önem Medium | AÇIK |  |  |
| P28 | BLG-0410 | src/storage/emit.rs:1220-1245 | P28-F03: notes/P28-scan.md:63; ham önem Medium | AÇIK |  |  |
| P28 | BLG-0411 | src/storage/emit.rs:786-1382, 1395-1429 | P28-F04: notes/P28-scan.md:77; ham önem Medium | AÇIK |  |  |
| P28 | BLG-0412 | src/storage/emit.rs:96-110 | P28-F05: notes/P28-scan.md:88; ham önem Low | AÇIK |  |  |
| P28 | BLG-0413 | src/storage/emit.rs:1417-1424 | P28-F06: notes/P28-scan.md:100; ham önem Low | AÇIK |  |  |
| P28 | BLG-0414 | src/storage/emit.rs:819, 1343 | P28-F07: notes/P28-scan.md:112; ham önem Low | AÇIK |  |  |
| P28 | BLG-0415 | src/storage/generated.rs:303-315, 327-343 | P28-F08: notes/P28-scan.md:122; ham önem Medium | AÇIK |  |  |
| P28 | BLG-0416 | src/storage/derived.rs:130-132, 633-810 | P28-F09: notes/P28-scan.md:133; ham önem Medium | AÇIK |  |  |
| P28 | BLG-0417 | src/storage/derived.rs:445-496, 524-529, 622, 744-752, 729-741 | P28-F10: notes/P28-scan.md:145; ham önem Low | AÇIK |  |  |
| P28 | BLG-0418 | src/storage/render.rs:296-363 | P28-F11: notes/P28-scan.md:159; ham önem Medium | AÇIK |  |  |
| P28 | BLG-0419 | src/storage/render.rs:376 | P28-F12: notes/P28-scan.md:172; ham önem Low | AÇIK |  |  |
| P28 | BLG-0420 | src/storage/render.rs:296-337, 351-359, 388-394 | P28-F13: notes/P28-scan.md:183; ham önem Low | AÇIK |  |  |
| P28 | BLG-0421 | src/storage/render.rs:456-511, 550-584, 615-625 | P28-F14: notes/P28-scan.md:193; ham önem Low | AÇIK |  |  |
| P28 | BLG-0422 | src/chain/chain_actor.rs:4289-4330 | P28-F15: notes/P28-scan.md:203; ham önem Medium | AÇIK |  |  |
| P28 | BLG-0423 | src/storage/view_grant.rs:229-276, 254-263 | P28-F16: notes/P28-scan.md:213; ham önem Medium | AÇIK |  |  |
| P28 | BLG-0424 | src/storage/view_grant.rs:404-412, 444-472 | P28-F17: notes/P28-scan.md:224; ham önem Low | AÇIK |  |  |
| P28 | BLG-0425 | src/storage/view_grant.rs:477-495, 578-601 | P28-F18: notes/P28-scan.md:234; ham önem Low | AÇIK |  |  |
| P28 | BLG-0426 | src/storage/view_grant.rs:324-347, 686-702, 962-974 | P28-F19: notes/P28-scan.md:244; ham önem Info | AÇIK |  |  |
| P29 | BLG-0427 | src/storage/qr_receive.rs:185-203 | P29-F01: notes/P29-scan.md:38; ham önem Medium | AÇIK |  |  |
| P29 | BLG-0428 | src/storage/qr_frame.rs:21-27, 188-198 | P29-F02: notes/P29-scan.md:58; ham önem Medium | AÇIK |  |  |
| P29 | BLG-0429 | src/storage/qr_video.rs:56-61, 160-197, 261-363 | P29-F03: notes/P29-scan.md:78; ham önem Low | AÇIK |  |  |
| P29 | BLG-0430 | src/storage/three_pipe.rs:185-198 | P29-F04: notes/P29-scan.md:98; ham önem Medium | AÇIK |  |  |
| P29 | BLG-0431 | src/storage/qr_video.rs:179-197, 201-203 | P29-F05: notes/P29-scan.md:117; ham önem Low | AÇIK |  |  |
| P29 | BLG-0432 | src/storage/qr_carousel.rs:244-308, 558-610 | P29-F06: notes/P29-scan.md:136; ham önem Low | AÇIK |  |  |
| P29 | BLG-0433 | src/storage/qr_encode.rs:577-592, 679-694 | P29-F07: notes/P29-scan.md:153; ham önem Low | AÇIK |  |  |
| P29 | BLG-0434 | src/storage/qr_reemit.rs:170-185 | P29-F08: notes/P29-scan.md:171; ham önem Low | AÇIK |  |  |
| P29 | BLG-0435 | src/storage/qr_recipe.rs:16-43, 108-129, 164-166 | P29-F09: notes/P29-scan.md:188; ham önem Low | AÇIK |  |  |
| P29 | BLG-0436 | src/storage/qr_reemit.rs:149-168 | P29-F10: notes/P29-scan.md:206; ham önem Low | AÇIK |  |  |
| P29 | BLG-0437 | src/storage/qr_codec.rs:26-53, 128-184 | P29-F11: notes/P29-scan.md:224; ham önem Low | AÇIK |  |  |
| P29 | BLG-0438 | src/storage/qr_carousel.rs:10-16, 896-897, 655-661 | P29-F12: notes/P29-scan.md:240; ham önem Info | AÇIK |  |  |
| P29 | BLG-0439 | src/storage/qr_*.rs (all 13 files) | P29-F13: notes/P29-scan.md:258; ham önem Info | AÇIK |  |  |
| P30 | BLG-0440 | src/chain/chain_actor.rs:4289-4327 | P30-F01: notes/P30-scan.md:46; ham önem High | AÇIK |  |  |
| P30 | BLG-0441 | src/storage/three_recipe.rs:219-241, 245-254 | P30-F02: notes/P30-scan.md:66; ham önem Medium | AÇIK |  |  |
| P30 | BLG-0442 | src/rpc/server.rs:3600-3664 | P30-F03: notes/P30-scan.md:84; ham önem Medium | AÇIK |  |  |
| P30 | BLG-0443 | src/rpc/server.rs:3665-3691 | P30-F04: notes/P30-scan.md:101; ham önem Medium | AÇIK |  |  |
| P30 | BLG-0444 | src/rpc/server.rs:3597-3663, 3699-3700 | P30-F05: notes/P30-scan.md:118; ham önem Medium | AÇIK |  |  |
| P30 | BLG-0445 | src/storage/reveal_gateway.rs:290-327 | P30-F06: notes/P30-scan.md:134; ham önem Medium | AÇIK |  |  |
| P30 | BLG-0446 | src/storage/emit.rs:1247-1256, 1283-1284 | P30-F07: notes/P30-scan.md:149; ham önem Low | AÇIK |  |  |
| P30 | BLG-0447 | src/storage/three_pipe.rs:176-184 | P30-F08: notes/P30-scan.md:165; ham önem Low | AÇIK |  |  |
| P30 | BLG-0448 | src/storage/three_gate.rs:41-58, 77-86 | P30-F09: notes/P30-scan.md:181; ham önem Low | AÇIK |  |  |
| P30 | BLG-0449 | src/storage/three_recipe.rs:221, 250 | P30-F10: notes/P30-scan.md:197; ham önem Low | AÇIK |  |  |
| P30 | BLG-0450 | src/storage/three_pipe.rs:297-302 | P30-F11: notes/P30-scan.md:212; ham önem Low | AÇIK |  |  |
| P30 | BLG-0451 | src/storage/three_gate.rs:121-157 | P30-F12: notes/P30-scan.md:229; ham önem Low | AÇIK |  |  |
| P30 | BLG-0452 | src/storage/emit.rs:1409-1424 | P30-F13: notes/P30-scan.md:245; ham önem Low | AÇIK |  |  |
| P30 | BLG-0453 | src/storage/three_reveal.rs:76-78 | P30-F14: notes/P30-scan.md:259; ham önem Info | AÇIK |  |  |
| P31 | BLG-0454 | src/ai/registry.rs:186-243 | P31-F01: notes/P31-scan.md:37; ham önem Medium | AÇIK |  |  |
| P31 | BLG-0455 | src/ai/types.rs:253-284 | P31-F02: notes/P31-scan.md:60; ham önem Medium | AÇIK |  |  |
| P31 | BLG-0456 | src/ai/types.rs:661-673 | P31-F03: notes/P31-scan.md:77; ham önem Low | AÇIK |  |  |
| P31 | BLG-0457 | src/ai/types.rs:944-951, 982-995 | P31-F04: notes/P31-scan.md:94; ham önem Low | AÇIK |  |  |
| P31 | BLG-0458 | src/ai/types.rs:202-251 | P31-F05: notes/P31-scan.md:111; ham önem Low | AÇIK |  |  |
| P31 | BLG-0459 | src/ai/types.rs:87-117 | P31-F06: notes/P31-scan.md:126; ham önem Low | AÇIK |  |  |
| P31 | BLG-0460 | src/ai/types.rs:716-726, 955-962 | P31-F07: notes/P31-scan.md:141; ham önem Info | AÇIK |  |  |
| P31 | BLG-0461 | src/ai/types.rs:738-751, 953-979 | P31-F08: notes/P31-scan.md:156; ham önem Info | AÇIK |  |  |
| P31 | BLG-0462 | src/ai/types.rs:357-385, 433-443 | P31-F09: notes/P31-scan.md:166; ham önem Info | AÇIK |  |  |
| P31 | BLG-0463 | src/ai/types.rs:1119-1175 | P31-F10: notes/P31-scan.md:176; ham önem Low | AÇIK |  |  |
| P31 | BLG-0464 | src/ai/types.rs:74-84 | P31-F11: notes/P31-scan.md:186; ham önem Low | AÇIK |  |  |
| P31 | BLG-0465 | src/ai/mod.rs:3007-3018, 3508-3526, 4249-4263, 4675-4696 | P31-F12: notes/P31-scan.md:196; ham önem Low | AÇIK |  |  |
| P31 | BLG-0466 | src/ai/mod.rs:3589-3603 | P31-F13: notes/P31-scan.md:210; ham önem Info | AÇIK |  |  |
| P32 | BLG-0467 | src/ai/registry.rs:722-753 | P32-F01: notes/P32-scan.md:45; ham önem High | AÇIK |  |  |
| P32 | BLG-0468 | src/execution/executor.rs:2146-2193 | P32-F02: notes/P32-scan.md:66; ham önem High | AÇIK |  |  |
| P32 | BLG-0469 | src/execution/executor.rs:2141-2144, 2210-2215, 2312-2365 | P32-F03: notes/P32-scan.md:87; ham önem High | AÇIK |  |  |
| P32 | BLG-0470 | src/network/proto_conversions.rs:202-222, 1046-1088 | P32-F04: notes/P32-scan.md:106; ham önem Medium | AÇIK |  |  |
| P32 | BLG-0471 | src/execution/executor.rs:2371 | P32-F05: notes/P32-scan.md:124; ham önem Medium | AÇIK |  |  |
| P32 | BLG-0472 | src/ai/types.rs:253-282 | P32-F06: notes/P32-scan.md:140; ham önem Medium | AÇIK |  |  |
| P32 | BLG-0473 | src/ai_inference/mod.rs:113-159 | P32-F07: notes/P32-scan.md:158; ham önem Medium | AÇIK |  |  |
| P32 | BLG-0474 | src/ai/registry.rs:490-496, 1440-1443 | P32-F08: notes/P32-scan.md:175; ham önem Medium | AÇIK |  |  |
| P32 | BLG-0475 | src/ai/registry.rs:1101-1161 | P32-F09: notes/P32-scan.md:191; ham önem Medium | AÇIK |  |  |
| P32 | BLG-0476 | src/execution/executor.rs:347-349 | P32-F10: notes/P32-scan.md:207; ham önem Low | AÇIK |  |  |
| P32 | BLG-0477 | src/ai/registry.rs:406, 415-429, 434-448 | P32-F11: notes/P32-scan.md:223; ham önem Low | AÇIK |  |  |
| P32 | BLG-0478 | src/ai/execution/guest.rs:1108-1130 | P32-F12: notes/P32-scan.md:240; ham önem Low | AÇIK |  |  |
| P32 | BLG-0479 | src/ai/execution/guest.rs:508 | P32-F13: notes/P32-scan.md:256; ham önem Low | AÇIK |  |  |
| P32 | BLG-0480 | src/ai/registry.rs:807-811, 957-1003, 1905-2035 | P32-F14: notes/P32-scan.md:274; ham önem Low | AÇIK |  |  |
| P32 | BLG-0481 | src/ai/registry.rs:1983-1993, 1953-1957 | P32-F15: notes/P32-scan.md:290; ham önem Low | AÇIK |  |  |
| P32 | BLG-0482 | src/ai/types.rs:206 | P32-F16: notes/P32-scan.md:305; ham önem Low | AÇIK |  |  |
| P32 | BLG-0483 | src/ai_inference/i18n.rs | P32-F17: notes/P32-scan.md:320; ham önem Low | AÇIK |  |  |
| P32 | BLG-0484 | src/ai/registry.rs:1492-1500, 1885-1897 | P32-F18: notes/P32-scan.md:334; ham önem Info | AÇIK |  |  |
| P33 | BLG-0485 | crates/ai-inference/crates/ai-serve/src/bridge.rs:103-186 | P33-F01: notes/P33-scan.md:45; ham önem Medium | AÇIK |  |  |
| P33 | BLG-0486 | crates/ai-inference/crates/ai-serve/src/bridge.rs:152-160 | P33-F02: notes/P33-scan.md:61; ham önem Medium | AÇIK |  |  |
| P33 | BLG-0487 | crates/ai-inference/crates/ai-serve/src/bridge.rs:103-171 | P33-F03: notes/P33-scan.md:75; ham önem Medium | AÇIK |  |  |
| P33 | BLG-0488 | crates/ai-inference/crates/ai-serve/src/config.rs:164-179, 424-437 | P33-F04: notes/P33-scan.md:94; ham önem Medium | AÇIK |  |  |
| P33 | BLG-0489 | crates/ai-inference/crates/ai-serve/src/config.rs:72-99, 191-213, 503-510 | P33-F05: notes/P33-scan.md:109; ham önem Medium | AÇIK |  |  |
| P33 | BLG-0490 | crates/ai-inference/crates/ai-serve/src/bridge.rs:12-19, 137-172 | P33-F06: notes/P33-scan.md:123; ham önem Low | AÇIK |  |  |
| P33 | BLG-0491 | crates/ai-inference/crates/ai-serve/src/fee_scenario.rs:57-59, 275-289 | P33-F07: notes/P33-scan.md:133; ham önem Medium | AÇIK |  |  |
| P33 | BLG-0492 | crates/ai-inference/crates/ai-serve/src/residency.rs:331-383, 247-276 | P33-F08: notes/P33-scan.md:149; ham önem Low | AÇIK |  |  |
| P33 | BLG-0493 | crates/ai-inference/crates/ai-serve/src/residency.rs:291-329, 438-453 | P33-F09: notes/P33-scan.md:159; ham önem Low | AÇIK |  |  |
| P33 | BLG-0494 | crates/ai-inference/crates/ai-serve/src/bridge.rs:152-154 | P33-F10: notes/P33-scan.md:169; ham önem Low | AÇIK |  |  |
| P33 | BLG-0495 | crates/ai-inference/crates/ai-integrations/src/rest.rs:38-88 | P33-F11: notes/P33-scan.md:181; ham önem Low | AÇIK |  |  |
| P33 | BLG-0496 | crates/ai-inference/crates/ai-integrations/src/ws.rs:79-103 | P33-F12: notes/P33-scan.md:195; ham önem Medium | AÇIK |  |  |
| P33 | BLG-0497 | crates/ai-inference/crates/ai-integrations/src/lib.rs:23-29, 49-68 | P33-F13: notes/P33-scan.md:212; ham önem Low | AÇIK |  |  |
| P33 | BLG-0498 | crates/ai-inference/crates/ai-integrations/src/ws.rs:24-33 | P33-F14: notes/P33-scan.md:224; ham önem Low | AÇIK |  |  |
| P33 | BLG-0499 | crates/ai-inference/crates/ai-serve/src/cost_forecast.rs:53-58, 144-162, 188-191 | P33-F15: notes/P33-scan.md:234; ham önem Low | AÇIK |  |  |
| P33 | BLG-0500 | crates/ai-inference/crates/ai-serve/src/staging.rs:33-40, 94-160, 162-185 | P33-F16: notes/P33-scan.md:246; ham önem Low | AÇIK |  |  |
| P33 | BLG-0501 | crates/ai-inference/crates/ai-serve/src/config.rs:215-241, 294-324 | P33-F17: notes/P33-scan.md:256; ham önem Info | AÇIK |  |  |
| P33 | BLG-0502 | crates/ai-inference/crates/ai-serve/src/*.rs (tests), crates/ai-inference/crates | P33-F18: notes/P33-scan.md:270; ham önem Low | AÇIK |  |  |
| P33 | BLG-0503 | crates/ai-inference/crates/ai-serve/src/residency.rs:165-183 | P33-F19: notes/P33-scan.md:280; ham önem Low | AÇIK |  |  |
| P34 | BLG-0504 | crates/ai-inference/crates/ai-knowledge/src/redact.rs:312-328 | P34-F01: notes/P34-scan.md:32; ham önem Medium | AÇIK |  |  |
| P34 | BLG-0505 | crates/ai-inference/crates/ai-knowledge/src/redact.rs:246-276 | P34-F02: notes/P34-scan.md:52; ham önem Medium | AÇIK |  |  |
| P34 | BLG-0506 | crates/ai-inference/crates/ai-knowledge/src/redact.rs:195-243 | P34-F03: notes/P34-scan.md:74; ham önem Medium | AÇIK |  |  |
| P34 | BLG-0507 | crates/ai-inference/crates/ai-knowledge/src/redact.rs:12-25 | P34-F04: notes/P34-scan.md:97; ham önem Medium | AÇIK |  |  |
| P34 | BLG-0508 | crates/ai-inference/Cargo.toml:7 | P34-F05: notes/P34-scan.md:115; ham önem Medium | AÇIK |  |  |
| P34 | BLG-0509 | crates/ai-inference/crates/ai-data/src/verify.rs:51-59 | P34-F06: notes/P34-scan.md:125; ham önem Medium | AÇIK |  |  |
| P34 | BLG-0510 | crates/ai-inference/crates/ai-core/src/manifest.rs:49-60 | P34-F07: notes/P34-scan.md:143; ham önem Medium | AÇIK |  |  |
| P34 | BLG-0511 | crates/ai-inference/crates/ai-tune/src/eval.rs:210-236 | P34-F08: notes/P34-scan.md:168; ham önem Medium | AÇIK |  |  |
| P34 | BLG-0512 | crates/ai-inference/crates/ai-ops/src/main.rs:133-145 | P34-F09: notes/P34-scan.md:188; ham önem Medium | AÇIK |  |  |
| P34 | BLG-0513 | crates/ai-inference/crates/ai-data/src/source.rs:30-66 | P34-F10: notes/P34-scan.md:207; ham önem Low | AÇIK |  |  |
| P34 | BLG-0514 | crates/ai-inference/crates/ai-knowledge/src/memory.rs:136-142 | P34-F11: notes/P34-scan.md:217; ham önem Medium | AÇIK |  |  |
| P34 | BLG-0515 | crates/ai-inference/crates/ai-knowledge/src/cache.rs:36-120 | P34-F12: notes/P34-scan.md:234; ham önem Low | AÇIK |  |  |
| P34 | BLG-0516 | crates/ai-inference/crates/ai-knowledge/src/embed.rs:64-71 | P34-F13: notes/P34-scan.md:250; ham önem Low | AÇIK |  |  |
| P34 | BLG-0517 | crates/ai-inference/crates/ai-ops/src/logparse.rs:19 | P34-F14: notes/P34-scan.md:266; ham önem Low | AÇIK |  |  |
| P34 | BLG-0518 | crates/ai-inference/crates/ai-ops/src/main.rs:26-45, 110-146, 208-213, 228 | P34-F15: notes/P34-scan.md:281; ham önem Low | AÇIK |  |  |
| P34 | BLG-0519 | crates/ai-inference/crates/ai-knowledge/src/redact.rs:12-25, 106-118 | P34-F16: notes/P34-scan.md:297; ham önem Low | AÇIK |  |  |
| P34 | BLG-0520 | crates/ai-inference/crates/ai-ops/src/main.rs:86-92 | P34-F17: notes/P34-scan.md:307; ham önem Info | AÇIK |  |  |
| P34 | BLG-0521 | crates/ai-inference/crates/ai-core/src/system_prompt.rs:317-357 | P34-F18: notes/P34-scan.md:321; ham önem Low | AÇIK |  |  |
| P35 | BLG-0522 | budzero/bud-compiler/src/codegen.rs:367-398 | P35-F01: notes/P35-scan.md:39; ham önem High | AÇIK |  |  |
| P35 | BLG-0523 | budzero/bud-compiler/src/codegen.rs:215-221, 271-287, 437-441, 647-649 | P35-F02: notes/P35-scan.md:59; ham önem High | AÇIK |  |  |
| P35 | BLG-0524 | budzero/bud-compiler/src/codegen.rs:702-704, 718-744, 751-754 | P35-F03: notes/P35-scan.md:79; ham önem High | AÇIK |  |  |
| P35 | BLG-0525 | budzero/bud-compiler/src/codegen.rs:263-268, 691-699 | P35-F04: notes/P35-scan.md:100; ham önem High | AÇIK |  |  |
| P35 | BLG-0526 | budzero/bud-compiler/src/codegen.rs:389-392 | P35-F05: notes/P35-scan.md:117; ham önem Medium | AÇIK |  |  |
| P35 | BLG-0527 | budzero/bud-compiler/src/codegen.rs:346-430 | P35-F06: notes/P35-scan.md:132; ham önem Medium | AÇIK |  |  |
| P35 | BLG-0528 | budzero/bud-compiler/src/sema.rs:534-553 | P35-F07: notes/P35-scan.md:147; ham önem Medium | AÇIK |  |  |
| P35 | BLG-0529 | budzero/bud-compiler/src/parser.rs:514-583 | P35-F08: notes/P35-scan.md:165; ham önem Medium | AÇIK |  |  |
| P35 | BLG-0530 | budzero/bud-compiler/src/lexer.rs:95-97 | P35-F09: notes/P35-scan.md:181; ham önem Medium | AÇIK |  |  |
| P35 | BLG-0531 | budzero/bud-compiler/src/parser.rs:693-712 | P35-F10: notes/P35-scan.md:195; ham önem Medium | AÇIK |  |  |
| P35 | BLG-0532 | budzero/bud-compiler/src/parser.rs:220-227 | P35-F11: notes/P35-scan.md:211; ham önem Medium | AÇIK |  |  |
| P35 | BLG-0533 | budzero/bud-compiler/src/lib.rs:82, 265, 298, 425, 453, 472 | P35-F12: notes/P35-scan.md:226; ham önem Medium | AÇIK |  |  |
| P35 | BLG-0534 | budzero/bud-compiler/src/codegen.rs:510-516 | P35-F13: notes/P35-scan.md:243; ham önem Medium | AÇIK |  |  |
| P35 | BLG-0535 | budzero/bud-compiler/src/ir/mod.rs:1-10 | P35-F14: notes/P35-scan.md:258; ham önem Medium | AÇIK |  |  |
| P35 | BLG-0536 | budzero/bud-compiler/src/ir/lower.rs:213-215, 269-306, 344-359 | P35-F15: notes/P35-scan.md:274; ham önem Low | AÇIK |  |  |
| P35 | BLG-0537 | budzero/bud-compiler/src/sema.rs:412-431 | P35-F16: notes/P35-scan.md:288; ham önem Low | AÇIK |  |  |
| P35 | BLG-0538 | budzero/bud-compiler/src/sema.rs:480-498 | P35-F17: notes/P35-scan.md:303; ham önem Low | AÇIK |  |  |
| P35 | BLG-0539 | budzero/bud-compiler/src/sema.rs:41-47, 691-785 | P35-F18: notes/P35-scan.md:316; ham önem Low | AÇIK |  |  |
| P35 | BLG-0540 | budzero/bud-compiler/src/codegen.rs:883-916 | P35-F19: notes/P35-scan.md:330; ham önem Low | AÇIK |  |  |
| P35 | BLG-0541 | budzero/bud-compiler/src/codegen.rs:25 | P35-F20: notes/P35-scan.md:344; ham önem Low | AÇIK |  |  |
| P35 | BLG-0542 | budzero/bud-compiler/src/lexer.rs:97 | P35-F21: notes/P35-scan.md:358; ham önem Low | AÇIK |  |  |
| P35 | BLG-0543 | budzero/bud-compiler/src/sema.rs:188, 197, 291 | P35-F22: notes/P35-scan.md:371; ham önem Low | AÇIK |  |  |
| P35 | BLG-0544 | budzero/bud-compiler/src/sema.rs:465-467, 505-506, 529-533, 579 | P35-F23: notes/P35-scan.md:385; ham önem Low | AÇIK |  |  |
| P35 | BLG-0545 | budzero/bud-compiler/src/sema.rs:317, 628-635 | P35-F24: notes/P35-scan.md:399; ham önem Info | AÇIK |  |  |
| P35 | BLG-0546 | budzero/bud-compiler/src/codegen.rs:746-758 | P35-F25: notes/P35-scan.md:413; ham önem Info | AÇIK |  |  |
| P35 | BLG-0547 | budzero/bud-compiler/src/sema.rs:19-27, 303-308 | P35-F26: notes/P35-scan.md:426; ham önem Info | AÇIK |  |  |
| P36 | BLG-0548 | src/network/proto_conversions.rs:202-228 | P36-F01: notes/P36-scan.md:35; ham önem High | AÇIK |  |  |
| P36 | BLG-0549 | src/network/peer_manager.rs:73-84, 85-101, 399-427 | P36-F02: notes/P36-scan.md:55; ham önem High | AÇIK |  |  |
| P36 | BLG-0550 | src/network/node.rs:2923-2937 | P36-F03: notes/P36-scan.md:74; ham önem Medium | AÇIK |  |  |
| P36 | BLG-0551 | src/network/sync_codec.rs:20-34, 56, 67 | P36-F04: notes/P36-scan.md:92; ham önem Medium | AÇIK |  |  |
| P36 | BLG-0552 | src/network/gossip_dedup.rs:167-192, 249-254 | P36-F05: notes/P36-scan.md:110; ham önem Medium | AÇIK |  |  |
| P36 | BLG-0553 | src/network/proto_conversions.rs:1698-1701, 1978-1984, 1991-1996, 989-997 | P36-F06: notes/P36-scan.md:128; ham önem Medium | AÇIK |  |  |
| P36 | BLG-0554 | src/network/gossip_dedup.rs:378-383 | P36-F07: notes/P36-scan.md:145; ham önem Low | AÇIK |  |  |
| P36 | BLG-0555 | src/network/peer_manager.rs:314-322, 380-397, 510-515, 357-366 | P36-F08: notes/P36-scan.md:163; ham önem Medium | AÇIK |  |  |
| P36 | BLG-0556 | src/network/peer_manager.rs:314-322, 357-366 | P36-F09: notes/P36-scan.md:180; ham önem Low | AÇIK |  |  |
| P36 | BLG-0557 | src/network/privacy.rs:236-254, 289-301, 318-324 | P36-F10: notes/P36-scan.md:196; ham önem Low | AÇIK |  |  |
| P36 | BLG-0558 | src/network/privacy.rs:211-213, 338 | P36-F11: notes/P36-scan.md:212; ham önem Low | AÇIK |  |  |
| P36 | BLG-0559 | src/network/peer_manager.rs:622-649 | P36-F12: notes/P36-scan.md:226; ham önem Low | AÇIK |  |  |
| P36 | BLG-0560 | src/network/protocol.rs:23-28 | P36-F13: notes/P36-scan.md:239; ham önem Low | AÇIK |  |  |
| P36 | BLG-0561 | src/network/proto_conversions.rs:19 | P36-F14: notes/P36-scan.md:253; ham önem Info | AÇIK |  |  |
| P36 | BLG-0562 | src/network/proto_conversions.rs:182, 190, 198, 355, 363, 371 | P36-F15: notes/P36-scan.md:266; ham önem Info | AÇIK |  |  |
| P36 | BLG-0563 | src/network/mobile.rs:445-450 | P36-F16: notes/P36-scan.md:279; ham önem Low | AÇIK |  |  |
| P36 | BLG-0564 | src/network/gossip_dedup.rs:150-161, 182-190, 357-359 | P36-F17: notes/P36-scan.md:295; ham önem Low | AÇIK |  |  |
| P36 | BLG-0565 | src/network/gossip_dedup.rs:20-383 | P36-F18: notes/P36-scan.md:309; ham önem Info | AÇIK |  |  |
| P36 | BLG-0566 | src/network/node.rs:2114 | P36-F19: notes/P36-scan.md:322; ham önem Medium | AÇIK |  |  |
| P37 | BLG-0567 | bud/Cargo.toml:1-3 | P37-F01: notes/P37-scan.md:49; ham önem Medium | AÇIK |  |  |
| P37 | BLG-0568 | bud/src/bud_format.rs:189-190, 245-250, 397-406 | P37-F02: notes/P37-scan.md:59; ham önem Medium | AÇIK |  |  |
| P37 | BLG-0569 | bud/src/bud_format.rs:272-291, 344-374, 351-358 | P37-F03: notes/P37-scan.md:76; ham önem Medium | AÇIK |  |  |
| P37 | BLG-0570 | bud/src/bud_format.rs:376-427 | P37-F04: notes/P37-scan.md:94; ham önem Medium | AÇIK |  |  |
| P37 | BLG-0571 | bud/src/bud_format.rs:302-326, 360-365 | P37-F05: notes/P37-scan.md:104; ham önem Low | AÇIK |  |  |
| P37 | BLG-0572 | bud/src/bin/bud.rs:819-826 | P37-F06: notes/P37-scan.md:114; ham önem Medium | AÇIK |  |  |
| P37 | BLG-0573 | bud/src/bud_format.rs:626-772, 774-796 | P37-F07: notes/P37-scan.md:130; ham önem Medium | AÇIK |  |  |
| P37 | BLG-0574 | bud/src/bud_format.rs:382, 399, 401, 409, 421, 429-609 | P37-F08: notes/P37-scan.md:144; ham önem Low | AÇIK |  |  |
| P37 | BLG-0575 | bud/src/bud_format.rs:834-841 | P37-F09: notes/P37-scan.md:154; ham önem Low | AÇIK |  |  |
| P37 | BLG-0576 | bud/src/bud_fixed_point.rs:26-31 | P37-F10: notes/P37-scan.md:171; ham önem Low | AÇIK |  |  |
| P37 | BLG-0577 | bud/src/bud_fixed_point.rs:116-119 | P37-F11: notes/P37-scan.md:187; ham önem Low | AÇIK |  |  |
| P37 | BLG-0578 | bud/src/bud_format_bft.rs:84-92, 130-183 | P37-F12: notes/P37-scan.md:197; ham önem Medium | AÇIK |  |  |
| P37 | BLG-0579 | bud/src/bud_format_bft.rs:130-183, 198-244 | P37-F13: notes/P37-scan.md:212; ham önem Low | AÇIK |  |  |
| P37 | BLG-0580 | bud/src/bud_format_block.rs:98-106, 129-131, 157-227 | P37-F14: notes/P37-scan.md:222; ham önem Medium | AÇIK |  |  |
| P37 | BLG-0581 | bud/src/bud_format_columnar.rs:112-206, 223-246 | P37-F15: notes/P37-scan.md:238; ham önem Medium | AÇIK |  |  |
| P37 | BLG-0582 | bud/src/bud_format_columnar.rs:112-120, 153-199, 475-482, 236-238 | P37-F16: notes/P37-scan.md:248; ham önem Low | AÇIK |  |  |
| P37 | BLG-0583 | bud/src/bud_format_catalog.rs:31-311, 314-363 | P37-F17: notes/P37-scan.md:258; ham önem Low | AÇIK |  |  |
| P37 | BLG-0584 | bud/src/bud_format_checkpoint.rs:1-107 | P37-F18: notes/P37-scan.md:268; ham önem Low | AÇIK |  |  |
| P37 | BLG-0585 | bud/src/bud_format_container.rs:246-264, 417-427, 505-547 | P37-F19: notes/P37-scan.md:278; ham önem Medium | AÇIK |  |  |
| P37 | BLG-0586 | bud/src/bud_format_container.rs:346-348, 505-523 | P37-F20: notes/P37-scan.md:294; ham önem Medium | AÇIK |  |  |
| P37 | BLG-0587 | bud/src/bud_format_container.rs:246-264, 391-415 | P37-F21: notes/P37-scan.md:304; ham önem Low | AÇIK |  |  |
| P37 | BLG-0588 | bud/src/bud_format_das.rs:27-64, 75-129 | P37-F22: notes/P37-scan.md:314; ham önem Medium | AÇIK |  |  |
| P37 | BLG-0589 | bud/src/bud_format_das.rs:132-153, 191-206 | P37-F23: notes/P37-scan.md:324; ham önem Low | AÇIK |  |  |
| P37 | BLG-0590 | bud/src/bud_format_das.rs:209-254 | P37-F24: notes/P37-scan.md:334; ham önem Medium | AÇIK |  |  |
| P37 | BLG-0591 | bud/src/bud_format_dedup.rs:53-107 | P37-F25: notes/P37-scan.md:344; ham önem Medium | AÇIK |  |  |
| P37 | BLG-0592 | bud/src/bud_format_economics.rs:271-307 | P37-F26: notes/P37-scan.md:359; ham önem Medium | AÇIK |  |  |
| P37 | BLG-0593 | bud/src/bud_format_economics.rs:233-269 | P37-F27: notes/P37-scan.md:374; ham önem Medium | AÇIK |  |  |
| P37 | BLG-0594 | bud/src/bud_format_bloom.rs:20-62, 84-114 | P37-F28: notes/P37-scan.md:384; ham önem Low | AÇIK |  |  |
| P37 | BLG-0595 | bud/src/bud_format_economics.rs:20-41, 75-97, 66-69 | P37-F29: notes/P37-scan.md:394; ham önem Low | AÇIK |  |  |
| P37 | BLG-0596 | bud/src/bud_format_edge.rs:35-61 | P37-F30: notes/P37-scan.md:404; ham önem Low | AÇIK |  |  |
| P37 | BLG-0597 | bud/src/bud_format_crop.rs:32-64, 97-138 | P37-F31: notes/P37-scan.md:414; ham önem Low | AÇIK |  |  |
| P37 | BLG-0598 | bud/src/bud_format_culling.rs:88-119, 187-236 | P37-F32: notes/P37-scan.md:424; ham önem Low | AÇIK |  |  |
| P37 | BLG-0599 | bud/src/bud_format_dictionary.rs:57-74, 84-94 | P37-F33: notes/P37-scan.md:434; ham önem Low | AÇIK |  |  |
| P37 | BLG-0600 | bud/src/bud3_live_test.rs:26-52, 54-99 | P37-F34: notes/P37-scan.md:444; ham önem Low | AÇIK |  |  |
| P37 | BLG-0601 | bud/src/bud_format_autozstd.rs | P37-F35: notes/P37-scan.md:454; ham önem Info | AÇIK |  |  |
| P37 | BLG-0602 | bud/src/bud_format_autozstd.rs:27-59 | P37-F36: notes/P37-scan.md:464; ham önem Info | AÇIK |  |  |
| P38 | BLG-0603 | bud/src/bud_format_engine.rs:208-248 | P38-F01: notes/P38-scan.md:45; ham önem Medium | AÇIK |  |  |
| P38 | BLG-0604 | bud/src/lib.rs:24-47, 127-149 | P38-F02: notes/P38-scan.md:63; ham önem Medium | AÇIK |  |  |
| P38 | BLG-0605 | bud/src/bud_format_integration.rs:99-123 | P38-F03: notes/P38-scan.md:70; ham önem Medium | AÇIK |  |  |
| P38 | BLG-0606 | bud/src/bud_format_logfield.rs:135-141 | P38-F04: notes/P38-scan.md:84; ham önem Medium | AÇIK |  |  |
| P38 | BLG-0607 | bud/src/bud_format_hw.rs:60-67, 94-107, 124-138 | P38-F05: notes/P38-scan.md:99; ham önem Medium | AÇIK |  |  |
| P38 | BLG-0608 | bud/src/bud_format_matrix.rs:25-31, 301-324, 332-354, 380-391 | P38-F06: notes/P38-scan.md:111; ham önem Medium | AÇIK |  |  |
| P38 | BLG-0609 | bud/src/bud_format_huffman.rs:35-91 | P38-F07: notes/P38-scan.md:124; ham önem Medium | AÇIK |  |  |
| P38 | BLG-0610 | bud/src/bud_format_markdown.rs:72-80, 64-65 | P38-F08: notes/P38-scan.md:135; ham önem Low | AÇIK |  |  |
| P38 | BLG-0611 | bud/src/bud_format_markdown.rs:64, 120-126, 11-19 | P38-F09: notes/P38-scan.md:147; ham önem Medium | AÇIK |  |  |
| P38 | BLG-0612 | bud/src/bud_format_fidelitygate.rs:39-72 | P38-F10: notes/P38-scan.md:158; ham önem Medium | AÇIK |  |  |
| P38 | BLG-0613 | bud/src/bud_format_media.rs:72-81 | P38-F11: notes/P38-scan.md:170; ham önem Low | AÇIK |  |  |
| P38 | BLG-0614 | bud/src/bud_format_msr.rs:126-142 | P38-F12: notes/P38-scan.md:181; ham önem Low | AÇIK |  |  |
| P38 | BLG-0615 | bud/src/bud_format_fountain.rs:81-153 | P38-F13: notes/P38-scan.md:195; ham önem Medium | AÇIK |  |  |
| P38 | BLG-0616 | bud/src/bud_format_fountain.rs:25-35, 49-67 | P38-F14: notes/P38-scan.md:209; ham önem Low | AÇIK |  |  |
| P38 | BLG-0617 | bud/src/bud_format_lrc.rs:47-57, 66-78, 108-117 | P38-F15: notes/P38-scan.md:216; ham önem Low | AÇIK |  |  |
| P38 | BLG-0618 | bud/src/bud_format_lazyrepair.rs:18-64 | P38-F16: notes/P38-scan.md:226; ham önem Low | AÇIK |  |  |
| P38 | BLG-0619 | bud/src/bud_format_engine.rs:305-321 | P38-F17: notes/P38-scan.md:237; ham önem Low | AÇIK |  |  |
| P38 | BLG-0620 | bud/src/bud_format_engine.rs:371-372 | P38-F18: notes/P38-scan.md:251; ham önem Low | AÇIK |  |  |
| P38 | BLG-0621 | bud/src/bud_format_engine.rs:131-143, 252-280, 468-501, 733-745 | P38-F19: notes/P38-scan.md:262; ham önem Low | AÇIK |  |  |
| P38 | BLG-0622 | bud/src/bud_format_erasure.rs:145-179, 85-93 | P38-F20: notes/P38-scan.md:273; ham önem Low | AÇIK |  |  |
| P38 | BLG-0623 | bud/src/bud_format_lowrank.rs:51-130, 11-17, 105-109 | P38-F21: notes/P38-scan.md:287; ham önem Low | AÇIK |  |  |
| P38 | BLG-0624 | bud/src/bud_format_model.rs:60-120, 169-221 | P38-F22: notes/P38-scan.md:299; ham önem Low | AÇIK |  |  |
| P38 | BLG-0625 | bud/src/bud_format_multifile.rs:67-90, 124-190, 193-286, 487-495 | P38-F23: notes/P38-scan.md:310; ham önem Low | AÇIK |  |  |
| P38 | BLG-0626 | bud/src/bud_format_edition.rs:22-30, 147-154, 119-135 | P38-F24: notes/P38-scan.md:324; ham önem Low | AÇIK |  |  |
| P38 | BLG-0627 | bud/src/bud_format_guardian.rs:68-87, 93-96, 129-137 | P38-F25: notes/P38-scan.md:336; ham önem Low | AÇIK |  |  |
| P38 | BLG-0628 | bud/src/bud_format_governance.rs:56-62, 120-130 | P38-F26: notes/P38-scan.md:347; ham önem Low | AÇIK |  |  |
| P38 | BLG-0629 | bud/src/bud_format_jpegre.rs:113-132, 134-183 | P38-F27: notes/P38-scan.md:358; ham önem Low | AÇIK |  |  |
| P38 | BLG-0630 | bud/src/bud_format_fastcdc.rs:26-155, 169-226, 198-202 | P38-F28: notes/P38-scan.md:369; ham önem Low | AÇIK |  |  |
| P38 | BLG-0631 | bud/src/bud_format_exepdf.rs:7-13, 66-103, 128-148 | P38-F29: notes/P38-scan.md:376; ham önem Info | AÇIK |  |  |
| P38 | BLG-0632 | 18 files: matrix (12 hits), lowrank (20), hardening (9), guardian (5), fidelityg | P38-F30: notes/P38-scan.md:383; ham önem Info | AÇIK |  |  |
| P38 | BLG-0633 | bud/src/bud_format_genomic.rs:42-99 | P38-F31: notes/P38-scan.md:390; ham önem Info | AÇIK |  |  |
| P39 | BLG-0634 | bud/src/bud_format_qrvideo.rs:250-264, 267-288, 290-314, 371-385, 218-222, 553-566 | P39-F01: notes/P39-scan.md:25; ham önem Medium | AÇIK |  |  |
| P39 | BLG-0635 | bud/src/bud_format_qrvideo.rs:267-288, 318-343 | P39-F02: notes/P39-scan.md:41; ham önem Medium | AÇIK |  |  |
| P39 | BLG-0636 | bud/src/bud_format_qrvideo.rs:1-11, 169-203, 132-146, 402-409 | P39-F03: notes/P39-scan.md:56; ham önem Medium | AÇIK |  |  |
| P39 | BLG-0637 | bud/src/bud_format_qrmatrix.rs:97-137, 1-13 | P39-F04: notes/P39-scan.md:69; ham önem Medium | AÇIK |  |  |
| P39 | BLG-0638 | bud/src/bud_format_por.rs:44-51, 56-66, 72-81, 87-106 | P39-F05: notes/P39-scan.md:84; ham önem Medium | AÇIK |  |  |
| P39 | BLG-0639 | bud/src/bud_format_pcap.rs:21, 62-95, 82-83, 146-150, 246-247, 371 | P39-F06: notes/P39-scan.md:99; ham önem Medium | AÇIK |  |  |
| P39 | BLG-0640 | bud/src/bud_format_office.rs:33-64, 71-78, 83-100, 104-198, 11-13, 230-232 | P39-F07: notes/P39-scan.md:113; ham önem Medium | AÇIK |  |  |
| P39 | BLG-0641 | bud/src/bud_format_scrub.rs:36-44, 76-84 | P39-F08: notes/P39-scan.md:128; ham önem Medium | AÇIK |  |  |
| P39 | BLG-0642 | bud/src/bud_format_recipe_record.rs:74-96, 99-106 | P39-F09: notes/P39-scan.md:141; ham önem Medium | AÇIK |  |  |
| P39 | BLG-0643 | bud/src/bud_format_recipe_record.rs:44-49, 65-70, 115-126 | P39-F10: notes/P39-scan.md:155; ham önem Medium | AÇIK |  |  |
| P39 | BLG-0644 | bud/src/bud_format_ratioconsensus.rs:44-52, 71-135, 157-180, 208-243 | P39-F11: notes/P39-scan.md:170; ham önem Medium | AÇIK |  |  |
| P39 | BLG-0645 | bud/src/bud_format_regeneration.rs:62-186 | P39-F12: notes/P39-scan.md:183; ham önem Medium | AÇIK |  |  |
| P39 | BLG-0646 | bud/src/bud_format_registry.rs:56-100, 126-151, 154-184, 46-53 | P39-F13: notes/P39-scan.md:196; ham önem Medium | AÇIK |  |  |
| P39 | BLG-0647 | bud/src/bud_format_securededup.rs:19-36, 42-75, 79-85 | P39-F14: notes/P39-scan.md:212; ham önem Medium | AÇIK |  |  |
| P39 | BLG-0648 | bud/src/bud_format_optical_transfer.rs:48-66, 73-107, 124-134, 12-15 | P39-F15: notes/P39-scan.md:224; ham önem Medium | AÇIK |  |  |
| P39 | BLG-0649 | bud/src/bud_format_recipe.rs:26, 39-54, 58-74, 76-84 | P39-F16: notes/P39-scan.md:236; ham önem Medium | AÇIK |  |  |
| P39 | BLG-0650 | bud/src/bud_format_revolutionary.rs:179-193, 256-258, 280-296, 340-347, 429-444 | P39-F17: notes/P39-scan.md:250; ham önem Medium | AÇIK |  |  |
| P39 | BLG-0651 | bud/src/bud_format_pgm.rs:112-124, 63-77, 33-35 | P39-F18: notes/P39-scan.md:263; ham önem Low | AÇIK |  |  |
| P39 | BLG-0652 | bud/src/bud_format_pointcloud.rs:27-62, 64-83, 85-94, 52-60 | P39-F19: notes/P39-scan.md:275; ham önem Low | AÇIK |  |  |
| P39 | BLG-0653 | bud/src/bud_format_optical.rs:51-76, 29-34, 1-5 | P39-F20: notes/P39-scan.md:287; ham önem Low | AÇIK |  |  |
| P39 | BLG-0654 | bud/src/bud_format_pact.rs:148-159, 201-242, 101-114 | P39-F21: notes/P39-scan.md:299; ham önem Low | AÇIK |  |  |
| P39 | BLG-0655 | bud/src/bud_format_repairband.rs:27-50, 53-64 | P39-F22: notes/P39-scan.md:311; ham önem Low | AÇIK |  |  |
| P39 | BLG-0656 | bud/src/bud_format_prodcost.rs:111-117 | P39-F23: notes/P39-scan.md:323; ham önem Low | AÇIK |  |  |
| P39 | BLG-0657 | bud/src/bud_format_pipe.rs:38-47, 141-161 | P39-F24: notes/P39-scan.md:333; ham önem Low | AÇIK |  |  |
| P39 | BLG-0658 | bud/src/bud_format_qrvideo.rs:66-103, 267-288, 169-203, 43-53 | P39-F25: notes/P39-scan.md:344; ham önem Low | AÇIK |  |  |
| P39 | BLG-0659 | bud/src/bud_format_nvc.rs:35-54, 56-68, 70-79 | P39-F26: notes/P39-scan.md:355; ham önem Low | AÇIK |  |  |
| P39 | BLG-0660 | bud/src/lib.rs | P39-F27: notes/P39-scan.md:368; ham önem Low | AÇIK |  |  |
| P39 | BLG-0661 | bud_format_pcap.rs:371; bud_format_recipe_record.rs:226-233; bud_format_servicec | P39-F28: notes/P39-scan.md:377; ham önem Low | AÇIK |  |  |
| P39 | BLG-0662 | bud/src/bud_format_revolutionary.rs:496 | P39-F29: notes/P39-scan.md:389; ham önem Info | AÇIK |  |  |
| P40 | BLG-0663 | bud/src/bud_format_zkbridge.rs:34-63, 79-91 | P40-F01: notes/P40-scan.md:55; ham önem Medium | AÇIK |  |  |
| P40 | BLG-0664 | bud/src/bud_format_zkproof.rs:94-110 | P40-F02: notes/P40-scan.md:77; ham önem Medium | AÇIK |  |  |
| P40 | BLG-0665 | bud/src/bud_format_timeseries.rs:25, 179, 236-270 | P40-F03: notes/P40-scan.md:94; ham önem Medium | AÇIK |  |  |
| P40 | BLG-0666 | bud/src/bud_format_view.rs:46-115, 118-139, 164 | P40-F04: notes/P40-scan.md:109; ham önem Medium | AÇIK |  |  |
| P40 | BLG-0667 | bud/src/quantum_chain.rs:49-90 | P40-F05: notes/P40-scan.md:123; ham önem Medium | AÇIK |  |  |
| P40 | BLG-0668 | bud/src/quantum_chain.rs:110-138 | P40-F06: notes/P40-scan.md:139; ham önem Medium | AÇIK |  |  |
| P40 | BLG-0669 | bud/src/quantum_chain.rs:140-235, 186-195 | P40-F07: notes/P40-scan.md:158; ham önem Medium | AÇIK |  |  |
| P40 | BLG-0670 | bud/src/provider.rs:83-96, 118-128 | P40-F08: notes/P40-scan.md:174; ham önem Medium | AÇIK |  |  |
| P40 | BLG-0671 | bud/src/gates.rs:152-173 | P40-F09: notes/P40-scan.md:188; ham önem Medium | AÇIK |  |  |
| P40 | BLG-0672 | bud/src/fidelity.rs:83-131 | P40-F10: notes/P40-scan.md:204; ham önem Low | AÇIK |  |  |
| P40 | BLG-0673 | bud/src/churn.rs:127-172, 130-134 | P40-F11: notes/P40-scan.md:213; ham önem Low | AÇIK |  |  |
| P40 | BLG-0674 | bud/src/churn.rs:33-44 | P40-F12: notes/P40-scan.md:222; ham önem Low | AÇIK |  |  |
| P40 | BLG-0675 | bud/src/bud_format_telemetry.rs:54-68 | P40-F13: notes/P40-scan.md:231; ham önem Low | AÇIK |  |  |
| P40 | BLG-0676 | bud/src/bud_format_social2.rs:117-128 | P40-F14: notes/P40-scan.md:240; ham önem Low | AÇIK |  |  |
| P40 | BLG-0677 | bud/src/bud_format_social.rs:102-127 | P40-F15: notes/P40-scan.md:249; ham önem Low | AÇIK |  |  |
| P40 | BLG-0678 | bud/src/bud_format_shamir.rs:118-167 | P40-F16: notes/P40-scan.md:258; ham önem Low | AÇIK |  |  |
| P40 | BLG-0679 | bud/src/bud_format_wal.rs:25-40, 63-107 | P40-F17: notes/P40-scan.md:267; ham önem Low | AÇIK |  |  |
| P40 | BLG-0680 | bud/src/bud_format_timeseries.rs:122, 177-178, 236-270 | P40-F18: notes/P40-scan.md:276; ham önem Low | AÇIK |  |  |
| P40 | BLG-0681 | bud/src/bud_format_videopipe.rs:40-85 | P40-F19: notes/P40-scan.md:285; ham önem Low | AÇIK |  |  |
| P40 | BLG-0682 | bud/src/bud_format_video.rs:106-146 | P40-F20: notes/P40-scan.md:294; ham önem Low | AÇIK |  |  |
| P40 | BLG-0683 | bud/src/bud_format_ux.rs:22-29, 75-111 | P40-F21: notes/P40-scan.md:303; ham önem Low | AÇIK |  |  |
| P40 | BLG-0684 | bud/src/bud_format_view.rs:51-115, 222-228 | P40-F22: notes/P40-scan.md:312; ham önem Low | AÇIK |  |  |
| P40 | BLG-0685 | bud/src/quantum_chain.rs:111-115 | P40-F23: notes/P40-scan.md:321; ham önem Low | AÇIK |  |  |
| P40 | BLG-0686 | bud/src/bud_format_spec.rs:28-35, 55-79, 89-104 | P40-F24: notes/P40-scan.md:330; ham önem Low | AÇIK |  |  |
| P40 | BLG-0687 | bud/src/bud_format_transpile.rs:18-39, 47-57, 64-82 | P40-F25: notes/P40-scan.md:339; ham önem Low | AÇIK |  |  |
| P40 | BLG-0688 | bud/src/bud_format_tricore.rs:1-110 | P40-F26: notes/P40-scan.md:348; ham önem Low | AÇIK |  |  |
| P40 | BLG-0689 | bud/src/quantum.rs:5-76 | P40-F27: notes/P40-scan.md:357; ham önem Info | AÇIK |  |  |
| P40 | BLG-0690 | bud/src/bud_format_zkproof.rs:88-95, 140-178 | P40-F28: notes/P40-scan.md:366; ham önem Info | AÇIK |  |  |
| P40 | BLG-0691 | bud/src/lib.rs:1-225 | P40-F29: notes/P40-scan.md:375; ham önem Info | AÇIK |  |  |
| P41 | BLG-0692 | src/bin/bud.rs:304-308 | P41-F01: notes/P41-scan.md:41; ham önem Medium | AÇIK |  |  |
| P41 | BLG-0693 | src/bin/budlum-relayer.rs:769-771 | P41-F02: notes/P41-scan.md:61; ham önem Medium | AÇIK |  |  |
| P41 | BLG-0694 | src/bin/budlum-relayer.rs:285-343, 352-361, 606-617 | P41-F03: notes/P41-scan.md:77; ham önem Medium | AÇIK |  |  |
| P41 | BLG-0695 | src/bin/budlum-relayer.rs:600-603, 606-617, 638-652 | P41-F04: notes/P41-scan.md:93; ham önem Medium | AÇIK |  |  |
| P41 | BLG-0696 | src/bin/budlum-relayer.rs:491-498, 504-508 | P41-F05: notes/P41-scan.md:111; ham önem Medium | AÇIK |  |  |
| P41 | BLG-0697 | src/bin/budlum-relayer.rs:181-197 | P41-F06: notes/P41-scan.md:128; ham önem Low | AÇIK |  |  |
| P41 | BLG-0698 | src/bin/bud.rs:243-265, 287-288 | P41-F07: notes/P41-scan.md:142; ham önem Low | AÇIK |  |  |
| P41 | BLG-0699 | budzero/bud-node/src/store.rs:106-136, 154-162 | P41-F08: notes/P41-scan.md:157; ham önem Medium | AÇIK |  |  |
| P41 | BLG-0700 | budzero/bud-node/src/bitswap.rs:64-78, 129-130 | P41-F09: notes/P41-scan.md:175; ham önem Low | AÇIK |  |  |
| P41 | BLG-0701 | budzero/bud-cli/src/main.rs:292-296, 893, 331 | P41-F10: notes/P41-scan.md:189; ham önem Low | AÇIK |  |  |
| P41 | BLG-0702 | budzero/bud-cli/src/main.rs:40-46, 369-371, 1088-1105 | P41-F11: notes/P41-scan.md:205; ham önem Low | AÇIK |  |  |
| P41 | BLG-0703 | budzero/bud-cli/src/main.rs:12-30, 995-1021 | P41-F12: notes/P41-scan.md:219; ham önem Medium | AÇIK |  |  |
| P41 | BLG-0704 | budzero/bud-cli/src/main.rs:533-593, 649-678 | P41-F13: notes/P41-scan.md:234; ham önem Low | AÇIK |  |  |
| P41 | BLG-0705 | src/bin/budlum-light.rs:121-189 | P41-F14: notes/P41-scan.md:249; ham önem Medium | AÇIK |  |  |
| P41 | BLG-0706 | src/bin/budlum-light.rs:80-96 | P41-F15: notes/P41-scan.md:265; ham önem Low | AÇIK |  |  |
| P41 | BLG-0707 | src/bin/budlum-relayer.rs:127-131, 225-229, 154-157, 254-257, 297-299 | P41-F16: notes/P41-scan.md:280; ham önem Low | AÇIK |  |  |
| P41 | BLG-0708 | src/bin/budlum-relayer.rs:1, 807-813 | P41-F17: notes/P41-scan.md:295; ham önem Low | AÇIK |  |  |
| P41 | BLG-0709 | budzero/bud-cli/tests/toolchain_end_to_end.rs:182-212, 254-277 | P41-F18: notes/P41-scan.md:310; ham önem Low | AÇIK |  |  |
| P41 | BLG-0710 | budzero/bud-node/src/sharding.rs:415-433, 445-449 | P41-F19: notes/P41-scan.md:325; ham önem Info | AÇIK |  |  |
| P41 | BLG-0711 | examples/print_genesis_hash.rs:1-20 | P41-F20: notes/P41-scan.md:340; ham önem Info | AÇIK |  |  |
| P42 | BLG-0712 | src/socialfi/vault.rs:235-256 | P42-F01: notes/P42-scan.md:41; ham önem High | AÇIK |  |  |
| P42 | BLG-0713 | src/rpc/server.rs:5410-5425 | P42-F02: notes/P42-scan.md:57; ham önem High | AÇIK |  |  |
| P42 | BLG-0714 | src/relayer/worker.rs:687-689, 993-1004 | P42-F03: notes/P42-scan.md:75; ham önem Medium | AÇIK |  |  |
| P42 | BLG-0715 | src/relayer/worker.rs:675-680 | P42-F04: notes/P42-scan.md:93; ham önem Medium | AÇIK |  |  |
| P42 | BLG-0716 | src/relayer/worker.rs:313-325, 654-712, 818-852 | P42-F05: notes/P42-scan.md:109; ham önem Medium | AÇIK |  |  |
| P42 | BLG-0717 | src/main.rs:1502-1517 | P42-F06: notes/P42-scan.md:126; ham önem Medium | AÇIK |  |  |
| P42 | BLG-0718 | src/relayer/policy.rs:1-48, 345-439 | P42-F07: notes/P42-scan.md:141; ham önem Medium | AÇIK |  |  |
| P42 | BLG-0719 | src/bns/registry.rs:58-108 | P42-F08: notes/P42-scan.md:156; ham önem Medium | AÇIK |  |  |
| P42 | BLG-0720 | src/execution/executor.rs:999-1006 | P42-F09: notes/P42-scan.md:171; ham önem Medium | AÇIK |  |  |
| P42 | BLG-0721 | src/bns/registry.rs:254-274 | P42-F10: notes/P42-scan.md:187; ham önem Low | AÇIK |  |  |
| P42 | BLG-0722 | src/bns/registry.rs:110-182 | P42-F11: notes/P42-scan.md:202; ham önem Low | AÇIK |  |  |
| P42 | BLG-0723 | src/bns/registry.rs:184-204, 220-232 | P42-F12: notes/P42-scan.md:217; ham önem Low | AÇIK |  |  |
| P42 | BLG-0724 | src/bns/registry.rs:7-32, 297-299 | P42-F13: notes/P42-scan.md:232; ham önem Low | AÇIK |  |  |
| P42 | BLG-0725 | src/bns/registry.rs:105, 92-104 | P42-F14: notes/P42-scan.md:248; ham önem Low | AÇIK |  |  |
| P42 | BLG-0726 | src/relayer/policy.rs:155-174, 527-551 | P42-F15: notes/P42-scan.md:263; ham önem Low | AÇIK |  |  |
| P42 | BLG-0727 | src/socialfi/vault.rs:93-95, 225-230 | P42-F16: notes/P42-scan.md:277; ham önem Low | AÇIK |  |  |
| P42 | BLG-0728 | src/socialfi/mod.rs:48-78 | P42-F17: notes/P42-scan.md:292; ham önem Low | AÇIK |  |  |
| P42 | BLG-0729 | src/budlumxyz/mod.rs:75-93 | P42-F18: notes/P42-scan.md:306; ham önem Low | AÇIK |  |  |
| P42 | BLG-0730 | src/budlumxyz/mod.rs:36-73 | P42-F19: notes/P42-scan.md:321; ham önem Low | AÇIK |  |  |
| P42 | BLG-0731 | src/execution/executor.rs:648-690 | P42-F20: notes/P42-scan.md:335; ham önem Low | AÇIK |  |  |
| P42 | BLG-0732 | src/deed/mod.rs:1-106 | P42-F21: notes/P42-scan.md:349; ham önem Low | AÇIK |  |  |
| P42 | BLG-0733 | src/gateway/atlas.rs:141-463 | P42-F22: notes/P42-scan.md:364; ham önem Low | AÇIK |  |  |
| P42 | BLG-0734 | src/rpc/server.rs:5429, 5466 | P42-F23: notes/P42-scan.md:379; ham önem Low | AÇIK |  |  |
| P42 | BLG-0735 | src/gateway/service.rs:10, 116, 143-166, 230-234, 263-274 | P42-F24: notes/P42-scan.md:393; ham önem Low | AÇIK |  |  |
| P42 | BLG-0736 | src/relayer/worker.rs:359-410, 833-850 | P42-F25: notes/P42-scan.md:408; ham önem Low | AÇIK |  |  |
| P42 | BLG-0737 | src/relayer/policy.rs:191 | P42-F26: notes/P42-scan.md:422; ham önem Info | AÇIK |  |  |
| P43 | BLG-0738 | crates/budscan/src/fetch.rs:121-127 | P43-F01: notes/P43-scan.md:59; ham önem Medium | AÇIK |  |  |
| P43 | BLG-0739 | src/pollen/offers.rs:693-720, 734-780 | P43-F02: notes/P43-scan.md:77; ham önem Medium | AÇIK |  |  |
| P43 | BLG-0740 | src/pollen/offers.rs:505-636 | P43-F03: notes/P43-scan.md:94; ham önem Medium | AÇIK |  |  |
| P43 | BLG-0741 | src/pollen/offers.rs:327-335, 693-720 | P43-F04: notes/P43-scan.md:113; ham önem Medium | AÇIK |  |  |
| P43 | BLG-0742 | crates/budscan/src/resolve.rs:192-214 | P43-F05: notes/P43-scan.md:130; ham önem Medium | AÇIK |  |  |
| P43 | BLG-0743 | crates/budscan/src/bns_proof.rs:104-117, 148-153 | P43-F06: notes/P43-scan.md:146; ham önem Low | AÇIK |  |  |
| P43 | BLG-0744 | crates/budscan/browser/patches/bud-protocol-handler.patch:36-37 (USE_LIBS), 261- | P43-F07: notes/P43-scan.md:160; ham önem Low | AÇIK |  |  |
| P43 | BLG-0745 | crates/budscan/browser/patches/address-bar-verification-badge.patch:120-130 | P43-F08: notes/P43-scan.md:176; ham önem Low | AÇIK |  |  |
| P43 | BLG-0746 | crates/budscan/src/resolve.rs:251-254 | P43-F09: notes/P43-scan.md:191; ham önem Low | AÇIK |  |  |
| P43 | BLG-0747 | src/pollen/offers.rs:418-432 | P43-F10: notes/P43-scan.md:207; ham önem Low | AÇIK |  |  |
| P43 | BLG-0748 | src/pollen/offers.rs:353-391 | P43-F11: notes/P43-scan.md:221; ham önem Low | AÇIK |  |  |
| P43 | BLG-0749 | src/pollen/data_rights.rs:196-210, 466-482 | P43-F12: notes/P43-scan.md:235; ham önem Low | AÇIK |  |  |
| P43 | BLG-0750 | src/pollen/offers.rs:815-858 | P43-F13: notes/P43-scan.md:250; ham önem Low | AÇIK |  |  |
| P43 | BLG-0751 | src/rpc/server.rs:5249-5262 | P43-F14: notes/P43-scan.md:264; ham önem Low | AÇIK |  |  |
| P43 | BLG-0752 | src/pollen/offers.rs:693-720, 734-780, 793-797 | P43-F15: notes/P43-scan.md:278; ham önem Low | AÇIK |  |  |
| P43 | BLG-0753 | crates/budscan/src/patchset.rs:210-226, 321-328 | P43-F16: notes/P43-scan.md:293; ham önem Low | AÇIK |  |  |
| P43 | BLG-0754 | crates/budscan/src/patchset.rs:210-226, 240-285 | P43-F17: notes/P43-scan.md:307; ham önem Low | AÇIK |  |  |
| P43 | BLG-0755 | crates/budscan/src/evm_audit.rs:162-197 | P43-F18: notes/P43-scan.md:321; ham önem Low | AÇIK |  |  |
| P43 | BLG-0756 | crates/budscan/src/name_rule.rs:219-240 | P43-F19: notes/P43-scan.md:337; ham önem Low | AÇIK |  |  |
| P43 | BLG-0757 | crates/budscan/src/fetch.rs:72, 110-118 | P43-F20: notes/P43-scan.md:351; ham önem Low | AÇIK |  |  |
| P43 | BLG-0758 | crates/budscan/src/light_client.rs:124-156 | P43-F21: notes/P43-scan.md:367; ham önem Low | AÇIK |  |  |
| P43 | BLG-0759 | crates/budscan/src/name_rule.rs:51-54 | P43-F22: notes/P43-scan.md:381; ham önem Info | AÇIK |  |  |
| P43 | BLG-0760 | src/pollen/data_rights.rs:142, 400, 440 | P43-F23: notes/P43-scan.md:398; ham önem Info | AÇIK |  |  |
| P44 | BLG-0761 | fuzz/fuzz_targets/zk_verifier.rs:65-87,119 | P44-F01: notes/P44-scan.md:60; ham önem Medium | AÇIK |  |  |
| P44 | BLG-0762 | fuzz/fuzz_targets/block_deserialize.rs:6 | P44-F02: notes/P44-scan.md:79; ham önem Medium | AÇIK |  |  |
| P44 | BLG-0763 | fuzz/fuzz_targets/consensus_validate.rs:46-80 | P44-F03: notes/P44-scan.md:97; ham önem Medium | AÇIK |  |  |
| P44 | BLG-0764 | fuzz/fuzz_targets/consensus_state_transition.rs:16-17,69-95,111-112 | P44-F04: notes/P44-scan.md:110; ham önem Medium | AÇIK |  |  |
| P44 | BLG-0765 | kani/src/lib.rs:304-326,463-490 | P44-F05: notes/P44-scan.md:128; ham önem Medium | AÇIK |  |  |
| P44 | BLG-0766 | kani/src/lib.rs:20-28,41-80,132-238 | P44-F06: notes/P44-scan.md:146; ham önem Medium | AÇIK |  |  |
| P44 | BLG-0767 | kani/src/lib.rs:181-222,279-291,456-461,454-490 | P44-F07: notes/P44-scan.md:163; ham önem Low | AÇIK |  |  |
| P44 | BLG-0768 | kani/src/lib.rs:341-434 | P44-F08: notes/P44-scan.md:179; ham önem Low | AÇIK |  |  |
| P44 | BLG-0769 | benches/domain_throughput.rs:557-604,497-517 | P44-F09: notes/P44-scan.md:194; ham önem Low | AÇIK |  |  |
| P44 | BLG-0770 | benches/micro/timing_safe.rs:51-62,442-481 | P44-F10: notes/P44-scan.md:212; ham önem Low | AÇIK |  |  |
| P44 | BLG-0771 | crates/bpqs/examples/dudect_poseidon2.rs:73-80,370-393,535-560,626-643 | P44-F11: notes/P44-scan.md:229; ham önem Medium | AÇIK |  |  |
| P44 | BLG-0772 | bud/kani/kani_bud_format.rs:1-53 | P44-F12: notes/P44-scan.md:245; ham önem Medium | AÇIK |  |  |
| P44 | BLG-0773 | bud/tests/test_integration.rs:288-336,338-383,385-433 | P44-F13: notes/P44-scan.md:262; ham önem Low | AÇIK |  |  |
| P44 | BLG-0774 | fuzz/README.md:4-9,22-45,69-85 | P44-F14: notes/P44-scan.md:278; ham önem Low | AÇIK |  |  |
| P44 | BLG-0775 | fuzz/fuzz_targets/bpqs_wots_reject.rs:33,57-67 | P44-F15: notes/P44-scan.md:294; ham önem Low | AÇIK |  |  |
| P44 | BLG-0776 | crates/bpqs/examples/checksum_domination_exact.rs:353-360 | P44-F16: notes/P44-scan.md:311; ham önem Info | AÇIK |  |  |
| P44 | BLG-0777 | fuzz/fuzz_targets/relayer_escrow.rs:3-6,122-127,187-191 | P44-F17: notes/P44-scan.md:326; ham önem Low | AÇIK |  |  |
| P44 | BLG-0778 | benches/single_node/internal_pipeline_tps.rs:86-125,135-141 | P44-F18: notes/P44-scan.md:342; ham önem Info | AÇIK |  |  |
| P44 | BLG-0779 | fuzz/fuzz_targets/consensus_state_transition.rs:6 | P44-F19: notes/P44-scan.md:358; ham önem Info | AÇIK |  |  |
| P44 | BLG-0780 | budzero/bud-proof/src/plonky3_prover.rs:1903 | P44-F20: notes/P44-scan.md:372; ham önem Info | AÇIK |  |  |
| P45 | BLG-0781 | src/tests/advisory_reachability.rs:459-522 | P45-F01: notes/P45-scan.md:41; ham önem Medium | AÇIK |  |  |
| P45 | BLG-0782 | src/tests/byzantine_settlement.rs:163-171 | P45-F02: notes/P45-scan.md:57; ham önem Medium | AÇIK |  |  |
| P45 | BLG-0783 | src/tests/byzantine_settlement.rs | P45-F03: notes/P45-scan.md:71; ham önem Medium | AÇIK |  |  |
| P45 | BLG-0784 | src/tests/chaos.rs:965-1000, 1002-1027, 1029-1069 | P45-F04: notes/P45-scan.md:84; ham önem Medium | AÇIK |  |  |
| P45 | BLG-0785 | src/tests/byzantine_settlement.rs:286-348, 393-487, 763-824, 910-984 | P45-F05: notes/P45-scan.md:101; ham önem Medium | AÇIK |  |  |
| P45 | BLG-0786 | src/tests/bridge_negatives.rs:144-236 | P45-F06: notes/P45-scan.md:114; ham önem Medium | AÇIK |  |  |
| P45 | BLG-0787 | src/tests/bridge_lifecycle.rs:1-5, 79-169, 171-249 | P45-F07: notes/P45-scan.md:124; ham önem Medium | AÇIK |  |  |
| P45 | BLG-0788 | src/tests/adversarial_p2p.rs:11-30, 33-43, 60-113 | P45-F08: notes/P45-scan.md:135; ham önem Medium | AÇIK |  |  |
| P45 | BLG-0789 | src/tests/bud_e2e.rs:64-77, 153-166, 228-312, 534-594 | P45-F09: notes/P45-scan.md:148; ham önem Low | AÇIK |  |  |
| P45 | BLG-0790 | src/tests/audit_findings_locks.rs:93-131, 205-237 | P45-F10: notes/P45-scan.md:158; ham önem Low | AÇIK |  |  |
| P45 | BLG-0791 | src/tests/bench_performance.rs:13-14, 160 | P45-F11: notes/P45-scan.md:168; ham önem Low | AÇIK |  |  |
| P45 | BLG-0792 | src/tests/advisory_reachability.rs:310-363, 420-456 | P45-F12: notes/P45-scan.md:178; ham önem Low | AÇIK |  |  |
| P45 | BLG-0793 | src/tests/chain_id_registry_locks.rs:220-224, 248-286 | P45-F13: notes/P45-scan.md:188; ham önem Low | AÇIK |  |  |
| P45 | BLG-0794 | src/tests/blockchair_fixture_locks.rs:782-847, 172-214 | P45-F14: notes/P45-scan.md:198; ham önem Low | AÇIK |  |  |
| P45 | BLG-0795 | src/tests/ai_inference_runtime.rs:55-86, 319-394, 592-613 | P45-F15: notes/P45-scan.md:208; ham önem Low | AÇIK |  |  |
| P45 | BLG-0796 | src/tests/chaos.rs:938, 965, 1002, 1029 | P45-F16: notes/P45-scan.md:218; ham önem Info | AÇIK |  |  |
| P45 | BLG-0797 | src/tests/bns.rs:7 | P45-F17: notes/P45-scan.md:228; ham önem Info | AÇIK |  |  |
| P46 | BLG-0798 | src/tests/consensus_expanded.rs:49-63 | P46-F01: notes/P46-scan.md:45; ham önem Medium | AÇIK |  |  |
| P46 | BLG-0799 | src/tests/finality_live_path.rs:99-124 | P46-F02: notes/P46-scan.md:63; ham önem Medium | AÇIK |  |  |
| P46 | BLG-0800 | src/tests/domain_edge_cases.rs:124-145 | P46-F03: notes/P46-scan.md:79; ham önem Medium | AÇIK |  |  |
| P46 | BLG-0801 | src/tests/domain_edge_cases.rs:25-41, 73-89, 194-216 | P46-F04: notes/P46-scan.md:97; ham önem Low | AÇIK |  |  |
| P46 | BLG-0802 | src/tests/disaster_recovery.rs:307-347, 381-392 | P46-F05: notes/P46-scan.md:114; ham önem Low | AÇIK |  |  |
| P46 | BLG-0803 | src/tests/hardening.rs:340-360 | P46-F06: notes/P46-scan.md:133; ham önem Low | AÇIK |  |  |
| P46 | BLG-0804 | src/tests/hardening_h5_h7_locks.rs:141-150, 215-227 | P46-F07: notes/P46-scan.md:150; ham önem Low | AÇIK |  |  |
| P46 | BLG-0805 | src/tests/consensus_lock_order_loom.rs:47-239, 251-286 | P46-F08: notes/P46-scan.md:165; ham önem Low | AÇIK |  |  |
| P46 | BLG-0806 | src/tests/finality_adversarial.rs:24 | P46-F09: notes/P46-scan.md:181; ham önem Low | AÇIK |  |  |
| P46 | BLG-0807 | src/tests/hash_input_serialize_locks.rs:30-60 | P46-F10: notes/P46-scan.md:196; ham önem Low | AÇIK |  |  |
| P46 | BLG-0808 | src/tests/distributed_settlement.rs:87-145, 270-318, 320-392 | P46-F11: notes/P46-scan.md:213; ham önem Low | AÇIK |  |  |
| P46 | BLG-0809 | src/tests/fork_choice_locks.rs:40-232 | P46-F12: notes/P46-scan.md:230; ham önem Info | AÇIK |  |  |
| P46 | BLG-0810 | src/tests/finality_live_path.rs:131-156 | P46-F13: notes/P46-scan.md:246; ham önem Low | AÇIK |  |  |
| P46 | BLG-0811 | src/tests/identity_tx_door.rs:209-213 | P46-F14: notes/P46-scan.md:261; ham önem Info | AÇIK |  |  |
| P46 | BLG-0812 | src/tests/finality_adversarial.rs:548, 743 | P46-F15: notes/P46-scan.md:277; ham önem Info | AÇIK |  |  |
| P47 | BLG-0813 | src/tests/load_test.rs:136-159 | P47-F01: notes/P47-scan.md:35; ham önem Medium | AÇIK |  |  |
| P47 | BLG-0814 | src/tests/manifest_commitment_locks.rs:1056-1081 | P47-F02: notes/P47-scan.md:55; ham önem Medium | AÇIK |  |  |
| P47 | BLG-0815 | src/tests/multi_consensus_locks.rs:284-303 | P47-F03: notes/P47-scan.md:74; ham önem Medium | AÇIK |  |  |
| P47 | BLG-0816 | src/tests/permissionless.rs:967-984 | P47-F04: notes/P47-scan.md:92; ham önem Medium | AÇIK |  |  |
| P47 | BLG-0817 | src/tests/permissionless.rs:1008-1015 | P47-F05: notes/P47-scan.md:110; ham önem Low | AÇIK |  |  |
| P47 | BLG-0818 | src/tests/operator_kill_chaos.rs:246-252 | P47-F06: notes/P47-scan.md:125; ham önem Low | AÇIK |  |  |
| P47 | BLG-0819 | src/tests/integration.rs:322-369 | P47-F07: notes/P47-scan.md:140; ham önem Low | AÇIK |  |  |
| P47 | BLG-0820 | src/tests/integration.rs:256-277 | P47-F08: notes/P47-scan.md:157; ham önem Low | AÇIK |  |  |
| P47 | BLG-0821 | src/tests/network_hardening_locks.rs:253-266, 200-203 | P47-F09: notes/P47-scan.md:173; ham önem Low | AÇIK |  |  |
| P47 | BLG-0822 | src/tests/integration.rs:474-490 | P47-F10: notes/P47-scan.md:188; ham önem Low | AÇIK |  |  |
| P47 | BLG-0823 | src/tests/liveness_consensus.rs:353-383, 400-449 | P47-F11: notes/P47-scan.md:205; ham önem Low | AÇIK |  |  |
| P47 | BLG-0824 | src/tests/integration.rs:627-734 | P47-F12: notes/P47-scan.md:221; ham önem Info | AÇIK |  |  |
| P47 | BLG-0825 | src/tests/operator_kill_chaos.rs:1-19, 351-370 | P47-F13: notes/P47-scan.md:231; ham önem Info | AÇIK |  |  |
| P47 | BLG-0826 | src/tests/integration.rs:1573-1584, 1673, 1256-1266, 1866-1884 | P47-F14: notes/P47-scan.md:241; ham önem Info | AÇIK |  |  |
| P47 | BLG-0827 | src/tests/load_test.rs:163-248 | P47-F15: notes/P47-scan.md:251; ham önem Low | AÇIK |  |  |
| P47 | BLG-0828 | src/tests/integration.rs:1344-1362 | P47-F16: notes/P47-scan.md:261; ham önem Low | AÇIK |  |  |
| P47 | BLG-0829 | src/tests/integration.rs:1319-1342 | P47-F17: notes/P47-scan.md:271; ham önem Low | AÇIK |  |  |
| P47 | BLG-0830 | src/tests/manifest_commitment_locks.rs:50-63 | P47-F18: notes/P47-scan.md:282; ham önem Info | AÇIK |  |  |
| P47 | BLG-0831 | src/tests/mempool_dos_locks.rs:165-183 | P47-F19: notes/P47-scan.md:292; ham önem Low | AÇIK |  |  |
| P47 | BLG-0832 | src/tests/permissionless_e2e.rs:50-59 | P47-F20: notes/P47-scan.md:302; ham önem Info | AÇIK |  |  |
| P48 | BLG-0833 | src/tests/qcblob_quorum.rs:157-201 | P48-F01: notes/P48-scan.md:43; ham önem Medium | AÇIK |  |  |
| P48 | BLG-0834 | src/tests/relayer_gates.rs:170-183 | P48-F02: notes/P48-scan.md:56; ham önem Low | AÇIK |  |  |
| P48 | BLG-0835 | src/tests/privacy_ai_execution.rs:28-141 | P48-F03: notes/P48-scan.md:68; ham önem Medium | AÇIK |  |  |
| P48 | BLG-0836 | src/tests/relayer_liveness.rs:151-170, 215-312, 453-467, 471-529 | P48-F04: notes/P48-scan.md:80; ham önem Medium | AÇIK |  |  |
| P48 | BLG-0837 | src/tests/poa_isolation.rs:24-286 | P48-F05: notes/P48-scan.md:93; ham önem Medium | AÇIK |  |  |
| P48 | BLG-0838 | src/tests/prover.rs:683-709 | P48-F06: notes/P48-scan.md:111; ham önem Low | AÇIK |  |  |
| P48 | BLG-0839 | src/tests/prover.rs:1-9, 119-168, 249-280 | P48-F07: notes/P48-scan.md:127; ham önem Info | AÇIK |  |  |
| P48 | BLG-0840 | src/tests/relayer_e2e.rs:74-98, 219-236, 238-272, 379-443 | P48-F08: notes/P48-scan.md:139; ham önem Low | AÇIK |  |  |
| P48 | BLG-0841 | src/tests/relayer_e2e.rs:294-301, 347-358, 373-376 | P48-F09: notes/P48-scan.md:154; ham önem Info | AÇIK |  |  |
| P48 | BLG-0842 | src/tests/real_chain_fixtures.rs:84-112, 150-166, 232-255, 257-295 | P48-F10: notes/P48-scan.md:166; ham önem Low | AÇIK |  |  |
| P48 | BLG-0843 | src/tests/persistence.rs:35-40, 313-336 | P48-F11: notes/P48-scan.md:178; ham önem Low | AÇIK |  |  |
| P48 | BLG-0844 | src/tests/proptest_core.rs:21-38 | P48-F12: notes/P48-scan.md:190; ham önem Low | AÇIK |  |  |
| P48 | BLG-0845 | src/tests/regeneration_recovery.rs:25-36, 108, 178 | P48-F13: notes/P48-scan.md:203; ham önem Low | AÇIK |  |  |
| P48 | BLG-0846 | src/tests/regression_lock.rs:330-356, 366-441, 595-643, 7-22 | P48-F14: notes/P48-scan.md:219; ham önem Low | AÇIK |  |  |
| P48 | BLG-0847 | src/tests/pollen_ai_data_rights.rs:114-320 | P48-F15: notes/P48-scan.md:233; ham önem Low | AÇIK |  |  |
| P48 | BLG-0848 | src/tests/privacy_ai_execution.rs:143-218 | P48-F16: notes/P48-scan.md:247; ham önem Info | AÇIK |  |  |
| P48 | BLG-0849 | src/tests/private_transfer_fee_market.rs:17-61 | P48-F17: notes/P48-scan.md:259; ham önem Info | AÇIK |  |  |
| P48 | BLG-0850 | src/tests/poa_onboarding_matrix.rs:163-190, 240-245 | P48-F18: notes/P48-scan.md:270; ham önem Info | AÇIK |  |  |
| P48 | BLG-0851 | src/tests/poisoned_lock_locks.rs:66-88, 96-109, 227-244 | P48-F19: notes/P48-scan.md:281; ham önem Low | AÇIK |  |  |
| P48 | BLG-0852 | src/tests/relayer_gates.rs:1-9 | P48-F20: notes/P48-scan.md:293; ham önem Info | AÇIK |  |  |
| P49 | BLG-0853 | src/tests/settlement_prod.rs:1-3 | P49-F01: notes/P49-scan.md:40; ham önem High | AÇIK |  |  |
| P49 | BLG-0854 | src/tests/tokenomics_proptest.rs:33-67 | P49-F02: notes/P49-scan.md:59; ham önem Medium | AÇIK |  |  |
| P49 | BLG-0855 | src/tests/tokenomics_proptest.rs:74-92, 100-122 | P49-F03: notes/P49-scan.md:73; ham önem Low | AÇIK |  |  |
| P49 | BLG-0856 | src/tests/security_auditor.rs:58-199 | P49-F04: notes/P49-scan.md:89; ham önem Medium | AÇIK |  |  |
| P49 | BLG-0857 | src/tests/storage_tx_door.rs:429-446 | P49-F05: notes/P49-scan.md:107; ham önem Medium | AÇIK |  |  |
| P49 | BLG-0858 | src/tests/slashing_matrix.rs:26-59, 82-131, 135-166, 236-265 | P49-F06: notes/P49-scan.md:127; ham önem Medium | AÇIK |  |  |
| P49 | BLG-0859 | src/tests/snapshot_chaos.rs:108-134, 224-243 | P49-F07: notes/P49-scan.md:144; ham önem Medium | AÇIK |  |  |
| P49 | BLG-0860 | src/tests/replay_audit.rs:62-69, 88-97, 99-106 | P49-F08: notes/P49-scan.md:162; ham önem Medium | AÇIK |  |  |
| P49 | BLG-0861 | src/tests/wall_clock_locks.rs:50-125 | P49-F09: notes/P49-scan.md:180; ham önem Low | AÇIK |  |  |
| P49 | BLG-0862 | src/tests/relayer_worker_locks.rs:1001-1013, 553-571, 641-698 | P49-F10: notes/P49-scan.md:193; ham önem Low | AÇIK |  |  |
| P49 | BLG-0863 | src/tests/v95_v98_canaries.rs:9-22 | P49-F11: notes/P49-scan.md:208; ham önem Low | AÇIK |  |  |
| P49 | BLG-0864 | src/tests/target_700.rs:88-102 | P49-F12: notes/P49-scan.md:223; ham önem Low | AÇIK |  |  |
| P49 | BLG-0865 | src/tests/storage_tx_door.rs:481-865 | P49-F13: notes/P49-scan.md:235; ham önem Info | AÇIK |  |  |
| P49 | BLG-0866 | src/tests/sharding_e2e.rs:113-157, 32-111 | P49-F14: notes/P49-scan.md:248; ham önem Low | AÇIK |  |  |
| P49 | BLG-0867 | src/tests/tokenomics.rs:293-295, 7-12 | P49-F15: notes/P49-scan.md:264; ham önem Info | AÇIK |  |  |
| P49 | BLG-0868 | src/tests/zkvm.rs:56-68, 189-216 | P49-F16: notes/P49-scan.md:276; ham önem Info | AÇIK |  |  |
| P49 | BLG-0869 | docs/audit/FINDINGS.md:226 | P49-F17: notes/P49-scan.md:288; ham önem Info | AÇIK |  |  |
| P50 | BLG-0870 | xtask/gates/src/gates/air_selectors.rs:119-141, 181-189 | P50-F01: notes/P50-scan.md:45; ham önem Medium | AÇIK |  |  |
| P50 | BLG-0871 | xtask/gates/src/gates/ci_workflow_guards.rs:123-135, 148-151 | P50-F02: notes/P50-scan.md:56; ham önem Medium | AÇIK |  |  |
| P50 | BLG-0872 | xtask/gates/src/gates/ci_workflow_guards.rs:40-101, 137-204 | P50-F03: notes/P50-scan.md:67; ham önem Medium | AÇIK |  |  |
| P50 | BLG-0873 | xtask/gates/src/gates/budscan_parity.rs:160-165, 175-183, 188-197 | P50-F04: notes/P50-scan.md:78; ham önem Medium | AÇIK |  |  |
| P50 | BLG-0874 | xtask/gates/src/gates/chain_id_is_not_hardcoded.rs:46-78 | P50-F05: notes/P50-scan.md:89; ham önem Medium | AÇIK |  |  |
| P50 | BLG-0875 | xtask/gates/src/gates/chain_id_is_not_hardcoded.rs:83-110 | P50-F06: notes/P50-scan.md:100; ham önem Low | AÇIK |  |  |
| P50 | BLG-0876 | xtask/gates/src/gates/capability_modules_are_wired.rs:55, 575-581, 656, 704-715, 85 | P50-F07: notes/P50-scan.md:110; ham önem Medium | AÇIK |  |  |
| P50 | BLG-0877 | xtask/gates/src/gates/capability_modules_are_wired.rs:604-610, 717-731 | P50-F08: notes/P50-scan.md:120; ham önem Info | AÇIK |  |  |
| P50 | BLG-0878 | xtask/gates/src/gates/ast_security_gates.rs:368-403, 328-357 | P50-F09: notes/P50-scan.md:130; ham önem Medium | AÇIK |  |  |
| P50 | BLG-0879 | xtask/gates/src/gates/ast_security_gates.rs:461-473, 540-554, 415-425 | P50-F10: notes/P50-scan.md:141; ham önem Medium | AÇIK |  |  |
| P50 | BLG-0880 | xtask/gates/src/gates/ast_security_gates.rs:69-123, 254-295, 415-456, 138-147 | P50-F11: notes/P50-scan.md:152; ham önem Low | AÇIK |  |  |
| P50 | BLG-0881 | xtask/gates/src/gates/bit_decompositions.rs:168-206, 213-300, 95-110, 344-352 | P50-F12: notes/P50-scan.md:162; ham önem Medium | AÇIK |  |  |
| P50 | BLG-0882 | xtask/gates/src/gates/accumulators_pinned.rs:37-64, 91-93, 251-256, 172-215 | P50-F13: notes/P50-scan.md:173; ham önem Medium | AÇIK |  |  |
| P50 | BLG-0883 | xtask/gates/src/gates/badges_current.rs:32-49, 134-183, 225-244 | P50-F14: notes/P50-scan.md:184; ham önem Low | AÇIK |  |  |
| P50 | BLG-0884 | xtask/gates/src/gates/cargo_vet.rs:80-117 | P50-F15: notes/P50-scan.md:194; ham önem Low | AÇIK |  |  |
| P50 | BLG-0885 | xtask/gates/src/gates/bns_names_are_safe_in_an_address_bar.rs:172-223 | P50-F16: notes/P50-scan.md:204; ham önem Medium | AÇIK |  |  |
| P50 | BLG-0886 | xtask/gates/src/gates/bns_names_are_safe_in_an_address_bar.rs:75-79, 160-167 | P50-F17: notes/P50-scan.md:214; ham önem Low | AÇIK |  |  |
| P50 | BLG-0887 | xtask/gates/src/gates/ai_inference_prompt_is_true.rs:458-488, 594-621, 416-441, 322-338 | P50-F18: notes/P50-scan.md:224; ham önem Low | AÇIK |  |  |
| P50 | BLG-0888 | xtask/gates/src/gates/ai_inference_reads.rs:77-111, 152-163 | P50-F19: notes/P50-scan.md:234; ham önem Low | AÇIK |  |  |
| P50 | BLG-0889 | xtask/gates/src/gates/ast_security_gates.rs:297, 483 | P50-F20: notes/P50-scan.md:244; ham önem Low | AÇIK |  |  |
| P50 | BLG-0890 | xtask/gates/src/gates/binding_claims.rs:100-140, 172-192, 37-71 | P50-F21: notes/P50-scan.md:254; ham önem Low | AÇIK |  |  |
| P50 | BLG-0891 | xtask/gates/src/gates/budscan_parity.rs:205-242 | P50-F22: notes/P50-scan.md:264; ham önem Low | AÇIK |  |  |
| P50 | BLG-0892 | xtask/gates/src/gates/capability_modules_are_wired.rs:631 | P50-F23: notes/P50-scan.md:274; ham önem Low | AÇIK |  |  |
| P50 | BLG-0893 | xtask/gates/src/gates/budscan_patchset.rs:55, 69, 73-74 | P50-F24: notes/P50-scan.md:284; ham önem Info | AÇIK |  |  |
| P51 | BLG-0894 | xtask/gates/src/gates/forgery_tests.rs:130-137 | P51-F01: notes/P51-scan.md:49; ham önem Medium | AÇIK |  |  |
| P51 | BLG-0895 | xtask/gates/src/gates/containment_defaults.rs:77-82 | P51-F02: notes/P51-scan.md:59; ham önem Medium | AÇIK |  |  |
| P51 | BLG-0896 | xtask/gates/src/gates/evidence_provenance.rs:171 | P51-F03: notes/P51-scan.md:69; ham önem Medium | AÇIK |  |  |
| P51 | BLG-0897 | xtask/gates/src/gates/dead_pub_api.rs:211-215 | P51-F04: notes/P51-scan.md:79; ham önem Medium | AÇIK |  |  |
| P51 | BLG-0898 | xtask/gates/src/gates/economy_invariants.rs:41 | P51-F05: notes/P51-scan.md:89; ham önem Medium | AÇIK |  |  |
| P51 | BLG-0899 | xtask/gates/src/gates/coverage.rs:32-43 | P51-F06: notes/P51-scan.md:99; ham önem Medium | AÇIK |  |  |
| P51 | BLG-0900 | xtask/gates/src/gates/every_opcode_forgery.rs:150, 215-227, 245 | P51-F07: notes/P51-scan.md:109; ham önem Medium | AÇIK |  |  |
| P51 | BLG-0901 | xtask/gates/src/gates/consensus_maps_ordered.rs:26-33, 185-216, 431-437, 467-492 | P51-F08: notes/P51-scan.md:119; ham önem Medium | AÇIK |  |  |
| P51 | BLG-0902 | xtask/gates/src/gates/coding_audit_samples_the_relationship.rs:119-156 | P51-F09: notes/P51-scan.md:129; ham önem Medium | AÇIK |  |  |
| P51 | BLG-0903 | xtask/gates/src/gates/docker_toolchain.rs:88-103 | P51-F10: notes/P51-scan.md:139; ham önem Medium | AÇIK |  |  |
| P51 | BLG-0904 | xtask/gates/src/gates/dead_pub_api.rs:216-218, 248-259 | P51-F11: notes/P51-scan.md:149; ham önem Low | AÇIK |  |  |
| P51 | BLG-0905 | xtask/gates/src/gates/coverage.rs:10-24, 61 | P51-F12: notes/P51-scan.md:159; ham önem Low | AÇIK |  |  |
| P51 | BLG-0906 | xtask/gates/src/gates/consensus_state_only_changes_in_blocks.rs:27-33, 75-125 | P51-F13: notes/P51-scan.md:169; ham önem Low | AÇIK |  |  |
| P51 | BLG-0907 | xtask/gates/src/gates/domain_tags.rs:44-78, 91-119, 137 | P51-F14: notes/P51-scan.md:179; ham önem Low | AÇIK |  |  |
| P51 | BLG-0908 | xtask/gates/src/gates/every_fuzz_target_is_run.rs:111-121, 169-184 | P51-F15: notes/P51-scan.md:189; ham önem Low | AÇIK |  |  |
| P51 | BLG-0909 | xtask/gates/src/gates/gas_calibration.rs:63-70, 73-93, 30-44 | P51-F16: notes/P51-scan.md:199; ham önem Low | AÇIK |  |  |
| P51 | BLG-0910 | xtask/gates/src/gates/fsf_import_boundaries.rs:120-129, 159-160, 232-244 | P51-F17: notes/P51-scan.md:209; ham önem Low | AÇIK |  |  |
| P51 | BLG-0911 | xtask/gates/src/gates/content_encryption_is_declared_and_bound.rs:155-185, 377-401 | P51-F18: notes/P51-scan.md:219; ham önem Low | AÇIK |  |  |
| P51 | BLG-0912 | xtask/gates/src/gates/containment_defaults.rs:16-21, 35-45 | P51-F19: notes/P51-scan.md:229; ham önem Low | AÇIK |  |  |
| P51 | BLG-0913 | xtask/gates/src/gates/cross_table_checks.rs:19-48 | P51-F20: notes/P51-scan.md:239; ham önem Low | AÇIK |  |  |
| P51 | BLG-0914 | xtask/gates/src/gates/consensus_maps_ordered.rs:376-382, 386, 516-519 | P51-F21: notes/P51-scan.md:249; ham önem Low | AÇIK |  |  |
| P51 | BLG-0915 | xtask/gates/src/gates/gates_are_wired.rs:396-430, 337-384 | P51-F22: notes/P51-scan.md:259; ham önem Info | AÇIK |  |  |
| P52 | BLG-0916 | xtask/gates/src/gates/minting_paths_are_counted.rs:28-32, 90-100, 117-121, 149-154 | P52-F01: notes/P52-scan.md:50; ham önem Medium | AÇIK |  |  |
| P52 | BLG-0917 | xtask/gates/src/gates/lock_failures.rs:11 | P52-F02: notes/P52-scan.md:68; ham önem Medium | AÇIK |  |  |
| P52 | BLG-0918 | xtask/gates/src/gates/module_coverage.rs:25-43, 185-196 | P52-F03: notes/P52-scan.md:84; ham önem Medium | AÇIK |  |  |
| P52 | BLG-0919 | xtask/gates/src/gates/guards_reachable.rs:128-147, 175-178, 186-203, 230-249 | P52-F04: notes/P52-scan.md:102; ham önem Medium | AÇIK |  |  |
| P52 | BLG-0920 | xtask/gates/src/gates/guards_reachable.rs:57 | P52-F05: notes/P52-scan.md:120; ham önem Medium | AÇIK |  |  |
| P52 | BLG-0921 | xtask/gates/src/gates/logup_multipliers.rs:30-39, 88-131, 193-202 | P52-F06: notes/P52-scan.md:137; ham önem Medium | AÇIK |  |  |
| P52 | BLG-0922 | xtask/gates/src/gates/named_tests.rs:17-22, 35-39 | P52-F07: notes/P52-scan.md:155; ham önem Medium | AÇIK |  |  |
| P52 | BLG-0923 | xtask/gates/src/gates/indexing_is_not_new.rs:53-120, 129-152, 176-211, 264-266 | P52-F08: notes/P52-scan.md:170; ham önem Medium | AÇIK |  |  |
| P52 | BLG-0924 | xtask/gates/src/gates/hash_inputs_are_length_prefixed.rs:63, 219-240, 267-295, 381-403 | P52-F09: notes/P52-scan.md:186; ham önem Medium | AÇIK |  |  |
| P52 | BLG-0925 | xtask/gates/src/gates/lock_failures.rs:24-26, 28, 59-62, 140-148 | P52-F10: notes/P52-scan.md:203; ham önem Low | AÇIK |  |  |
| P52 | BLG-0926 | xtask/gates/src/gates/gov_slash_evidence_is_validator_only.rs:67-134, 193-209 | P52-F11: notes/P52-scan.md:221; ham önem Low | AÇIK |  |  |
| P52 | BLG-0927 | xtask/gates/src/gates/gating_flags.rs:12, 49-63, 78-125 | P52-F12: notes/P52-scan.md:236; ham önem Low | AÇIK |  |  |
| P52 | BLG-0928 | xtask/gates/src/gates/gates_do_not_panic.rs:97-106, 133-134, 153-158, 246-248 | P52-F13: notes/P52-scan.md:252; ham önem Low | AÇIK |  |  |
| P52 | BLG-0929 | xtask/gates/src/gates/git_deps_audited.rs:28, 32, 35-43 | P52-F14: notes/P52-scan.md:268; ham önem Low | AÇIK |  |  |
| P52 | BLG-0930 | xtask/gates/src/gates/multi_ratio_consensus.rs:13-15, 86-108, 125, 139, 164-185 | P52-F15: notes/P52-scan.md:284; ham önem Low | AÇIK |  |  |
| P52 | BLG-0931 | xtask/gates/src/gates/metrics_are_written.rs:107-176, 295-322, 340 | P52-F16: notes/P52-scan.md:301; ham önem Low | AÇIK |  |  |
| P52 | BLG-0932 | xtask/gates/src/gates/generated_content.rs:27, 54-56, 90 | P52-F17: notes/P52-scan.md:316; ham önem Low | AÇIK |  |  |
| P52 | BLG-0933 | xtask/gates/src/gates/license_consistency.rs:60, 77-83, 134-136, 153-182, 199 | P52-F18: notes/P52-scan.md:333; ham önem Low | AÇIK |  |  |
| P52 | BLG-0934 | xtask/gates/src/gates/no_conflict_markers.rs:26, 37-39, 45-50, 108-111 | P52-F19: notes/P52-scan.md:349; ham önem Low | AÇIK |  |  |
| P52 | BLG-0935 | xtask/gates/src/gates/kani.rs:63-92, 279-292 | P52-F20: notes/P52-scan.md:365; ham önem Low | AÇIK |  |  |
| P52 | BLG-0936 | xtask/gates/src/gates/genesis_schema.rs:68-73, 100-134, 143 | P52-F21: notes/P52-scan.md:381; ham önem Low | AÇIK |  |  |
| P52 | BLG-0937 | xtask/gates/src/gates/geiger.rs:27-41 | P52-F22: notes/P52-scan.md:395; ham önem Low | AÇIK |  |  |
| P52 | BLG-0938 | xtask/gates/src/gates/mermaid.rs:312, 363-366 | P52-F23: notes/P52-scan.md:410; ham önem Info | AÇIK |  |  |
| P53 | BLG-0939 | xtask/gates/src/gates/no_idle_code.rs:117-128, 728-788, 986-993 | P53-F01: notes/P53-scan.md:43; ham önem Medium | AÇIK |  |  |
| P53 | BLG-0940 | xtask/gates/src/gates/no_idle_code.rs:93-102, 161-174 | P53-F02: notes/P53-scan.md:53; ham önem Low | AÇIK |  |  |
| P53 | BLG-0941 | xtask/gates/src/gates/no_idle_code.rs:906-908, 112 | P53-F03: notes/P53-scan.md:63; ham önem Low | AÇIK |  |  |
| P53 | BLG-0942 | xtask/gates/src/gates/no_orphan_source_files.rs:64, 577-583 | P53-F04: notes/P53-scan.md:73; ham önem Medium | AÇIK |  |  |
| P53 | BLG-0943 | xtask/gates/src/gates/no_orphan_source_files.rs:439-463, 465-488 | P53-F05: notes/P53-scan.md:83; ham önem Low | AÇIK |  |  |
| P53 | BLG-0944 | xtask/gates/src/gates/no_new_shell_gates.rs:32-52 | P53-F06: notes/P53-scan.md:93; ham önem Low | AÇIK |  |  |
| P53 | BLG-0945 | xtask/gates/src/gates/no_unicode_dashes.rs:24-39, 99-103 | P53-F07: notes/P53-scan.md:103; ham önem Low | AÇIK |  |  |
| P53 | BLG-0946 | xtask/gates/src/gates/no_upstream_brands.rs:174-199, 211-212, 61-95 | P53-F08: notes/P53-scan.md:113; ham önem Low | AÇIK |  |  |
| P53 | BLG-0947 | xtask/gates/src/gates/named_tests.rs:15-27 | P53-F09: notes/P53-scan.md:123; ham önem Low | AÇIK |  |  |
| P53 | BLG-0948 | xtask/gates/src/gates/one_house_guards.rs:125-160 | P53-F10: notes/P53-scan.md:133; ham önem Medium | AÇIK |  |  |
| P53 | BLG-0949 | xtask/gates/src/gates/one_house_guards.rs:241-267, 270-281, 293-312 | P53-F11: notes/P53-scan.md:143; ham önem Medium | AÇIK |  |  |
| P53 | BLG-0950 | xtask/gates/src/gates/paid_content.rs:316-326, 361-372 | P53-F12: notes/P53-scan.md:153; ham önem Medium | AÇIK |  |  |
| P53 | BLG-0951 | xtask/gates/src/gates/paid_content.rs:43-61, 73-89, 146-188, 220-245, 279-358 | P53-F13: notes/P53-scan.md:163; ham önem Medium | AÇIK |  |  |
| P53 | BLG-0952 | xtask/gates/src/gates/parallel_execution.rs:117-136, 193-209 | P53-F14: notes/P53-scan.md:173; ham önem Medium | AÇIK |  |  |
| P53 | BLG-0953 | xtask/gates/src/gates/parallel_execution.rs:214-216, 160-172 | P53-F15: notes/P53-scan.md:183; ham önem Medium | AÇIK |  |  |
| P53 | BLG-0954 | xtask/gates/src/gates/parallel_execution.rs:223-238, 41-110 | P53-F16: notes/P53-scan.md:193; ham önem Low | AÇIK |  |  |
| P53 | BLG-0955 | xtask/gates/src/gates/pinned_downloads.rs:458-465, 471-502, 499 | P53-F17: notes/P53-scan.md:203; ham önem Medium | AÇIK |  |  |
| P53 | BLG-0956 | xtask/gates/src/gates/proof_deps_are_exactly_pinned.rs:27, 44-66, 90 | P53-F18: notes/P53-scan.md:213; ham önem Medium | AÇIK |  |  |
| P53 | BLG-0957 | xtask/gates/src/gates/qr_is_derivative_only.rs:186-199, 208-217 | P53-F19: notes/P53-scan.md:223; ham önem Low | AÇIK |  |  |
| P53 | BLG-0958 | xtask/gates/src/gates/quantum_account.rs:305-324, 335-394 | P53-F20: notes/P53-scan.md:233; ham önem Low | AÇIK |  |  |
| P53 | BLG-0959 | xtask/gates/src/gates/readme_no_deny.rs:518-522, 465-483, 485-500 | P53-F21: notes/P53-scan.md:243; ham önem Low | AÇIK |  |  |
| P53 | BLG-0960 | xtask/gates/src/gates/reduction_claims.rs:12-31, 88-102 | P53-F22: notes/P53-scan.md:253; ham önem Low | AÇIK |  |  |
| P53 | BLG-0961 | xtask/gates/src/gates/refusals_no_mutate.rs:267-274, 281-338, 387-421 | P53-F23: notes/P53-scan.md:263; ham önem Medium | AÇIK |  |  |
| P53 | BLG-0962 | xtask/gates/src/gates/refusals_no_mutate.rs:12, 196-246 | P53-F24: notes/P53-scan.md:273; ham önem Medium | AÇIK |  |  |
| P53 | BLG-0963 | xtask/gates/src/gates/refusals_no_mutate.rs:162-193, 429 | P53-F25: notes/P53-scan.md:283; ham önem Low | AÇIK |  |  |
| P53 | BLG-0964 | xtask/gates/src/gates/regen_health_door.rs:245-275 | P53-F26: notes/P53-scan.md:293; ham önem Medium | AÇIK |  |  |
| P53 | BLG-0965 | xtask/gates/src/gates/regen_health_door.rs:276-286, 215-220 | P53-F27: notes/P53-scan.md:303; ham önem Low | AÇIK |  |  |
| P54 | BLG-0966 | .github/semver-exceptions.txt:20 | P54-F01: notes/P54-scan.md:35; ham önem Medium | AÇIK |  |  |
| P54 | BLG-0967 | xtask/gates/src/gates/semver.rs:233-238 | P54-F02: notes/P54-scan.md:52; ham önem Medium | AÇIK |  |  |
| P54 | BLG-0968 | xtask/gates/src/gates/semver.rs:212-219 | P54-F03: notes/P54-scan.md:68; ham önem Low | AÇIK |  |  |
| P54 | BLG-0969 | xtask/gates/src/gates/required_tests.rs:190-243 | P54-F04: notes/P54-scan.md:82; ham önem Medium | AÇIK |  |  |
| P54 | BLG-0970 | xtask/gates/src/gates/relay.rs:428-467 | P54-F05: notes/P54-scan.md:99; ham önem Medium | AÇIK |  |  |
| P54 | BLG-0971 | xtask/gates/src/gates/relay.rs:215-220, 242, 326, 397-398 | P54-F06: notes/P54-scan.md:117; ham önem Low | AÇIK |  |  |
| P54 | BLG-0972 | xtask/gates/src/gates/security_scans_can_fail.rs:166-219 | P54-F07: notes/P54-scan.md:133; ham önem Medium | AÇIK |  |  |
| P54 | BLG-0973 | xtask/gates/src/gates/security_scans_can_fail.rs:237-264, 283-294 | P54-F08: notes/P54-scan.md:149; ham önem Medium | AÇIK |  |  |
| P54 | BLG-0974 | xtask/gates/src/gates/rust_literals.rs:171-177 | P54-F09: notes/P54-scan.md:164; ham önem Medium | AÇIK |  |  |
| P54 | BLG-0975 | xtask/gates/src/gates/regeneration.rs:77-80, 775-806 | P54-F10: notes/P54-scan.md:182; ham önem Medium | AÇIK |  |  |
| P54 | BLG-0976 | xtask/gates/src/gates/regeneration.rs:812-828, 842-881, 1103-1144 | P54-F11: notes/P54-scan.md:197; ham önem Medium | AÇIK |  |  |
| P54 | BLG-0977 | xtask/gates/src/gates/regeneration.rs:664-747, 889-898 | P54-F12: notes/P54-scan.md:215; ham önem Low | AÇIK |  |  |
| P54 | BLG-0978 | xtask/gates/src/gates/security_parameters.rs:241-265, 744-763, 837-847 | P54-F13: notes/P54-scan.md:232; ham önem Medium | AÇIK |  |  |
| P54 | BLG-0979 | xtask/gates/src/gates/security_parameters.rs:461-530, 574-609, 704-739, 864-866 | P54-F14: notes/P54-scan.md:248; ham önem Low | AÇIK |  |  |
| P54 | BLG-0980 | xtask/gates/src/gates/self_derived_ids_cover_every_field.rs:238-276, 297-316, 346-374 | P54-F15: notes/P54-scan.md:264; ham önem Medium | AÇIK |  |  |
| P54 | BLG-0981 | xtask/gates/src/gates/self_derived_ids_cover_every_field.rs:45, 421-473 | P54-F16: notes/P54-scan.md:280; ham önem Low | AÇIK |  |  |
| P54 | BLG-0982 | xtask/gates/src/gates/rejection_tests.rs:243-248 | P54-F17: notes/P54-scan.md:296; ham önem Medium | AÇIK |  |  |
| P54 | BLG-0983 | xtask/gates/src/gates/rejection_tests.rs:23, 63-148 | P54-F18: notes/P54-scan.md:315; ham önem Low | AÇIK |  |  |
| P54 | BLG-0984 | xtask/gates/src/gates/release_profile_pins.rs:100-107 | P54-F19: notes/P54-scan.md:331; ham önem Low | AÇIK |  |  |
| P54 | BLG-0985 | xtask/gates/src/gates/repair_fires.rs:11-110 | P54-F20: notes/P54-scan.md:347; ham önem Low | AÇIK |  |  |
| P54 | BLG-0986 | xtask/gates/src/gates/serialize_map_keys_are_strings.rs:136-155, 184-228, 290-326, 376 | P54-F21: notes/P54-scan.md:364; ham önem Low | AÇIK |  |  |
| P54 | BLG-0987 | xtask/gates/src/gates/semver.rs:201-204, 171 | P54-F22: notes/P54-scan.md:380; ham önem Info | AÇIK |  |  |
| P54 | BLG-0988 | xtask/gates/src/gates/semver.rs:320-338 | P54-F23: notes/P54-scan.md:396; ham önem Info | AÇIK |  |  |
| P55 | BLG-0989 | xtask/gates/src/gates/udeps.rs:73-80 | P55-F01: notes/P55-scan.md:46; ham önem Medium | AÇIK |  |  |
| P55 | BLG-0990 | xtask/gates/src/gates/udeps.rs:48-72 | P55-F02: notes/P55-scan.md:56; ham önem Low | AÇIK |  |  |
| P55 | BLG-0991 | xtask/gates/src/gates/suppressions_are_justified.rs:687 | P55-F03: notes/P55-scan.md:66; ham önem Medium | AÇIK |  |  |
| P55 | BLG-0992 | xtask/gates/src/gates/suppressions_are_justified.rs:554-603, 708 | P55-F04: notes/P55-scan.md:76; ham önem Low | AÇIK |  |  |
| P55 | BLG-0993 | xtask/gates/src/gates/slash_expression.rs:147-151, 134 | P55-F05: notes/P55-scan.md:86; ham önem Medium | AÇIK |  |  |
| P55 | BLG-0994 | xtask/gates/src/gates/slash_expression.rs:310, 346-347 | P55-F06: notes/P55-scan.md:96; ham önem Low | AÇIK |  |  |
| P55 | BLG-0995 | xtask/gates/src/gates/threshold_rates.rs:83-96, 186-202 | P55-F07: notes/P55-scan.md:106; ham önem Medium | AÇIK |  |  |
| P55 | BLG-0996 | storage_penalties.rs:347-349; uncheckable_proof.rs:29-31; threshold_rates.rs:206 | P55-F08: notes/P55-scan.md:116; ham önem Medium | AÇIK |  |  |
| P55 | BLG-0997 | xtask/gates/src/gates/timing_safe.rs:39, 53-85, 99-102, 150-171 | P55-F09: notes/P55-scan.md:126; ham önem Medium | AÇIK |  |  |
| P55 | BLG-0998 | xtask/gates/src/gates/storage_proof_boundary.rs:27-59 | P55-F10: notes/P55-scan.md:136; ham önem Low | AÇIK |  |  |
| P55 | BLG-0999 | xtask/gates/src/gates/storage_penalties.rs:17-63, 113-117, 157, 293, 323-345 | P55-F11: notes/P55-scan.md:146; ham önem Low | AÇIK |  |  |
| P55 | BLG-1000 | xtask/gates/src/gates/tee_trust_boundary_is_structural.rs:16-21, 302-312, 386-397, 160-224 | P55-F12: notes/P55-scan.md:156; ham önem Low | AÇIK |  |  |
| P55 | BLG-1001 | xtask/gates/src/gates/transcript_mirrors.rs:53-103, 36-37 | P55-F13: notes/P55-scan.md:166; ham önem Low | AÇIK |  |  |
| P55 | BLG-1002 | xtask/gates/src/gates/tree_pin.rs:75-80, 83 | P55-F14: notes/P55-scan.md:176; ham önem Low | AÇIK |  |  |
| P55 | BLG-1003 | xtask/gates/src/gates/three_is_recipe_only.rs:28-62 | P55-F15: notes/P55-scan.md:187; ham önem Low | AÇIK |  |  |
| P55 | BLG-1004 | xtask/gates/src/gates/uncheckable_proof.rs:45-55, 76-81, 98 | P55-F16: notes/P55-scan.md:197; ham önem Low | AÇIK |  |  |
| P55 | BLG-1005 | xtask/gates/src/gates/the_image_builds_what_the_manifest_declares.rs:38-64, 71-103, 108-140 | P55-F17: notes/P55-scan.md:207; ham önem Low | AÇIK |  |  |
| P55 | BLG-1006 | xtask/gates/src/gates/test_modules_can_see_what_they_test.rs:38-65, 77-140, 143-173, 233-254, 322-339 | P55-F18: notes/P55-scan.md:217; ham önem Low | AÇIK |  |  |
| P55 | BLG-1007 | xtask/gates/src/gates/source_reading.rs:62-79, 82-101, 142-158 | P55-F19: notes/P55-scan.md:227; ham önem Low | AÇIK |  |  |
| P55 | BLG-1008 | xtask/gates/src/gates/typos.rs:22, 69-74, 39 | P55-F20: notes/P55-scan.md:237; ham önem Low | AÇIK |  |  |
| P55 | BLG-1009 | xtask/gates/src/gates/tree_is_english.rs:85-107, 120, 161-319, 438-447, 531-534 | P55-F21: notes/P55-scan.md:247; ham önem Low | AÇIK |  |  |
| P55 | BLG-1010 | shard_placement.rs:315,327; storage_penalties.rs:416; threshold_rates.rs:147,211 | P55-F22: notes/P55-scan.md:257; ham önem Info | AÇIK |  |  |
| P55 | BLG-1011 | xtask/gates/src/gates/suppressions_are_justified.rs:59-64 | P55-F23: notes/P55-scan.md:267; ham önem Info | AÇIK |  |  |
| P56 | BLG-1012 | xtask/gates/src/gates/named_tests.rs:19-24 | P56-F01: notes/P56-scan.md:34; ham önem Medium | AÇIK |  |  |
| P56 | BLG-1013 | xtask/gates/src/gates/zero_tests_witness.rs:132, 97-101, 189-291 | P56-F02: notes/P56-scan.md:50; ham önem Medium | AÇIK |  |  |
| P56 | BLG-1014 | xtask/gates/src/gates/zkvm_immune_regeneration.rs:227-248 | P56-F03: notes/P56-scan.md:67; ham önem Medium | AÇIK |  |  |
| P56 | BLG-1015 | xtask/gates/src/gates/zkvm_immune_regeneration.rs:36-48, 83-109 | P56-F04: notes/P56-scan.md:84; ham önem Medium | AÇIK |  |  |
| P56 | BLG-1016 | xtask/gates/src/gates/wire_fields_are_signed.rs:258-277, 404-440 | P56-F05: notes/P56-scan.md:100; ham önem Medium | AÇIK |  |  |
| P56 | BLG-1017 | xtask/gates/src/gates/wire_fields_are_signed.rs:39, 148-168, 186-205, 337-358 | P56-F06: notes/P56-scan.md:111; ham önem Low | AÇIK |  |  |
| P56 | BLG-1018 | xtask/gates/src/gates/untrusted_manifests.rs:292-323 | P56-F07: notes/P56-scan.md:127; ham önem Medium | AÇIK |  |  |
| P56 | BLG-1019 | xtask/gates/src/gates/untrusted_manifests.rs:164-179, 265-289, 480-488, 558-635 | P56-F08: notes/P56-scan.md:143; ham önem Low | AÇIK |  |  |
| P56 | BLG-1020 | xtask/gates/src/gates/value_transfers.rs:46-60, 192-209, 304-330 | P56-F09: notes/P56-scan.md:160; ham önem Low | AÇIK |  |  |
| P56 | BLG-1021 | xtask/gates/src/gates/zero_address_sender_is_verified.rs:139-150, 108-124, 192, 211-263 | P56-F10: notes/P56-scan.md:175; ham önem Medium | AÇIK |  |  |
| P56 | BLG-1022 | xtask/gates/src/gates/zero_storage_frozen.rs:23-41, 74-160 | P56-F11: notes/P56-scan.md:191; ham önem Low | AÇIK |  |  |
| P56 | BLG-1023 | xtask/gates/src/gates/wallet_node_address_agree.rs:26-37, 96-137 | P56-F12: notes/P56-scan.md:206; ham önem Low | AÇIK |  |  |
| P56 | BLG-1024 | xtask/gates/src/gates/workflows_produce_jobs.rs:113-133, 142-167, 198-206, 329-336 | P56-F13: notes/P56-scan.md:221; ham önem Low | AÇIK |  |  |
| P56 | BLG-1025 | xtask/gates/src/gates/zizmor.rs:10-11, 40-45, 143-157, 163-221 | P56-F14: notes/P56-scan.md:236; ham önem Low | AÇIK |  |  |
| P56 | BLG-1026 | xtask/gates/src/gates/zizmor.rs:83,85 | P56-F15: notes/P56-scan.md:251; ham önem Info | AÇIK |  |  |
| P56 | BLG-1027 | xtask/gates/src/main.rs:1316-1334, 1361-1364 | P56-F16: notes/P56-scan.md:261; ham önem Info | AÇIK |  |  |
| P57 | BLG-1028 | xtask/tools/src/audit_guard.rs:56-62, 145-152 | P57-F01: notes/P57-scan.md:47; ham önem Medium | AÇIK |  |  |
| P57 | BLG-1029 | xtask/tools/Cargo.toml:1-20 | P57-F02: notes/P57-scan.md:67; ham önem Medium | AÇIK |  |  |
| P57 | BLG-1030 | xtask/tools/src/smoke_rpc.rs:108-110, 172-194 | P57-F03: notes/P57-scan.md:77; ham önem Medium | AÇIK |  |  |
| P57 | BLG-1031 | xtask/tools/src/smoke_rpc.rs:56, 133-142, 208-218 | P57-F04: notes/P57-scan.md:95; ham önem Medium | AÇIK |  |  |
| P57 | BLG-1032 | xtask/tools/src/backup_drill.rs:82-110, 168, 191-200 | P57-F05: notes/P57-scan.md:112; ham önem Medium | AÇIK |  |  |
| P57 | BLG-1033 | xtask/tools/src/fsf_fit.rs:284-322 | P57-F06: notes/P57-scan.md:129; ham önem Low | AÇIK |  |  |
| P57 | BLG-1034 | xtask/tools/src/audit_guard.rs:121-130, 218-228 | P57-F07: notes/P57-scan.md:145; ham önem Low | AÇIK |  |  |
| P57 | BLG-1035 | xtask/tools/src/audit_guard.rs:133-238 | P57-F08: notes/P57-scan.md:160; ham önem Low | AÇIK |  |  |
| P57 | BLG-1036 | xtask/tools/src/audit_guard.rs:134-137, 202-216, 230-237 | P57-F09: notes/P57-scan.md:170; ham önem Low | AÇIK |  |  |
| P57 | BLG-1037 | xtask/tools/src/step_reachability.rs:186-222, 302-311 | P57-F10: notes/P57-scan.md:185; ham önem Low | AÇIK |  |  |
| P57 | BLG-1038 | xtask/tools/src/step_reachability.rs:85-102, 132, 247-287, 374-388 | P57-F11: notes/P57-scan.md:201; ham önem Low | AÇIK |  |  |
| P57 | BLG-1039 | xtask/tools/src/fsf_fit.rs:185-191, 228 | P57-F12: notes/P57-scan.md:216; ham önem Low | AÇIK |  |  |
| P57 | BLG-1040 | xtask/tools/src/lib.rs:96-105 | P57-F13: notes/P57-scan.md:226; ham önem Low | AÇIK |  |  |
| P57 | BLG-1041 | xtask/tools/src/sbom.rs:38, 59-87, 135-142 | P57-F14: notes/P57-scan.md:242; ham önem Low | AÇIK |  |  |
| P57 | BLG-1042 | xtask/tools/src/seed_corpus.rs:31-63 | P57-F15: notes/P57-scan.md:259; ham önem Low | AÇIK |  |  |
| P57 | BLG-1043 | xtask/tools/src/prepush.rs:39-50, 126-136 | P57-F16: notes/P57-scan.md:269; ham önem Low | AÇIK |  |  |
| P57 | BLG-1044 | xtask/tools/src/rpc.rs:62-77, 86-94 | P57-F17: notes/P57-scan.md:285; ham önem Low | AÇIK |  |  |
| P57 | BLG-1045 | xtask/tools/src/docker_smoke.rs:36, 177-193 | P57-F18: notes/P57-scan.md:302; ham önem Info | AÇIK |  |  |
| P57 | BLG-1046 | xtask/tools/src/bin/budlum-tools.rs:25-27, 109-130 | P57-F19: notes/P57-scan.md:312; ham önem Info | AÇIK |  |  |
| P58 | BLG-1047 | - | P58-F01: FINDINGS.md:981; ham önem Medium | AÇIK |  |  |
| P58 | BLG-1048 | - | P58-F02: FINDINGS.md:982; ham önem Medium | AÇIK |  |  |
| P58 | BLG-1049 | workflows/benchmark.yml:30-71 | P58-F03: FINDINGS.md:983; ham önem Medium | AÇIK |  |  |
| P58 | BLG-1050 | workflows/miri.yml:159, 219 | P58-F04: FINDINGS.md:984; ham önem Medium | AÇIK |  |  |
| P58 | BLG-1051 | codeql/codeql-config.yml:36-68 | P58-F05: FINDINGS.md:985; ham önem Medium | AÇIK |  |  |
| P58 | BLG-1052 | - | P58-F06: FINDINGS.md:986; ham önem Low | AÇIK |  |  |
| P58 | BLG-1053 | - | P58-F07: FINDINGS.md:987; ham önem Low | AÇIK |  |  |
| P58 | BLG-1054 | - | P58-F08: FINDINGS.md:988; ham önem Low | AÇIK |  |  |
| P58 | BLG-1055 | - | P58-F09: FINDINGS.md:989; ham önem Low | AÇIK |  |  |
| P58 | BLG-1056 | - | P58-F10: FINDINGS.md:990; ham önem Low | AÇIK |  |  |
| P58 | BLG-1057 | - | P58-F11: FINDINGS.md:991; ham önem Low | AÇIK |  |  |
| P58 | BLG-1058 | - | P58-F12: FINDINGS.md:992; ham önem Info | AÇIK |  |  |
| P59 | BLG-1059 | workflows/provenance.yml:69-83 | P59-F01: FINDINGS.md:993; ham önem Medium | AÇIK |  |  |
| P59 | BLG-1060 | budzero/deny.toml | P59-F02: FINDINGS.md:994; ham önem Medium | AÇIK |  |  |
| P59 | BLG-1061 | supply-chain/audits.toml: | P59-F03: FINDINGS.md:995; ham önem Medium | AÇIK |  |  |
| P59 | BLG-1062 | fuzz/Cargo.toml:21 | P59-F04: FINDINGS.md:996; ham önem Medium | AÇIK |  |  |
| P59 | BLG-1063 | - | P59-F05: FINDINGS.md:997; ham önem Low | AÇIK |  |  |
| P59 | BLG-1064 | .quality/deny.toml: | P59-F06: FINDINGS.md:998; ham önem Low | AÇIK |  |  |
| P59 | BLG-1065 | bud/Cargo.toml | P59-F07: FINDINGS.md:999; ham önem Low | AÇIK |  |  |
| P59 | BLG-1066 | crates/wallet-core/Cargo.toml:47 | P59-F08: FINDINGS.md:1000; ham önem Low | AÇIK |  |  |
| P59 | BLG-1067 | - | P59-F09: FINDINGS.md:1001; ham önem Info | AÇIK |  |  |
| P60 | BLG-1068 | - | P60-F01: FINDINGS.md:670; ham önem Critical | AÇIK |  |  |
| P60 | BLG-1069 | - | P60-F02: FINDINGS.md:671; ham önem High | AÇIK |  |  |
| P60 | BLG-1070 | - | P60-F03: FINDINGS.md:672; ham önem Medium | AÇIK |  |  |
| P60 | BLG-1071 | - | P60-F04: FINDINGS.md:673; ham önem Medium | AÇIK |  |  |
| P60 | BLG-1072 | - | P60-F05: FINDINGS.md:674; ham önem Medium | AÇIK |  |  |
| P60 | BLG-1073 | - | P60-F06: FINDINGS.md:675; ham önem Medium | AÇIK |  |  |
| P60 | BLG-1074 | xtask/tools/src/audit_guard.rs:43 | P60-F07: FINDINGS.md:676; ham önem Medium | AÇIK |  |  |
| P60 | BLG-1075 | - | P60-F08: FINDINGS.md:677; ham önem Low | AÇIK |  |  |
| P60 | BLG-1076 | - | P60-F09: FINDINGS.md:678; ham önem Info | AÇIK |  |  |
| P61 | BLG-1077 | config/archive.toml:5-7 | P61-F01: FINDINGS.md:1027; ham önem High | AÇIK |  |  |
| P61 | BLG-1078 | src/rpc/server.rs:128-135 | P61-F02: FINDINGS.md:1028; ham önem High | AÇIK |  |  |
| P61 | BLG-1079 | ops/docker-compose.yml:23-45, 63-68 | P61-F03: FINDINGS.md:1029; ham önem Medium | AÇIK |  |  |
| P61 | BLG-1080 | - | P61-F04: FINDINGS.md:1030; ham önem Medium | AÇIK |  |  |
| P61 | BLG-1081 | config/mainnet-genesis.json:5-6 | P61-F05: FINDINGS.md:1031; ham önem Medium | AÇIK |  |  |
| P61 | BLG-1082 | - | P61-F06: FINDINGS.md:1032; ham önem Low | AÇIK |  |  |
| P61 | BLG-1083 | config/mainnet.toml:23-26, 35-38 | P61-F07: FINDINGS.md:1032; ham önem Low | AÇIK |  |  |
| P61 | BLG-1084 | - | P61-F08: FINDINGS.md:1032; ham önem Low | AÇIK |  |  |
| P61 | BLG-1085 | - | P61-F09: FINDINGS.md:1032; ham önem Low | AÇIK |  |  |
| P61 | BLG-1086 | - | P61-F10: FINDINGS.md:1032; ham önem Low | AÇIK |  |  |
| P61 | BLG-1087 | - | P61-F11: FINDINGS.md:1032; ham önem Low | AÇIK |  |  |
| P61 | BLG-1088 | - | P61-F12: FINDINGS.md:1032; ham önem Low | AÇIK |  |  |
| P62 | BLG-1089 | .github/workflows/security-hardening.yml:334-377 | P62-F01: FINDINGS.md:185; ham önem Medium | AÇIK |  |  |
| P62 | BLG-1090 | - | P62-F02: FINDINGS.md:186; ham önem Info | AÇIK |  |  |
| P62 | BLG-1091 | crates/budscan/src/query.rs:110 | P62-F03: FINDINGS.md:187; ham önem Medium | AÇIK |  |  |
| P62 | BLG-1092 | crates/bpqs/src/tests.rs:103 | P62-F04: FINDINGS.md:188; ham önem Low | AÇIK |  |  |
| P62 | BLG-1093 | src/bin/bud.rs:38 | P62-F05: FINDINGS.md:189; ham önem Medium | AÇIK |  |  |
| P62 | BLG-1094 | docs/CONTRIBUTING.md:5 | P62-F06: FINDINGS.md:190; ham önem Medium | AÇIK |  |  |
| P0 | BLG-1095 | src/storage/qr_video.rs:293 | CI #124 Repo Lint gate hash-inputs-length-prefixed: `ty` and `data` hashed with no length between them (iş 115030778593); BLG-0004 alt kalemi | AÇIK |  |  |
| P0 | BLG-1096 | src/storage/three_pipe.rs (decode_qr_video) | CI #124 Repo Lint gate no-idle-code: yeni boşta kalan genel öğe `pub fn decode_qr_video`; BLG-0004 alt kalemi | AÇIK |  |  |
| P0 | BLG-1097 | docs/audit/SECOND_PASS_ALPHA.md:323; notes/second-pass-alpha-P01.md:105 | CI #124 Repo Lint gate tree-is-english: Türkçe sözcük `Katmani`; BLG-0004 alt kalemi | AÇIK |  |  |
| P0 | BLG-1098 | src/domain/deal_open.rs:430-720; src/tests/storage_tx_door.rs:654-764; budzero/bud-proof/src/relayer.rs:718; budzero/bud-vm/src/lib.rs:1694 | CI #124 Repo Lint gate rejection-tests-assert-rejection: 18 teste ret adı verilmiş ama ret doğrulanmıyor; BLG-0004 alt kalemi | AÇIK |  |  |
| P0 | BLG-1099 | docs/bulgular/*; xtask/gates/src/gates/tree_is_english.rs | docs/bulgular dosyaları Türkçe: tree-is-english kapısı kırmızı olur; şablon İngilizceye çevrilir, defter için tam dosya adı muafiyeti (sahibin direktifi defteri Türkçe ister) | AÇIK |  |  |
