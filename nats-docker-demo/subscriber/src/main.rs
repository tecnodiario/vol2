use async_nats::Client;
use chrono::{DateTime, Utc};
use tokio_stream::StreamExt;
use serde::Deserialize;

#[derive(Deserialize)]
struct Msg {
    nome: String,
    ts: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connessione al broker
    let client: Client = async_nats::connect("nats://nats:4222").await?;
    // Iscrizione al subject "amici.cli"
    let mut sub = client.subscribe("amici.cli").await?;
    println!("Subscriber in ascolto su 'amici.cli'...");

    while let Some(message) = sub.next().await {
        let data = String::from_utf8(message.payload.to_vec())?;
        // Parse JSON in struct Msg
        let msg: Msg = serde_json::from_str(&data)?;
        let now: DateTime<Utc> = Utc::now();
        let delta = now.signed_duration_since(msg.ts);

        println!(
            "[amici.cli] ricevuto nome='{}', ts_sent='{}', ts_recv='{}', delta={} ms",
            msg.nome,
            msg.ts.to_rfc3339(),
            now.to_rfc3339(),
            delta.num_milliseconds()
        );
    }

    Ok(())
}
