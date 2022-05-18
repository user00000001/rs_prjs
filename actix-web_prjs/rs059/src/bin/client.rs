use std::env;
use futures_util::{future, pin_mut, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{
    // connect_async,
    Connector,
    connect_async_tls_with_config,
    tungstenite::protocol::{Message, WebSocketConfig}
};
use native_tls::TlsConnector;

#[tokio::main]
async fn main() {
    let connect_addr =
        env::args().nth(1).unwrap_or_else(|| panic!("this program requires at least one argument"));

    let url = url::Url::parse(&connect_addr).unwrap();

    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    // let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let connector = Connector::NativeTls(TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .build().unwrap());
    let config = WebSocketConfig::default();
    let (ws_stream, _) = connect_async_tls_with_config(url, Some(config), Some(connector)).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (write, read) = ws_stream.split();

    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = {
        read.for_each(|message| async {
            let data = message.unwrap().into_data();
            tokio::io::stdout().write_all(&data).await.unwrap();
        })
    };

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
}

async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(1) if &buf[..1] == b"\n" => {
                println!(
                    "Type something, press ENTER, then echo the same thing"
                );
                continue
            },
            Ok(n) => n,
        };
        buf.truncate(n);
        tx.unbounded_send(Message::binary(buf)).unwrap();
    }
}