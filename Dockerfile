FROM rust:1.94.0-slim AS builder

WORKDIR /usr/src/

COPY . .

RUN cargo build --release

FROM debian:trixie-slim

WORKDIR /usr/app

COPY --from=builder /usr/src/config config
COPY --from=builder /usr/src/target/release/quotes_backend-cli quotes_backend-cli

ENTRYPOINT ["/usr/app/quotes_backend-cli"]
