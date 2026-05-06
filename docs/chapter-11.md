> [!TIP]
> トレイトを使って多相性のあるコードを書く方法は、2つある。<br>
> トレイトオブジェクトとジェネリクスだ。

キーワード where は必須ではないので、可読性を考慮して使用すれば OK っぽい

```rust
// where を使わない版
fn run_query<M: Mapper + Serialize, R: Reducer + Serialize>(
    data: &DataSet,
    map: M,
    reduce: R,
) -> Results {
    ...
}

// where を使う版
fn run_query<M, R>(data: &DataSet, map: M, reduce: R) -> Results
where
    M: Mapper + Serialize, R: Reducer + Serialize
{
    ...
}
```

生存期間は機械語レベルのコードには影響を与えない。

### トレイトオブジェクト

トレイト型への参照

腹落ちしていないので、とりあえず、写経しておく

```rust
let mut buf: Vec<u8> = vec![];
// これ
let writer: &mut dyn Writer = &mut buf;
```

### トレイトオブジェクトとジェネリック関数のどちらを使うか

複数の方が入り混じっているコレクションを扱う場合には、トレイトオブジェクトを使うのが正しい。

```rust
trait Vegetable {
    ...
}

// ジェネリック構造体を使うパターン
// サラダが1種類の野菜になってしまうので厳しい
struct Salad<V: Vegetable> {
    veggies: Vec<V>
}

// トレイトオブジェクトを使うパターン
// veggies には任意の野菜を入れることができる (1種類に固定されない)
struct Salad {
    veggies: Vec<Box<dyn Vegetable>>
}
```

トレイトオブジェクトを使うもう一つの理由

- コンパイルされたコードの総量を減らすこと
  - ジェネリック関数は型ごとにコンパイルされる

ジェネリクスを使う理由

- スピード (dyn キーワードによる動的ディスパッチがない)
- すべてのトレイトがトレイトオブジェクトを対応するとは限らない (以下が例)
  - 型関連関数
  - Self 型を使うトレイト
- 複数のトレイトを用いた型制約ができる

```rust
pub trait Iterator {
    // 関連型
    type Item;

    fn next(&mut self) -> Option<Self::Item>
}

impl Iterator for Args {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        ...
    }
}
```

関数の返り値の型をトレイトオブジェクトで書いている場合、impl trait で置き換えられないか考えてみる

```rust
// コンパイルできるけど危ないコード
fn dot(v1: &[i64], v2: &[i64]) -> i64 {
    let mut total = 0;
    for i in 0..v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}
```
