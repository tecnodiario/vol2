// Esempio di client NATS con TLS/mTLS in Rust
use async_nats::{ConnectOptions};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::time::{sleep, Duration};
use std::env;
use std::path::PathBuf;

#[derive(Serialize)]
struct AuthRequest { user_id: String, token: String }

#[derive(Deserialize, Debug)]
struct AuthResponse { session_id: String, valid: bool }

// Funzione per ottenere il percorso del file di certificato
fn cert_file(name: &str) -> PathBuf {
    let base = env::var("TLS_CERT_DIR")
        .unwrap_or_else(|_| "/etc/worker/certs".into());
    PathBuf::from(base).join(name)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Leggiamo i file PEM montati nel container
    let ca_file     = PathBuf::from(cert_file("ca.cert.pem"));
    let client_cert = PathBuf::from(cert_file("client.cert.pem"));
    let client_key  = PathBuf::from(cert_file("client.key.pem"));

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
    println!("Connesso al broker NATS in modalità TLS (mTLS)");


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