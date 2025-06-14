use async_nats::Client;
use tokio_stream::StreamExt;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use uuid::Uuid;


// Nel JWT inserisci anche il campo 'token'
#[derive(Deserialize)]
struct AuthRequest {
    sub:    String,
    token:  String,
    exp:    usize,
    jti:    String,
}

// Struttura che descrive esattamente i claim attesi nel JWT
#[derive(Serialize, Debug)]
struct AuthResponse {
    session_id: String,
    valid:      bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let public_key    = std::fs::read("public.pem")?;
    let decoding_key  = DecodingKey::from_rsa_pem(&public_key)?;
    let client: Client = async_nats::connect("nats://nats:4222").await?;
    let mut sub       = client.subscribe("auth.request").await?;
    println!("Server B in ascolto su 'auth.request'...");

    while let Some(msg) = sub.next().await {
        // Estrazione del JWT dal payload JSON
        let v: serde_json::Value = serde_json::from_slice(&msg.payload)?;
        let token_str = v["jwt"].as_str().unwrap();

        // Verifica firma + exp e deserializzazione in AuthRequest
        let data = match decode::<AuthRequest>(
            token_str,
            &decoding_key,
            &Validation::new(Algorithm::RS256),
        ) {
            Ok(d) => d.claims,
            Err(err) => {
                // Se la firma non è valida o il token è scaduto, inviamo subito false
                let resp = AuthResponse {
                    session_id: String::new(),
                    valid:      false,
                };
                let resp_json = serde_json::to_string(&resp)?;          // AuthResponse → String
                client.publish(msg.reply.unwrap(), resp_json.into()).await?;  // String → Bytes

                continue;
            }
        };

        // Controllo applicativo: il campo `token` deve esattamente contenere "valid"
        if data.token == "valid" {
            println!("Utente '{}' ha token valido: '{}'", data.sub, data.token);
        } else {
            println!("Utente '{}' ha token NON valido: '{}'", data.sub, data.token);
        }

        // Generazione del session_id solo se il token è "valid"
        let is_valid = data.token == "valid";
        let session_id = if is_valid {
            Uuid::new_v4().to_string()
        } else {
            String::new()
        };

        // Invio della risposta al subject di reply
        let response = AuthResponse { session_id, valid: is_valid };
        let resp_json = serde_json::to_string(&response)?;          // AuthResponse → String
        client.publish(msg.reply.unwrap(), resp_json.into()).await?;  // String → Bytes

        println!("Risposta inviata: {:?}", response);
    }

    Ok(())
}