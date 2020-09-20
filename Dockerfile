# depend: docker pull rust:1.45.2-buster
FROM rust:1.45.2-buster as vendor

WORKDIR /build
COPY ./Cargo.toml .
COPY ./Cargo.lock .
RUN mkdir .cargo \
    && cargo vendor > .cargo/config

# depend: docker pull rust:1.45.2-buster
FROM rust:1.45.2-buster as build

COPY --from=vendor /build /build
WORKDIR /build
COPY ./sso sso
RUN cargo build --release

# depend: docker pull debian:10.5
FROM debian:10.5
ENV DEBIAN_FRONTEND="noninteractive"

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=build /build/target/release/sso_cli /usr/local/bin/sso_cli
COPY --from=build /build/target/release/sso_server /usr/local/bin/sso_server
RUN chmod +x /usr/local/bin/sso_cli \
    /usr/local/bin/sso_server

RUN mkdir -p /config
WORKDIR /config
ENTRYPOINT ["sso_server", "--config", "sso"]
