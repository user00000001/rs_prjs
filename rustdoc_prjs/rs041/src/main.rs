use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener,TcpStream};
use std::time::Duration;

use thread_pool::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //     println!("Connection established!");
    //     std::thread::spawn(|| { // unlimited threads may cause DoS by essential requests.
    //         handle_connection(stream);
    //     });
    // }

    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(||{ // echo -ne "GET /sleep HTTP/1.1\r\n" | nc -v 127.0.0.1 7878 & echo -ne "GET / HTTP/1.1\r\n" | nc -v 127.0.0.1 7878
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        std::thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let rsp = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(rsp.as_bytes()).unwrap();
    stream.flush().unwrap();
}