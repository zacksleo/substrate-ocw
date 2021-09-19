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