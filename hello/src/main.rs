use std::net::TcpListener;

/// https://doc.rust-jp.rs/book-ja/ch20-01-single-threaded.html
fn main() {
    let listenter = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listenter.incoming() {
        let stream = stream.unwrap();

        // 接続が確立しました
        println!("Connection established!");
    }
}
