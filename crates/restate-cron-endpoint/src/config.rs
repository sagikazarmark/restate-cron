use crate::restate_config::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Config {
    #[serde(default)]
    pub restate: RestateConfig,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct RestateConfig {
    #[serde(default)]
    pub service: ServiceOptionsConfig,
}
