use config::{Config, Environment, File};
use serde::Deserialize;
use std::env;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::OnceLock;

use crate::error::AppResult;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub distribution: DistributionConfig,
    pub server: ServerConfig,
    pub cache: CacheConfig,
    pub broker: BrokerConfig,
    pub database: DatabaseConfig,
    pub tracing: TracingConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DistributionConfig {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub address: SocketAddr,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind")]
pub enum CacheConfig {
    Memory,
    Redis(RedisCacheConfig),
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedisCacheConfig {
    pub address: String,
    pub key_prefix: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind")]
pub enum BrokerConfig {
    Memory,
    Nats(NatsBrokerConfig),
}

#[derive(Debug, Clone, Deserialize)]
pub struct NatsBrokerConfig {
    pub address: String,
    pub user: String,
    pub password: String,
    pub subject_prefix: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind")]
pub enum DatabaseConfig {
    Memory,
    Postgres(PostgresDatabaseConfig),
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostgresDatabaseConfig {
    pub connection: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind")]
pub enum TracingConfig {
    Memory,
    Jaeger(JaegerTracingConfig),
}

#[derive(Debug, Clone, Deserialize)]
pub struct JaegerTracingConfig {
    pub address: String,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

const CONFIG_PATH_ENV: &str = "APP_CONFIG_PATH";
const CONFIG_DEFAULT: &str = "config";
const ENV_PREFIX: &str = "APP";

const DISTRIBUTION_VERSION_KEY: &str = "distribution.version";

impl AppConfig {
    pub fn get() -> &'static Self {
        static INSTANCE: OnceLock<AppConfig> = OnceLock::new();
        INSTANCE.get_or_init(|| Self::load().unwrap())
    }

    fn load() -> AppResult<Self> {
        let mut config_builder =
            Config::builder().set_default(DISTRIBUTION_VERSION_KEY, VERSION)?;

        // Initial "default" configuration file
        let default_path = Path::new(CONFIG_DEFAULT).join("default");
        config_builder = config_builder.add_source(File::with_name(default_path.to_str().unwrap()));

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        let local_path = Path::new(CONFIG_DEFAULT).join("local");
        config_builder = config_builder
            .add_source(File::with_name(local_path.to_str().unwrap()).required(false));

        // Add override settings file.
        let override_path = env::var(CONFIG_PATH_ENV).ok();
        if let Some(override_path) = override_path {
            config_builder =
                config_builder.add_source(File::with_name(&override_path).required(false));
        }

        // Add in settings from the environment (with a prefix of APP)
        config_builder =
            config_builder.add_source(Environment::with_prefix(ENV_PREFIX).separator("__"));

        // Set derived properties
        // let config = config_builder.build()?;
        // let mut config_builder = Config::builder();

        let config = config_builder
            // .add_source(config)
            .build()?
            .try_deserialize()?;
        Ok(config)
    }
}
