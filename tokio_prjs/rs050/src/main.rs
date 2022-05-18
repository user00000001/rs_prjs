use tokio_stream::StreamExt;
use mini_redis::client;

#[tokio::main]
async fn main() -> mini_redis::Result<()> {
    let mut stream = tokio_stream::iter(&[1,2,3]);
    while let Some(v) = stream.next().await {
        println!("Got = {}", v);
    }

    tokio::spawn(async {
        publish().await
    });
    subscribe().await?;
    println!("DONE");
    Ok(())
}

async fn publish() -> mini_redis::Result<()> {
    let mut client = client::connect("127.0.0.1:6378").await?;

    client.publish("numbers", "1".into()).await?;
    client.publish("numbers", "two".into()).await?;
    client.publish("numbers", "3".into()).await?;
    client.publish("numbers", "four".into()).await?;
    client.publish("numbers", "five".into()).await?;
    client.publish("numbers", "6".into()).await?;
    client.publish("numbers", "7".into()).await?;
    Ok(())
}

async fn subscribe() -> mini_redis::Result<()> {
    let client = client::connect("127.0.0.1:6378").await?;
    let subscriber = client.subscribe(vec!["numbers".to_string()]).await?;
    // let messages = subscriber.into_stream();
    // let messages = subscriber.into_stream().take(3);
    let messages = subscriber.into_stream()
        .filter(|msg| match msg {
            Ok(msg) if msg.content.len() == 1 => true,
            _ => false,
        })
        .take(3);
    // let messages = subscriber.into_stream()
    //     .filter(|msg| match msg {
    //         Ok(msg) if msg.content.len() == 1 => true,
    //         _ => false,
    //     })
    //     .map(|msg| msg.unwrap().content)
    //     .take(3);

    tokio::pin!(messages);
    
    while let Some(msg) = messages.next().await {
        println!("got = {:?}", msg);
    }
    Ok(())
}