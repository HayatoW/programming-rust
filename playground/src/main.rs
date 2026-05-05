// 検証用の書き捨て

fn main() {}

fn describe_point(x: i32, y: i32) -> &'static str {
    use std::cmp::Ordering::*;
    match (x.cmp(&0), y.cmp(&0)) {
        (Equal, Equal) => "at the origin",
        (_, Equal) => "on the x axis",
        (Equal, _) => "on the y axis",
        (Greater, Greater) => "in the first quadrant",
        (Less, Greater) => "in the second quadrant",
        _ => "somewhere else",
    }
}

fn _match(next_char: char) {
    match next_char {
        '0'..'9' => {
            "a digit";
        }
        'a'..='z' => {
            "a lowercase letter";
        }
        'A'..='Z' => {
            "an uppercase letter";
        }
        _ => {
            "something else";
        }
    }
}
