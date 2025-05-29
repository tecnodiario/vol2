use async_nats::Client;
use chrono::Utc;
use serde_json::json;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connessione al broker
    let client: Client = async_nats::connect("nats://nats:4222").await?;
    // Invia un solo messaggio con timestamp
    let now = Utc::now();
    let payload = json!({
        "nome": "Alice",
        "ts": now.to_rfc3339(),
    });
    client
        .publish("amici.cli", payload.to_string().into())
        .await?;
    println!("[publisher] inviato: {}", payload);
    // opzionale: tieni vivo per 1 secondo per garantire flush
    sleep(Duration::from_secs(1)).await;
    Ok(())
}
