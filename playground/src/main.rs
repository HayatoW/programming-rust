//! 検証用の書き捨て

use std::ops::{Add, Mul};

use num::Num;

fn main() {}

fn dot_v2<N: Num + Copy>(v1: &[N], v2: &[N]) -> N {
    let mut total = N::zero();
    for i in 0..v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

fn dot<N>(v1: &[N], v2: &[N]) -> N
where
    N: Add<Output = N> + Mul<Output = N> + Default + Copy,
{
    let mut total = N::default();
    for i in 0..v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

#[test]
fn test_dot() {
    assert_eq!(dot(&[1, 2, 3, 4], &[1, 1, 1, 1]), 10);
    assert_eq!(dot(&[53.0, 7.0], &[1.0, 5.0]), 88.0);
}

#[test]
fn test_dot_v2() {
    assert_eq!(dot_v2(&[1, 2, 3, 4], &[1, 1, 1, 1]), 10);
    assert_eq!(dot_v2(&[53.0, 7.0], &[1.0, 5.0]), 88.0);
}
