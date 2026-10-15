# syntax=docker/dockerfile:1
# check=skip=CopyIgnoredFile

FROM --platform=$BUILDPLATFORM tonistiigi/xx:1.10.0@sha256:40fe21f44bb8cddf1eb1226db190d0a984118f62ee02a91b77d99448d28264a6 AS xx

FROM --platform=$BUILDPLATFORM rust:1.99.0-bookworm@sha256:114c7a4425406451c2866b6aafe69fe29b1b298832db1277d411ac73c82d04d6 AS base

ARG CARGO_CHEF_VERSION=0.1.77
RUN cargo install cargo-chef --version $CARGO_CHEF_VERSION --locked

COPY --from=xx / /

WORKDIR /usr/src/app


FROM base AS deps

COPY . .

RUN cargo chef prepare --recipe-path recipe.json


FROM base AS builder

RUN apt-get update && \
    apt-get install -y --no-install-recommends clang lld && \
    rm -rf /var/lib/apt/lists/*

ARG TARGETPLATFORM

RUN xx-apt-get update && \
    xx-apt-get install -y --no-install-recommends gcc libc6-dev && \
    rm -rf /var/lib/apt/lists/*

COPY --from=deps /usr/src/app/recipe.json recipe.json

RUN xx-cargo chef cook --locked --release --recipe-path recipe.json

COPY . .

RUN xx-cargo build --locked --release --bin restate-cron
RUN xx-verify ./target/$(xx-cargo --print-target-triple)/release/restate-cron
RUN cp ./target/$(xx-cargo --print-target-triple)/release/restate-cron /usr/local/bin/restate-cron


FROM debian:13.7-slim@sha256:a29215f6a35e51e22adffa17f89e9d2ef06214e64a2bad10d765c46aea49f11f

COPY --from=builder /usr/local/bin/restate-cron /usr/local/bin/

ENV RUST_LOG=info

EXPOSE 9080

USER 65532:65532

CMD ["restate-cron"]
