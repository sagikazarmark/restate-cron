# restate-cron

A Rust library for adding cron scheduling capabilities to [Restate](https://restate.dev/) services.

## Installation

```bash
cargo add restate-cron
```

## Usage

The library provides a `CronJob` object service that you can add to your Restate endpoint:

```rust
use restate_cron::ObjectImpl;
use restate_sdk::prelude::*;

#[tokio::main]
async fn main() {
    HttpServer::new(Endpoint::builder().bind(ObjectImpl::default().serve()).build())
        .listen_and_serve("0.0.0.0:9080".parse().unwrap())
        .await;
}
```

### Custom Rhai engine

You can provide a custom Rhai engine with additional functions for dynamic payloads:

```rust
use restate_cron::ObjectImpl;

let mut engine = rhai::Engine::new();
// Register custom functions...

let service = ObjectImpl::new(engine);
```

## API

The `CronJob` object exposes these handlers:

| Handler | Method | Description |
|---------|--------|-------------|
| `create` | POST | Create a new cron job |
| `replace` | POST | Create or replace an existing job |
| `cancel` | POST | Cancel an existing job |
| `get` | GET | Retrieve job details |
| `getNextRun` | GET | Get next scheduled execution time |

## Job specification

```json
{
  "schedule": "0 */5 * * * *",
  "target": { ... },
  "payload": { ... }
}
```

### Schedule

Standard cron expression with seconds precision:

```
┌──────────── second (0-59)
│ ┌────────── minute (0-59)
│ │ ┌──────── hour (0-23)
│ │ │ ┌────── day of month (1-31)
│ │ │ │ ┌──── month (1-12)
│ │ │ │ │ ┌── day of week (0-6, Sunday=0)
│ │ │ │ │ │
* * * * * *
```

## License

Licensed under the [MIT License](../LICENSE).
