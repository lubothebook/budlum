# BUD K0 discovery report

Date: 2026-10-07. Branch: claude/zkvm-bud-completion-84r6jc. Scope: read only. No code changed. No test run for this report.

Every file:line below was read by the author in this checkout. Scout leads that did not hold are listed in section 9. Where I did not trace something, the text says so.

## 1 Method and search commands

Rules followed: `rg -n` and `sed -n 'a,bp'` only, no whole-file reads of large files, no target/ and no Cargo.lock.

Search patterns run (all from the repo root, mostly under src/, plus bud/, budzero/, crates/, xtask/, fuzz/, docs/):

- Module list and sizes: `ls src/storage`, `wc -l src/storage/qr_*.rs src/storage/three_*.rs`, `rg -n "^(pub )?mod |pub mod" src/storage/mod.rs`
- Wiring markers: `rg -n "WIRING|WIRED|UNWIRED" src/storage -g '*.rs'`
- QR entry points: `rg -n "encode_qr_video|decode_qr_video|qr_feed_preview|reemit|RecipeEmitter|ThreeNftRegistry|ThreeMeter|RevealSession" src crates bud budzero examples benches xtask`
- Production callers per module: `rg -o -I` over crate::storage paths in src, excluding src/storage and src/tests (glob filters), then per-module `rg -n` for one_share, one_view, lrc, msr, server_admission, CustodyLedger, payload_crypt, pact_binding, social_delete
- Public items: `rg -n "pub (struct|enum|fn|trait|const)" src/storage/{manifest,erasure,assignment,lrc,pruning,lifecycle,server_admission,mobile_self,living_threshold,provider,three_gate,three_nft,three_recipe,qr_reemit,qr_recipe}.rs`
- Limits: `rg -n "MAX_K|MAX_BLOCK_LEN|DROP_HEADER_LEN|MAX_QR_PAYLOAD|MAX_PAYLOAD_CONTENT|MAX_VIDEO_FRAMES|MAX_TRANSFORM_IN|MAX_PREVIEW_CONTENT_BYTES|ONESHOT_REPAIR" src/storage`
- Tests: `rg -c "#\[test\]" src/storage/<file>.rs` and `rg -n "^\s+fn [a-z_0-9]+\(\)" src/storage/<file>.rs`
- Fuzz and property tests: `ls fuzz/fuzz_targets`, `rg -n "qr_|three_|storage::" fuzz -l`, `rg -n "proptest|quickcheck|arbitrary" Cargo.toml`, `rg -ln "proptest" src`
- NFT and burn: `rg -n "NftBurn|NftMint|NftTransfer" src`, `rg -n "prune_content|tombstone|Tombstone|hard_prune" src`, `rg -n "StoragePrune" src`, `rg -n "\.produce_block\(" src`
- DAO: `rg -n -i "\bdao\b" src`, `rg -il "dao|governance" src`, `sed -n` over src/core/governance.rs ProposalType and GovernanceAction
- New-feature probes: `rg -n -i "tombstone|StorageClaim|heartbeat"`, `rg -n -i "fingerprint|perceptual|phash|simhash|near.?dup"`, `rg -n -i "lifetime"`, `rg -n -i "0\.016|lepton"` over src, bud, docs
- Touch points: `rg -n "storage::|ContentId|ContentManifest" crates/wallet-core crates/budscan crates/note-packing`, `rg -n "crate::storage" src/gateway src/budlumxyz src/pollen src/sdk src/cli src/bns`, `rg -n -i gallery src crates`
- Hash definitions: `rg -n "BDLM_CONTENT_V1" src budzero crates xtask bud`, `rg -n "pub struct ContentId" budzero crates`
- Older docs: `rg -n "^#"` over docs/BUD_STORAGE_ROADMAP.md, docs/BUD_CONTENT_ENCRYPTION.md, bud/FORMAT-V2.md, src/storage/README.md, then `sed -n` of the relevant parts
- Directive: BUD-AI-KAPSAMLI-DIREKTIF.md (untracked file in the repo root) read by section with `sed -n`.

The bud/ crate (package bud-core) is not a dependency of the root crate. `rg -n "bud-core|bud_core" Cargo.toml` finds nothing. It is a separate research tree with its own QR-video model (bud/src/bud_format_qrvideo.rs).

## 2 QR video system

### 2.1 Verdict

The system exists in src/storage. It is one pipeline, called A0 to A5 in its comments. Its product is a BDLV container that holds QR symbols stored as PNG images. It is a "lab container", not a real video file. The module header says so: src/storage/qr_video.rs:1-8. H.264 and VP9 are named as optional external channels and no muxer is linked: src/storage/qr_codec.rs:44-45 (AV1 forbidden) and the test `h264_allowed_but_mux_not_linked` at qr_codec.rs:199.

### 2.2 Entry points

| Step | Function | File:line |
|---|---|---|
| A0 classify and pin | `transform_content(input, opts)` | src/storage/transformed.rs:307 |
| A1 container | `pack_payload_opts(kind, content, allow_zlib)`, `unpack_payload` | src/storage/qr_payload.rs:158, 199 |
| A2 fountain | `CarouselEncoder::new`, `drop_at`, `CarouselDecoder` | src/storage/qr_carousel.rs (encoder near 348, count fn 436) |
| A3 frame | `pack_frame`, `unpack_frame`, `fold_frame_digests` | src/storage/qr_frame.rs:105, 130, 205 |
| QR symbol | `QrMatrix::encode`, `encode_at` | src/storage/qr_matrix.rs:91, 103 |
| PNG | `frame_to_qr_png`, `matrix_to_png` | src/storage/qr_png.rs:101, 64 |
| A4 container | `QrVideo::from_optical_frames`, `to_bytes`, `from_bytes`, `blob_commitment` | src/storage/qr_video.rs:99, 139, 160, 201 |
| Encode facade | `encode_qr_video(content, block_len, seal_key)` (private `encode_plain` at 135, `encode_payload` at 155) | src/storage/three_pipe.rs:273 |
| Decode facade | `decode_qr_video(video_blob)`, `decode_frames(stream_commitment, frames)` | src/storage/three_pipe.rs:297, 215 |
| Image decode | `png_to_optical_frame` (uses `rqrr`), `demux_optical_frames` | src/storage/qr_video.rs:224, 249 |
| Recipe re-emit | `VideoRecipe::reemit(body)`, `frame_at`, `frame_stream` | src/storage/three_recipe.rs:264, 291, 306 |
| Packed-body re-emit | `RecipeEmitter::open`, `frame_at`, `emit_frames`, `verify_stream_id` | src/storage/qr_reemit.rs:95, 140, 149, 176 |
| Front seat | `qr_feed_preview(content, policy, manifest)`, `qr_feed_frames_burst` | src/storage/emit.rs:765, 1370 |

The decode path is: BDLV bytes -> `QrVideo::from_bytes` -> PNG -> `rqrr` -> optical frames -> `ProgressiveReceiver` -> carousel decoder -> `unpack_payload`. See three_pipe.rs:297-302 and 215-228.

### 2.3 Data structures and frame format

