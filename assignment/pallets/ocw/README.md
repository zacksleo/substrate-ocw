# Substrate Offchain Worker

## 核心代码

https://github.com/zacksleo/substrate-ocw/blob/master/assignment/pallets/ocw/src/lib.rs#L115

1. 使用 #[serde(deserialize_with=)] 宏, 将 json 数据反序列化成 struct
2. 使用 #[serde(rename_all = "camelCase")] 宏, 将驼峰转为下划线格式的字段名

```rust

    // polkadot price
    pub type PolkadotPrice = (u64, Permill);


    // polkadot price data
    #[derive(Deserialize, Encode, Decode, Default, Debug)]
    #[serde(rename_all = "camelCase")]
    struct PriceInfo {
      #[serde(deserialize_with = "de_string_to_tuple")]
      price_usd: PolkadotPrice,
    }

    // 将价格字符串反序列化为元组 (u64, Permill)
    pub fn de_string_to_tuple<'de, D>(de: D) -> Result<PolkadotPrice, D::Error>
    where
      D: Deserializer<'de>,
    {
      let s: &str = Deserialize::deserialize(de)?;
      let price_usd: Vec<&str> = s.split(".").collect();
      let price_usd_num: u64 = price_usd[0].parse().unwrap();
      let price_usd_permill: Permill = Permill::from_parts(price_usd[1][..6].parse::<u32>().unwrap());
      Ok((price_usd_num, price_usd_permill))
    }

```

对于作业中的问题, 这里采用 “不签名但具签名信息的交易”, 原因如下:

需要知道该交易来源是谁，但不需要该用户付手续费, 故使用“不签名但具签名信息的交易”

```rust

      /// 获取当前 DOT 的美元价格, 并提交到回链上
      fn fetch_price_info() -> Result<(), Error<T>> {

        let mut lock = StorageLock::<BlockAndTime<Self>>::with_block_and_time_deadline(
          b"offchain-demo-price::lock", LOCK_BLOCK_EXPIRATION,
          rt_offchain::Duration::from_millis(LOCK_TIMEOUT_EXPIRATION),
        );

        // We try to acquire the lock here. If failed, we know the `fetch_n_parse` part inside is being
        //   executed by previous run of ocw, so the function just returns.
        if let Ok(_guard) = lock.try_lock() {
          match Self::fetch_n_parse::<DataWrapper<PriceInfo>>(HTTP_POLKADOT_PRICE_API) {
            Ok(info) => {
              // 需要知道该交易来源是谁，但不需要该用户付手续费, 故使用“不签名但具签名信息的交易”
              let _ = Self::offchain_unsigned_tx_signed_payload_price(info.data.price_usd);
            }
            Err(err) => {
              return Err(err);
            }
          }
        }

        Ok(())
      }

```

其他, 改造部分代码, 添加泛型, 以最大程度的简化代码

如将请求方法, 统一封装为 `	fn fetch_from_remote(url: &str) -> Result<Vec<u8>, Error<T>> `,  通过 url 就可以获取数据


### 编译日志

