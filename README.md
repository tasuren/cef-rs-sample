# cef-rs-sample
[cef-rs](https://github.com/tauri-apps/cef-rs)を使って、何かしら作ってみる試み。
ちょっとした勉強目的なので、macOSでしか動作確認していません。

ここでやったことは一部、備忘録として[このスクラップ](https://zenn.dev/tasuren/scraps/01f47381e351d1)に記録しています。

## クレート一覧
- cef-rs-sample  
  普通にCEFを起動するだけ。Chrome Runtime Styleを使う。
- cef-rs-osr-sample  
  cef-rsに加え、winitとsoftbufferを用いてOff-Screen Renderingをする例。
- cef-rs-sample-helper
  CEFのためのヘルパー。
- cef-rs-sample-bundle-macos
  cef-rs-sampleかcef-rs-osr-sampleのmacOS向けアプリバンドルを作るプログラム。
  macOSでビルドする場合、これを実行する。

## ビルド方法
[Cargo.toml](./Cargo.toml)にあるcef-rsのバージョンがサポートするCEFを、cef-rsのリポジトリの説明を参考に`export-cef-dir`コマンドで用意してください。

私はまだmacOSでしか試していないのですが、macOSなら以下のコマンドでアプリのバンドルが`./target/debug`に作成されます。
```shell
cargo run -p cef-rs-sample-bundle-macos
```

ちなみに、以下のようにバンドル作成時に`--osr`とつけると、winitクレートとsoftbufferクレートを使ったOff-Screen Renderingをする方をビルドできます。
```shell
cargo run -p cef-rs-sample-bundle-macos -- --osr
```

その他にも、`--run`で、ビルド後に自動で動かすことができます。

## 謝辞
cef-rsとcef-rsのリポジトリにあるサンプルコードをベースに実装しています。
