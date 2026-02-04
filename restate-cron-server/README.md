# restate-cron-server

A standalone server that provides cron scheduling for [Restate](https://restate.dev/) services.

## Installation

### Docker

Multi-platform images (amd64, arm64) are available on GitHub Container Registry:

```bash
docker pull ghcr.io/sagikazarmark/restate-cron:latest
```

### From source

```bash
cargo install --path .
```

## Usage

### Running the server

```bash
restate-cron --port 9080
```

### With configuration file

```bash
restate-cron --config config.toml --port 9080
```

### Docker

```bash
docker run -p 9080:9080 ghcr.io/sagikazarmark/restate-cron:latest
```

### Register with Restate

```bash
restate deployments register http://localhost:9080
```

## Configuration

### CLI arguments

```
--config <FILE>    Configuration file path
--port <PORT>      Listen port (default: 9080)
```

### Environment variables

| Variable | Description | Default |
|----------|-------------|---------|
| `CONFIG_FILE` | Path to configuration file | - |
| `PORT` | Server listen port | 9080 |
| `RUST_LOG` | Log level | info |

### Configuration file

Supports JSON, YAML, and TOML formats.

```toml
[restate.service]
inactivity_timeout = "5m"
abort_timeout = "10m"
idempotency_retention = "1h"
journal_retention = "24h"
enable_lazy_state = true
ingress_private = false

# Retry policy
retry_policy_initial_interval = "100ms"
retry_policy_exponentiation_factor = 2.0
retry_policy_max_interval = "30s"
retry_policy_max_attempts = 5
retry_policy_on_max_attempts = "pause"  # or "kill"

# Handler-specific overrides
[restate.service.handlers.run]
inactivity_timeout = "3m"
```

## License

Licensed under the [MIT License](../LICENSE).
