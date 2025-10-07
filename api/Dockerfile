FROM rust:slim as builder
RUN apt update && apt install -y pkg-config libssl-dev build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .
RUN cargo build --release -j $(nproc)

FROM debian:stable-slim
COPY --from=builder /app/target/release/api-rust /usr/bin/api
EXPOSE 4000
CMD ["api", "server"]