- A1 packed container "BDL3": magic 4, version 1, flags 1 (bit0 zlib), kind 1, orig_len 8, sha256 32. Header length 47. qr_payload.rs:39-46. Kinds: ContentBytes, PublicRecipeWire, EncryptedContent (qr_payload.rs:56-67). Zlib level 9 is kept only if it shrinks (qr_payload.rs:158-190).
- A2 drop header "BDLD": 24 bytes. Magic 4, version 1, flags 1, seq 4, k 2, block_len 2, total_len 4, degree 1, pad 1, body hash 4. `DROP_HEADER_LEN` at qr_carousel.rs:66, and the test `assert_eq!(DROP_HEADER_LEN, 24)` at qr_carousel.rs:1045. Scout said 28. That is wrong.
- A3 frame header: 18 bytes. Magic `BD 3A` 2, version 1, flags 1, seq 4, stream id prefix 4, drop_len 2, frame digest 4. `THREE_FRAME_HEADER_LEN` at qr_frame.rs:44. Parsing and checks at qr_frame.rs:130-170. The prefix is the first 4 bytes of the stream commitment (qr_frame.rs:180).
- A4 container "BDLV": magic 4, version 1, flags 1, fps u16, frame_count u32, stream_commitment 32, recipe_commitment 32, then repeated (png_len u32, png). qr_video.rs:12-24 and 139-152. fps is a display hint only (qr_video.rs:36).
- Recipe objects: `ThreeRecipePublic` (payload_commitment, carousel params, stream_id, block_len) at qr_recipe.rs:17. `VideoRecipe` (content bytes, transform knobs, kind, block_len, repair permillage, fps, content_sha256, video_commitment, frame_count) at three_recipe.rs:158. Sealed forms at qr_recipe.rs:34 and three_recipe.rs:182.

### 2.4 Error correction and loss tolerance

- QR level is fixed at EC level L. `THREE_QR_EC` at qr_matrix.rs:16. `encode_at` refuses any other level (qr_matrix.rs:104-106). The rule is that the fountain code handles erasure and QR ECC handles pixel damage (qr_matrix.rs:15).
- The QR encoder is in-tree: qr_encode.rs. It is pinned to byte mode, level L, masks tried 0 to 7. The test `encoded_matrix_reads_back_through_an_independent_decoder` (qr_encode.rs:1012) and `every_version_boundary_roundtrips` (qr_encode.rs:1029) read the symbols back through `rqrr`.
- Fountain: systematic pass first, then repair drops. Repair rows are a uniform random subset, decoded by Gauss-Jordan over GF(2). Doc at qr_carousel.rs:855-880. The PRNG is SHA-256 in counter mode, seeded by seq (`SeqRng`, qr_carousel.rs:901-935). No floats, no clock, no `rand` in qr_*.rs or three_*.rs outside tests (rg for f32/f64/SystemTime/thread_rng; the only `Instant::now` is in a test at three_recipe.rs:687).
- Repair margin: 15 percent of k. `ONESHOT_REPAIR_PERMILLAGE = 150` at qr_carousel.rs:62. Frame count `oneshot_drop_count` at qr_carousel.rs:436.
- What the loss tests model: dropped frames and a flipped bit in a drop body (qr_video.rs:575-600, test at 488). The body flip is caught by the 4-byte body hash. They work on frame bytes. They do not touch the raster.
- Raster damage: the PNG decoder accepts only our own layout. It reads width and height from IHDR, assumes 8-bit RGB, rejects any PNG filter other than 0, and does not check chunk CRC (qr_video.rs:258-331; comment on CRC at qr_video.rs:605-606). A re-encoded, resized or re-compressed frame will not decode. It fails closed, but it has no tolerance.

### 2.5 Limits

| Limit | Value | File:line |
|---|---|---|
| Payload content | 64 MiB | qr_payload.rs:51 |
| A0 input | 64 MiB | transformed.rs:284 |
| Carousel bytes | 64 MiB | qr_carousel.rs:41 |
| Source blocks k | 4096 | qr_carousel.rs:39 |
| Block length | 8168 (8192 minus 24). Scout said 7920. Wrong. | qr_carousel.rs:47 |
| QR payload per symbol | 2953 | qr_matrix.rs:22 |
| Frames per video | 50000 | qr_video.rs:38 |
| PNG side | 8192 px | qr_png.rs:58 |
| Default block | 200 | qr_carousel.rs:35 |
| RPC preview body | 1 MiB | emit.rs:86 |
| Gateway read body | 10 MiB | src/gateway/service.rs:10 |

Effective ceiling for content that must go through the video: a frame is 18 + 24 + block bytes and must fit 2953, so block is at most 2911. With k at most 4096 the packed container is at most 4096 x 2911 = 11,923,456 bytes. With the default block of 200 it is at most 819,200 bytes. The 64 MiB constants cannot be reached through the video path. The RPC accepts only 1 MiB. The frame cap (50000) is never the binding limit: k = 4096 gives 4710 frames. `plan()` refuses k above MAX_K before encoding (emit.rs:690-705).

### 2.6 Supported content classes

A0 has 11 classes: Generic, TextOrganic, EntropyMedia, EntropyArchive, Ciphertext, RecipeWire, VectorOrganic, RasterFlat, AudioPcm, DocumentOrganic, Exec (transformed.rs:54-77). Classification is by magic bytes and an optional MIME hint. The class only decides whether zlib is tried. All classes use the same pipe.

Refused today:

- Zero bytes. Refused at four pipe layers (plus two more checks below): `transform_content` (transformed.rs:311), `pack_payload_opts` (qr_payload.rs:163, `PayloadError::Empty`), `CarouselParams::from_payload` (qr_carousel.rs:165), `QrMatrix::encode_at` (qr_matrix.rs:107). `unpack_payload` also refuses `orig_len == 0` (qr_payload.rs:230). The emit layer has its own `EmitError::Empty` (emit.rs:240).
- Anything above the effective ceiling in 2.5.

Not covered by any code in src: real video files as input (mp4, webm) are just bytes to this pipe. They are class EntropyMedia and are carried opaque. That is lossless, but the pipe does not understand video.

What "recipe" means per class. For Generated content (a generator id plus a seed, src/storage/generated.rs:169-194) the recipe alone regenerates the bytes, and the gateway checks this (src/gateway/service.rs:28-85). For organic content the code states that a short recipe cannot regenerate it (three_recipe.rs:10-16). The recipe then rebuilds the video from the stored body. The record that holds both is `RecipeRecord::WithBody { recipe, body }` (three_nft.rs:159-190). So the object on the network for organic content is the body, not a bare recipe. The directive's "only the recipe is on chain" holds for the Generated class only. See section 8, item 4.

### 2.7 Decoder behaviour on untrusted input

Bounded or fail-closed, with the line that does it:

- BDLV parse: magic, version, n between 1 and 50000, offsets checked with `get` (qr_video.rs:160-197). `Vec::with_capacity(n)` is bounded by 50000.
- PNG: side cap before inflate, inflate capped at expected size plus one (qr_video.rs:296-304, 333-352). Tests `hostile_oversize_ihdr_is_refused_before_inflate` and `zip_bomb_idat_is_refused` (qr_video.rs:634, 647).
- Payload inflate capped at 64 MiB plus one (qr_payload.rs:270-285). Test `unpack_refuses_a_zlib_body_that_expands_past_the_cap`.
- Frame: magic, version, flags, stream prefix, length, digest, nested drop (qr_frame.rs:130-170).
- Receiver: first writer wins per seq, conflicting bodies counted and refused, seq table bounded at 2 x MAX_K (qr_receive.rs:79, 116-155).
- Final check: the sha256 inside the A1 header over the unpacked bytes (qr_payload.rs:240-247).

Weak points:

- `decode_qr_video` takes the stream commitment from the video's own header (three_pipe.rs:297-302). It does not compare against any recipe, ContentId or manifest the caller holds. A self-consistent forged video decodes to its own content. Only `qr_feed_preview` compares commitments (emit.rs:909-920), and it compares against the encode it just made.
- The frame digest is 4 bytes and the dedup tag is FNV-1a 32 (qr_receive.rs:214-221). Final integrity rests on the sha256 in A1, which is inside the same container the attacker supplies.
- No fuzz target covers any of this (section 2.9).

