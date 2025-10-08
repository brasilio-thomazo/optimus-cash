FROM rust:slim as builder
RUN apt update && apt install -y pkg-config libssl-dev build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .
RUN cargo build --release -j $(nproc)

FROM debian:stable-slim
COPY --from=builder /app/target/release/api /usr/bin/api
ENV ARGON2_THREADS=1
ENV ARGON2_MEMORY=32768
ENV ARGON2_TIME=1
ENV ARGON2_LEN=32
EXPOSE 4000
CMD ["api", "server"]
