FROM rust as build

WORKDIR /usr/src/api-service
COPY . .

RUN cargo install --path .

FROM alpine:latest

COPY --from=build /usr/local/cargo/bin/fluffy_rest_api /usr/local/bin/api-service

CMD ["api-service"]
