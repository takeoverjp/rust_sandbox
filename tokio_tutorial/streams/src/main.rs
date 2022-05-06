use mini_redis::client;
use tokio_stream::StreamExt;

async fn publish() -> mini_redis::Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    for s in vec!["1", "two", "3", "four", "five", "6"] {
        client.publish("numbers", s.into()).await?;
    }

    Ok(())
}

async fn subscribe() -> mini_redis::Result<()> {
    let client = client::connect("127.0.0.1:6379").await?;
    let subscriber = client.subscribe(vec!["numbers".to_string()]).await?;
    let messages = subscriber.into_stream();

    tokio::pin!(messages);

    while let Some(msg) = messages.next().await {
        println!("got = {:?}", msg);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> mini_redis::Result<()> {
    tokio::spawn(async { publish().await });

    subscribe().await?;

    println!("DONE");

    Ok(())
}
