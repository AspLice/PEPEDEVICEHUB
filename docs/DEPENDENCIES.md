# Dependencies and license review

公開ライセンスは所有者未決定。依存のライセンスとは別に管理する。
Windows metadata scanner の外部依存:

- hidapi 2.6.7: MIT (https://crates.io/crates/hidapi/2.6.7), windows-native backend を限定使用。
- serde_json 1.x: MIT OR Apache-2.0 (https://crates.io/crates/serde_json)。数値と固定文字列の JSON 出力のみ。

正確な推移依存と checksum は Cargo.lock に記録。Windows CI で本物の backend をビルド・列挙検証する。
外部ライセンスの本文は docs/licenses に保持し、検証用 artifact にも同梱する。
脆弱性の継続監査は security workflow で Cargo.lock を対象に実行する。

## Lockfile registry licenses

| Crate | Version | Declared license |
|---|---|---|
| cc | 1.11.2 | MIT OR Apache-2.0 |
| cfg-if | 1.0.5 | MIT OR Apache-2.0 |
| find-msvc-tools | 0.1.15 | MIT OR Apache-2.0 |
| hidapi | 2.6.7 | MIT |
| itoa | 1.0.19 | MIT OR Apache-2.0 |
| libc | 0.2.191 | MIT OR Apache-2.0 |
| memchr | 2.8.3 | Unlicense OR MIT |
| pkg-config | 0.3.34 | MIT OR Apache-2.0 |
| proc-macro2 | 1.0.107 | MIT OR Apache-2.0 |
| quote | 1.0.47 | MIT OR Apache-2.0 |
| serde | 1.0.229 | MIT OR Apache-2.0 |
| serde_core | 1.0.229 | MIT OR Apache-2.0 |
| serde_derive | 1.0.229 | MIT OR Apache-2.0 |
| serde_json | 1.0.154 | MIT OR Apache-2.0 |
| shlex | 2.0.1 | MIT OR Apache-2.0 |
| syn | 3.0.10 | MIT OR Apache-2.0 |
| unicode-ident | 1.0.27 | (MIT OR Apache-2.0) AND Unicode-3.0 |
| windows-link | 0.2.1 | MIT OR Apache-2.0 |
| windows-sys | 0.61.2 | MIT OR Apache-2.0 |
| zmij | 1.0.23 | MIT |
