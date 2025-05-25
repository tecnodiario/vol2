FROM rust:1.86-slim AS builder
WORKDIR /app
# Copia Cargo.toml e sorgenti
COPY Cargo.toml .
COPY src src
RUN cargo build --release

FROM debian:bullseye-slim
# Runtime
RUN apt-get update && apt-get upgrade -y && apt-get clean
# Copia l'eseguibile dal builder    
COPY --from=builder /app/target/release/subscriber /usr/local/bin/subscriber
CMD ["subscriber"]