### 2.8 Determinism

Positive evidence: golden vectors for payload, carousel drop, stream commitment, PNG bytes and BDLV bytes (tests `wire_bytes_match_the_golden_vector`, `commitment_matches_the_golden_vector`, `drop_wire_matches_the_golden_vectors`, `stream_commitment_matches_the_golden_vector`, `png_bytes_match_the_golden_digest`, `video_wire_matches_the_golden_vectors`), plus `drop_at_is_deterministic`, `churn_reemitted_video_is_deterministic`, `recipe_reproduces_the_video_bit_equal`.

Risks:

- Zlib output is produced by `flate2` with the `rust_backend` (miniz_oxide) at level best (qr_payload.rs:264-268, qr_png.rs:135-145; Cargo.toml:29). Packed bytes and PNG bytes, and therefore `payload_commitment` and the video commitment, depend on that crate version. The qr_png.rs test comment pins flate2 1.1.9 and miniz_oxide 0.8.9 (qr_png.rs:227).
- `zlib_deflate` falls back to stored blocks on an encoder error (qr_png.rs:143-145). A silent alternate output path.
- The cross-platform CI job in .github/workflows/determinism.yml compares consensus output on three platforms. I found no storage or QR content in it (rg for storage|qr_|three returned nothing).

### 2.9 Tests present, and what is absent

Counts of `#[test]` by file (rg -c): qr_payload 14, qr_receive 5, qr_carousel 22, qr_frame 7, qr_recipe 5, qr_matrix 5, qr_png 4, qr_codec 5, qr_encode 17, qr_reemit 4, qr_video (see names above), three_pipe 8, three_recipe 9, three_nft 8, three_meter 2. I did not run them.

Present:

- Content class matrix: `matrix_e2e_each_class_round_trips` (transformed.rs, about line 614). It uses 10 synthetic samples of about 3000 bytes (fake magic bytes plus a pattern) at block_len 64. Also `every_class_compresses_to_a_video_and_a_fixed_size_recipe` (three_pipe.rs:556), 11 classes.
- QR version boundaries: `every_version_boundary_roundtrips`.
- Frame loss and bit rot at frame level: `oneshot_survives_sparse_frame_loss`, `k10_channel_loss_survives_the_video_container`, `repair_recovers_from_random_loss_within_15_percent`.
- Negative cases: tampered body, bad magic, bad version, truncated, foreign stream, conflicting body, wrong body for recipe, hostile IHDR, zip bomb.

Absent:

- Fuzz: fuzz/fuzz_targets has 15 targets (block, transaction, snapshot, vm, zk, evm, budl and so on). None names storage, qr or three. `rg -n "qr_|three_|storage::" fuzz -l` returns nothing.
- Property-based roundtrip: proptest is a dependency (Cargo.toml:167) and is used in src/tests/proptest_core.rs, tokenomics_proptest.rs and src/storage/erasure.rs. Not in any qr_ or three_ file.
- Lossy raster transport: no test re-encodes, resizes or compresses the PNG frames.
- Second independent decoder for the carousel layer: see 2.10.
- Large content, zero bytes (refused, tested as refusal only), one byte, exact multiples of block_len at the pipe level: I did not find a pipe-level boundary sweep. `empty_refused` tests exist per layer.
- Cross-platform determinism test for QR output.
- Verification audit records.

### 2.10 Two decoders: what exists

- `decode_frames` (three_pipe.rs:215-228) is a thin wrapper over `ProgressiveReceiver`. `qr_feed_preview` compares `ProgressiveReceiver` with `decode_frames` and calls that "two decoders" (emit.rs:902-908). They share the same receiver and the same carousel decoder, so this is one decoder path used twice.
- Independence exists only at the symbol layer: `qr_encode.rs` (in-tree encoder) is read back by `rqrr` (external crate) in tests, and production decode uses `rqrr` (qr_video.rs:224-247, Cargo.toml:35). There is no second reader on the production path.

### 2.11 WIRED or UNWIRED

Chain and consensus path: none of the QR code is reached by block validation, the executor, or any transaction type. `rg` finds no use of encode_qr_video, decode_qr_video, VideoRecipe, ThreeNftRegistry or RecipeEmitter in src/execution, src/chain, src/domain or src/network.

Production RPC path (read only):

- `bud_storageQrFeedPreview` -> src/rpc/server.rs:3284 -> `qr_feed_preview` at server.rs:3311. Declared at src/rpc/api.rs:550.
- `bud_storageQrFeedFrames` -> server.rs (just after 3314) -> `qr_feed_frames_burst`.
- `bud_storageOpenReveal`, `bud_storageRevealFrames`, `bud_storageCloseReveal` (api.rs:636 and after) -> `RevealGateway` (constructed at server.rs:305, 320, 336; session cap at 3625) -> `RevealSession` -> `RecipeEmitter`.
- `qr_feed_preview` stores nothing. It uses a scratch `InMemoryStorageProvider` that is dropped (emit.rs:1-30; provider.rs:8-19).

The preview RPC takes `seal_seed` as 32 bytes of hex from the caller (server.rs:3284-3305). That is key material over RPC. See open question 11.

Marker check:

- qr_reemit.rs:3-7 says the stream pin is checked on the reveal path and that the path is "not yet reachable from a binary". Both parts are wrong or stale. The reveal RPC exists. And three_reveal.rs:81-89 says the session deliberately does not call `verify_stream_id`. The only callers of `verify_stream_id` are emit.rs:1055 and three_nft.rs:321. So the reveal read path serves frames with no stream-identity check.
- three_regime.rs:1 says unwired. Confirmed: no caller outside its tests (rg).
- msr.rs:3 says unwired. Confirmed: only its own tests read it.
- three_recipe, three_nft, three_meter: the markers say they are wired through `qr_feed_preview`. True for the preview RPC only. `ThreeNftRegistry` itself has no caller in src outside the re-export (rg: only mod.rs:169). The NFT metadata type `ThreeNftMeta` is used by emit.rs:63-65 and nothing else.

Verdict: the QR pipeline is real, tested at unit level, deterministic by golden vectors, and reachable only as a read-only preview and reveal service. It is UNWIRED to uploads, validation and storage commitments.

## 3 src/storage module map

Status rules: WIRED means a production caller outside src/storage and outside tests that I read. PARTIAL means a production caller reaches only part of it. UNWIRED means no such caller. Test counts are `#[test]` counts from rg -c where I ran it; "n/c" means I did not count.

