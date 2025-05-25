use async_nats::Client;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connetti al broker NATS
    let client: Client = async_nats::connect("nats://nats:4222").await?;
    for i in 1..=5 {
        let msg = format!("Messaggio numero {}", i);
        client.publish("amici", msg.clone().into()).await?;
        println!("Inviato: {}", msg);
        sleep(Duration::from_secs(1)).await;
    }
    Ok(())
}
