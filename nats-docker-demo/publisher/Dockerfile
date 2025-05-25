FROM rust:1.86-slim AS builder
WORKDIR /app
COPY Cargo.toml .
COPY src src
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get upgrade -y && apt-get clean
COPY --from=builder /app/target/release/publisher /usr/local/bin/publisher
CMD ["publisher"]
