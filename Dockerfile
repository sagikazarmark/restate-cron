FROM --platform=$BUILDPLATFORM tonistiigi/xx:1.9.0@sha256:c64defb9ed5a91eacb37f96ccc3d4cd72521c4bd18d5442905b95e2226b0e707 AS xx

FROM --platform=$BUILDPLATFORM rust:1.92.0-slim@sha256:6cff8a33b03d328aa58d00dedda6a3c5bbee4b41e21533932bffd90d7d58f9c4 AS builder

COPY --from=xx / /

RUN apt-get update && apt-get install -y clang lld

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY bin/Cargo.toml ./bin/
COPY lib/Cargo.toml ./lib/

RUN mkdir -p bin/src && echo "fn main() {}" > bin/src/main.rs
RUN mkdir -p lib/src && echo "// dummy" > lib/src/lib.rs

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
FROM debian:13.2-slim@sha256:4bcb9db66237237d03b55b969271728dd3d955eaaa254b9db8a3db94550b1885

COPY --from=builder /usr/local/bin/restate-cron /usr/local/bin/

ENV RUST_LOG=info

EXPOSE 9080

CMD ["restate-cron"]
