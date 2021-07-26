FROM rust:alpine as builder
RUN apk add musl-dev

COPY Cargo.toml /app/
COPY ./src /app/src/

WORKDIR /app
RUN cargo build --release
RUN strip /app/target/release/speicher-spieler

FROM scratch

COPY --from=builder /app/target/release/speicher-spieler /speicher-spieler
COPY ./templates/ /templates
COPY Rocket.toml /

EXPOSE 2555
ENTRYPOINT ["/speicher-spieler"]
