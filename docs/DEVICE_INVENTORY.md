# Windows 読み取り専用 PnP 調査

通常権限の Windows 11 PowerShell で、リポジトリをダウンロード・展開して実行する。

```powershell
powershell.exe -NoProfile -File .\scripts\Get-DeviceInventory.ps1
```

実行ポリシーで拒否された場合は、信頼できる checkout の scripts 内の3ファイルを確認し、
ファイルのプロパティの「許可する」または `Unblock-File` でそのファイルだけブロック解除する。
管理者起動やマシン全体の実行ポリシー変更は不要。

結果の JSON をこのチャットに貼り付ける。自動アップロードやファイル保存は行わない。
所有者が対応機器を見分けるため、SUPERLIGHT の接続方法（有線/レシーバー）と
TAC75 HE が純正 V4 / Web ドライバーのどちらを使用するかも、分かる範囲で添える。
シリアル番号そのものは共有しない。

## 出力範囲と限界

Windows PnP の present デバイスの USB/HID InstanceId から VID/PID・MI・REV の公開トークンのみ抽出。
パス、シリアル、任意の FriendlyName は返さない。HID を開かず、レポート送受信・入力収集・設定変更をしない。
同じ公開識別子を集約するため、複数の同型機を区別できず、物理デバイス数にもならない。
VID 046D はメーカー候補のみ。SUPERLIGHT の世代やレシーバー配下の論理 ID は確定しない。
USB PnP は無線レシーバーも含むため、マウスの有線/無線を断定しない。
REV は PnP トークンにすぎず、ファームウェア版・ハードウェアリビジョンとして扱わない。
Usage Page / Usage / firmware / hardware_revision / product_name は未取得を表す null。
この予備調査だけでは仕様書第5章の完全な HID 調査や Phase 1 は完了しない。

## テスト

Windows で `powershell.exe -NoProfile -File .\scripts\Test-DeviceInventory.ps1`。
匿名の fixture による境界・秘匿・集約テストと CI ホストの PnP 列挙を検証する。
実機モデルの確認はユーザーの Windows PC でのみ可能。
