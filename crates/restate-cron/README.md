# restate-cron

[![crates.io](https://img.shields.io/crates/v/restate-cron?style=flat-square)](https://crates.io/crates/restate-cron)
[![docs.rs](https://img.shields.io/docsrs/restate-cron?style=flat-square)](https://docs.rs/restate-cron)

**Cron scheduling service for [Restate](https://restate.dev/).**

`restate-cron` provides a `CronJob` virtual object that can be bound to a Restate endpoint.

## Install

```toml
[dependencies]
restate-cron = "0.10"
```

## Quick Start

Bind the cron service to your Restate endpoint:

```rust
use restate_cron::CronJob;
use restate_sdk::{endpoint::Endpoint, service::IntoServiceDefinition};

let endpoint = Endpoint::builder()
    .bind(CronJob::default().into_service_definition())
    .build();
```

### Custom Rhai Engine

Provide a custom Rhai engine to add functions for dynamic payloads:

```rust
use restate_cron::CronJob;

let mut engine = rhai::Engine::new();
// Register custom functions.

let service = CronJob::new(engine);
```

## Feature Flags

This crate does not define feature flags.

## API

The `CronJob` object exposes these handlers:

| Handler | Method | Description |
|---------|--------|-------------|
| `create` | POST | Create a new cron job |
| `replace` | POST | Create or replace an existing job |
| `cancel` | POST | Cancel an existing job |
| `get` | GET | Retrieve job details |
| `getNextRun` | GET | Get the next scheduled execution time |

Target invocations are sent without waiting for their result. The next run is scheduled immediately, so executions of the target may overlap.

## Job Specification

```json
{
  "schedule": "0 */5 * * * *",
  "target": { "type": "service", "name": "Greeter", "handler": "greet" },
  "payload": { "type": "json", "content": "World" }
}
```

Cron expressions include seconds:

```text
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

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](../../LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
