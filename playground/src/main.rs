//! 検証用の書き捨て

use std::io::Write;

fn main() {}

fn say_hello<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}
