ARG SERVICE

FROM rust:1.84-alpine3.21 AS builder

ARG SERVICE
ENV PKG_CONFIG_ALLOW_CROSS=1 \
    OPENSSL_STATIC=1

RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    protobuf-dev

WORKDIR /usr/src/t_lounge
COPY Cargo.* ./

COPY libs/rs/convertions libs/rs/convertions/
COPY libs/rs/env-config libs/rs/env-config/
COPY libs/rs/postgres-helper libs/rs/postgres-helper/
COPY libs/rs/service-helper libs/rs/service-helper/
COPY protos protos/
COPY services/rs/gateway/Cargo.toml services/rs/gateway/Cargo.toml
COPY services/rs/admin/Cargo.toml services/rs/admin/Cargo.toml
COPY services/rs/client/Cargo.toml services/rs/client/Cargo.toml
COPY services/rs/notification/Cargo.toml services/rs/notification/Cargo.toml

RUN for crate in gateway admin client notification; do \
    mkdir -p services/rs/$crate/src; \
    echo 'fn main() {}' > services/rs/$crate/src/main.rs; \
    done
RUN cargo fetch --target "$(uname -m)-unknown-linux-musl"

COPY services/rs/$SERVICE services/rs/$SERVICE/
RUN cargo build --release -p $SERVICE --target "$(uname -m)-unknown-linux-musl"
RUN cp "target/$(uname -m)-unknown-linux-musl/release/$SERVICE" target/release/$SERVICE

FROM alpine:3.21

ARG SERVICE
ENV BIN=$SERVICE

RUN apk add --no-cache ca-certificates

WORKDIR /t_lounge

RUN adduser -DH t_lounge
USER t_lounge

COPY --from=builder /usr/src/t_lounge/target/release/$SERVICE $SERVICE

CMD /t_lounge/$BIN
