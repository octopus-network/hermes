FROM rust:1-buster AS build-env

ARG TAG
WORKDIR /root
COPY . /root

RUN cargo build --release -p ibc-relayer-cli

FROM debian:buster-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates
RUN update-ca-certificates
RUN useradd -m hermes -s /bin/bash
WORKDIR /home/hermes
USER hermes:hermes

COPY --chown=0:0 --from=build-env /root/target/release/hermes /usr/bin/hermes
COPY --chown=0:0 --from=build-env /root/config.toml /home/hermes/.hermes/config.toml

VOLUME ["/home/hermes/.hermes/keys"]
ENTRYPOINT ["/usr/bin/hermes"]
