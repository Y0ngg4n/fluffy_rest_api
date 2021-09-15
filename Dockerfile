FROM rust as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/fluffy_rest_api
COPY . .

RUN cargo install --path .

FROM debian

COPY --from=build /usr/local/cargo/bin/fluffy_rest_api /usr/local/bin/fluffy_rest_api

CMD ["fluffy_rest_api"]