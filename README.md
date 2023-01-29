# erobeam

## ビルド

```console
$ sudo apt update -y
$ sudo apt install libopus-dev ffmpeg
$ cargo build --release
$ sudo mv target/release/erobeam /usr/local/bin
```

libopus-devはビルドに必要で、ffmpegは実行に必要です。

## 実行

[Discord Developer Portal](https://discord.com/developers/applications)で予めbotを作成します。

取得したbotのtokenはerobeam.tomlの`bot.token`に置いてください。

```console
$ RUST_LOG=info EROBEAM_CONFIG_PATH=/etc/erobeam.toml erobeam
```

## botの使い方

botは以下のリンクで招待できます。

`https://discord.com/oauth2/authorize?client_id=<CLIENT_ID>&permissions=<PERMISSION>&scope=bot`

CLIENT_IDは作成したアプリのIDです。

PERMISSIONは`2184251392`とかで良いと思います。（最低限チャットが見れて、VCに参加して発言が出来たら良いはず）

1. VCに参加して、`>>join`とチャットに書き込むとVCにerobeamが参加します。
2. `>>play https://chobit.cc/...`とでもして、chobitの音声作品リンクを書き込むとそれらをキューに放り込みます。
3. `>>leave`を書き込むとチャットから抜けます

`>>`はコマンドのプレフィックスで、erobeam.tomlの`bot.prefix`から変更できます。

- join ... 実行者のいるVCに参加します
- leave ... 実行者のいるサーバーのVCから抜けます
- play ... リンクを指定して再生する
- stop ... 再生を止める
- pause ... 再生を一時停止する
- resume ... 再生を再開する
- skip ... 次の音源にスキップする
- np ... 再生中の音源が何か返します
- queue ... キューに入った音源を返します
- ping ... pong!を返します
- help ... ヘルプが出ますが特に情報がないです
- seek ... 秒数を指定してシークしますが今のところ機能しません