```bash
➜  assignment git:(master) cargo build --release
warning: unused config key `build.rustc_wrapper` in `/Users/zacksleo/.cargo/config`
   Compiling libc v0.2.98
   Compiling proc-macro2 v1.0.28
   Compiling unicode-xid v0.2.2
   Compiling syn v1.0.74
   Compiling version_check v0.9.3
   Compiling cfg-if v1.0.0
   Compiling autocfg v1.0.1
   Compiling serde v1.0.130
   Compiling serde_derive v1.0.130
   Compiling log v0.4.14
   Compiling scopeguard v1.1.0
   Compiling memchr v2.4.0
   Compiling smallvec v1.6.1
   Compiling lazy_static v1.4.0
   Compiling typenum v1.13.0
   Compiling futures v0.1.31
   Compiling slab v0.4.3
   Compiling byteorder v1.4.3
   Compiling cfg-if v0.1.10
   Compiling ppv-lite86 v0.2.10
   Compiling futures-core v0.3.16
   Compiling pin-project-lite v0.2.7
   Compiling futures-io v0.3.16
   Compiling anyhow v1.0.42
   Compiling proc-macro-hack v0.5.19
   Compiling futures-sink v0.3.16
   Compiling futures-task v0.3.16
   Compiling proc-macro-nested v0.1.7
   Compiling futures-channel v0.3.16
   Compiling pin-utils v0.1.0
   Compiling getrandom v0.1.16
   Compiling subtle v2.4.1
   Compiling either v1.6.1
   Compiling opaque-debug v0.3.0
   Compiling tinyvec_macros v0.1.0
   Compiling static_assertions v1.1.0
   Compiling block-padding v0.2.1
   Compiling crunchy v0.2.2
   Compiling cpufeatures v0.1.5
   Compiling arrayref v0.3.6
   Compiling byte-tools v0.3.1
   Compiling arrayvec v0.5.2
   Compiling constant_time_eq v0.1.5
   Compiling keccak v0.1.0
   Compiling opaque-debug v0.2.3
   Compiling fake-simd v0.1.2
   Compiling subtle v1.0.0
   Compiling itoa v0.4.7
   Compiling radium v0.6.2
   Compiling ryu v1.0.5
   Compiling signature v1.3.1
   Compiling regex-syntax v0.6.25
   Compiling libm v0.2.1
   Compiling serde_json v1.0.67
   Compiling wyz v0.2.0
   Compiling tap v1.0.1
   Compiling funty v1.1.0
   Compiling arrayvec v0.7.1
   Compiling byte-slice-cast v1.0.0
   Compiling rustc-hash v1.1.0
   Compiling hex v0.4.3
   Compiling rustc-hex v2.1.0
   Compiling adler v1.0.2
   Compiling arrayvec v0.4.12
   Compiling parity-util-mem v0.10.0
   Compiling fnv v1.0.7
   Compiling sp-std v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling zstd-safe v3.0.1+zstd.1.4.9
   Compiling parity-wasm v0.42.2
   Compiling erased-serde v0.3.16
   Compiling ref-cast v1.0.6
   Compiling slog v2.7.0
   Compiling downcast-rs v1.2.0
   Compiling ansi_term v0.12.1
   Compiling hash-db v0.15.2
   Compiling nodrop v0.1.14
   Compiling memory_units v0.3.0
   Compiling environmental v1.1.3
   Compiling gimli v0.25.0
   Compiling tiny-keccak v2.0.2
   Compiling dyn-clone v1.0.4
   Compiling async-trait v0.1.51
   Compiling rustc-demangle v0.1.20
   Compiling base58 v0.1.0
   Compiling convert_case v0.4.0
   Compiling paste v1.0.5
   Compiling bytes v1.0.1
   Compiling unicode-segmentation v1.8.0
   Compiling remove_dir_all v0.5.3
   Compiling bitflags v1.2.1
   Compiling futures-timer v3.0.2
   Compiling fixedbitset v0.2.0
   Compiling multimap v0.8.3
   Compiling matches v0.1.8
   Compiling crossbeam-utils v0.8.5
   Compiling semver-parser v0.7.0
   Compiling cache-padded v1.1.1
   Compiling untrusted v0.7.1
   Compiling spin v0.5.2
   Compiling parking v2.0.0
   Compiling fastrand v1.5.0
   Compiling waker-fn v1.1.0
   Compiling percent-encoding v2.1.0
   Compiling event-listener v2.5.1
   Compiling httparse v1.4.1
   Compiling bytes v0.5.6
   Compiling data-encoding v2.3.2
   Compiling pin-project-lite v0.1.12
   Compiling async-task v4.0.3
   Compiling signal-hook v0.3.9
   Compiling pin-project-internal v0.4.28
   Compiling atomic-waker v1.0.0
   Compiling unsigned-varint v0.5.1
   Compiling stable_deref_trait v1.2.0
   Compiling try-lock v0.2.3
   Compiling crc32fast v1.2.1
   Compiling bs58 v0.4.0
   Compiling fallible-iterator v0.2.0
   Compiling quick-error v1.2.3
   Compiling void v1.0.2
   Compiling target-lexicon v0.12.1
   Compiling asn1_der v0.7.4
   Compiling prometheus v0.11.0
   Compiling tower-service v0.3.1
   Compiling httpdate v0.3.2
   Compiling wasmparser v0.78.2
   Compiling base64 v0.13.0
   Compiling more-asserts v0.2.1
   Compiling crossbeam-epoch v0.9.5
   Compiling rayon-core v1.9.1
   Compiling termcolor v1.1.2
   Compiling wasmtime-cache v0.27.0
   Compiling wasm-bindgen-shared v0.2.74
   Compiling cpp_demangle v0.3.3
   Compiling linked-hash-map v0.5.4
   Compiling ipnet v2.3.1
   Compiling bumpalo v3.7.0
   Compiling pkg-config v0.3.19
   Compiling cpuid-bool v0.2.0
   Compiling maybe-uninit v2.0.0
   Compiling scoped-tls v1.0.0
   Compiling base64 v0.12.3
   Compiling match_cfg v0.1.0
   Compiling wasm-bindgen v0.2.74
   Compiling nohash-hasher v0.2.0
   Compiling parity-send-wrapper v0.1.0
   Compiling hex_fmt v0.3.0
   Compiling radium v0.5.3
   Compiling base-x v0.2.8
   Compiling glob v0.3.0
   Compiling take_mut v0.2.2
   Compiling ucd-trie v0.1.3
   Compiling rawpointer v0.2.1
   Compiling ip_network v0.3.4
   Compiling failure_derive v0.1.8
   Compiling sc-consensus-slots v0.10.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling bindgen v0.59.1
   Compiling winapi v0.3.9
   Compiling core-foundation-sys v0.7.0
   Compiling peeking_take_while v0.1.2
   Compiling shlex v1.0.0
   Compiling lazycell v1.3.0
   Compiling percent-encoding v1.0.1
   Compiling mio-named-pipes v0.1.7
   Compiling retain_mut v0.1.3
   Compiling pdqselect v0.1.0
   Compiling camino v1.0.5
   Compiling maplit v1.0.2
   Compiling unicode-width v0.1.8
   Compiling futures-timer v2.0.2
   Compiling platforms v1.1.0
   Compiling same-file v1.0.6
   Compiling names v0.11.0
   Compiling vec_map v0.8.2
   Compiling strsim v0.8.0
   Compiling ansi_term v0.11.0
   Compiling quick-error v2.0.1
   Compiling safe-mix v1.0.1
   Compiling instant v0.1.10
   Compiling lock_api v0.4.4
   Compiling lock_api v0.3.4
   Compiling tracing-core v0.1.18
   Compiling sharded-slab v0.1.1
   Compiling tinyvec v1.3.1
   Compiling value-bag v1.0.0-alpha.7
   Compiling ahash v0.7.4
   Compiling generic-array v0.14.4
   Compiling proc-macro-error-attr v1.0.4
   Compiling proc-macro-error v1.0.4
   Compiling nom v6.1.2
   Compiling unicase v2.6.0
   Compiling futures-macro v0.3.16
   Compiling futures-util v0.3.16
   Compiling indexmap v1.7.0
   Compiling num-traits v0.2.14
   Compiling num-integer v0.1.44
   Compiling miniz_oxide v0.4.4
   Compiling num-bigint v0.2.6
   Compiling num-rational v0.2.4
   Compiling memoffset v0.6.4
   Compiling rayon v1.5.1
   Compiling crossbeam-utils v0.7.2
   Compiling atomic v0.5.0
   Compiling memoffset v0.5.6
   Compiling crossbeam-epoch v0.8.2
   Compiling num-rational v0.4.0
   Compiling block-padding v0.1.5
   Compiling itertools v0.10.1
   Compiling ed25519 v1.2.0
   Compiling libloading v0.7.0
   Compiling trie-root v0.16.0
   Compiling blake2b_simd v0.5.11
   Compiling blake2s_simd v0.5.11
   Compiling itertools v0.9.0
   Compiling http v0.2.4
   Compiling tokio-sync v0.1.8
   Compiling tokio-service v0.1.0
   Compiling heck v0.3.3
   Compiling unicode-bidi v0.3.5
   Compiling concurrent-queue v1.2.2
   Compiling semver v0.9.0
   Compiling semver v0.6.0
   Compiling form_urlencoded v1.0.1
   Compiling async-mutex v1.4.0
   Compiling async-lock v2.4.0
   Compiling owning_ref v0.4.1
   Compiling wasmi-validation v0.4.0
   Compiling humantime v1.3.0
   Compiling dns-parser v0.8.0
   Compiling lru-cache v0.1.2
   Compiling linked_hash_set v0.1.4
   Compiling regex-automata v0.1.10
   Compiling matrixmultiply v0.3.1
   Compiling pest v2.1.3
   Compiling clang-sys v1.2.0
   Compiling textwrap v0.11.0
   Compiling walkdir v2.3.2
   Compiling addr2line v0.16.0
   Compiling parity-wasm v0.32.0
   Compiling unicode-normalization v0.1.19
   Compiling rustc_version v0.2.3
   Compiling build-helper v0.1.1
   Compiling http-body v0.3.1
   Compiling matchers v0.0.1
   Compiling semver-parser v0.10.2
   Compiling pest_meta v2.1.3
   Compiling async-channel v1.6.1
   Compiling quicksink v0.1.2
   Compiling aho-corasick v0.7.18
   Compiling object v0.26.0
   Compiling futures-lite v1.12.0
   Compiling bstr v0.2.16
   Compiling parking_lot_core v0.8.3
   Compiling num_cpus v1.13.0
   Compiling getrandom v0.2.3
   Compiling parking_lot_core v0.7.2
   Compiling time v0.1.44
   Compiling iovec v0.1.4
   Compiling net2 v0.2.37
   Compiling signal-hook-registry v1.4.0
   Compiling socket2 v0.4.1
   Compiling socket2 v0.3.19
   Compiling atty v0.2.14
   Compiling mach v0.3.2
   Compiling dirs-sys-next v0.1.2
   Compiling errno v0.2.7
   Compiling hostname v0.3.1
   Compiling if-addrs v0.6.5
   Compiling memmap2 v0.2.3
   Compiling fs2 v0.4.3
   Compiling rand v0.4.6
   Compiling dirs-sys v0.3.6
   Compiling fdlimit v0.2.1
   Compiling rpassword v5.0.1
   Compiling jobserver v0.1.22
   Compiling which v4.2.2
   Compiling quote v1.0.9
   Compiling uint v0.9.1
   Compiling hash256-std-hasher v0.15.2
   Compiling bitvec v0.20.4
   Compiling generic-array v0.12.4
   Compiling blake2-rfc v0.2.18
   Compiling idna v0.2.3
   Compiling idna v0.1.5
   Compiling snow v0.7.2
   Compiling parking_lot_core v0.6.2
   Compiling parking_lot v0.9.0
   Compiling hyper v0.12.36
   Compiling crossbeam-channel v0.5.1
   Compiling smallvec v0.6.14
   Compiling bitvec v0.19.5
   Compiling security-framework-sys v1.0.0
   Compiling core-foundation v0.7.0
   Compiling miow v0.3.7
   Compiling futures-cpupool v0.1.8
   Compiling threadpool v1.8.1
   Compiling substrate-build-script-utils v3.0.0 (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling rand_core v0.6.3
   Compiling rand_core v0.5.1
   Compiling parking_lot v0.11.1
   Compiling parking_lot v0.10.2
   Compiling bytes v0.4.12
   Compiling clap v2.33.3
   Compiling directories-next v2.0.0
   Compiling regex v1.5.4
   Compiling resolv-conf v0.7.0
   Compiling directories v3.0.2
   Compiling region v2.2.0
   Compiling cc v1.0.69
   Compiling prost-build v0.7.0
   Compiling rand v0.3.23
   Compiling digest v0.8.1
   Compiling block-buffer v0.7.3
   Compiling crypto-mac v0.7.0
   Compiling digest v0.9.0
   Compiling block-buffer v0.9.0
   Compiling crypto-mac v0.8.0
   Compiling block-cipher v0.8.0
   Compiling universal-hash v0.4.1
   Compiling aead v0.3.2
   Compiling cipher v0.2.5
   Compiling url v2.2.2
   Compiling security-framework v1.0.0
   Compiling url v1.7.2
   Compiling tokio-executor v0.1.10
   Compiling crossbeam-queue v0.2.3
   Compiling rand_chacha v0.3.1
   Compiling ocw-example v3.0.0-monthly-2021-08 (/Users/zacksleo/projects/github/zacksleo/substrate-ocw/assignment/node)
   Compiling integer-sqrt v0.1.5
   Compiling approx v0.5.0
   Compiling num-complex v0.4.0
   Compiling rand_chacha v0.2.2
   Compiling rand_pcg v0.2.1
   Compiling once_cell v1.8.0
   Compiling http v0.1.21
   Compiling string v0.2.1
   Compiling tokio-buf v0.1.1
   Compiling Inflector v0.11.4
   Compiling hmac v0.7.1
   Compiling pbkdf2 v0.3.0
   Compiling sha2 v0.8.2
   Compiling sha-1 v0.8.2
   Compiling crossbeam-deque v0.8.1
   Compiling pbkdf2 v0.4.0
   Compiling stream-cipher v0.7.1
   Compiling aes-soft v0.5.0
   Compiling sha2 v0.9.5
   Compiling hmac v0.8.1
   Compiling sha3 v0.9.1
   Compiling blake2 v0.9.1
   Compiling sha-1 v0.9.7
   Compiling polyval v0.4.5
   Compiling poly1305 v0.6.2
   Compiling salsa20 v0.7.2
   Compiling tokio-current-thread v0.1.7
   Compiling tokio-timer v0.2.13
   Compiling rand v0.8.4
   Compiling crossbeam-deque v0.7.4
   Compiling chrono v0.4.19
   Compiling rand v0.7.3
   Compiling thread_local v1.1.3
   Compiling blocking v1.0.2
   Compiling async-executor v1.4.1
   Compiling zstd-sys v1.4.20+zstd.1.4.9
   Compiling backtrace v0.3.61
   Compiling ring v0.16.20
   Compiling blake3 v0.3.8
   Compiling wasmtime-runtime v0.27.0
   Compiling psm v0.1.14
   Compiling libz-sys v1.1.3
   Compiling libloading v0.5.2
   Compiling simba v0.5.1
   Compiling hmac-drbg v0.2.0
   Compiling aes v0.5.0
   Compiling ghash v0.3.1
   Compiling http-body v0.1.0
   Compiling cexpr v0.5.0
   Compiling hashbrown v0.11.2
   Compiling fixed-hash v0.7.0
   Compiling rand_distr v0.4.1
   Compiling tempfile v3.2.0
   Compiling twox-hash v1.6.0
   Compiling cuckoofilter v0.5.0
   Compiling libsecp256k1 v0.3.5
   Compiling aes-gcm v0.7.0
   Compiling lru v0.6.6
   Compiling synstructure v0.12.5
   Compiling pest_generator v2.1.3
   Compiling wasmi v0.9.0
   Compiling sp-panic-handler v3.0.0 (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling flate2 v1.0.20
   Compiling fs-swap v0.2.6
   Compiling ctor v0.1.20
   Compiling thiserror-impl v1.0.26
   Compiling zeroize_derive v1.1.0
   Compiling impl-trait-for-tuples v0.2.1
   Compiling tracing-attributes v0.1.15
   Compiling parity-util-mem-derive v0.1.0
   Compiling sp-debug-derive v3.0.0 (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling ref-cast-impl v1.0.6
   Compiling dyn-clonable-impl v0.9.0
   Compiling derive_more v0.99.16
   Compiling pin-project-internal v1.0.8
   Compiling prost-derive v0.7.0
   Compiling enum-as-inner v0.3.3
   Compiling minicbor-derive v0.6.4
   Compiling libp2p-swarm-derive v0.23.0
   Compiling frame-support-procedural-tools-derive v3.0.0 (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling data-encoding-macro-internal v0.1.10
   Compiling nalgebra-macros v0.1.0
   Compiling strum_macros v0.20.1
   Compiling structopt-derive v0.4.15
   Compiling pest_derive v2.1.0
   Compiling zeroize v1.4.1
   Compiling thiserror v1.0.26
   Compiling dyn-clonable v0.9.0
   Compiling pin-project v1.0.8
   Compiling data-encoding-macro v0.1.12
   Compiling pin-project v0.4.28
   Compiling failure v0.1.8
   Compiling prost v0.7.0
   Compiling curve25519-dalek v3.1.0
   Compiling merlin v2.0.1
   Compiling curve25519-dalek v2.1.3
   Compiling secrecy v0.7.0
   Compiling chacha20 v0.5.0
   Compiling tiny-bip39 v0.8.0
   Compiling minicbor v0.8.1
   Compiling multibase v0.8.0
   Compiling tracing v0.1.26
   Compiling tracing-log v0.1.2
   Compiling trie-db v0.22.6
   Compiling mio v0.6.23
   Compiling polling v2.1.0
   Compiling kv-log-macro v1.0.7
   Compiling want v0.3.0
   Compiling env_logger v0.7.1
   Compiling pwasm-utils v0.18.1
   Compiling tokio-io v0.1.13
   Compiling tokio-threadpool v0.1.18
   Compiling globset v0.4.8
   Compiling want v0.2.0
   Compiling parity-db v0.2.4
   Compiling wasm-bindgen-backend v0.2.74
   Compiling wasm-gc-api v0.1.11
   Compiling librocksdb-sys v6.20.3
   Compiling strum v0.20.0
   Compiling structopt v0.3.22
   Compiling chacha20poly1305 v0.6.0
   Compiling nalgebra v0.27.1
   Compiling prost-types v0.7.0
   Compiling schnorrkel v0.9.1
   Compiling x25519-dalek v1.1.1
   Compiling tracing-futures v0.2.5
   Compiling async-io v1.6.0
   Compiling file-per-thread-logger v0.1.4
   Compiling mio-uds v0.6.8
   Compiling mio-extras v2.0.6
   Compiling tokio-reactor v0.1.12
   Compiling tokio-codec v0.1.2
   Compiling tokio-fs v0.1.7
   Compiling futures-executor v0.3.16
   Compiling asynchronous-codec v0.6.0
   Compiling trust-dns-proto v0.20.3
   Compiling asynchronous-codec v0.5.0
   Compiling wasm-bindgen-macro-support v0.2.74
   Compiling substrate-bip39 v0.4.2
   Compiling async-process v1.1.0
   Compiling async-global-executor v2.0.2
   Compiling if-watch v0.2.2
   Compiling ed25519-dalek v1.0.1
   Compiling impl-serde v0.3.1
   Compiling tracing-serde v0.1.2
   Compiling cranelift-entity v0.74.0
   Compiling cranelift-codegen-shared v0.74.0
   Compiling regalloc v0.0.31
   Compiling toml v0.5.8
   Compiling bincode v1.3.3
   Compiling semver v0.11.0
   Compiling cargo-platform v0.1.1
   Compiling tokio v0.2.25
   Compiling parity-ws v0.10.1
   Compiling tokio-uds v0.2.7
   Compiling tokio-tcp v0.1.4
   Compiling tokio-udp v0.1.6
   Compiling futures v0.3.16
   Compiling unsigned-varint v0.7.0
   Compiling unsigned-varint v0.6.0
   Compiling async-std v1.9.0
   Compiling wasm-bindgen-macro v0.2.74
   Compiling tracing-subscriber v0.2.19
   Compiling sp-serializer v3.0.0 (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling jsonrpc-core v15.1.0
   Compiling handlebars v3.5.5
   Compiling gimli v0.24.0
   Compiling object v0.24.0
   Compiling h2 v0.1.26
   Compiling cranelift-bforest v0.74.0
   Compiling trust-dns-resolver v0.20.3
   Compiling petgraph v0.5.1
   Compiling cranelift-codegen-meta v0.74.0
   Compiling wasm-timer v0.2.5
   Compiling rw-stream-sink v0.2.1
   Compiling sp-utils v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling yamux v0.9.0
   Compiling soketto v0.4.2
   Compiling libp2p-pnet v0.20.0
   Compiling intervalier v0.4.0
   Compiling exit-future v0.2.0
   Compiling tokio v0.1.22
   Compiling proc-macro-crate v1.0.0
   Compiling proc-macro-crate v0.1.5
   Compiling multistream-select v0.10.2
   Compiling cargo_metadata v0.13.1
   Compiling jsonrpc-pubsub v15.1.0
   Compiling tokio-util v0.3.1
   Compiling sct v0.6.1
   Compiling webpki v0.21.4
   Compiling addr2line v0.15.2
   Compiling async-std-resolver v0.20.3
   Compiling jsonrpc-server-utils v15.1.0
   Compiling tokio-named-pipes v0.1.0
   Compiling frame-support-procedural-tools v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling jsonrpc-derive v15.1.0
   Compiling parity-scale-codec-derive v2.2.0
   Compiling sp-runtime-interface-proc-macro v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sp-api-proc-macro v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling multihash-derive v0.7.2
   Compiling scale-info-derive v0.7.0
   Compiling sc-chain-spec-derive v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-tracing-proc-macro v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling jsonrpc-client-transports v15.1.0
   Compiling statrs v0.15.0
   Compiling h2 v0.2.7
   Compiling ct-logs v0.7.0
   Compiling js-sys v0.3.51
   Compiling rustls v0.19.1
   Compiling webpki-roots v0.21.1
   Compiling rustls v0.18.1
   Compiling parity-tokio-ipc v0.4.0
   Compiling jsonrpc-ws-server v15.1.0
   Compiling libp2p-core v0.28.3
   Compiling libp2p-gossipsub v0.30.1
   Compiling libp2p-plaintext v0.28.0
   Compiling libp2p-floodsub v0.29.0
   Compiling libp2p-identify v0.29.0
   Compiling libp2p-relay v0.2.0
   Compiling libp2p-kad v0.30.0
   Compiling libp2p-noise v0.30.0
   Compiling sc-network v0.10.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling frame-support-procedural v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling multihash v0.13.2
   Compiling jsonrpc-core-client v15.1.0
   Compiling jsonrpc-http-server v15.1.0
   Compiling parity-scale-codec v2.2.0
   Compiling linregress v0.4.3
   Compiling jsonrpc-ipc-server v15.1.0
   Compiling futures-rustls v0.21.1
   Compiling cranelift-codegen v0.74.0
   Compiling tokio-rustls v0.14.1
   Compiling rustls-native-certs v0.4.0
   Compiling wasm-bindgen-futures v0.4.24
   Compiling parity-multiaddr v0.11.2
   Compiling cid v0.6.1
   Compiling hyper v0.13.10
   Compiling impl-codec v0.5.1
   Compiling sp-storage v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sp-wasm-interface v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sp-tracing v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sp-arithmetic v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling fork-tree v3.0.0 (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling scale-info v0.10.0
   Compiling sp-version-proc-macro v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling primitive-types v0.10.1
   Compiling sp-externalities v0.10.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling finality-grandpa v0.14.3
   Compiling sp-runtime-interface v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling libp2p-swarm v0.29.0
   Compiling libp2p-dns v0.28.1
   Compiling libp2p-websocket v0.29.0
   Compiling libp2p-uds v0.28.0
   Compiling libp2p-tcp v0.28.0
   Compiling libp2p-yamux v0.32.0
   Compiling libp2p-mplex v0.28.0
   Compiling libp2p-deflate v0.28.0
   Compiling libp2p-wasm-ext v0.28.2
   Compiling memory-db v0.27.0
   Compiling kvdb v0.10.0
   Compiling sp-core v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling libp2p-mdns v0.30.2
   Compiling libp2p-ping v0.29.0
   Compiling libp2p-request-response v0.11.0
   Compiling substrate-prometheus-endpoint v0.9.0 (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling hyper-rustls v0.21.0
   Compiling sp-database v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling kvdb-memorydb v0.10.0
   Compiling sc-proposer-metrics v0.9.0 (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sp-trie v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sp-keystore v0.10.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-allocator v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling frame-metadata v14.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sp-rpc v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling libp2p v0.37.1
   Compiling sp-state-machine v0.10.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-telemetry v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-peerset v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling zstd v0.6.1+zstd.1.4.9
   Compiling sp-maybe-compressed-blob v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling substrate-wasm-builder v5.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling node-template-runtime v3.0.0-monthly-2021-08 (/Users/zacksleo/projects/github/zacksleo/substrate-ocw/assignment/runtime)
   Compiling cranelift-frontend v0.74.0
   Compiling cranelift-native v0.74.0
   Compiling cranelift-wasm v0.74.0
   Compiling wasmtime-environ v0.27.0
   Compiling wasmtime-debug v0.27.0
   Compiling wasmtime-cranelift v0.27.0
   Compiling wasmtime-obj v0.27.0
   Compiling wasmtime-profiling v0.27.0
   Compiling wasmtime-jit v0.27.0
   Compiling sp-io v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-executor-common v0.10.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling wasmtime v0.27.0
   Compiling sc-executor-wasmi v0.10.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sp-application-crypto v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sp-tasks v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-executor-wasmtime v0.10.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sp-runtime v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-keystore v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sp-version v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sp-inherents v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sp-staking v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sp-consensus-slots v0.10.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sp-consensus-vrf v0.10.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-rpc-server v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sp-keyring v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sp-api v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-executor v0.10.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sp-finality-grandpa v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sp-session v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sp-offchain v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sp-transaction-pool v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling frame-system-rpc-runtime-api v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sp-consensus v0.10.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling frame-support v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sp-block-builder v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sp-timestamp v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sp-authorship v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sp-transaction-storage-proof v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sp-blockchain v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sp-consensus-babe v0.10.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sp-consensus-aura v0.10.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-transaction-pool-api v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-client-api v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-consensus v0.10.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-block-builder v0.10.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-consensus-uncles v0.10.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-state-db v0.10.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-tracing v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-light v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-transaction-pool v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-basic-authorship v0.10.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-consensus-epochs v0.10.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-consensus-babe v0.10.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-consensus-aura v0.10.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling frame-system v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-network-gossip v0.10.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-informant v0.10.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-offchain v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-finality-grandpa v0.10.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling frame-benchmarking v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling pallet-transaction-payment v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling pallet-authorship v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling frame-executive v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling pallet-sudo v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling pallet-ocw v3.1.0 (/Users/zacksleo/projects/github/zacksleo/substrate-ocw/assignment/pallets/ocw)
   Compiling pallet-randomness-collective-flip v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling pallet-timestamp v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling pallet-template v3.0.0-monthly-2021-08 (/Users/zacksleo/projects/github/zacksleo/substrate-ocw/assignment/pallets/template)
   Compiling pallet-balances v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling pallet-transaction-payment-rpc-runtime-api v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling pallet-session v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling pallet-transaction-payment-rpc v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-chain-spec v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-rpc-api v0.10.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-rpc v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling substrate-frame-rpc-system v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling pallet-aura v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling pallet-grandpa v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling rocksdb v0.17.0
   Compiling kvdb-rocksdb v0.12.1
   Compiling sc-client-db v0.10.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-service v0.10.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling sc-cli v0.10.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
   Compiling frame-benchmarking-cli v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=monthly-2021-08#4d28ebeb)
    Finished release [optimized] target(s) in 11m 02s
```

