use std::{convert::TryFrom, str::FromStr};

use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use cron::Schedule;
use restate_sdk::{context::RequestTarget, prelude::*};
use rhai::packages::Package;
use rhai_chrono::ChronoPackage;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::TerminalExt;
use crate::utils::RequestExt;

#[restate_sdk::object]
#[name = "CronJob"]
pub trait Object {
    /// Create a new cron job.
    async fn create(job: Json<JobSpec>) -> HandlerResult<()>;
    /// Create a new or replace an existing cron job.
    async fn replace(job: Json<JobSpec>) -> HandlerResult<()>;
    /// Cancel an existing cron job.
    async fn cancel() -> HandlerResult<()>;
    /// Internal handler for running the cron job.
    async fn run() -> HandlerResult<()>;
    /// Get the details of an existing cron job.
    #[shared]
    async fn get() -> HandlerResult<Json<JobSpec>>;
    /// Get the next run time of an existing cron job.
    #[shared]
    #[name = "getNextRun"]
    async fn get_next_run() -> HandlerResult<Json<NextRun>>;
}

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

pub struct ObjectImpl {
    rhai_engine: rhai::Engine,
}

impl ObjectImpl {
    pub fn new(rhai_engine: rhai::Engine) -> Self {
        Self { rhai_engine }
    }

    async fn schedule_next(ctx: &ObjectContext<'_>, schedule: String) -> Result<()> {
        let (next_time, schedule) = ctx
            .run(async || {
                let next_time = parse_schedule(schedule)?
                    .upcoming(Utc)
                    .next()
                    .ok_or_else(|| anyhow!("No upcoming schedule found"))
                    .terminal_with_code(404)?;

                let duration = (next_time - Utc::now())
                    .to_std()
                    .map_err(|err| anyhow!("Failed to convert duration: {}", err))
                    .terminal_with_code(422)?;

                Ok(Json((next_time, duration)))
            })
            .await
            .map_err(|err| anyhow!("Error: {}", err))?
            .into_inner();

        let handle = ctx
            .object_client::<ObjectClient>(ctx.key())
            .run()
            .send_after(schedule);

        let next_run = NextRun {
            invocation_id: handle
                .invocation_id()
                .await
                .map_err(|err| anyhow!("Failed to get invocation ID: {}", err))?
                .to_string(),
            timestamp: next_time,
        };

        ctx.set::<Json<NextRun>>(NEXT_RUN, Json(next_run));

        Ok(())
    }

    async fn _create(&self, ctx: &ObjectContext<'_>, job: Json<JobSpec>) -> HandlerResult<()> {
        // Check if job already exists
        if ctx.get::<Json<JobSpec>>(JOB_SPEC).await?.is_some() {
            return Err(TerminalError::new_with_code(409, "Cron job already exists").into());
        }

        let job = job.into_inner();

        // Validate job specification and return early if invalid
        job.validate()?;

        ctx.set::<Json<JobSpec>>(JOB_SPEC, Json(job.clone()));

        ObjectImpl::schedule_next(ctx, job.schedule).await?;

        Ok(())
    }

    async fn _cancel(&self, ctx: &ObjectContext<'_>) -> HandlerResult<()> {
        // Get next run
        let next_run = ctx.get::<Json<NextRun>>(NEXT_RUN).await?;

        // Clear state
        ctx.clear_all();

        // Cancel the next scheduled invocation
        if let Some(next_run) = next_run.map(|s| s.into_inner()) {
            ctx.invocation_handle(next_run.invocation_id)
                .cancel()
                .await?;
        }

        Ok(())
    }
}

impl Default for ObjectImpl {
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

impl Object for ObjectImpl {
    async fn create(&self, ctx: ObjectContext<'_>, job: Json<JobSpec>) -> HandlerResult<()> {
        self._create(&ctx, job).await
    }

    async fn replace(&self, ctx: ObjectContext<'_>, job: Json<JobSpec>) -> HandlerResult<()> {
        self._cancel(&ctx).await?;
        self._create(&ctx, job).await
    }

    async fn cancel(&self, ctx: ObjectContext<'_>) -> HandlerResult<()> {
        self._cancel(&ctx).await
    }

    async fn run(&self, ctx: ObjectContext<'_>) -> HandlerResult<()> {
        let job = ctx.get::<Json<JobSpec>>(JOB_SPEC).await?;

        // Job is not scheduled, do nothing
        if job.is_none() {
            return Ok(());
        }

        let job = job.unwrap().into_inner();
        let target = job.target.clone();
        let content_type = "application/json".to_string();
        let idempotency_key = ctx.headers().get("x-restate-id").map(|v| v.to_string());

        if let Some(payload) = &job.payload {
            let data = match payload {
                Payload::Json { content: data } => Json(data.clone()),
                Payload::Rhai { content: script } => {
                    let result = self.rhai_engine.eval::<rhai::Dynamic>(script.as_str())?;

                    let value = rhai::serde::from_dynamic::<serde_json::Value>(&result)?;

                    Json(value)
                }
            };

            ctx.request::<_, ()>(target.into(), data)
                .header("Content-Type".to_string(), content_type)
                .idempotency_key_maybe(idempotency_key)
                .call()
                .await?;
        } else {
            ctx.request::<(), ()>(target.into(), ())
                .idempotency_key_maybe(idempotency_key)
                .call()
                .await?;
        }

        // Schedule the next invocation
        ObjectImpl::schedule_next(&ctx, job.schedule).await?;

        Ok(())
    }

    async fn get(&self, ctx: SharedObjectContext<'_>) -> HandlerResult<Json<JobSpec>> {
        ctx.get::<Json<JobSpec>>(JOB_SPEC)
            .await?
            .ok_or_else(|| TerminalError::new_with_code(404, "Cron job not found").into())
    }

    async fn get_next_run(&self, ctx: SharedObjectContext<'_>) -> HandlerResult<Json<NextRun>> {
        ctx.get::<Json<NextRun>>(NEXT_RUN)
            .await?
            .ok_or_else(|| TerminalError::new_with_code(404, "Cron job not found").into())
    }
}

fn parse_schedule(schedule: String) -> Result<Schedule, HandlerError> {
    Schedule::from_str(schedule.as_str())
        .map_err(|err| anyhow!("Failed to parse schedule: {}", err))
        .terminal_with_code(422)
}
