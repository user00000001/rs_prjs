use mini_redis::{client, Result};
use std::io::{Error, ErrorKind};

// cargo install mini-redis
// mini-redis-server --port 6378
// mini-redis-cli --port 6378 get hello

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6378").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    let result1 = check_it("world!".to_owned()).await?;
    println!("hello, {}", result1);
    check_it("11111world!".to_owned()).await?;

    Ok(())
}

async fn check_it(s: String) -> Result<String> {
    if s.len() > 10 {
        return Err(Box::new(Error::new(ErrorKind::Other, "s is to long for 10")));
    }
    Ok(s)
}