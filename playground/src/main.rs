//! 検証用の書き捨て

use std::fmt::Display;

fn main() {
    let mut a = Appellation {
        name: "Zeus".to_string(),
        nicknames: vec![
            "cloud collector".to_string(),
            "king of the gods".to_string(),
        ],
    };
    println!("before assignment");
    a = Appellation {
        name: "Hera".to_string(),
        nicknames: vec![],
    };
    println!("at end of block");

    let boxed_lunch: RcBox<String> = RcBox {
        ref_count: 1,
        value: "lunch".to_string(),
    };

    let boxed_displayable: &RcBox<dyn Display> = &boxed_lunch;

    display(&boxed_lunch);
}

struct Appellation {
    name: String,
    nicknames: Vec<String>,
}

impl Drop for Appellation {
    fn drop(&mut self) {
        print!("Dropping {}", self.name);
        if !self.nicknames.is_empty() {
            print!(" (AKA {})", self.nicknames.join(", "));
        }
        println!("");
    }
}

struct RcBox<T: ?Sized> {
    ref_count: usize,
    value: T,
}

fn display(boxed: &RcBox<dyn Display>) {
    println!("For your enjoyment: {}", &boxed.value);
}