| Module | Status | Production caller (file:line) | Tests |
|---|---|---|---|
| content_id | WIRED | used across src (about 100 uses); `ContentId::of` content_id.rs:44 | n/c |
| manifest | WIRED | `execute_storage_tx` register_manifest src/domain/storage_tx.rs:370 calls `validate_untrusted` (manifest.rs:729) | n/c |
| erasure | PARTIAL | only `verify_object_encoding` at src/rpc/server.rs:3270 and `column_is_correctly_encoded` via `verify_coding_audit` src/domain/storage_deal.rs:2202. `encode_object` and `reconstruct_object` have no production caller (rg) | proptest inside |
| assignment | WIRED | `annotate_expected_holders` src/domain/storage_deal.rs:3090, from the chain actor sweep (marker at assignment.rs:56) | n/c |
| lrc | UNWIRED | read only by msr.rs and mod.rs | n/c |
| msr | UNWIRED | marker msr.rs:3; own tests only | n/c |
| lifecycle | PARTIAL | `lifecycle_state` src/domain/storage_deal.rs:3337 reads the enum | n/c |
| living_threshold | PARTIAL | demand half: `AccessEstimate` src/domain/storage_deal.rs:2559, 2574-2579 and `required_replicas_with_demand` 1542. `decide` and `break_even_rate_scaled` are node-local by design (living_threshold.rs:85-105) | n/c |
| pruning | WIRED | `PruningPolicy` src/cli/commands.rs:1109, node.rs hard-prune gate (about 1715) | n/c |
| mobile_self | PARTIAL | `declare_self_host_policy` storage_deal.rs:1188 via `StorageTx::DeclareSelfHostPolicy` storage_tx.rs:45, 340-368. `CustodyLedger`, `decide_upload_custody` have no caller outside storage | n/c |
| server_admission | UNWIRED | no caller outside src/storage (rg) | n/c |
| one_share, one_view | UNWIRED | no caller outside src/storage (rg). They model a 1.0 share NFT and a single-screen view | n/c |
| dictionary | WIRED | marker dictionary.rs:46; `register_manifest_with_source` | n/c |
| derived | WIRED | marker derived.rs:130 | n/c |
| generated | WIRED | `generate_content` src/gateway/service.rs:49 | n/c |
| render | WIRED | `render`, `render_id` src/gateway/service.rs:318-321, RPC `bud_gatewayRenderContent` | n/c |
| merkle_trie | WIRED | marker merkle_trie.rs:10; `GetAccountProof` | n/c |
| view_grant | WIRED | `issue_view_grant`, `revoke_view_grant` storage_deal.rs:1561, 1602; chain_actor.rs:1108, 1134 | n/c |
| social_delete | PARTIAL | `StorageRegistry::social_delete` storage_deal.rs:1950 and RPC server.rs:3468. It only revokes grants and rotates the key; it does not delete content. Hook is `NopThreeHook` (storage_deal.rs:1971) | 3 |
| reveal_gateway, three_rpc, three_reveal | PARTIAL | RPC `bud_storageOpenReveal` family, server.rs:305, 3600-3665 | n/c |
| three_hooks | PARTIAL | only `NopThreeHook` is used (storage_deal.rs:1971) | n/c |
| db | WIRED | `Storage` used by gateway/service.rs:4 and chain | n/c |
| traits | WIRED | `DurableCommitBatch` | n/c |
| pact_binding | WIRED | `PactRegistry` src/account_abstraction/registry.rs:24 | n/c |
| provider | PARTIAL | scratch instance inside `qr_feed_preview` only (provider.rs:8-19) | n/c |
| payload_crypt | PARTIAL | reached through `encode_qr_video` seal path, so only the preview RPC | n/c |
| three_gate | PARTIAL | `refuse_durable_derivative` runs inside `InMemoryStorageProvider::put` (marker three_gate.rs:3) | n/c |
| emit | PARTIAL | RPC only: server.rs:3284, 3311 | 17 in emit tests (n/c exact) |
| qr_payload, qr_carousel, qr_frame, qr_matrix, qr_png, qr_encode, qr_codec, qr_receive, qr_recipe, qr_video, transformed, three_pipe | PARTIAL | reachable only through emit and reveal RPC (section 2.11) | 14, 22, 7, 5, 4, 17, 5, 5, 5, n/c, n/c, 8 |
| qr_reemit | PARTIAL | `RecipeEmitter` via reveal RPC and emit; marker text is stale (section 2.11) | 4 |
| three_recipe | PARTIAL | emit.rs:1266 (`recipe.reemit`) | 9 |
| three_nft | PARTIAL | `ThreeNftMeta` in emit only; `ThreeNftRegistry` has no caller | 8 |
| three_meter | PARTIAL | emit.rs (`ThreeMeter`), preview only | 2 |
| three_regime | UNWIRED | marker three_regime.rs:1 | n/c |
| three_visibility | PARTIAL | `delete_implies_key_rotate` social_delete.rs:56; emit.rs:75 | n/c |
| fixed_point | not traced | no outside-storage caller found by rg for `fixed_point::` | n/c |

## 4 Type signatures

ContentId (src/storage/content_id.rs:40-46):

- `pub struct ContentId(pub Hash32);`
- `pub fn of(chunk: &[u8]) -> Self` = `hash_fields_bytes(&[b"BDLM_CONTENT_V1", chunk])`. `hash_fields_bytes` is SHA-256 over u64-LE length prefix plus bytes of each field (src/core/hash.rs:13-20).
- Also `of_subrange` (59) and `of_subrange_for_deal` (104).
- The owner and size are not in the id (manifest.rs:333-345 for the manifest).

Manifest (src/storage/manifest.rs):

- `ShardRef { index: u32, shard_id: ContentId, size: u32, kind: ShardKind }` at 25. `ShardKind { Data, Parity }` at 39.
- `ErasureScheme { k: u32, n: u32 }` at 196. Methods: `replication`, `parity_count`, `loss_tolerance`, `repair_margin` (ceil(parity/3), floor 2, cap parity), `overhead_per_mille`, `validate`.
- `ContentManifest { manifest_id, owner, dictionary_id: Option<ContentId>, total_size: u64, shard_count: u32, shards: Vec<ShardRef>, erasure, source: ContentSource, edition: BudStorageEdition, content_size: u64, encryption: ContentEncryption }` at 331.
- `validate_untrusted(&self) -> Result<(), String>` at 729. It checks counts, sizes, k and n, and `verify_id`. It comment-states that the chain holds no bytes and cannot tell ciphertext from anything else (manifest.rs:797-801).
- `manifest_id_from_shards` at 970. `is_recoverable(live)` 870, `needs_repair(live, margin)` 884.
- `ContentSource { Stored, Generated(GeneratedSpec), SealedGenerated(SealedGeneratedSpec), ... }` at generated.rs:169. `BudStorageEdition { Classic = 1, Three = 3 }` at generated.rs:109; `admits_body` true only for Classic.

Erasure (src/storage/erasure.rs):

- `ReedSolomon::new(data_shards, parity_shards) -> Result<Self, ErasureError>` at 301, systematic Cauchy over GF(2^8). `MAX_TOTAL_SHARDS = 255` at 67.
- `encode_parity`, `reconstruct`, `column_is_correctly_encoded(parity_index, column, parity_byte) -> bool` at 495.
- `encode_object(data, scheme) -> Result<EncodedObject, ErasureError>` at 667. `verify_object_encoding(data, &manifest)` 732. `reconstruct_object` 783.
- There is no default scheme constant. (10,16) and (20,26) appear only in comments (manifest.rs:231-232, assignment.rs:48, lrc.rs:4, msr.rs:156).

Shard placement (src/storage/assignment.rs):

- `ShardCandidate { address: Address, stake: u64 }` at 94. Stake zero is excluded.
- `assign_shard(&ContentId, &Hash32, &[ShardCandidate], replicas: usize) -> Result<Vec<Address>, AssignmentError>` at 185.
- `assign_object(&[ContentId], &Hash32, &[ShardCandidate]) -> Result<Vec<Address>, AssignmentError>` at 231.
- `displaced_shards(previous, current) -> Vec<usize>` at 283.

Coding audit (src/domain/storage_deal.rs):

