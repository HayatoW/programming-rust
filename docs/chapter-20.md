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
