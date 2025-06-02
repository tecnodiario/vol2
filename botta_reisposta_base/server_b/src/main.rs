use async_nats::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio_stream::StreamExt;
use uuid::Uuid;

/// Aggiungiamo `Debug` e `Deserialize` per poter deserializzare il JSON e stampare la struct
#[derive(Debug, Deserialize)]
struct AuthRequest { user_id: String, token: String }

#[derive(Serialize, Debug)]
struct AuthResponse { session_id: String, valid: bool }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connessione al broker NATS
    let client: Client = async_nats::connect("nats://nats:4222").await?;
    
    // Generazione di un ID univoco per il server
    let server_id = Uuid::new_v4().to_string();
    

    // Sottoscrizione al canale di autenticazione
    let mut sub = client.subscribe("auth.request").await?;
    println!("Server A (ID: {}) in ascolto sul canale auth.request...", server_id);

    while let Some(msg) = sub.next().await {
        // Elaborazione della richiesta di autenticazione
        // msg.payload è un Vec<u8> contenente il JSON
        // Con `serde_json::from_slice` lo trasformiamo in AuthRequest
        let req: AuthRequest = serde_json::from_slice(&msg.payload)?;
        println!("Ricevuta richiesta di autenticazione da {}: {:?}", req.user_id, req);

        let is_valid = req.token == "valid";
        // Simulazione di una risposta di autenticazione
        let resp = AuthResponse {
            session_id: if is_valid {server_id.clone()}
            else { "none".to_string() },
            valid: is_valid, // Logica semplificata per la validità del token
        };

        // Invio della risposta al mittente
        // msg.reply contiene il subject di risposta a cui inviare il JSON
        let reply_subject = msg.reply.unwrap();
        client.publish(reply_subject.clone(), json!(resp).to_string().into()).await?;
        println!("Risposta inviata a {}: {:?}", reply_subject, resp);
    }

    Ok(())
}
// Nota: Questo server rimarrà in esecuzione finché non viene interrotto manualmente.