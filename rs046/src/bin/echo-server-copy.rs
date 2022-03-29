use tokio::net::TcpListener;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        // let (socket, _) = listener.accept().await?;
        // tokio::spawn(async move {
        //     let (mut rd, mut wr) = io::split(socket);
        //     if io::copy(&mut rd, &mut wr).await.is_err() {
        //         eprintln!("failed to copy");
        //     }
        // });

        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = vec![0;1024];
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => return, // TCP stream is shut down, a call to read() returns Ok(0), avoid read loop
                    Ok(n) => {
                        match String::from_utf8(buf.clone()) {
                            Ok(s) => print!("{}", s),
                            _ => ()
                        }
                        if socket.write_all(&buf[..n]).await.is_err() {
                            eprintln!("failed to write all");
                            return;
                        }
                    }
                    Err(_) => {
                        eprintln!("failed to read");
                        return;
                    }
                }
            }
        });
    }
}