- `CodingAudit { manifest_id: ContentId, parity_index: u32, column: u64 }` at 387. Not stored; derived.
- `StorageRegistry::derive_coding_audit(entropy: &Hash32, manifest: &ContentManifest, challenge_id: u64) -> Result<CodingAudit, StorageError>` at 2100. `verify_coding_audit(&self, &CodingAudit, data_column: &[u8], parity_byte: u8)` at 2181.
- The chain actor sweep derives audits and only logs "scheduled" (src/chain/chain_actor.rs:3153-3185). Commands `DeriveCodingAudit` and `AnswerCodingAudit` exist (chain_actor.rs:499, 505). I did not trace an in-block audit answer.

Repair (src/domain/storage_deal.rs and manifest.rs):

- `open_repair_tickets_for_free_slots(&mut self, now_epoch) -> usize` at 3477. `objects_needing_repair(margin)` 3541. `objects_below_own_repair_margin()` 3566. `under_replicated_shards(epoch)` 3438. `unrecoverable_objects()` near 3590.
- `ContentManifest::needs_repair(live, margin) -> bool`.

Living threshold (src/storage/living_threshold.rs):

- Constants: `ACCESS_HALF_LIFE_EPOCHS = 720` (115), `ACCESS_SCALE = 1_000_000` (122), `HYSTERESIS_SIXTEENTHS = 4` (164).
- `Lever { size_millionths, cpu_nanos_per_byte }` 168. `OperatorRates { disk_picodollars_per_byte_epoch, cpu_picodollars_per_nano }` 184.
- `AccessEstimate::{new, record_read, rate_scaled, from_events}` 270-346. `AccessEvent { epoch, count }` 326.
- `break_even_rate_scaled(lever, object_bytes, rates)` 390. `decide(...) -> Result<Decision, ThresholdError>` 553 with `Decision { Apply, Revert, Hold }`.
- The module doc already reports the measured rates 0.29 USD per TB per month owned disk and 0.0025 USD per hour processor (living_threshold.rs:19-25 area).

Self-host policy:

- `StorageTx::DeclareSelfHostPolicy { manifest_id, policy: MobileSelfContentPolicy, profile: MobileSelfProfile }` (src/domain/storage_tx.rs:45-49).
- `MobileSelfProfile { owner, device_commitment: [u8;32], availability: MobileAvailabilityClass, max_storage_bytes: u64, metered_network_ok, battery_saver_aware, last_seen_block: u64 }` (mobile_self.rs:32).
- `MobileSelfContentPolicy { content_id, owner, critical, required_paid_replicas: u16, self_host_allowed }` (mobile_self.rs:107).
- `OperatorClass { AlwaysOn, Mobile }` (storage_deal.rs:134), set by `StorageTx::DeclareOperatorClass`.
- `StorageTx { RegisterManifest, DeclareOperatorClass, DeclareSelfHostPolicy, OpenDeal(StorageDealOpen) }` at storage_tx.rs:33-54. The executor arm is executor.rs:2461-2494. `open_deal` returns `OpenDealOnMainnet` on the mainnet chain id (storage_tx.rs:251-262), per STATUS.md decision D6.
- There is no StorageClaim, Heartbeat, or suspended/active PSN state type (rg -i, section 7). The word heartbeat appears only in gossipsub settings (src/network/node.rs:803-863).

Recipe and NFT metadata:

- `ThreeRecipePublic`, `ThreeRecipeSealed`, `ThreeRecipe` (qr_recipe.rs:17, 34, 47). `three_recipe_digest` 56.
- `VideoRecipe`, `VideoRecipeSealed` (three_recipe.rs:158, 182). `commitment()` 227 is domain tagged `BDLM_THREE_VIDEO_RECIPE_V1`.
- `ThreeNftMeta { recipe_commitment, video_commitment: Option, visibility: MetadataVisibility, preview: PreviewMode, preview_content_id: Option }` (three_nft.rs:45).
- There is no field named `recipe_hash`, `output_commitment`, `lifetime` or `generator_id` in any of these. The nearest are `commitment()`, `content_sha256` and `video_commitment`.

Registry NFT (src/socialfi):

- `Nft { id: u64, owner: Address, content_id: ContentId, minted_at_epoch: u64, author_name: Option<String>, luminance: u64, tags: Vec<String> }` (types.rs:9). No visibility, lifetime or recipe field.
- `NftRegistry { nfts, ownership, next_id }` with `mint`, `transfer`, `burn` (socialfi/mod.rs:16, 48, 103, 124).

## 5 NFT and deletion flow

### 5.1 State of the NFT module

- Transaction types NftMint, NftTransfer, NftBurn, NftBoost exist (src/core/transaction.rs:274, 1155, 1403; proto_conversions.rs:79, 884).
- Mint: executor.rs:736-755. The data is `(ContentId, Option<String>)`. The executor does not check that the ContentId has a registered manifest, and does not check that no other NFT uses the same ContentId.
- Transfer and burn both refuse a token listed in a vault folder or a folder that is not empty (executor.rs:757-775, 794-814).
- State root includes the NFT registry (`NftRegistry::root`, socialfi/mod.rs, tag `BDLM_NFT_REGISTRY_V6`).
- Public versus private (social versus gallery) is not a field on the NFT. The vault module lists tokens in folders (src/socialfi/vault.rs). Visibility for Three objects lives in `MetadataVisibility` (three_nft.rs:34), which is not attached to any on-chain NFT.

### 5.2 Burn flow, step by step

1. Owner signs `NftBurn` with `data = bincode(u64 nft_id)`. RPC helper builds a template at src/rpc/server.rs:4869.
2. Executor arm (executor.rs:794-840) applies the folder locks, then `nft_registry.burn(id, &tx.from)` (socialfi/mod.rs:124). The only authority check is `nft.owner == owner`, else `NftError::NotOwner`. The NFT row and ownership entry are removed. The ContentId is returned. The executor only logs it (executor.rs:826-828).
3. Before the executor runs, `collect_nft_burn_cids_from_state` reads the ContentIds from the pre-state (blockchain.rs:4483-4500).
4. In `apply_block_effects`, after the block, for each burned ContentId the code calls `storage_registry.prune_content(&content_id, epoch)` (blockchain.rs:4528-4533). This is consensus state.
5. `prune_content` (storage_deal.rs:3609-3628) sets every Active deal of that manifest to Expired and then `manifests.remove(manifest_id)` (line 3625). Slashed or Expired deals are kept as an audit trail.
6. Physical delete on a node. `validate_and_add_block` and `produce_block` return the pruned ContentIds (blockchain.rs:4725). The node receives them in two places: gossip block accept sends `NodeCommand::StoragePrune` (network/node.rs:2198), and the interactive `block` command (src/main.rs:1674-1678). The node handler checks `PruningPolicy::should_prune_historical_state` and then calls `storage_node.store().delete(&ContentId(cid))` (node.rs:1693-1735). An archive node refuses (node.rs about 1715-1724).
7. The automatic block producer loop drops the list: `Some((block, _pruned_cids))` at src/main.rs:1608. On that path the producer node never sends the physical delete for its own blocks.

### 5.3 What is missing against directive 8.5

