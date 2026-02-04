FROM --platform=$BUILDPLATFORM tonistiigi/xx:1.9.0@sha256:c64defb9ed5a91eacb37f96ccc3d4cd72521c4bd18d5442905b95e2226b0e707 AS xx

FROM --platform=$BUILDPLATFORM rust:1.93.0-slim@sha256:df6ca8f96d338697ccdbe3ccac57a85d2172e03a2429c2d243e74f3bb83ba2f5 AS builder

COPY --from=xx / /

RUN apt-get update && apt-get install -y clang lld

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY restate-cron-server/Cargo.toml ./restate-cron-server/
COPY restate-cron/Cargo.toml ./restate-cron/

RUN mkdir -p restate-cron-server/src && echo "fn main() {}" > restate-cron-server/src/main.rs
RUN mkdir -p restate-cron/src && echo "// dummy" > restate-cron/src/lib.rs

RUN cargo fetch --locked

ARG TARGETPLATFORM

RUN xx-apt-get update && \
    xx-apt-get install -y \
    gcc \
    g++ \
    libc6-dev \
    pkg-config

COPY . ./

ARG RESTATE_SERVICE_NAME

RUN xx-cargo build --release --bin restate-cron
RUN xx-verify ./target/$(xx-cargo --print-target-triple)/release/restate-cron
RUN cp -r ./target/$(xx-cargo --print-target-triple)/release/restate-cron /usr/local/bin/restate-cron


# FROM alpine:3.23.0@sha256:51183f2cfa6320055da30872f211093f9ff1d3cf06f39a0bdb212314c5dc7375
FROM debian:13.3-slim@sha256:77ba0164de17b88dd0bf6cdc8f65569e6e5fa6cd256562998b62553134a00ef0

COPY --from=builder /usr/local/bin/restate-cron /usr/local/bin/

ENV RUST_LOG=info

EXPOSE 9080

CMD ["restate-cron"]
