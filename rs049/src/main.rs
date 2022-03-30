use tokio::net::{TcpStream, TcpListener};
use tokio::sync::{oneshot, mpsc};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::io;
use std::thread;
use std::time::Duration;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};

#[tokio::main]
async fn main() -> io::Result<()> {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();
    tokio::spawn(async {
        let _ = tx1.send("one");
    });
    tokio::spawn(async {
        let _ = tx2.send("two");
    });
    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    };
//--------------------
    let (mut tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();
    tokio::spawn(async {
        tokio::select! {
            val = some_operation() => {
                let _ = tx1.send(val);
            }
            _ = tx1.closed() => {
                println!("tx1 closed");
            }
        }
    });
    tokio::spawn(async {
        thread::sleep(Duration::from_secs(1));
        let _ = tx2.send("two");
    });
    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    }
//------------------
    let (tx, rx) = oneshot::channel();
    tokio::spawn(async move {
        tx.send("done").unwrap();
    });
    tokio::select! {
        socket = TcpStream::connect("localhost:3456") => {
            println!("Socket connected {:?}", socket)
        }
        msg = rx => {
            println!("received message first {:?}", msg);
        }
    }
//-------------------
    let out = tokio::select! {
        res = computation1() => res,
        res = computation2() => res,
    };
    println!("Got = {}", out);

    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();
    let mut out = String::new();
    tokio::spawn(async move {
        tx1.send("haha").unwrap();
    });
    tokio::spawn(async move {
        tx2.send("hoho").unwrap();
    });
    tokio::select! {
        val = rx1 => {
            match val {
                Ok(val) => out.push_str(val),
                Err(_) => ()
            }
        }
        val = rx2 => {
            match val {
                Ok(val) => out.push_str(val),
                Err(_) => ()
            }
        }
    }
    println!("{}", out);
//-----------------------
    let (tx1, mut rx1) = mpsc::channel(128);
    let (tx2, mut rx2) = mpsc::channel(128);
    let (tx3, mut rx3) = mpsc::channel(128);
    tokio::spawn(async move {
        tx1.send("msg from tx1").await.unwrap();
    });
    tokio::spawn(async move {
        tx2.send("msg from tx2").await.unwrap();
    });
    tokio::spawn(async move {
        tx3.send("msg from tx3").await.unwrap();
    });
    loop {
        let msg = tokio::select! {
            Some(msg) = rx1.recv() => msg,
            Some(msg) = rx2.recv() => msg,
            Some(msg) = rx3.recv() => msg,
            else => break,
        };
        println!("Got {}", msg);
        // break;
    }
//-----------------------
    let (tx, mut rx) = mpsc::channel(128);
    let mut done = false;
    let operation = action(None);
    tokio::pin!(operation);
    tokio::spawn(async move {
        let _ = tx.send(1).await;
        let _ = tx.send(2).await;
        let _ = tx.send(3).await;
    });
    loop {
        tokio::select! {
            res = &mut operation, if !done => {
                done = true;
                if let Some(v) = res {
                    println!("Got = {}", v);
                    return Ok(());
                }
            }
            Some(v) = rx.recv() => {
                if v % 2 == 0 {
                    operation.set(action(Some(v)));
                    println!("recv {}", v);
                    break;
                }
            }
            // else => break,
        }
    }
//-----------------------
    let data = "hello\r\n".as_bytes();
    let addr1 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)), 9000);
    let addr2 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)), 9000);
    tokio::spawn(async move {
        race(data, addr1, addr2).await.unwrap();
    });
//-----------------------
    let (tx, rx) = oneshot::channel();
    tokio::spawn(async move {
        thread::sleep(Duration::from_secs(5));
        tx.send(()).unwrap();
    });
    let listener = TcpListener::bind("localhost:3456").await?; // echo -en "haha\r\n"| nc 127.0.0.1 3456
    tokio::select! {
        res = async {
            loop {
                let (socket, _) = listener.accept().await?;
                tokio::spawn(async move { process(socket).await });
            }
            // Help the rust type inferencer out
            #[allow(unreachable_code)]
            Ok::<_, io::Error>(())
        } => {
            res?
        }
        _ = rx => {
            println!("terminating accept loop");
        }
    };
    Ok(())
}

async fn some_operation() -> String {
    thread::sleep(Duration::from_secs(1));
    "from some_operation".to_owned()
}

async fn process(mut socket: TcpStream) {
    let mut buffer = vec![0;1024];
    socket.read(&mut buffer).await.unwrap();
    match String::from_utf8(buffer) {
        Ok(s) => print!("{}", s),
        Err(_) => (),
    }
}

async fn computation1() -> String {
    std::thread::sleep(Duration::from_secs(1));
    "computation1".to_owned()
}

async fn computation2() -> String {
    std::thread::sleep(Duration::from_secs(1));
    "computation2".to_owned()
}

async fn race(data: &[u8], addr1: SocketAddr, addr2: SocketAddr) -> io::Result<()> {
    tokio::select! {
        Ok(_) = async {
            let mut socket = TcpStream::connect(addr1).await?;
            socket.write_all(data).await?;
            socket.flush().await?;
            Ok::<_, io::Error>(())
        } => {}
        Ok(_) = async {
            let mut socket = TcpStream::connect(addr2).await?;
            socket.write_all(data).await?;
            socket.flush().await?;
            Ok::<_, io::Error>(())
        } => {}
        else => {
            println!("3");
        } // else for select! default
    };
    Ok(())
}

async fn action(input: Option<i32>) -> Option<String> {
    let i = match input {
        Some(input) => input,
        None => return None,
    };
    Some(i.to_string())
}