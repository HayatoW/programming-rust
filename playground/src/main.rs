// 検証用の書き捨て

use std::collections::HashMap;

fn main() {}

enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    /// Object を示す HashMap を Box で囲んでいるのは、
    /// すべての Json 値をよりコンパクトにするため。
    Object(Box<HashMap<String, Json>>),
}
