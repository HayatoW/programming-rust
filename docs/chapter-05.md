- 共有参照 (shared reference) `&T` は「ref T」と発音する
- 可変参照 (mutable reference) `&mut T` は「ref mute T」と発音する

Rust では参照は明示的に & 演算子で作られ、参照解決も * 演算子で明示的に行われる。<br>
ただし、参照があまりに一般的に使われるので、 . 演算子が必要に応じて暗黙に左のオペランドを参照解決するようになっている。

```rust
struct Anime { name: &'static str, bechdel_pass: bool }

let anime = Anime { name: "Aria: The Animation", bechdel_pass: true };
let anime_ref = &anime;

assert_eq!(anime_ref.name, "Aria: The Animation");
// 上と同じ意味だが、参照解決を明示的にしている
assert_eq!((*anime_ref).name, "Aria: The Animation");
```

Rust コンパイラは、プログラム中のすべての参照型に対して、その参照の使われ方によって生じる制約を反映した **生存期間** (lifetime) を割り当てる。

参照の生存期間は肩の一部であり、実行時の表現はない。

Rust における、グローバル変数に該当するものは、static と呼ばれる。

生存期間 `'a` は tick A と発音する<br>
「任意の生存期間 a に対して」という意味
