FROM rust:1.61 AS builder

WORKDIR /leetcode-discord-bot
COPY . .
RUN cargo build --release

FROM ubuntu:20.04
COPY --from=builder /leetcode-discord-bot/target/release/leetcode-discord-bot leetcode-discord-bot
COPY --from=builder /leetcode-discord-bot/config.yaml config.yaml

RUN apt-get update && apt install -y openssl && apt install -y ca-certificates

ENTRYPOINT ["/leetcode-discord-bot"]
