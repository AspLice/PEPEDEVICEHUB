# Windows HID metadata scanner

GitHub Actions の成功した実行ページの Artifacts から device-hub-scan-windows をダウンロードして展開する。
Run-HidInventory.cmd をダブルクリックし、生成された hid-inventory.json をチャットに送る。
Rust、PowerShell のポリシー変更、管理者権限は不要。マウスのレシーバーとキーボードを接続する。
ZIP 内の device-hub-scan.exe は署名なしの開発検証用で、正式リリースではない。
実行がブロックされた場合は警告内容を確認して知らせる。端末の保護設定全体は無効にしない。

## 取得するもの

VID/PID、Usage Page、Usage、interface_number、release_number_raw の6項目のみ。
対象は 046D:C547 と 3151:502D に限定。パス、シリアル、任意の製品文字列は出力しない。
release_number_raw はデバイス属性の生値で、firmware のバージョンや実リビジョンとは断定しない。
同じ公開 metadata は集約するので物理台数や個体識別には使えない。
対象が見つからない場合も JSON の devices は空配列となる。これはバッテリー取得成功ではない。

hidapi の windows-native backend が OS の HID metadata を列挙する際に内部でハンドルを開き、
デバイスの属性・descriptor・文字列を取得する。文字列はアプリの出力や保存に含めない。
アプリから HID report の read/write/get_feature_report/send_feature_report は呼ばない。
バッテリー照会、設定変更、入力収集、通知有効化、外部アップロードはしない。
生成ファイルは実行したフォルダーに置くため、個人の他のファイルを入れていない専用フォルダーで実行する。
CI の実行確認は runner の列挙テストであり、対象実機の成功は結果受領後に判定する。
