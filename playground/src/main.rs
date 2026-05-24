//! 検証用の書き捨て

use std::cmp::Ordering;

// 2つの f64 値を比較する。NaN ならパニックを起こす
fn cmp(lhs: &f64, rhs: &f64) -> Ordering {
    lhs.partial_cmp(rhs).unwrap()
}

fn main() {
    let numbers = [1.0, 4.0, f64::NAN, 2.0];
    assert_eq!(numbers.iter().copied().max_by(cmp), Some(4.0));
    assert_eq!(numbers.iter().copied().min_by(cmp), Some(1.0));
}
