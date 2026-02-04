use std::collections::HashMap;
use std::time::Duration;

use restate_sdk::prelude::{HandlerOptions, ServiceOptions};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct ServiceOptionsConfig {
    #[serde(default)]
    pub metadata: HashMap<String, String>,

    #[serde(default, with = "humantime_serde")]
    pub inactivity_timeout: Option<Duration>,

    #[serde(default, with = "humantime_serde")]
    pub abort_timeout: Option<Duration>,

    #[serde(default, with = "humantime_serde")]
    pub idempotency_retention: Option<Duration>,

    #[serde(default, with = "humantime_serde")]
    pub journal_retention: Option<Duration>,

    #[serde(default)]
    pub enable_lazy_state: Option<bool>,

    #[serde(default)]
    pub ingress_private: Option<bool>,

    #[serde(default, with = "humantime_serde")]
    pub retry_policy_initial_interval: Option<Duration>,

    #[serde(default)]
    pub retry_policy_exponentiation_factor: Option<f64>,

    #[serde(default, with = "humantime_serde")]
    pub retry_policy_max_interval: Option<Duration>,

    #[serde(default)]
    pub retry_policy_max_attempts: Option<u64>,

    #[serde(default)]
    pub retry_policy_on_max_attempts: Option<RetryPolicyOnMaxAttemptsConfig>,

    #[serde(default)]
    pub handlers: HashMap<String, HandlerOptionsConfig>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct HandlerOptionsConfig {
    #[serde(default)]
    pub metadata: HashMap<String, String>,

    #[serde(default, with = "humantime_serde")]
    pub inactivity_timeout: Option<Duration>,

    #[serde(default, with = "humantime_serde")]
    pub abort_timeout: Option<Duration>,

    #[serde(default, with = "humantime_serde")]
    pub idempotency_retention: Option<Duration>,

    #[serde(default, with = "humantime_serde")]
    pub workflow_retention: Option<Duration>,

    #[serde(default, with = "humantime_serde")]
    pub journal_retention: Option<Duration>,

    #[serde(default)]
    pub ingress_private: Option<bool>,

    #[serde(default)]
    pub enable_lazy_state: Option<bool>,

    #[serde(default, with = "humantime_serde")]
    pub retry_policy_initial_interval: Option<Duration>,

    #[serde(default)]
    pub retry_policy_exponentiation_factor: Option<f64>,

    #[serde(default, with = "humantime_serde")]
    pub retry_policy_max_interval: Option<Duration>,

    #[serde(default)]
    pub retry_policy_max_attempts: Option<u64>,

    #[serde(default)]
    pub retry_policy_on_max_attempts: Option<RetryPolicyOnMaxAttemptsConfig>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum RetryPolicyOnMaxAttemptsConfig {
    Pause,
    Kill,
}

// Conversion implementations
impl From<ServiceOptionsConfig> for ServiceOptions {
    fn from(config: ServiceOptionsConfig) -> Self {
        let mut opts = ServiceOptions::new();

        for (key, value) in config.metadata {
            opts = opts.metadata(key, value);
        }

        if let Some(timeout) = config.inactivity_timeout {
            opts = opts.inactivity_timeout(timeout);
        }

        if let Some(timeout) = config.abort_timeout {
            opts = opts.abort_timeout(timeout);
        }

        if let Some(retention) = config.idempotency_retention {
            opts = opts.idempotency_retention(retention);
        }

        if let Some(retention) = config.journal_retention {
            opts = opts.journal_retention(retention);
        }

        if let Some(enable) = config.enable_lazy_state {
            opts = opts.enable_lazy_state(enable);
        }

        if let Some(private) = config.ingress_private {
            opts = opts.ingress_private(private);
        }

        if let Some(interval) = config.retry_policy_initial_interval {
            opts = opts.retry_policy_initial_interval(interval);
        }

        if let Some(factor) = config.retry_policy_exponentiation_factor {
            opts = opts.retry_policy_exponentiation_factor(factor);
        }

        if let Some(interval) = config.retry_policy_max_interval {
            opts = opts.retry_policy_max_interval(interval);
        }

        if let Some(attempts) = config.retry_policy_max_attempts {
            opts = opts.retry_policy_max_attempts(attempts);
        }

        if let Some(on_max) = config.retry_policy_on_max_attempts {
            opts = match on_max {
                RetryPolicyOnMaxAttemptsConfig::Pause => opts.retry_policy_pause_on_max_attempts(),
                RetryPolicyOnMaxAttemptsConfig::Kill => opts.retry_policy_kill_on_max_attempts(),
            };
        }

        for (handler_name, handler_config) in config.handlers {
            opts = opts.handler(handler_name, handler_config.into());
        }

        opts
    }
}

impl From<HandlerOptionsConfig> for HandlerOptions {
    fn from(config: HandlerOptionsConfig) -> Self {
        let mut opts = HandlerOptions::new();

        for (key, value) in config.metadata {
            opts = opts.metadata(key, value);
        }

        if let Some(timeout) = config.inactivity_timeout {
            opts = opts.inactivity_timeout(timeout);
        }

        if let Some(timeout) = config.abort_timeout {
            opts = opts.abort_timeout(timeout);
        }

        if let Some(retention) = config.idempotency_retention {
            opts = opts.idempotency_retention(retention);
        }

        if let Some(retention) = config.workflow_retention {
            opts = opts.workflow_retention(retention);
        }

        if let Some(retention) = config.journal_retention {
            opts = opts.journal_retention(retention);
        }

        if let Some(private) = config.ingress_private {
            opts = opts.ingress_private(private);
        }

        if let Some(enable) = config.enable_lazy_state {
            opts = opts.enable_lazy_state(enable);
        }

        if let Some(interval) = config.retry_policy_initial_interval {
            opts = opts.retry_policy_initial_interval(interval);
        }

        if let Some(factor) = config.retry_policy_exponentiation_factor {
            opts = opts.retry_policy_exponentiation_factor(factor);
        }

        if let Some(interval) = config.retry_policy_max_interval {
            opts = opts.retry_policy_max_interval(interval);
        }

        if let Some(attempts) = config.retry_policy_max_attempts {
            opts = opts.retry_policy_max_attempts(attempts);
        }

        if let Some(on_max) = config.retry_policy_on_max_attempts {
            opts = match on_max {
                RetryPolicyOnMaxAttemptsConfig::Pause => opts.retry_policy_pause_on_max_attempts(),
                RetryPolicyOnMaxAttemptsConfig::Kill => opts.retry_policy_kill_on_max_attempts(),
            };
        }

        opts
    }
}
