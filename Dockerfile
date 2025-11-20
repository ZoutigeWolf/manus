FROM rust:latest AS builder

WORKDIR /

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /

COPY --from=builder /target/release/manus .

EXPOSE 3069

CMD ["./manus"]