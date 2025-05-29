# Esempio: Ambiente Docker con NATS, Publisher, Subscriber e CLI

Questo documento descrive come mettere in piedi, con Docker Compose, un ambiente minimale per sperimentare NATS con un **publisher** Rust, un **subscriber** Rust e un client interattivo (`natsio/nats-box`).

---

## 1. Prerequisiti

* Docker & Docker Compose installati
* Accesso a internet per scaricare le immagini

---

## 2. Struttura dei file

```text
nats-docker-demo-async/
├── docker-compose.yml
├── publisher/
│   ├── Dockerfile
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── subscriber/
    ├── Dockerfile
    ├── Cargo.toml
    └── src/
        └── main.rs
```



---

## 3. Comandi per build & run

1. **Build di tutti i servizi**

   ```bash
   docker-compose up --build -d
   ```

2. **Controlla i log**

   ```bash
   docker-compose logs -f subscriber
   docker-compose logs -f publisher
   ```

3. **Test manuale via CLI**
   Apri una shell `cli`:

   ```bash
   docker-compose run --rm cli
   ```

   e poi, dentro `ash`:

   ```bash
   # pubblica con timestamp corrente
   nats pub amici.cli '{"nome":"Bob","ts":"2025-05-28T16:55:00Z"}'

   # inizia a sottoscrivere su amici.cli
   nats sub amici.cli
   ```

4. **Stop & Cleanup**

   ```bash
   docker-compose down --volumes --remove-orphans
   ```

---

Con questo setup hai un **end-to-end** che mostra:

* **Publisher Rust** che allega un campo `ts` (timestamp ISO8601)
* **Subscriber Rust** che calcola il ritardo tra invio e ricezione
* **CLI interattiva** (`natsio/nats-box`) per provare comandi manuali
* **Docker Compose** per orchestrare tutto con un solo comando

Buona sperimentazione con NATS in Docker!
