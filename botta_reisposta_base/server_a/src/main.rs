use async_nats::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::time::{sleep, Duration};

#[derive(Serialize)]
struct AuthRequest { user_id: String, token: String }

#[derive(Deserialize, Debug)]
struct AuthResponse { session_id: String, valid: bool }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client: Client = async_nats::connect("nats://nats:4222").await?;
    println!("Server A invia richiesta di autenticazione...");

    let req = AuthRequest { user_id: "user123".into(), token: "valid".into() };

    let reply = client.request("auth.request", json!(req).to_string().into()).await?;

    let resp: AuthResponse = serde_json::from_slice(&reply.payload)?;
    println!("Risposta: {:?}", resp);

    if resp.valid {
        println!("Autenticazione riuscita, sessione: {}", resp.session_id);
    } else {
        println!("Autenticazione fallita");
    }
    println!("Server A attende 1 secondo prima di chiudere...");
    sleep(Duration::from_secs(1)).await;
    Ok(())  
}