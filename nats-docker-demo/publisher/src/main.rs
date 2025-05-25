use nats::asynk::Connection;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nc: Connection = nats::asynk::connect("nats:4222").await?;
    for i in 1..=5 {
        let msg = format!("Messaggio numero {}", i);
        nc.publish("amici", msg.as_bytes()).await?;
        println!("Inviato: {}", msg);
        sleep(Duration::from_secs(1)).await;
    }
    Ok(())
}
