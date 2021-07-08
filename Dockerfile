FROM rust:1.43.1 as build

RUN apt-get update
RUN apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/fluffy_rest_api
COPY . .

RUN RUSTFLAGS=-Clinker=musl-gcc cargo install -—release —target=x86_64-unknown-linux-musl

FROM alpine:latest

COPY --from=build /usr/local/cargo/bin/fluffy_rest_api /usr/local/bin/fluffy_rest_api

CMD ["fluffy_rest_api"]