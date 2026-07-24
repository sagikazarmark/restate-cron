# restate-cron

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/sagikazarmark/restate-cron/dagger.yaml?style=flat-square)](https://github.com/sagikazarmark/restate-cron/actions/workflows/dagger.yaml)
[![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/sagikazarmark/restate-cron/badge?style=flat-square)](https://securityscorecards.dev/viewer/?uri=github.com/sagikazarmark/restate-cron)
[![crates.io](https://img.shields.io/crates/v/restate-cron?style=flat-square)](https://crates.io/crates/restate-cron)
[![docs.rs](https://img.shields.io/docsrs/restate-cron?style=flat-square)](https://docs.rs/restate-cron)

**Cron scheduling service for [Restate](https://restate.dev/).**

## Features

- **Standard cron expressions** with seconds precision
- **Multiple invocation targets** for services, objects, and workflows
- **Static or dynamic payloads** using JSON or [Rhai](https://rhai.rs/) scripts
- **Automatic rescheduling** after each execution
- **Durable execution** built on Restate

## Quick Start

Add the library to your application:

```toml
[dependencies]
restate-cron = "0.10"
```

See the [`restate-cron` Quick Start](crates/restate-cron/README.md#quick-start) for an endpoint setup example and the service API.

## Packages

| Package | Description |
|---------|-------------|
| [`restate-cron`](crates/restate-cron/) | Library for adding cron scheduling to Restate services |
| [`restate-cron-endpoint`](crates/restate-cron-endpoint/) | Ready-to-use endpoint hosting the cron service |

## Standalone Server

Run the server image and register its endpoint with Restate:

```bash
docker run -p 9080:9080 ghcr.io/sagikazarmark/restate-cron:latest
restate deployments register http://localhost:9080
```

See the [`restate-cron-endpoint` README](crates/restate-cron-endpoint/README.md) for configuration and deployment options.

## Development

Minimum verification:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`

Or run the same checks (fmt, clippy, test, doc, and build) in a container with [Dagger](https://dagger.io), exactly as CI does:

- `dagger check`

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
