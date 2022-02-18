use std::cmp::PartialEq;

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddr1 {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
struct IpAddrV4 {
    address: String,
}

#[derive(Debug)]
struct IpAddrV6 {
    address: String,
}

#[derive(Debug)]
enum IpAddr3 {
    V4(IpAddrV4),
    V6(IpAddrV6),
}

impl PartialEq for IpAddrKind {
    fn eq(&self, other: &IpAddrKind) -> bool {
        match (self, other) {
            (IpAddrKind::V4, IpAddrKind::V4) | (IpAddrKind::V6, IpAddrKind::V6)=> true,
            _ => false
        }
    }
}

#[derive(Debug)]
struct EnterMessage {

}

#[derive(Debug)]
struct ExitMessage {

}

#[derive(Debug)]
enum Message {
    Enter(EnterMessage),
    Exit(ExitMessage),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    println!("{:?}", (IpAddrKind::V4, IpAddrKind::V6));
    let four = IpAddrKind::V4;
    route(four);
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    let home1 = IpAddr1::V4(String::from("127.0.0.1"));
    let loopback1 = IpAddr2::V6(String::from("::1"));
    let home2 = IpAddr3::V4(IpAddrV4{address: String::from("127.0.0.1")});
    println!("{:#?}", (home, loopback, IpAddr1::V6(String::from("::1")), IpAddr2::V4(127,0,0,1), home1, loopback1, home2, IpAddr3::V6(IpAddrV6{address: String::from("::1")})));
    let msg = Message::Enter(EnterMessage{});
    let msg1 = Message::Exit(ExitMessage{});
    let msg_tuple = (msg, msg1);
    println!("{:?}", msg_tuple);
    let (msg, msg1) = msg_tuple;
    msg.call();
    msg1.call();

    let mut o: Option<i32> = Some(100);
    match o {
        Some(v) => println!("Have a {} value", v),
        None => println!("Have no value")
    }
    o = None;
    match o {
        Some(v) => println!("Have a {} value", v),
        None => println!("Have no value")
    }
}

fn route(e: IpAddrKind) {
    if e == IpAddrKind::V4 {
        println!("ipv4");
    }
    if e == IpAddrKind::V6 {
        println!("ipv6")
    }
    match e {
        IpAddrKind::V4 => println!("ipv4"),
        IpAddrKind::V6 => println!("ipv6"),
    }
}