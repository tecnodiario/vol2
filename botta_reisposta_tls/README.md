# Esempio: Botta e Risposta con NATS - versione base

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
   docker-compose logs -f server_a
   docker-compose logs -f server_b
   ```

3. **Clean up
    '''bash
    docker-compose down --volumes --remove-orphans
    '''