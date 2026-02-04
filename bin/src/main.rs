mod config;
mod restate_config;

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use figment::{
    Figment,
    providers::{Env, Format, Json, Toml, Yaml},
};
use restate_sdk::{endpoint::Endpoint, http_server::HttpServer};

use restate_cron::*;

use crate::config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    tracing_subscriber::fmt::init();

    let config = cli.load_config()?;

    let mut endpoint = Endpoint::builder();

    endpoint =
        endpoint.bind_with_options(ObjectImpl::default().serve(), config.restate.service.into());

    let bind_addr = format!("0.0.0.0:{}", cli.port);

    // Create and start the HTTP server
    HttpServer::new(endpoint.build())
        .listen_and_serve(bind_addr.parse()?)
        .await;

    Ok(())
}

#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    /// Path to config file (supports JSON, YAML, or TOML)
    #[arg(long, value_name = "FILE", env = "CONFIG_FILE")]
    config: Option<PathBuf>,

    /// Port to listen on
    #[arg(long, default_value = "9080", env = "PORT")]
    port: u16,
}

impl Cli {
    fn load_config(&self) -> Result<Config> {
        let mut figment = Figment::new();

        if let Some(path) = self.config.as_deref() {
            if !path.exists() {
                anyhow::bail!("Config file not found: {}", path.display());
            }

            figment = match path.extension().and_then(|s| s.to_str()) {
                Some("toml") => figment.merge(Toml::file(path)),
                Some("json") => figment.merge(Json::file(path)),
                Some("yaml") | Some("yml") => figment.merge(Yaml::file(path)),
                _ => anyhow::bail!(
                    "Unsupported config file format. Use .toml, .json, .yaml, or .yml"
                ),
            };
        }

        figment = figment.merge(Env::raw().split("__")).merge(
            Env::prefixed("OPENDAL_")
                .filter(|k| k.starts_with("profile_"))
                .map(move |key| key.as_str().replacen("_", ".", 2).into()),
            // .split("_"),
        );

        figment.extract().context("Failed to parse configuration")
    }
}
