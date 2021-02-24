use std::env;

use config::{Config, ConfigError, File};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub web: Web,
}

#[derive(Debug, Deserialize)]
pub struct Web {
    pub port: u16,
}

impl Settings {
    pub fn load() -> Result<Self, ConfigError> {
        let mut settings = Config::new();
        settings.merge(File::with_name("config/default.yaml"))?;

        let env = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        settings.merge(File::with_name(&format!("config/{}", env)).required(true))?;

        println!("debug: {:?}", settings.get_bool("debug"));
        println!("web: {:?}", settings.get::<Web>("web"));
        settings.try_into()
    }
}
