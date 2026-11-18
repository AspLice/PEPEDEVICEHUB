# 開発環境

README のコマンドをリポジトリルートで実行する。Rust は rust-toolchain.toml で固定。
クラウドでは既存 checkout を利用し、要求なしに worktree を作らない。
このインスタンスの Rust は /workspace/.devicehub に配置。

```sh
export CARGO_HOME=/workspace/.devicehub/cargo
export RUSTUP_HOME=/workspace/.devicehub/rustup
export PATH="$CARGO_HOME/bin:$PATH"
cd /workspace/PEPEDEVICEHUB
```

Mock snapshot は即時終了し、再起動する常駐サービスはまだない。
GitHub API 接続と Git HTTPS proxy の権限は別。Git 読取成功だけで Issue/PR 操作可能と判断しない。
API の Forbidden は現在のブロッカー。秘密値をチャットやリポジトリに書かない。
Windows で同じ cargo 検証を実行し、CI 結果も確認する。
