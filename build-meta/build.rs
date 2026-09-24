// https://microsoft.github.io/RustTraining/engineering-book/ch01-build-scripts-buildrs-in-depth.html#pattern-1-compile-time-constants
use std::process::Command;

fn main() {
    println!("cargo::rerun-if-changed=.git/HEAD");
    println!("cargo::rerun-if-changed=.git/refs");

    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .expect("git not found");
}
