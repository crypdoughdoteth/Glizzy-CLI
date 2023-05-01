FROM rust:latest as builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

FROM debian:buster-slim as release

RUN apt-get update && apt-get install  && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /usr/src/app/target/release/glizzy .

RUN chmod +x /app/glizzy

CMD ["./glizzy"]