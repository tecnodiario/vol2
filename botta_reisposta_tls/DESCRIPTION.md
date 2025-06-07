```rust
// Import delle opzioni di connessione per NATS
use async_nats::ConnectOptions;
``` 
Qui stiamo caricando la struttura principale che ci permetterà di configurare il client NATS con il supporto TLS e mTLS: `ConnectOptions` è il builder che aggiunge certificati e regole di sicurezza prima di stabilire la connessione.

```rust
// Registrazione del provider crittografico AWS-LC per Rustls
aws_lc_rs::default_provider()
    .install_default()
    .expect("failed to install rustls crypto provider");
``` 
Questa chiamata iniziale è fondamentale: senza di essa Rustls non sa quale motore crittografico usare per cifrare e decifrare i dati. Con `install_default()` indichiamo di usare AWS-LC, fornendo un backend performante e supportato.

```rust
// Calcolo dinamico dei percorsi ai file PEM
go cert_file = cert_file("client.cert.pem");
``` 
La funzione `cert_file` legge la directory base dei certificati da `TLS_CERT_DIR`, cadenzando un percorso flessibile che si adatta all’ambiente. In questo modo, il codice non ha percorsi hardcoded ma costruisce i PathBuf in base a una singola fonte di verità.

```rust
// Configurazione delle opzioni di connessione con TLS/mTLS\let options = ConnectOptions::new()
    .require_tls(true)
    .add_root_certificates(ca_file)
    .add_client_certificate(client_cert, client_key);
``` 
Con questa catena di metodi stiamo forzando l’uso del canale cifrato, importando il certificato della CA per verificare il broker e il certificato+chiave del client per permettere l’autenticazione reciproca. Ogni chiamata restituisce lo stesso builder, arricchito passo per passo.

```rust
// Connessione reale al broker in modalità TLS\ ddar client = options.connect("tls://nats:4222").await?;
``` 
È il momento di stabilire il collegamento. Passiamo l’URL del broker con `tls://` per garantire l’avvio di un handshake sicuro. Se il certificato del broker non corrisponde alle aspettative (firma valida e SAN giusto), la connessione fallisce.

```rust
// Esempio di richiesta/risposta con JSON
let req = AuthRequest { user_id: "user123".into(), token: "valid".into() };
let reply = client.request("auth.request", json!(req).to_string().into()).await?;
``` 
Qui inviamo una `AuthRequest` serializzata in JSON, usando il pattern Request/Reply di NATS. Il metodo `request` attende in modo asincrono la risposta dal servizio B, che deve processare e ritornare un `AuthResponse`.

```rust
// Deserializzazione e log dell’esito
let resp: AuthResponse = serde_json::from_slice(&reply.payload)?;
println!("Autenticazione validità: {}", resp.valid);
``` 
Alla ricezione, trasformiamo il payload JSON in una `AuthResponse`. Il campo `valid` ci indica se il token era corretto, permettendoci di proseguire con logica di business differente in base all’esito.

```rust
// Pausa finale per permettere il logging e la chiusura ordinata
sleep(Duration::from_secs(1)).await;
``` 
Per concludere, introduciamo un breve ritardo che lascia il tempo alle operazioni in background — come la conferma di pubblicazione — di completarsi prima di terminare il processo, evitando uscite premature.
