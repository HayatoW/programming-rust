# 20. 非同期プログラミング

同期関数から非同期関数を呼び出すときは `block_on` を使う

```rust
fn main() -> std::io::Result<()> {
    use async_std::task;

    let response = task::block_on(cheapo_request("https://example.com", 80, "/"))?;
    println!("{}", response);
    Ok(())
}
```

- 非同期ブロックには返り値の返り値の型が指定できない
- `async_std::task::spawn_local` はスレッドプールで処理を実行する。複数のスレッドで単一タスクが実行される可能性があるので、スレッドローカルストレージの使用は NG
