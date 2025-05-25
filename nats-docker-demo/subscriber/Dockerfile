FROM rust:1.72-slim AS builder
WORKDIR /app
# Copia Cargo.toml e sorgenti
COPY Cargo.toml .
COPY src src
RUN cargo build --release

FROM debian:bullseye-slim
# Runtime
COPY --from=builder /app/target/release/subscriber /usr/local/bin/subscriber
CMD ["subscriber"]