| Directive 8.5 item | State | Evidence |
|---|---|---|
| Owner may burn | Present | socialfi/mod.rs:124-129 |
| DAO may burn, independent of owner | Absent. No ProposalType or GovernanceAction touches NFTs, manifests or storage. List: ChangeBaseFee, ChangeBlockReward, SlashValidator, ParameterUpdate, WhitelistVerifier, DewhitelistVerifier, SetEncryptionPolicy, SetConstitutionParameter, VerifyHubApp, UnfreezeConsensusDomain | src/core/governance.rs:185-236 and 684-699 |
| DAO may not override decrypt or read | A hard guardrail exists | src/core/constitution.rs:14-16 (`NoGovernanceReadOverride`), pollen/data_rights.rs:8 |
| Burn targets the NFT, recipe deleted with it | Partial. Manifest is removed from the registry. The recipe objects (`ThreeNftRegistry`, `RecipeRecord`) have no remove method and no caller | three_nft.rs:237-320 |
| Validator physically deletes | Partial. Node-local, gated by pruning policy, key is the burned ContentId only, not each shard id; not sent from the automatic producer loop | node.rs:1693-1735, main.rs:1608 |
| Tombstone record | Absent. `rg -i tombstone` finds only a comment in account.rs:3513 | section 7 |
| No erasure for this class | Not modelled. No per-class erasure policy | n/a |
| Irreversible | Registry removal is irreversible in state. Nothing prevents re-registering the same manifest id later (`register_manifest` accepts any id not present; storage_tx.rs:388-393) | storage_tx.rs:370 |
| Unauthorised delete refused | Owner check only on the NFT. `ChainCommand::StoragePrune` calls `prune_content` with no authority check from a CLI or RPC trigger (chain_actor.rs:3443-3459) | chain_actor.rs:3443 |

Issues seen while reading, to be checked (not claimed as bugs):

- Shared ContentId. Two NFTs may carry the same ContentId (no uniqueness check at mint). Burning one prunes the manifest for both. Dedup is by design (manifest.rs:336-345), so this is a design question.
- The node deletes key `ContentId(cid)` in the bud-node store. The bud-node ContentId hash is not the same function as the chain ContentId (section 8, item 6). I did not trace which key the store uses for shard bytes.
- `ChainCommand::StoragePrune` mutates `blockchain.state.storage_registry` outside a block (chain_actor.rs:3447-3452). That conflicts with the rule that consensus state changes only in blocks (there is a gate named consensus_state_only_changes_in_blocks in xtask/gates). I did not run the gate.
- A sibling doc comment says physical deletion of chunks was "a separate verification matter" (src/tests/hard_prune.rs:11-13). The test only locks the registry effect.

## 6 GAP TABLE against Priority Zero

| # | Rule (directive 1.2 to 1.5) | Current state and evidence | Gap | Proposed step |
|---|---|---|---|---|
| 1 | Every content class converts, including zero bytes and very large | 11 classes all use one pipe (transformed.rs:54). Zero bytes refused in 6 places (2.6). Effective max about 0.8 MB at default block, about 11.9 MB at best block (2.5). RPC caps at 1 MiB (emit.rs:86) | Zero bytes, anything over about 11.9 MB, no segmenting | Needs owner decision on the empty-content encoding and on segmentation (section 8). Then extend A1 and A2, with tests |
| 2 | Both directions: recipe -> video -> content and content -> video -> recipe | content -> video -> content: `encode_qr_video` / `decode_qr_video`. content -> recipe: `VideoRecipe::from_encoded` (three_recipe.rs:203). recipe -> video: `reemit(body)` needs the body (three_recipe.rs:264). A recipe alone yields content only for Generated sources | "Recipe alone to content" is false for organic content | Define recipe kind per class. Decide whether WithBody counts as "recipe" (section 8, item 4) |
| 3 | Roundtrip equal by ContentId and recipe_hash | Equality is by sha256 inside A1 (qr_payload.rs:240) and `content_sha256` (three_recipe.rs). `ContentId::of` is compared only in emit for edition Three (emit.rs:861-865). No `recipe_hash` name. `decode_qr_video` compares to no external id (three_pipe.rs:297) | No caller-supplied expected id | Add a verify function that takes expected ContentId and recipe commitment, and compares |
| 4 | Commitment equality (output_commitment) | `VideoRecipe.video_commitment` binds the BDLV blob, `content_sha256` binds content. `reemit` checks both (three_recipe.rs:264-285) | No `output_commitment` in the form the directive uses. Videos depend on zlib bytes (2.8) | Pin which hash is the output commitment. Add compression-independent commitment |
| 5 | Verify over the real transport path | Only in-memory frame bytes. Loss tests drop frames and flip body bits (qr_video.rs:575-600). PNG decode accepts only our layout (qr_video.rs:258-331). No H.264/VP9 muxer (qr_codec.rs:199) | Re-encode, resize, compression, save and read back are all untested and would fail | Choose the real carrier (images only or video). Add a transport simulator and a tolerant frame reader. This is new code on the decode side and touches rule 1.1.3 |
| 6 | Independent second decoder path | Same `ProgressiveReceiver` twice (three_pipe.rs:221, emit.rs:902-908). `rqrr` vs in-tree encoder only in tests (qr_encode.rs:1012) | No independent carousel/unpack path | A second decode of A1 and A2 written separately (small), plus a differential test. Conflicts with directive 1.1.3; ask first |
| 7 | Three verification points: client, validator, reader | Client: none in src (wallet-core has no storage code). Validator: `register_manifest` sees only the manifest; it cannot see bytes (manifest.rs:797-801; storage_tx.rs:370). Reader: gateway checks size only for stored paths (gateway/service.rs:12-19); recipe path rehashes and compares (service.rs:28-85); bud-node bitswap rehashes (budzero/bud-node/src/bitswap.rs:215-221) | Validator point has no bytes to check. Reader point misses stored paths | Needs design: what the validator can check without bytes (a signed client attestation, a sampled challenge). Consensus surface. Separate design note |
| 8 | Determinism across platforms | No floats, clock or rand in qr_ and three_ code. Golden vectors (2.8). zlib via miniz_oxide pinned by test comment | No cross-platform CI for QR bytes. Dependency drift can change commitments | Add golden-vector job on three platforms for QR bytes. Consider storing the video commitment over a canonical, compression-free form |
| 9 | Audit record per verification | None. `EmitError` returns, `tracing` logs elsewhere. No record type | Missing | New record type and sink. Decide if on chain or node-local (section 8) |
| 10 | Fail-closed | Parsers refuse on malformed input (2.7). `decode_qr_video` accepts self-consistent forgeries. Reveal path skips the stream check (three_reveal.rs:81-89). PNG encoder falls back to stored on error (qr_png.rs:143-145) | Partial | Close the forged-video hole with the expected-id compare (row 3). Add a stream check on reveal when the full set is known |
| 11 | Test: content-type matrix | 10 synthetic samples (transformed.rs, about 614) and 11 classes (three_pipe.rs:556) | No real files, no large files, no encrypted via full chain at scale | Add real corpus fixtures |
| 12 | Test: boundary sizes | Version boundary sweep at the QR level (qr_encode.rs:1029). Per-layer empty tests | No pipe-level sweep at k = 1, block multiples, MAX_K, MAX_K + 1 | Add sweep |
| 13 | Test: property-based roundtrip | proptest exists in the crate; not used in qr_ or three_ | Missing | Add proptest for random bytes and block_len |
| 14 | Test: lossy transport | Frame level only | Pixel level missing | With row 5 |
| 15 | Test: decoder fuzz | No fuzz target (fuzz/fuzz_targets listing) | Missing | Targets for `QrVideo::from_bytes`, `png_to_optical_frame`, `unpack_frame`, `Drop::from_bytes`, `unpack_payload`, `ProgressiveReceiver`. The xtask gate every_fuzz_target_is_run exists; wire new targets into it |
| 16 | Test: differential of two decoders | Missing (row 6) | Missing | With row 6 |
| 17 | Test: determinism | Golden vectors on one platform at a time | No multi-platform compare | Row 8 |
| 18 | Test: negative tests | Many, per layer. None that corrupt a whole video and expect a verification layer to reject it, because there is no such layer | Partial | After row 3 |

