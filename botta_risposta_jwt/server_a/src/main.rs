use async_nats::{Client};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::time::{sleep};
use jsonwebtoken::{encode, EncodingKey, Header};
use uuid::Uuid;

#[derive(Serialize)]
struct AuthRequest { user_id: String, token: String, exp: usize, jti: String }

#[derive(Deserialize, Debug)]
struct AuthResponse { session_id: String, valid: bool }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Carico la chiave privata per firmare il JWT
    let private_key = std::fs::read("private.pem")?;
    let encoding_key = EncodingKey::from_rsa_pem(&private_key)?;

    // 2. Connettiamoci al broker NATS
    let client: Client = async_nats::connect("nats://nats:4222").await?;

    // 3. Generiamo un ID univoco per la sessione
    let session_id = Uuid::new_v4().to_string();

    // 4. Creiamo il JWT con le informazioni di autenticazione
    let claims = AuthRequest {
        user_id: "user123".into(),
        token: "valid".into(),
        exp: (chrono::Utc::now() + chrono::Duration::minutes(5)).timestamp() as usize,
        jti: session_id.clone(),
    };
    
    // 3. Firmo il JWT con RS256
    let token = encode(&Header::new(jsonwebtoken::Algorithm::RS256), &claims, &encoding_key)?;

    // 4. Inviamo la richiesta di autenticazione
    let req = json!({ "jwt": token });
    let reply = client.request("auth.request", req.to_string().into()).await?;

    // 5. Gestiamo la risposta
    let response: AuthResponse = serde_json::from_slice(&reply.payload)?;
    
    println!("Risposta autenticazione: {:?}", response);

    if response.valid {
        println!("Autenticazione riuscita, sessione: {}", response.session_id);
    } else {
        println!("Autenticazione fallita");
    }
    println!("Server A attende 1 secondo prima di chiudere...");
    // 7. Mantengo vivo il container per un attimo
    sleep(std::time::Duration::from_secs(1)).await;
    Ok(())
}