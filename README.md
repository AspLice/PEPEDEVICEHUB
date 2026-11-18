# Device HUB

Windows 11 向けの軽量な周辺機器管理アプリ。Rust / egui を予定し、Electron は使用しません。

現在は **Phase 0 の初期構築**です。物理デバイス対応、GUI、常駐、トレイ、IPC は未実装です。
Mock の数値はシミュレーションであり、SUPERLIGHT / TAC75 HE の実機対応を示しません。

## 開発

Rust 1.91.1（rust-toolchain.toml）とネイティブリンカーを用意します。

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build --workspace --release --locked
cargo run --locked -p device-hub-agent -- --mock --once
```

最後のコマンドは Mock の状態を一度表示して終了します。ハードウェアにはアクセスしません。
Windows では MSVC Rust toolchain と Visual Studio Build Tools の C++ 開発ツールが必要です。
Linux 検証は Windows 実機検証の代用にはなりません。

## 資料

- [正式仕様書](docs/SPECIFICATION.md)
- [アーキテクチャ](docs/ARCHITECTURE.md)
- [開発手順](docs/DEVELOPMENT.md)
- [ロードマップ・Issue 計画](docs/ROADMAP.md)
- [安全性](docs/SECURITY.md)
- [実機検証](docs/HARDWARE_TESTING.md)

公開ライセンスは所有者未決定です。LICENSE は設定していません。

Windows の予備調査: [読み取り専用 PnP 調査ツール](docs/DEVICE_INVENTORY.md)。実機結果待ちであり、対応機種はまだ未確認です。

HID++ のオフライン通信基盤と未確認事項: [調査記録](protocols/logitech/HIDPP20.md)。実機バッテリー取得は未実装です。

実行ファイルによる詳細調査: [Windows HID metadata scanner](docs/NATIVE_HID_INVENTORY.md)。
