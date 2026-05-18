```rust
// なぜ B?
fn map<B, F>(self, f: F) -> impl Iterator<Item=B>
    where Self: Sized, F: FnMut(Self::Item) -> B;
```

> flatten メソッドには少し変わった使い方がある。<br>
> Vec<Option<...>> に対して、Some の値だけを取得したい場合には、flatten が使える。

```rust
assert_eq!(
    vec![None, Some("day"), None, Some("one")]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>(),
    vec!["day", "one"]
);
```

> これができるのは、Option を要素が1個もしくは0個のシーケンスとして捉える IntoIterator を実装しているからだ。
