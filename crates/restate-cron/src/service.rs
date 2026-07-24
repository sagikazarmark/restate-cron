use std::{convert::TryFrom, str::FromStr};

use chrono::{DateTime, Utc};
use cron::Schedule;
use restate_sdk::{context::RequestTarget, prelude::*};
use rhai::packages::Package;
use rhai_chrono::ChronoPackage;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[schemars(example = example_cron_job())]
pub struct JobSpec {
    /// Cron schedule for the job (eg. "*/1 * * * * *").
    pub schedule: String,
    /// Target service to be called.
    pub target: ServiceType,
    /// Payload to be sent to the target service.
    pub payload: Option<Payload>,
}

fn example_cron_job() -> JobSpec {
    JobSpec {
        schedule: "0 */1 * * * *".to_string(),
        target: ServiceType::Service {
            name: "Greeter".to_string(),
            handler: "greet".to_string(),
        },
        payload: Some(Payload::Json {
            content: serde_json::json!("World"),
        }),
    }
}

impl JobSpec {
    fn validate(&self) -> HandlerResult<()> {
        parse_schedule(self.schedule.clone())?;

        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Payload {
    Json { content: serde_json::Value },
    Rhai { content: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ServiceType {
    Service {
        name: String,
        handler: String,
    },
    Object {
        name: String,
        key: String,
        handler: String,
    },
    Workflow {
        name: String,
        key: String,
        handler: String,
    },
}

impl From<ServiceType> for RequestTarget {
    fn from(val: ServiceType) -> Self {
        match val {
            ServiceType::Service { name, handler } => RequestTarget::Service { name, handler },
            ServiceType::Object { name, key, handler } => {
                RequestTarget::Object { name, key, handler }
            }
            ServiceType::Workflow { name, key, handler } => {
                RequestTarget::Workflow { name, key, handler }
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct NextRun {
    /// Invocation ID of the next run.
    invocation_id: String,
    /// Timestamp of the next run.
    timestamp: DateTime<Utc>,
}

pub struct CronJob {
    rhai_engine: rhai::Engine,
}

impl CronJob {
    pub fn new(rhai_engine: rhai::Engine) -> Self {
        Self { rhai_engine }
    }

    async fn schedule_next(ctx: &ObjectContext<'_>, schedule: String) -> HandlerResult<()> {
        let (next_time, schedule) = ctx
            .run(async || {
                let next_time =
                    parse_schedule(schedule)?
                        .upcoming(Utc)
                        .next()
                        .ok_or_else(|| {
                            TerminalError::new("No upcoming schedule found").with_code(404)
                        })?;

                let duration = (next_time - Utc::now()).to_std().map_err(|err| {
                    TerminalError::new(format!("Failed to convert duration: {}", err))
                        .with_code(422)
                })?;

                Ok(Json((next_time, duration)))
            })
            .await?
            .into_inner();

        let handle = ctx
            .object_client::<CronJobClient>(ctx.key())
            .run()
            .send_after(schedule)
            .await?;

        let next_run = NextRun {
            invocation_id: handle.invocation_id().to_string(),
            timestamp: next_time,
        };

        ctx.set::<Json<NextRun>>(NEXT_RUN, Json(next_run));

        Ok(())
    }

    async fn _create(&self, ctx: &ObjectContext<'_>, job: Json<JobSpec>) -> HandlerResult<()> {
        // Check if job already exists
        if ctx.get::<Json<JobSpec>>(JOB_SPEC).await?.is_some() {
            return Err(TerminalError::new("Cron job already exists")
                .with_code(409)
                .into());
        }

        let job = job.into_inner();

        // Validate job specification and return early if invalid
        job.validate()?;

        ctx.set::<Json<JobSpec>>(JOB_SPEC, Json(job.clone()));

        CronJob::schedule_next(ctx, job.schedule).await?;

        Ok(())
    }

    async fn _cancel(&self, ctx: &ObjectContext<'_>) -> HandlerResult<()> {
        // Get next run
        let next_run = ctx.get::<Json<NextRun>>(NEXT_RUN).await?;

        // Clear state
        ctx.clear_all();

        // Cancel the next scheduled invocation
        if let Some(next_run) = next_run.map(|s| s.into_inner()) {
            ctx.invocation_handle(next_run.invocation_id).cancel();
        }

        Ok(())
    }
}

impl Default for CronJob {
    fn default() -> Self {
        let mut engine = rhai::Engine::new();

        {
            let package = ChronoPackage::new();
            package.register_into_engine(&mut engine);
        }

        Self {
            rhai_engine: engine,
        }
    }
}

const JOB_SPEC: &str = "job_spec";
const NEXT_RUN: &str = "next_run";

#[restate_sdk::object(name = "CronJob")]
impl CronJob {
    /// Create a new cron job.
    #[handler]
    async fn create(&self, ctx: ObjectContext<'_>, job: Json<JobSpec>) -> HandlerResult<()> {
        self._create(&ctx, job).await
    }

    /// Create a new or replace an existing cron job.
    #[handler]
    async fn replace(&self, ctx: ObjectContext<'_>, job: Json<JobSpec>) -> HandlerResult<()> {
        self._cancel(&ctx).await?;
        self._create(&ctx, job).await
    }

    /// Cancel an existing cron job.
    #[handler]
    async fn cancel(&self, ctx: ObjectContext<'_>) -> HandlerResult<()> {
        self._cancel(&ctx).await
    }

    /// Internal handler for running the cron job.
    #[handler(ingress_private)]
    async fn run(&self, ctx: ObjectContext<'_>) -> HandlerResult<()> {
        let job = ctx.get::<Json<JobSpec>>(JOB_SPEC).await?;

        // Job is not scheduled, do nothing
        if job.is_none() {
            return Ok(());
        }

        let job = job.unwrap().into_inner();
        let target = job.target.clone();
        let content_type = "application/json".to_string();

        if let Some(payload) = &job.payload {
            let data = match payload {
                Payload::Json { content: data } => Json(data.clone()),
                Payload::Rhai { content: script } => {
                    ctx.run(async || {
                        let result = self
                            .rhai_engine
                            .eval::<rhai::Dynamic>(script.as_str())
                            .terminal()?;
                        let value =
                            rhai::serde::from_dynamic::<serde_json::Value>(&result).terminal()?;

                        Ok(Json(value))
                    })
                    .name("evaluate-rhai-payload")
                    .await?
                }
            };

            ctx.request::<_, ()>(target.into(), data)
                .header("Content-Type".to_string(), content_type)
                .idempotency_key(ctx.invocation_id())
                .send()
                .await?;
        } else {
            ctx.request::<(), ()>(target.into(), ())
                .idempotency_key(ctx.invocation_id())
                .send()
                .await?;
        }

        // Schedule the next invocation
        CronJob::schedule_next(&ctx, job.schedule).await?;

        Ok(())
    }

    /// Get the details of an existing cron job.
    #[handler]
    async fn get(&self, ctx: SharedObjectContext<'_>) -> HandlerResult<Json<JobSpec>> {
        ctx.get::<Json<JobSpec>>(JOB_SPEC).await?.ok_or_else(|| {
            TerminalError::new("Cron job not found")
                .with_code(404)
                .into()
        })
    }

    /// Get the next run time of an existing cron job.
    #[handler(name = "getNextRun")]
    async fn get_next_run(&self, ctx: SharedObjectContext<'_>) -> HandlerResult<Json<NextRun>> {
        ctx.get::<Json<NextRun>>(NEXT_RUN).await?.ok_or_else(|| {
            TerminalError::new("Cron job not found")
                .with_code(404)
                .into()
        })
    }
}

fn parse_schedule(schedule: String) -> HandlerResult<Schedule> {
    let schedule = Schedule::from_str(schedule.as_str()).map_err(|err| {
        TerminalError::new(format!("Failed to parse schedule: {}", err)).with_code(422)
    })?;

    Ok(schedule)
}

#[cfg(test)]
mod tests {
    use restate_sdk::{
        discovery::{HandlerType, ServiceType as RestateServiceType},
        service::Discoverable,
    };

    use super::CronJob;

    #[test]
    fn discovers_cron_job_api() {
        let service = <CronJob as Discoverable>::discover();

        assert_eq!(service.name.as_str(), "CronJob");
        assert_eq!(service.ty, RestateServiceType::VirtualObject);

        let mut handlers: Vec<_> = service
            .handlers
            .iter()
            .map(|handler| handler.name.as_str())
            .collect();
        handlers.sort_unstable();
        assert_eq!(
            handlers,
            ["cancel", "create", "get", "getNextRun", "replace", "run"]
        );

        let mut shared_handlers: Vec<_> = service
            .handlers
            .iter()
            .filter(|handler| handler.ty == Some(HandlerType::Shared))
            .map(|handler| handler.name.as_str())
            .collect();
        shared_handlers.sort_unstable();
        assert_eq!(shared_handlers, ["get", "getNextRun"]);

        let run = service
            .handlers
            .iter()
            .find(|handler| handler.name.as_str() == "run")
            .unwrap();
        assert_eq!(run.ingress_private, Some(true));
    }
}
