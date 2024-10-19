use config::{Config, ConfigError, Environment, File};
use once_cell::sync::Lazy;
use serde_derive::Deserialize;
use std::env;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Server {
    pub base_url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub debug: bool,
    pub server: Server,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());

        let s = Config::builder()
            .add_source(File::with_name(&format!("settings/settings.{}.toml", run_mode)).required(true))
            .add_source(Environment::with_prefix("app"))
            .set_default("debug", false)?
            .build()?;

        println!("debug: {:?}", s.get_bool("debug"));
        println!("server: {:?}", s.get::<String>("server.base_url"));

        s.try_deserialize()
    }
}

pub static SETTINGS: Lazy<Arc<Settings>> = Lazy::new(|| {
    let settings = Settings::new().expect("Failed to initialize Settings");
    Arc::new(settings)
});
