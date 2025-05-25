use nats::asynk::Connection;
use tokio::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nc: Connection = nats::asynk::connect("nats:4222").await?;
    let mut sub = nc.subscribe("amici".into()).await?;
    println!("Subscriber in ascolto sul canale 'amici'...");
    while let Some(msg) = sub.next().await {
        let text = String::from_utf8(msg.data.clone())?;
        println!("[amici] {}", text);
    }
    Ok(())
}
