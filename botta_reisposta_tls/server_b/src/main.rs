use async_nats::ConnectOptions;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio_stream::StreamExt;
use uuid::Uuid;
use std::path::PathBuf;

/// Aggiungiamo `Debug` e `Deserialize` per poter deserializzare il JSON e stampare la struct
#[derive(Debug, Deserialize)]
struct AuthRequest { user_id: String, token: String }

#[derive(Serialize, Debug)]
struct AuthResponse { session_id: String, valid: bool }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   // 1. Leggiamo i file PEM montati nel container
    let ca_file     = PathBuf::from("/etc/certs/ca.cert.pem");
    let client_cert = PathBuf::from("/etc/certs/client.cert.pem");
    let client_key  = PathBuf::from("/etc/certs/client.key.pem");

    // 2. Costruiamo il builder delle opzioni TLS
    //    - require_tls(true) forza il TLS
    //    - add_root_certificates carica la CA per verificare il server
    //    - add_client_certificate carica il certificato + chiave per l’mTLS
    let options = ConnectOptions::new()
        .require_tls(true)
        .add_root_certificates(ca_file)
        .add_client_certificate(client_cert, client_key);

    // 3. Connettiamoci al broker in modalità TLS 
    // (l’URL può essere "tls://..." o "nats://..." a seconda della configurazione)
    let client = options.connect("tls://nats:4222").await?;

    // 4. Ora possiamo usare il client come al solito
    //    ma con la sicurezza TLS/mTLS abilitata
    println!("Server A connesso al broker NATS in modalità TLS (mTLS)");
 
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