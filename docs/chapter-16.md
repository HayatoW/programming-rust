# VecDeque<T>

deque - deck と同じ発音

先頭と末尾の双方に対して、効率的に追加削除できる。

# テキストの追加と挿入

String は Add<&str> と AddAssign<&str> を実装している

```rust
let left = "partners".to_string();
let mut right = "crime".to_string();
assert_eq!(left + " in " + right, "partners in crime");
```

&str は + 演算子の左オペランドにはできない

```rust
// これは書けない
let parenthetical = "(" + string + ")";

// こう書く
let parenthetical = "(".to_string() + &string + ")";
```

# 他の型から文字列への変換

- ある型が Display を実装しているなら、標準ライブラリが自動的に std::str::ToString トレイトを実装する。ToString トレイトは、Display トレイトが導入されるよりも前から存在するもので、やや柔軟性に欠ける。独自の型に関しては Display を実装するのがおすすめ
