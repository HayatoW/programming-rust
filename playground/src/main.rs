// 検証用の書き捨て

use std::{
    error::Error,
    fs,
    io::{self, BufRead, Write, stderr},
    path::Path,
};

fn main() {
    let share = pirate_share(100, 0);
    println!("一人につき {} 個の略奪品をゲット！", share);
}

fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    half / crew_size as u64
}

fn print_error(mut err: &dyn Error) {
    let _ = writeln!(stderr(), "error: {}", err);
    while let Some(source) = err.source() {
        let _ = writeln!(stderr(), "caused by: {}", source);
        err = source;
    }
}

fn move_all(src: &Path, dst: &Path) -> io::Result<()> {
    for entry_result in src.read_dir()? {
        let entry = entry_result?;
        let dst_file = dst.join(entry.file_name());
        fs::rename(entry.path(), dst_file)?
    }
    Ok(())
}

fn read_numbers(file: &mut dyn BufRead) -> Result<Vec<i64>, io::Error> {
    let mut numbers = vec![];
    for line_result in file.lines() {
        let line = line_result?;
        // このエラーには thiserror クレートを使って対応するのが適している
        numbers.push(line.parse()?);
    }
    Ok(numbers)
}