## 7 BUD 1.0, 2.0, 3.0 gap lists

### 7.1 BUD 1.0 personal storage node

Exists:

- `MobileSelfProfile`, `MobileSelfContentPolicy`, `DeclareSelfHostPolicy` in block (storage_tx.rs:45, mobile_self.rs).
- `OperatorClass::Mobile` with the rule that a mobile operator holds no primary replica (docs/BUD_STORAGE_ROADMAP.md, section "Phones cannot hold the primary") and `DeclareOperatorClass` tx.
- Retrieval challenge with signed answer, interim proof only (README.md warning 1).
- Unwired model code: `one_share.rs` (a share NFT marker, no storage fee), `one_view.rs` (single screen), `server_admission.rs`, `CustodyLedger`. These match the directive's idea but have no caller.
- `MISSED_CHALLENGE_COOLDOWN_SECS = 6 h` (storage_deal.rs:121).

Missing:

- `StorageClaim` transaction (account id, device id, capacity, address, validity window, nonce). No type, no tx variant, no executor arm.
- `Heartbeat` message and the suspended/active state machine. `StorageLifecycleState` (lifecycle.rs:9) describes deal states, not device state.
- Visibility link from social content to a claim. Reference becoming "unavailable".
- PSN runtime for mobile and desktop targets. The `crates/` list has wallet-core, note-packing, budscan, bpqs, ai-inference; none has storage code (rg).
- Offline tolerance defaults (directive open question 3).

### 7.2 BUD 2.0 compression and cost

Exists:

- A0 classify, A1 zlib-if-smaller. No other codec in src (transformed.rs, qr_payload.rs).
- Dictionary objects (dictionary.rs), derived regions (derived.rs), generated content (generated.rs), render (render.rs). Measured claim in the roadmap: 72.6 percent reduction versus 3x replication, volume weighted (docs/BUD_STORAGE_ROADMAP.md, near line 454-480).
- Reed-Solomon coder, coding audit, repair tickets, placement. Mostly unwired for production writes (3 above).
- A separate crate `bud/` with codecs and measurements: `bud_format_jpegre.rs` (it measures JPEG compressibility and records a decision; the header says it moves to JXL later, so it is not a Lepton-like recoder), `bud_format_dedup.rs` (tenant-local dedup index, no cross-tenant), `measure_ratios.rs`. The root crate does not use it.
- The 0.016 USD figure appears only in bud/ (FORMAT-V2.md:150-153, 174; bud/src/bin/bud.rs:414-418). FORMAT-V2 says the honest price at the time was about 0.031. There is no constant or gate for it in src.

Missing:

- A lossless image/audio/video recoder in the root crate (Lepton-like). No `lepton` string anywhere in src, bud or docs.
- Erasure schedule (10,16) then (20,26) with a one-week health gate and old shard deletion after verification. No default scheme, no migration code.
- Cost measurement harness in the units of the directive (7.4), with a repeatable command.
- Per-access fee charged to the reader. Deals use `fee_per_byte_epoch` escrow (storage_tx.rs:68-86) and are refused on mainnet (storage_tx.rs:251).
- State expiry for unread Stored content.
- Wiring of encode and reconstruct into a production write path, and an in-block audit answer (STATUS.md owner decision B1/B2).
- AI audit pipeline (7.6). Not looked for in src; I found no such module. The xtask gates are the nearest thing.

### 7.3 BUD 3.0 recipe NFTs

Exists: sections 2 and 5. Recipe types, QR video, reveal sessions, NFT metadata type, burn with manifest removal.

Missing:

- Canonical recipe standard with version, kind, generator_id, seed, params, output_spec, output_commitment, visibility, lifetime (directive 8.3). `GeneratedSpec` exists in generated.rs for the Generated class; not read in full.
- Link between an NFT and a recipe commitment. `Nft` has only a ContentId.
- Lifetime selection, prepaid rent, extension, expiry into deletion, notification. No `lifetime` anywhere in src/storage or src/domain related to NFTs (the only hit is `max_events_per_lifetime` in regeneration_stage.rs:181).
- DAO burn authority.
- Tombstone record and a test that regeneration fails after burn.
- Public NFT in social feed and private NFT in gallery. Social/gallery consumers: gateway render (service.rs:283) and vault folders; no feed query.
- Fingerprint interface and duplicate index (directive 9.4): `rg -i "fingerprint|perceptual|phash|simhash"` finds no code in src or bud. Exact dedup exists only as manifest-id equality (`AlreadyRegistered`, storage_tx.rs:388-393).

## 8 Contradictions and open questions

Process contradictions:

1. The directive says no local repo and to work through Workspace (BUD-AI-KAPSAMLI-DIREKTIF.md 0.2.4). This session works in a cloud checkout of the repo (/home/user/budlum). I read and wrote files in that checkout. No Workspace connector was used. Directive 0.2.3 says conflicts must be reported. This is that report.
2. The directive asks for separate report files per gate (directive 13). CLAUDE.md section 4 item 6 says reports go in PR descriptions with per-file test counts. This file follows the directive because the owner asked for it by path. The PR description should link it.
3. The directive 0.3 uses "Budlum L1". CLAUDE.md Z12 forbids "L1" in public text. This report avoids the term.
4. Directive 1.1.3 forbids writing a new QR encoder or decoder. Directive 1.3.4 requires an independent second decoder path, and 1.5.5 and 8.6.4 require fuzzed decoders. A second decoder is new code. Directive 2.5.5 makes two conflicting rules a stop condition. K1 needs an owner answer before coding.

Technical contradictions:

5. Derivative versus universal surface. The existing design says the QR video is a derivative and must never be durable storage. See three_gate.rs:3-10, three_recipe.rs:19-22, bud/src/bud_format_qrvideo.rs:3-6 (it also says QR video grows compressed bytes 12 to 18 times). The directive makes the QR video the verification surface of every content and publication path. These can agree only if the video is regenerated on demand and never stored, and verification runs at upload and at read. Nobody has written this down.
6. Four ContentId definitions.
   - src: SHA-256 over length-prefixed fields (tag, then chunk) (content_id.rs:45, core/hash.rs:13-20).
   - crates/budscan: same as src (crates/budscan/src/content_id.rs:47). Parity checked by xtask budscan_parity (not run).
   - budzero/bud-node: SHA-256 of tag then chunk, no length prefixes (budzero/bud-node/src/store.rs:29-37), while its comment says it matches budlum-core.
   - bud/: SHA3-256 with a u64 length (bud/src/bud_format_container.rs:138).
   By reading, the bud-node id differs from the chain id for the same bytes. The node hard prune deletes by `bud_node::store::ContentId(cid)` using a chain id (node.rs:1726). I did not compute sample values or find a test linking the two. Ask for a decision and a cross-check test.
7. Directive 3.1.1 says owner and size are not in the content id. True for `ContentManifest` (manifest.rs:333-345). src/storage/README.md warning 3 still says the owner is covered. README is stale.
8. Directive 3.2 says the access counter is not bound to the manifest and binding touches consensus. Code: `access_events` is a log inside `StorageRegistry` (storage_deal.rs:697), fed when a challenge is answered (living_threshold.rs:85-92), and `required_replicas_with_demand` reads it (storage_deal.rs:1542). So the demand half is already in registry state. I did not check whether it is folded into the registry root. Directive 7.3 treats it as a consensus design task.
9. Directive 3.2 says erasure (10,16) is coded. The coder takes any (k, n) up to 255 shards; no (10,16) constant exists in production code.
10. Directive 3.1.4 allows only two actors, user and validator. Code has `OperatorClass` and operator bonds, consents and cooldowns. docs/VALIDATOR_ROLES.md exists; I did not read it.
11. `bud_storageQrFeedPreview` takes a seal seed as an RPC argument (server.rs:3284-3305). Directive 3.1.5 says keys come from the wallet seed on the client. A seed sent to a node is a key leak path. Owner must decide whether this RPC stays.
12. qr_reemit.rs:3-7 and three_reveal.rs:81-89 contradict each other (section 2.11). qr_video.rs:334 says the PNG encoder writes stored zlib only, but qr_png.rs:135-145 deflates at best level.

