# アーキテクチャ

## Phase 0 実装

- device-core: バッテリー値の検証、接続・充電状態、Capability、読み取り専用 Adapter trait。
- device-mock: 明示的な架空デバイス。切断、スリープ、再接続、不明残量を再現。
- device-hub-agent: --mock --once に限定した snapshot CLI。指定なしの実機動作は拒否。

外部クレート依存なし。公開ライセンス未決定。Phase 0 に不要な GUI/HID 依存はまだ加えない。
Adapter の時刻は観測時刻であり、スリープ時の値には stale を付ける。
書き込み API はまだ設けず、安全性契約と実機検証後に別 Issue で設計する。

## 将来の境界（未実装）

Agent: HID 所有、検出、イベント、トレイ、設定、通知。
UI: egui/eframe、Capability で操作を制限、終了時にプロセス終了。
IPC: Windows Named Pipe、現在ユーザー ACL、version、入力検証、timeout。
Adapter: Logitech HID++ の Feature 動的解決、Akko は実機リビジョン別。
現在は Agent の排他起動、常駐、Named Pipe、実機プロトコルはいずれも存在しない。

## Phase 1 のオフライン通信基盤

device-hidpp: HID++2 discovery codec と有限待機の状態処理。物理 I/O は含まない。
詳細: [HID++調査](../protocols/logitech/HIDPP20.md)。
