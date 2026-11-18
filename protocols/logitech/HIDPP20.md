# HID++ 2.0 調査・オフライン codec

## 確認情報と範囲

ユーザー申告: 初代 SUPERLIGHT、レシーバー経由。PnP: 046D:C547（レシーバー）。
マウス自身の論理 device index、Feature 一覧、firmware、Usage/interface は未取得。
他のレシーバーや SUPERLIGHT 2 に一般化しない。

調査源: Solaar commit e7304c4c451cc9bb4f206a914844525e67856a28。
- https://github.com/pwr-Solaar/Solaar/blob/e7304c4c451cc9bb4f206a914844525e67856a28/lib/logitech_receiver/base.py
- https://github.com/pwr-Solaar/Solaar/blob/e7304c4c451cc9bb4f206a914844525e67856a28/lib/logitech_receiver/hidpp20.py
- https://github.com/pwr-Solaar/Solaar/blob/e7304c4c451cc9bb4f206a914844525e67856a28/lib/logitech_receiver/base_usb.py

参考ファイルのヘッダーは GPL-2.0-or-later。既存コードを転載・翻訳・依存として追加せず、
ワイヤ形式の確認事実を基に小さな独自 codec と合成テストを作成した。
本プロジェクトの公開ライセンスは引き続き所有者未決定。

## ワイヤ形式

| byte | 意味 |
|---|---|
| 0 | Report ID: short 0x10 (7 bytes), long 0x11 (20 bytes) |
| 1 | device index（実機で確認し、レシーバー PID と区別） |
| 2 | Feature index（Feature ID ではない） |
| 3 | 上位4bit: function、下位4bit: software ID |
| 4.. | parameters、short は3 bytes、long は16 bytes |

Root.GetFeature: index=0、function=0、parameters は Feature ID の big-endian 2 bytes。
応答 parameters[0] の index=0 は未対応として扱う。
HID++2 error: byte2=0xff、byte3=要求feature、byte4=要求function/software、byte5=error。
旧 HID++ error の byte2=0x8f も区別し、通常応答と混同しない。
software ID=0 の通知は応答と分離。device/feature/function/software の一致を検証する。
Bluetooth の index 反転や Centurion 拡張は対象外で、曖昧な一致は受け付けない。

## バッテリー取得の今後の手順（未実装・未検証）

1. HID interface とレシーバー配下のマウス index を読み取り専用で確定。
2. Root.GetFeature で 0x1004 Unified Battery と 0x1000 Battery Status を動的に照合。
3. 参考実装では 0x1004 の get は function 1、0x1000 は function 0。
4. 実機で公開される Feature と応答を検証し、残量・充電・不明・概算を区別する。

既存実装でも0の扱いや概算値が異なるため、応答の0を機械的に実測0%と扱わない。
この段階で残量 decoder や対応済み Capability を追加しない。
設定変更用 Feature、ペアリング操作、通知有効化、firmware 更新は実装しない。

## 実装の境界

device-hidpp は transport なし。Root.GetFeature の合成パケット作成、short/long parser、
応答・通知・エラー照合、absolute monotonic timeout と有限フレーム予算のみ。
既存 Agent へ接続しておらず、実機を開かない。ソフトウェアID・device index は呼び出し側で管理する。
呼び出し側は read 自体を remaining 以下に制限する必要があり、この codec だけでは OS I/O を中断できない。
応答完了後は PendingDiscovery を破棄する。自動リトライなし。

合成パケットは個人情報を含まない。実機試験・通信キャプチャ・パフォーマンス測定は未実施。
#2 は完全な HID metadata 待ち、#3/#5 も本変更だけで完了しない。
