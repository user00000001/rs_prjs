use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use tokio::task::{self, yield_now};
use std::rc::Rc;

// cargo run --example hello-redis

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6378").await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let handle = tokio::spawn(async move {
            process(socket).await;
            true
        });
        println!("Other works");
        let out = handle.await.unwrap();
        println!("GOT {:?}", out);
        break
    }
    let v = vec![1,2,3];
    task::spawn(async move {
        println!("Here's a vec: {:?}", v);
    });

    tokio::spawn(async {
        // The scope forces `rc` to drop before `.await`.
        {
            let rc = Rc::new("hello");
            println!("{}", rc);
        }
        yield_now().await;
    }).await.unwrap();
}

async fn process(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;
    let mut db = HashMap::new();
    let mut connection = Connection::new(socket);
    // if let Some(frame) = connection.read_frame().await.unwrap() {
    //     println!("GOT: {:?}", frame);
    //     let rsp = Frame::Error("unimplemented".to_string());
    //     connection.write_frame(&rsp).await.unwrap();
    // }
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let rsp = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd)
        };
        connection.write_frame(&rsp).await.unwrap();
    }
}