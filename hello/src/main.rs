use std::env;
use std::str::FromStr;

fn main() {
    let mut numbers = Vec::new();

    // args で得られるイテレータが最初に生成する値は、常にそのプログラムの名前となるので、スキップ
    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    // & 演算子はベクタの2番目以降の要素への参照 (reference) を借用している
    for m in &numbers[1..] {
        // * は、m を参照解決 (dereferences) する演算子で、参照されている値を返す
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

/// https://en.wikipedia.org/wiki/Euclidean_algorithm
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}
