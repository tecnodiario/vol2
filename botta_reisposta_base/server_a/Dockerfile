FROM rust:latest AS builder
WORKDIR /app
COPY Cargo.toml .
COPY src src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update \
	&& apt-get upgrade -y \
	&& apt-get install -y --no-install-recommends ca-certificates \
	&& rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/server_a /usr/local/bin/server_a
CMD ["server_a"]