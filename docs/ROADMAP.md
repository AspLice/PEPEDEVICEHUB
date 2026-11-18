# 開発計画

以下の計画を GitHub Issue と7件の milestone に登録済み。
main は安定版、develop は統合、機能は PR で develop に統合。マージは品質条件を満たせば個別承認不要。
ユーザー承認により空の初期コミットから main / develop を作成済み。初期実装は feature/project-bootstrap に分離。

| 仮ID | Issue / milestone | 依存 | 完了条件・検証 | 実機・リスク |
|---|---|---|---|---|
| P0 | Workspace・Mock・CI / Phase 0 | base 初期化 | fmt/clippy/test/release、Windows CI、PR | 実機なし |
| P1a | 読み取り専用 scanner / Phase 1 | P0 統合 | Windows 列挙、匿名化、VID/PID/世代・revision 確定 | Windows 実機必須、識別情報の漏洩 |
| P1b | Logitech HID++ 調査 / Phase 1 | P1a | 出典・ライセンス・Feature/応答仕様文書 | 未確認送信禁止 |
| P1c | Akko protocol 調査 / Phase 1 | P1a | revision別仕様と未確認事項 | 入力キャプチャ禁止 |
| P1d | Agent・HID・IPC / Phase 1 | P0, P1a, P1b | ACL、version、timeout、切断、Mock、異常入力テスト | 排他と個人情報 |
| P2 | バッテリー・トレイ・通知 / v0.1.0 | P1d | G HUBなし、GUI終了後継続、復帰、性能測定 | SUPERLIGHT 実機必須 |
| P3 | DPI・rate・profiles / v0.2.0 | P2 | 検証済み設定、バックアップ、読み戻し | 初回書き込み承認 |
| P4 | TAC75 HE / v0.3.0 | P1c,P1d | revision別の読取・検証済み設定 | 初回書き込み承認 |
| P5 | egui GUI・共通設定 / v0.4.0 | P2,P3,P4 | 日本語、theme、Capability、保存 | Windows 統合試験 |
| P6 | 配布・最終品質 / v1.0.0 | P5 | installer、portable、hash、長時間/性能/実機試験 | 公開承認必須 |

Issue 作成時は目的、範囲、依存、完了条件、テスト、実機要否、リスクを上記から展開する。
未承認の release workflow は作らず、後続 Phase で配布方針確定後に追加する。

## GitHub Issues

- [P0: #1](https://github.com/AspLice/PEPEDEVICEHUB/issues/1)
- [P1a: #2](https://github.com/AspLice/PEPEDEVICEHUB/issues/2)
- [P1b: #3](https://github.com/AspLice/PEPEDEVICEHUB/issues/3)
- [P1c: #4](https://github.com/AspLice/PEPEDEVICEHUB/issues/4)
- [P1d: #5](https://github.com/AspLice/PEPEDEVICEHUB/issues/5)
- [P2: #6](https://github.com/AspLice/PEPEDEVICEHUB/issues/6)
- [P3: #7](https://github.com/AspLice/PEPEDEVICEHUB/issues/7)
- [P4: #8](https://github.com/AspLice/PEPEDEVICEHUB/issues/8)
- [P5: #9](https://github.com/AspLice/PEPEDEVICEHUB/issues/9)
- [P6: #10](https://github.com/AspLice/PEPEDEVICEHUB/issues/10)
