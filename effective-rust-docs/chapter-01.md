# 型

Rust の enum 型は代数データ型を実現する。

## データ構造を表現するために型システムを用いる

### 基本型

- コレクションのサイズ (.len) は usize で返すことが一般的
- コレクションのインデックス (.len) は usize を用いることが一般的
- [The Absolute Minimum Every Software Developer Absolutely, Positively Must Know About Unicode and Character Sets (No Excuses!)](https://www.joelonsoftware.com/2003/10/08/the-absolute-minimum-every-software-developer-absolutely-positively-must-know-about-unicode-and-character-sets-no-excuses/) を読め

### 集約型

配列、タプル、構造体

### enum

代数データ型として用いる (ADT: Algebraic Data Type)<br>
プログラムのデータ構造の不変条件を Rust の型システムにエンコードできる
