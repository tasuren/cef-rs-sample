# cef-sample-rs
[cef-rs](https://github.com/tauri-apps/cef-rs)を使って、ミニブラウザを勉強がてら作ってみる試み。
ちょっとした勉強目的なので、macOSでしか動作確認していません。

## ビルド方法
[Cargo.toml](./Cargo.toml)にあるcef-rsのバージョンがサポートするCEFを、cef-rsのリポジトリの説明を参考に`export-cef-dir`コマンドで用意してください。
パスを通すように説明がががあると思います。環境を汚したくないなら、それは以下のように`.cargo/config.toml`に書くと良いかもしれません。
```toml
[env]
CEF_PATH = { value = "パス", force = true }
DYLD_FALLBACK_LIBRARY_PATH = { value = "パス", force = true }
```

私はまだmacOSでしか試していないのですが、macOSなら以下のコマンドでアプリのバンドルが作成されます。
```
cargo run -p cef-sample-bundle-macos
```

ターミナルでアプリを実行したいなら、実行ファイルの直接パスを指定して実行すると良いと思います。
```
./target/debug/cef-sample-chrome.app/Contents/MacOS/cef-sample-chrome
```

## 謝辞
cef-rsとcef-rsのリポジトリにあるサンプルコードをベースに実装しています。

## 元と違う点
cef-rsのサンプルをベースに作っていますが、macではアプリを起動してもアプリアイコンがドック表示されません。
このため、`Info.plist`の`LSUIElement`の値を0にしています。
