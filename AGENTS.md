# Device HUB 開発指示

正式仕様は docs/SPECIFICATION.md（v1.0.0）。競合時はユーザー指示と正式仕様を優先する。
Windows 11 / Rust / egui を採用予定。Electron 禁止。オフライン動作、軽量性、安全性を優先。
Agent が HID を所有し GUI はバージョン付き・現在ユーザー限定 Named Pipe で通信する設計。
Phase 0 は core / mock / CLI snapshot のみ。実装予定を実装済みと扱わない。

## 開発管理

既存 checkout を利用する。クラウドタスクは分離済みであり、明示要求なしに worktree を作らない。
開始時に git status、ブランチ、関連仕様、Issue、依存関係を確認する。
通常は develop から feature/<Issue>-<name> を作成。初期構築のみ feature/project-bootstrap。
main / develop に直接コミットしない。Conventional Commits（英語）を使う。
機能別 Issue に目的・依存・完了条件・テスト・実機要否・リスクを記録する。
feature PR は develop 向け。PR に Issue、変更、依存、検証、実機未検証、リスクを明記。
ユーザーの明示承認なくマージ、正式タグ・リリース、ブランチ削除、強制 push、履歴変更をしない。
空リポジトリの base branch 初期化や設定変更は確認する。ユーザー変更を保護する。
ライセンスはユーザー決定まで追加しない。

## 検証

cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build --workspace --release --locked
cargo run --locked -p device-hub-agent -- --mock --once

CI の Windows 結果とローカル Linux の結果を区別する。ゼロテスト・未実施を合格と呼ばない。

## ハードウェア・解析

SUPERLIGHT 初代のバッテリーを優先。レシーバー PID と論理 ID を区別し実機照合する。
TAC75 HE は有線・バッテリーなし。リビジョン確認前に通信方式を断定しない。
Feature ID だけで capability を有効化しない。未検証操作を UI に開放しない。
ランダム・未確認 HID コマンドは禁止。初回設定書き込みは仕様書に従い承認を得る。
ファームウェア更新、キャリブレーション、初期化は初期対象外。
同一機器への通信を直列化。有限タイムアウトと有限リトライを設計する。
生キャプチャ、シリアル、HID パス、個人情報、入力、認証情報をコミットしない。
既存コード参照時は出典とライセンスを調べ、未確認事項を確定情報にしない。

## 性能と報告

目標: Agent WS <=10MB、Private Bytes <=20MB、GUI合計 <=100MB、idle CPU <=0.1%。
Windows 実測前に達成を主張しない。常時描画・高頻度 polling を避ける。
完了報告は Issue、branch、base、commit、PR、変更、fmt/clippy/test/build/CI/実機、
性能、未完了、次の作業、必要承認を記載する。詳細は仕様書第35章。
