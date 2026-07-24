# restate-cron-endpoint

**Standalone endpoint hosting the cron scheduling service for [Restate](https://restate.dev/).**

`restate-cron-endpoint` hosts the `restate-cron` service over HTTP as the `restate-cron` binary.

## Install

Multi-platform images for amd64 and arm64 are available from GitHub Container Registry:

```bash
docker pull ghcr.io/sagikazarmark/restate-cron:latest
```

Build the binary from a repository checkout with:

```bash
cargo install --path crates/restate-cron-endpoint
```

## Quick Start

Run the server and register it with Restate:

```bash
restate-cron --port 9080
restate deployments register http://localhost:9080
```

Or run the published container image:

```bash
docker run -p 9080:9080 ghcr.io/sagikazarmark/restate-cron:latest
```

## Feature Flags

This crate does not define feature flags.

## Configuration

Pass configuration through CLI arguments:

```text
--config <FILE>    Configuration file path
--port <PORT>      Listen port (default: 9080)
```

The server reads these environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `CONFIG_FILE` | Path to configuration file | - |
| `PORT` | Server listen port | 9080 |
| `RUST_LOG` | Log level | info |

Configuration files may use JSON, YAML, or TOML:

```toml
[restate.service]
inactivity_timeout = "5m"
abort_timeout = "10m"
idempotency_retention = "1h"
journal_retention = "24h"
enable_lazy_state = true
ingress_private = false

retry_policy_initial_interval = "100ms"
retry_policy_exponentiation_factor = 2.0
retry_policy_max_interval = "30s"
retry_policy_max_attempts = 5
retry_policy_on_max_attempts = "pause"

[restate.service.handlers.run]
inactivity_timeout = "3m"
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](../../LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
