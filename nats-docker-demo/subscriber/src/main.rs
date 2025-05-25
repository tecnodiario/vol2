use async_nats::Client;
use tokio_stream::StreamExt;   // Per usare il metodo `next` su `sub`


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connetti al broker NATS nel servizio 'nats'
    let client: Client = async_nats::connect("nats://nats:4222").await?;
    // Sottoscrivi il subject "amici"
    let mut sub = client.subscribe("amici").await?;
    println!("Subscriber in ascolto su 'amici'...");

    // Elabora ogni messaggio man mano che arriva
    while let Some(message) = sub.next().await {
        let text = String::from_utf8(message.payload.to_vec())?;
        println!("[amici] {}", text);
    }

    Ok(())
}