//! 検証用の書き捨て

use std::collections::HashSet;

fn main() {
    let squares = [4, 9, 16, 25, 36, 49, 64];
    let (powers_of_two, impure): (HashSet<i32>, HashSet<i32>) =
        squares.iter().partition(|&n| n & (n - 1) == 0);
}

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

fn parse_i32_bytes(b: &[u8]) -> GenericResult<i32> {
    Ok(std::str::from_utf8(b)?.parse::<i32>()?)
}