## 运行日志

```bash
➜  assignment git:(master) ✗ ./target/release/ocw-example --tmp --dev
2021-09-19 19:56:33 Running in --dev mode, RPC CORS has been disabled.
2021-09-19 19:56:33 Substrate Node
2021-09-19 19:56:33 ✌️  version 3.0.0-monthly-2021-08-c146ce5-x86_64-macos
2021-09-19 19:56:33 ❤️  by Substrate DevHub <https://github.com/substrate-developer-hub>, 2017-2021
2021-09-19 19:56:33 📋 Chain specification: Development
2021-09-19 19:56:33 🏷 Node name: cruel-match-8001
2021-09-19 19:56:33 👤 Role: AUTHORITY
2021-09-19 19:56:33 💾 Database: RocksDb at /var/folders/t4/bd7qj94x28qf3ldn7zkxpwnm0000gn/T/substratebgMvgT/chains/dev/db
2021-09-19 19:56:33 ⛓  Native runtime: node-template-100 (node-template-1.tx1.au1)
2021-09-19 19:56:34 🔨 Initializing Genesis block/state (state: 0xdb8d…cd6d, header-hash: 0x4e4e…9b10)
2021-09-19 19:56:34 👴 Loading GRANDPA authority set from genesis on what appears to be first startup.
2021-09-19 19:56:34 ⏱  Loaded block-time = 6s from block 0x4e4eaf4e2119a39faa165c3b2c91500a17c2e5382bee983c44b03a8049d59b10
2021-09-19 19:56:34 Using default protocol ID "sup" because none is configured in the chain specs
2021-09-19 19:56:34 🏷 Local node identity is: 12D3KooWFqEZvEwU5R6k4yrBxRWSYQ9PCCKVR6vsjumzw6vuth8N
2021-09-19 19:56:34 📦 Highest known block at #0
2021-09-19 19:56:34 〽️ Prometheus exporter started at 127.0.0.1:9615
2021-09-19 19:56:34 Listening for new connections on 127.0.0.1:9944.
2021-09-19 19:56:36 🙌 Starting consensus session on top of parent 0x4e4eaf4e2119a39faa165c3b2c91500a17c2e5382bee983c44b03a8049d59b10
2021-09-19 19:56:36 🎁 Prepared block for proposing at 1 [hash: 0xb3db079733c5d440c4e8399fb5d40bb1077755523f309a40a0e76903f0e0111f; parent_hash: 0x4e4e…9b10; extrinsics (1): [0x4f1e…3ebc]]
2021-09-19 19:56:36 🔖 Pre-sealed block for proposal at 1. Hash now 0x46b1d36c592a219ace88467852fa28fc73876f946f8a07f56dea8e2085aa420b, previously 0xb3db079733c5d440c4e8399fb5d40bb1077755523f309a40a0e76903f0e0111f.
2021-09-19 19:56:36 ✨ Imported #1 (0x46b1…420b)
2021-09-19 19:56:36 Hello World from offchain workers!
2021-09-19 19:56:39 💤 Idle (0 peers), best: #1 (0x46b1…420b), finalized #0 (0x4e4e…9b10), ⬇ 0 ⬆ 0
2021-09-19 19:56:42 🙌 Starting consensus session on top of parent 0x46b1d36c592a219ace88467852fa28fc73876f946f8a07f56dea8e2085aa420b
2021-09-19 19:56:42 submit_number_unsigned: 1
2021-09-19 19:56:42 Number vector: [1]
2021-09-19 19:56:42 🎁 Prepared block for proposing at 2 [hash: 0xe1879def5e48f327ab9f0aa475a83e91adbc235b553c19130593892c3e48a275; parent_hash: 0x46b1…420b; extrinsics (2): [0xd2af…678b, 0xad43…44a0]]
2021-09-19 19:56:42 🔖 Pre-sealed block for proposal at 2. Hash now 0xc64bc3e587349e5516b02046105c7605f1d084371ba10ff22132065603af31dd, previously 0xe1879def5e48f327ab9f0aa475a83e91adbc235b553c19130593892c3e48a275.
2021-09-19 19:56:42 ✨ Imported #2 (0xc64b…31dd)
2021-09-19 19:56:42 Hello World from offchain workers!
2021-09-19 19:56:44 💤 Idle (0 peers), best: #2 (0xc64b…31dd), finalized #0 (0x4e4e…9b10), ⬇ 0 ⬆ 0
2021-09-19 19:56:48 🙌 Starting consensus session on top of parent 0xc64bc3e587349e5516b02046105c7605f1d084371ba10ff22132065603af31dd
2021-09-19 19:56:48 submit_number_unsigned_with_signed_payload: (2, MultiSigner::Sr25519(d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d (5GrwvaEF...)))
2021-09-19 19:56:48 Number vector: [1, 2]
2021-09-19 19:56:48 🎁 Prepared block for proposing at 3 [hash: 0x9513392df2467e8aacf90ac81396514058d202d4ada491cbf2ce30c3569cb640; parent_hash: 0xc64b…31dd; extrinsics (2): [0x84c7…57b4, 0x5241…c013]]
2021-09-19 19:56:48 🔖 Pre-sealed block for proposal at 3. Hash now 0xa6d88b47c38a3e17ca320a99136cbce63ebfc9fc15d2696034ac0812ffd9648a, previously 0x9513392df2467e8aacf90ac81396514058d202d4ada491cbf2ce30c3569cb640.
2021-09-19 19:56:48 ✨ Imported #3 (0xa6d8…648a)
2021-09-19 19:56:48 Hello World from offchain workers!
2021-09-19 19:56:48 sending request to: https://api.github.com/orgs/substrate-developer-hub
2021-09-19 19:56:48 response: {"login":"substrate-developer-hub","id":47530779,"node_id":"MDEyOk9yZ2FuaXphdGlvbjQ3NTMwNzc5","url":"https://api.github.com/orgs/substrate-developer-hub","repos_url":"https://api.github.com/orgs/substrate-developer-hub/repos","events_url":"https://api.github.com/orgs/substrate-developer-hub/events","hooks_url":"https://api.github.com/orgs/substrate-developer-hub/hooks","issues_url":"https://api.github.com/orgs/substrate-developer-hub/issues","members_url":"https://api.github.com/orgs/substrate-developer-hub/members{/member}","public_members_url":"https://api.github.com/orgs/substrate-developer-hub/public_members{/member}","avatar_url":"https://avatars.githubusercontent.com/u/47530779?v=4","description":"Documentation, samples, and tutorials for the Substrate framework for building blockchains.","name":"Substrate Developer Hub","company":null,"blog":"https://substrate.dev/","location":null,"email":null,"twitter_username":"substrate_io","is_verified":false,"has_organization_projects":true,"has_repository_projects":true,"public_repos":34,"public_gists":0,"followers":0,"following":0,"html_url":"https://github.com/substrate-developer-hub","created_at":"2019-02-11T14:59:31Z","updated_at":"2020-10-03T13:48:59Z","type":"Organization"}
2021-09-19 19:56:49 💤 Idle (0 peers), best: #3 (0xa6d8…648a), finalized #1 (0x46b1…420b), ⬇ 0 ⬆ 0
2021-09-19 19:56:54 🙌 Starting consensus session on top of parent 0xa6d88b47c38a3e17ca320a99136cbce63ebfc9fc15d2696034ac0812ffd9648a
2021-09-19 19:56:54 🎁 Prepared block for proposing at 4 [hash: 0x086d1d401e5e9ab1c591439247b815e1a3e5391a9f0bd845568d1c650088bb39; parent_hash: 0xa6d8…648a; extrinsics (1): [0x3f02…4ad2]]
2021-09-19 19:56:54 🔖 Pre-sealed block for proposal at 4. Hash now 0x3c8cd759a99c2af94743c1d36818965fe95a6aa4ad269d67b04e174669baca3c, previously 0x086d1d401e5e9ab1c591439247b815e1a3e5391a9f0bd845568d1c650088bb39.
2021-09-19 19:56:54 ✨ Imported #4 (0x3c8c…ca3c)
2021-09-19 19:56:54 Hello World from offchain workers!
2021-09-19 19:56:54 sending request to: https://api.coincap.io/v2/assets/polkadot
2021-09-19 19:56:54 💤 Idle (0 peers), best: #4 (0x3c8c…ca3c), finalized #1 (0x46b1…420b), ⬇ 0 ⬆ 0
2021-09-19 19:56:55 Unexpected http request status code: 429
2021-09-19 19:56:55 fetch_from_remote error: HttpFetchingError
2021-09-19 19:56:55 offchain_worker error: HttpFetchingError
2021-09-19 19:56:59 💤 Idle (0 peers), best: #4 (0x3c8c…ca3c), finalized #2 (0xc64b…31dd), ⬇ 0 ⬆ 0
2021-09-19 19:57:00 🙌 Starting consensus session on top of parent 0x3c8cd759a99c2af94743c1d36818965fe95a6aa4ad269d67b04e174669baca3c
2021-09-19 19:57:00 🎁 Prepared block for proposing at 5 [hash: 0x95204df7a12df763a77ac866217d3756b91c22d12dd324033afce6c425ec3868; parent_hash: 0x3c8c…ca3c; extrinsics (1): [0x71fc…180d]]
2021-09-19 19:57:00 🔖 Pre-sealed block for proposal at 5. Hash now 0xa31b4c1a17dff5382a4e43b8d77b52621da5271592c7d64b96ef0ef3d636acd7, previously 0x95204df7a12df763a77ac866217d3756b91c22d12dd324033afce6c425ec3868.
2021-09-19 19:57:00 ✨ Imported #5 (0xa31b…acd7)
2021-09-19 19:57:00 Hello World from offchain workers!
2021-09-19 19:57:04 💤 Idle (0 peers), best: #5 (0xa31b…acd7), finalized #3 (0xa6d8…648a), ⬇ 0 ⬆ 0
2021-09-19 19:57:06 🙌 Starting consensus session on top of parent 0xa31b4c1a17dff5382a4e43b8d77b52621da5271592c7d64b96ef0ef3d636acd7
2021-09-19 19:57:06 submit_number_signed: (5, d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d (5GrwvaEF...))
2021-09-19 19:57:06 Number vector: [1, 2, 5]
2021-09-19 19:57:06 🎁 Prepared block for proposing at 6 [hash: 0xfee3295b7359aafd8d9108e8967ae0eb88b854160e328867151a9fd1443232f7; parent_hash: 0xa31b…acd7; extrinsics (2): [0x9ccc…192a, 0xace1…afa5]]
2021-09-19 19:57:06 🔖 Pre-sealed block for proposal at 6. Hash now 0x9538d13a4d0ffe2bd6b4fdd73b969ece1cce93e3766e1795087082f8706c0f41, previously 0xfee3295b7359aafd8d9108e8967ae0eb88b854160e328867151a9fd1443232f7.
2021-09-19 19:57:06 ✨ Imported #6 (0x9538…0f41)
2021-09-19 19:57:06 Hello World from offchain workers!
2021-09-19 19:57:09 💤 Idle (0 peers), best: #6 (0x9538…0f41), finalized #4 (0x3c8c…ca3c), ⬇ 0 ⬆ 0
2021-09-19 19:57:12 🙌 Starting consensus session on top of parent 0x9538d13a4d0ffe2bd6b4fdd73b969ece1cce93e3766e1795087082f8706c0f41
2021-09-19 19:57:12 submit_number_unsigned: 6
2021-09-19 19:57:12 Number vector: [1, 2, 5, 6]
2021-09-19 19:57:12 🎁 Prepared block for proposing at 7 [hash: 0xc8fa90b9d9a3365112fc0029a1de5926f61a03c08472173fdc5d1a68ca89c668; parent_hash: 0x9538…0f41; extrinsics (2): [0x519c…f54b, 0x64ea…7be6]]
2021-09-19 19:57:12 🔖 Pre-sealed block for proposal at 7. Hash now 0x96c68f859042f320e3b1d1c961058a0065b162813417cad2bfc45d074b4b5397, previously 0xc8fa90b9d9a3365112fc0029a1de5926f61a03c08472173fdc5d1a68ca89c668.
2021-09-19 19:57:12 ✨ Imported #7 (0x96c6…5397)
2021-09-19 19:57:12 Hello World from offchain workers!
2021-09-19 19:57:18 cached gh-info: { login: substrate-developer-hub, blog: https://substrate.dev/, public_repos: 34 }
2021-09-19 19:57:19 💤 Idle (0 peers), best: #8 (0xdf9a…448d), finalized #6 (0x9538…0f41), ⬇ 0 ⬆ 0
2021-09-19 19:57:24 🙌 Starting consensus session on top of parent 0xdf9a594f1b4b03ab9a269c14855ca2f899b8cb56ea896ca601acf0deddd7448d
2021-09-19 19:57:24 🎁 Prepared block for proposing at 9 [hash: 0x0eeb350551527f649dd1097f340eabbbda809f03a702478bc55b59815081935b; parent_hash: 0xdf9a…448d; extrinsics (1): [0x5565…c5a5]]
2021-09-19 19:57:24 🔖 Pre-sealed block for proposal at 9. Hash now 0x599f673147e6ed58100cd0a78bf83b0121b47b3b958da7a7955526225c50cc16, previously 0x0eeb350551527f649dd1097f340eabbbda809f03a702478bc55b59815081935b.
2021-09-19 19:57:24 ✨ Imported #9 (0x599f…cc16)
2021-09-19 19:57:24 Hello World from offchain workers!
2021-09-19 19:57:24 sending request to: https://api.coincap.io/v2/assets/polkadot
2021-09-19 19:57:24 response: {"data":{"id":"polkadot","rank":"8","symbol":"DOT","name":"Polkadot","supply":"1030866753.5754000000000000","maxSupply":null,"marketCapUsd":"34350009272.8500610313149153","volumeUsd24Hr":"519731362.4194616042560985","priceUsd":"33.3214832602879371","changePercent24Hr":"-4.4379821572414742","vwap24Hr":"34.7046567573065752","explorer":"https://polkascan.io/polkadot"},"timestamp":1632052644560}
2021-09-19 19:57:24 💤 Idle (0 peers), best: #9 (0x599f…cc16), finalized #6 (0x9538…0f41), ⬇ 0 ⬆ 0
2021-09-19 19:57:29 💤 Idle (0 peers), best: #9 (0x599f…cc16), finalized #7 (0x96c6…5397), ⬇ 0 ⬆ 0
2021-09-19 19:57:30 🙌 Starting consensus session on top of parent 0x599f673147e6ed58100cd0a78bf83b0121b47b3b958da7a7955526225c50cc16
2021-09-19 19:57:30 submit_price_unsigned_with_signed_payload: ((
    33,
    Permill(
        321483,
    ),
), MultiSigner::Sr25519(
    d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d (5GrwvaEF...),
))
2021-09-19 19:57:30 Number vector: [(33, Permill(321483))]
2021-09-19 19:57:30 🎁 Prepared block for proposing at 10 [hash: 0x306f4b31121d8a01febb93f7f005ee140525dc0df7820fe3fd223c8ad4b7188a; parent_hash: 0x599f…cc16; extrinsics (2): [0x4c92…e9db, 0xb27f…36d5]]
2021-09-19 19:57:30 🔖 Pre-sealed block for proposal at 10. Hash now 0xf3c2fba34780de6ec515112c63c9041fbd1e380137250202047451cccf02df5d, previously 0x306f4b31121d8a01febb93f7f005ee140525dc0df7820fe3fd223c8ad4b7188a.
2021-09-19 19:57:30 ✨ Imported #10 (0xf3c2…df5d)

```