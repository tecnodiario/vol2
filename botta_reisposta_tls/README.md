# Esempio: Botta e Risposta con NATS - versione icura TLS(mTLS)

In questo esempio creiamo due microservizi Rust—**Server A** (requester) e **Server B** (responder)—che comunicano via NATS usando il pattern **Request/Reply**.  
Server A invia a Server B un payload contenente `user_id` e `token`; Server B verifica il token e risponde con un `session_id` valido.

Con questo esempio “botta e risposta” hai un proof-of-concept di Request/Reply in NATS, dove Server A delega a Server B la validazione di un token e riceve in risposta un codice di sessione.

---

1. **Build di tutti i servizi**

   ```bash
   docker-compose up --build -d
   ```

2. **Controlla i log**

   ```bash
   docker-compose logs -f nats

   docker-compose logs -f server_a
   docker-compose logs -f server_b
   ```

3. **Clean up**
    '''bash
    docker-compose run --rm cli nats sub auth.request -s tls://nats:4222
    '''

## Riepilogo

1. ***CA***

Genera ca.key.pem (chiave privata)

Genera ca.cert.pem (certificato pubblico, self-signed)

1. ***Server (broker NATS)***

Genera server.key.pem (chiave privata)

Crea server.csr.pem (richiesta)

Firma la CSR: ottieni server.cert.pem

1. ***Client (server_a / server_b)***

Genera client.key.pem (chiave privata)

Crea client.csr.pem (richiesta)

Firma la CSR: ottieni client.cert.pem

1. ***Docker Compose***

Monta i certificati nel container nats (broker) e in server_a/server_b (client)

Avvia nats-server con --tls … --tlscert … --tlskey … --tlscacert … --tlscertrequired true

1. ***Codice Rust***

Carica client.cert.pem, client.key.pem e ca.cert.pem con tokio_rustls

Ora la comunicazione è cifrata (TLS) e autenticata da entrambi i lati (mTLS).

Seguendo questi passaggi otterrai un’infrastruttura in grado di scambiare messaggi NATS tra server_a e server_b in modo completamente sicuro: il broker accetta solo connessioni TLS e richieste di certificato client, e i client presentano a loro volta il loro certificato firmato dalla CA, creando un canale di comunicazione fidato e a prova di man-in-the-middle.
