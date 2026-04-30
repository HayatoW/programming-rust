# プログラミング Rust 第2版 勉強

[プログラミング Rust 第2版 - O'Reilly Japan](https://www.oreilly.co.jp/books/9784873119786/)

![mandelbrot/mandel.png](mandelbrot/mandel.png)


## 用語

- 文字列スライス (`&str`)
  - 文字列データへの **参照 (借用)**
- turbofish `::<>`
- 型パラメーター `T`
- ドキュメントコメント `///`
- 参照 `&`
- 参照解決 (dereference) `*`
- 属性 (attribute) `#[test]` など
- `mut` ミュートと発音する
- `fn` ファンと発音する
- ワイルドカードパターン `_`
- unit 型 `()` 1つの状態しか持たないのでユニット
- `move` キーワード クロージャーが利用する変数の所有権を取得する

## Tips

- main 関数には戻り値がない
  - ? 演算子による Error 返却はできない
