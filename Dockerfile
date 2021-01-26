# Builder Image
FROM rust:alpine as builder
RUN apk add openssl-dev musl-dev
RUN mkdir /src
WORKDIR /src
COPY . .
RUN cargo build --release

# Run
FROM alpine:latest
RUN mkdir /app
COPY --from=builder /src/target/release/bujang-bujang-bot /app/bujang-bot
WORKDIR /app
CMD ["./bujang-bot"]
