use std::{error::Error, io};

fn http_get_main(url: &str) -> Result<(), Box<dyn Error>> {
    // HTTP リクエストを送り、レスポンスを受け取る
    let mut response = reqwest::blocking::get(url)?;
    if !response.status().is_success() {
        Err(format!("{}", response.status()))?;
    }

    // レスポンスのボディを読み出し、標準出力に書き出す
    let stdout = io::stdout();
    io::copy(&mut response, &mut stdout.lock())?;

    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: network URL");
        return;
    }

    if let Err(err) = http_get_main(&args[1]) {
        eprintln!("error: {}", err);
    }
}
