FROM rust as build

WORKDIR /usr/src/fluffy_rest_api
COPY . .

RUN cargo install --path .

FROM alpine:latest

COPY --from=build /usr/local/cargo/bin/fluffy_rest_api /usr/local/bin/fluffy_rest_api

CMD ["fluffy_rest_api"]
