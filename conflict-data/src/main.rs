use std::{rc::Rc, thread};

fn main() {
    let rc1 = Rc::new("ouch".to_string());
    let rc2 = rc1.clone();
    thread::spawn(move || {
        rc2.clone();
    });
    rc1.clone();
}
