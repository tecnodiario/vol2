FROM rust:1.72-slim AS builder
WORKDIR /app
COPY Cargo.toml .
COPY src src
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /app/target/release/publisher /usr/local/bin/publisher
CMD ["publisher"]