Open questions for the owner (nothing was assumed in the report):

1. Empty content. Today it is refused in 6 places. A fix changes the A1 wire format (`orig_len == 0` is refused on read). Which encoding for zero bytes?
2. Size ceiling. Segment large content into several videos, or raise k, or accept an upper bound and refuse above it? Directive 1.2.2 forbids refusal as an outcome.
3. Carrier. Must a frame survive real H.264 or VP9 re-encoding, or only lossless PNG sequences? This decides the QR size, EC level (fixed at L today; directive 8.6.3 wants it configurable) and the reader.
4. What is a "recipe" for organic content: a fixed-size `ThreeRecipePublic` plus a stored body, or something else? Directive 8.1 says only the recipe is on chain.
5. Validator verification. The chain holds no bytes (manifest.rs:797-801). What may a validator check before a commitment, and with what evidence? This touches consensus and needs its own design note and PR (directive 2.3.3).
6. Where verification audit records live: chain, node log, or both. Directive 1.3.7 does not say.
7. DAO authority over NFTs: new `ProposalType`, its quorum, its delay, and how it fits the constitution guardrails.
8. Tombstone content: what fields, who pays for it, and whether it lives in registry state.
9. Shared ContentId and burn: does a burn delete content that another NFT still names? Directive 8.5.1 says there is no reference counter.
10. Physical delete: should every node delete, or only those whose policy allows? Archive nodes refuse today (node.rs about 1715). And fix the automatic producer loop that drops the list (main.rs:1608).
11. Manual `ChainCommand::StoragePrune` (chain_actor.rs:3443): keep, remove, or restrict.
12. Hash for ContentId across crates (item 6).
13. Directive open questions 1, 3, 4, 5, 6 (cost mix, offline thresholds, price curve, access counter design, scope of in-consensus duplicate check) are unanswered in the repo. Not touched.
14. STATUS.md owner decision D6 refuses OpenDeal on mainnet. The directive's lifetime pricing and per-access fees need a mainnet path. How do they relate?

Differences between older BUD docs and the directive:

| Topic | Older doc | Directive | Note |
|---|---|---|---|
| QR video role | Derivative, not storage. bud/src/bud_format_qrvideo.rs:3-6; three_gate.rs:3-10; FORMAT-V2 has no QR video | Universal verification surface, publication path | Item 5 above |
| Content id hash | docs/BUD_STORAGE_ROADMAP.md: SHA-256 plus tag. bud/FORMAT-V2.md: SHA3-256 plus tag plus length | "Domain tagged hash", no algorithm | Item 6 |
| Owner in identity | src/storage/README.md warning 3: covered. manifest.rs:333-345: excluded | Excluded (3.1.1) | README stale |
| Erasure | README warning 5: nobody computes parity, chain never sees shard bytes. Roadmap: coder closed, production encoding open | (10,16) exists but unwired, then (20,26) | Consistent on unwired. No default scheme in code |
| Proof of storage | Interim availability only (README warning 1; roadmap Gap 1) | Same, plus honest labelling | Consistent |
| Coding audit | README: sampled column, probabilistic; roadmap Gap 3b | Wire it | Consistent. Chain actor only logs scheduling |
| Cost | Roadmap: 0.82x multiplier, 72.6 percent reduction versus 3x replication. FORMAT-V2: 0.016 needs about 17x ratio, honest price about 0.031. living_threshold: 0.29 USD/TB/month disk | 0.016 target, 0.6x compression, 0.18 composite, 0.29 raw | FORMAT-V2 and directive disagree on what ratio is realistic. Directive 7.1 asks for a mix-specific measurement |
| Dedup | bud/ dedup is tenant-local, no cross-tenant (bud_format_dedup.rs:1-8) | Same bytes, one copy, same ContentId (7.2.4, 3.1.1) | Cross-tenant dedup is in the directive; the older bud/ decision rejects it for privacy |
| Roles | OperatorClass AlwaysOn/Mobile; phones cannot hold primary | Only user and validator | Item 10 |
| Deletion | Roadmap: stale shards derived, not stored; no tombstone | Stored tombstone, physical delete, DAO burn | New work |
| Encryption | docs/BUD_CONTENT_ENCRYPTION.md: declared, not enforced, chain carries no key material | Client AEAD, keys from wallet seed, flag in manifest | Consistent. RPC seed argument conflicts (item 11) |
| Storage writes | STATUS.md owner decision B1: all writes become signed in-block transactions | Silent on tx model; asks for StorageClaim tx | Fits; StorageClaim would be a new `StorageTx` variant |
| Recipe on chain | three_recipe.rs: organic content cannot be a short recipe; body kept | Only recipe on chain for every content | Item 4 |

## 9 Summary of the K0 verdict

- The QR video system exists and is the right base. It meets part of 8.6: frame header with version, stream prefix, seq and digest; fountain code; fixed EC level L; bounded parsers. It fails Priority Zero in these ways: zero bytes refused, effective size limit about 0.8 to 11.9 MB, no real transport path, no independent second decoder, no caller-supplied id check in `decode_qr_video`, no fuzz, no property test, no cross-platform proof, no audit records, no validator or client verification point.
- Everything QR is reachable only through two read-only RPC families. It is not wired into uploads, validation, NFT mint or storage commitments.
- The NFT burn path removes the NFT and the manifest in consensus state and asks nodes to delete. It has no DAO path, no tombstone, no recipe-record delete, and the automatic producer loop drops the prune list.
- No code exists for StorageClaim, Heartbeat, PSN states, lifetime pricing, fingerprinting, Lepton-like recoding, the 0.016 gate or the erasure migration.
- Stop conditions in directive 2.5 that apply now: rule conflict between 1.1.3 and 1.3.4 (item 4 of section 8), and consensus-surface work for validator verification (open question 5). K1 should not start coding until the owner answers open questions 1 to 5 and 12.

Scout leads that did not hold:

- Drop header 28 bytes: it is 24 (qr_carousel.rs:66, test at 1045).
- MAX_BLOCK_LEN 7920: it is 8168 (qr_carousel.rs:47).
- "rqrr ProgressiveReceiver as a second decode path": `ProgressiveReceiver` is the same path as `decode_frames`; rqrr is the symbol reader, not a second one.
- "three_regime, msr unwired": confirmed. "qr_reemit.rs:7 unwired (reveal session)": the marker says so, but the reveal RPC exists, so the statement is stale.
- "three_nft wired through qr_feed_preview": true for `ThreeNftMeta` only; `ThreeNftRegistry` has no caller.
- "chain_actor.rs ~3410 notifies node": it only logs; the real senders are node.rs:2198 and main.rs:1677.
- "storage_deal.rs ~3609 removes manifest at ~3625": confirmed at 3609 and 3625.
- "no DAO authority over NFTs", "no StorageClaim or Heartbeat", "no perceptual fingerprint", "no 0.016 constant in src", "no tombstone": confirmed by rg.
