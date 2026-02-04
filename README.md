# Restate cron service

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/sagikazarmark/restate-cron/ci.yaml?style=flat-square)
![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/sagikazarmark/restate-cron/badge?style=flat-square)

A cron scheduling service for [Restate](https://restate.dev/) that enables scheduled invocations of services, objects, and workflows using standard cron expressions.

## Features

- Standard cron expression scheduling (with seconds precision)
- Multiple invocation targets: services, objects, and workflows
- Static JSON or dynamic [Rhai](https://rhai.rs/) script payloads
- Automatic rescheduling after each execution
- Built on Restate's durable execution model

## Packages

This repository contains two packages:

| Package | Description |
|---------|-------------|
| [restate-cron](restate-cron/) | Library for building Restate services with cron scheduling |
| [restate-cron-server](restate-cron-server/) | Ready-to-use server with the cron service |

### restate-cron

A Rust library that provides a `CronJob` object service you can integrate into your own Restate applications. Use this if you want to embed cron scheduling into an existing service or customize the server configuration.

See the [restate-cron README](restate-cron/README.md) for usage details.

### restate-cron-server

A standalone server that exposes the cron service over HTTP. Use this if you want a ready-to-deploy cron scheduling solution.

```bash
docker run -p 9080:9080 ghcr.io/sagikazarmark/restate-cron:latest
```

See the [restate-cron-server README](restate-cron-server/README.md) for configuration and deployment options.

## Development

### Prerequisites

- Rust 1.93+
- A running Restate server for testing

### Build

```bash
cargo build
```

### Test

```bash
cargo test
```

### Lint

```bash
cargo fmt --check
cargo clippy
```

## License

The project is licensed under the [MIT License](LICENSE).
