
[egui](https://github.com/emilk/egui) のサンプルである [hello_world](https://github.com/emilk/egui/tree/master/examples/hello_world) を動かしてみたメモ。

[2021年末に基本的な使い方を記事にまとめていた](https://zenn.dev/tris/articles/rust-egui-01)が、2024年9月次点ではバージョンが当時の `0.15` から `0.28` に上がりかなり変化があるため、改めて試してみた。


## 依存関係

```sh
cargo add eframe
```

画像を表示するために、`egui_extras`([リポジトリ](https://github.com/emilk/egui/tree/master/crates/egui_extras)) を追加する。このとき、`--features="image"` により画像表示を有効化する。

```sh
cargo add egui_extras --features="image"
```


## 画像の準備

[画像ファイル](https://github.com/emilk/egui/blob/master/crates/egui/assets/ferris.png)をダウンロードする。

`assets` フォルダを作成し、その中に画像ファイルを配置する。


## コーディング

`main.rs` を記述する。

[元のソース](https://github.com/emilk/egui/blob/master/examples/hello_world/src/main.rs)をコピペしてきたものから、一部以下のような修正を加えている。

- コメントの翻訳・追加した。
- ログ出力部分をコメントアウトした。
- 画像ファイルのパスをローカル実行環境に合わせた。`main.rs` から見た相対パスを記述。


## 実行

```sh
cargo run
```

これにより、ローカルでネイティブアプリとして実行される。



