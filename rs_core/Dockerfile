FROM rust:1.83 AS builder

WORKDIR /usr/src/rs_core

COPY . .

RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && apt-get install -y \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /usr/src/rs_core/target/release/rs_core .

EXPOSE 8080

CMD ["./rs_core"]
