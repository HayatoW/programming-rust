列挙型の構成要素を variant もしくは constructors (構成子) と呼ぶ

パブリックな列挙型のコンストラクタとフィールドはすべて、自動的にパブリックになる。

```rust
match account {
    // ref キーワードを付与すると値は借用となる (消費されない)
    Account { ref name, ref language, .. } => {
        ui.greet(name, language);
        // account は消費されていないので使える
        ui.show_settings(&account);
    }
}
```

```rust
match self.get_selection() {
    Shape::Rect(top_left, bottom_right) => optimized_paint(&Shape::Rect(top_left, bottom_right))
    other_shape => print_outline(other_shape.get_outline())
}

以下と等価
match self.get_selection() {
    rect @ Shape::Rect(..) => optimized_paint(&rect)
    other_shape => print_outline(other_shape.get_outline())
}
```
