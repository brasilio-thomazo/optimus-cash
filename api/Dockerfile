FROM rust:slim as builder
RUN apt update && apt install -y pkg-config libssl-dev build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:stable-slim
WORKDIR /app
COPY --from=builder /app/target/release/api-rust .
EXPOSE 4000
CMD ["./api-rust"]
