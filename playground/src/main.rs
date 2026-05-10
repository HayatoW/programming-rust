//! 検証用の書き捨て

fn main() {
    let my_str = "hello".to_string();
    let f = drop(my_str);
    call_twice(f);
}

fn call_twice<F: Fn()>(closure: F) {
    closure();
    closure();
}
