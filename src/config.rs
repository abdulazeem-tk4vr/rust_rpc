use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub port: u64,
    pub host: String,
    pub node_urls: Vec<String>,
    pub request_timeout: u64,
    pub verbose: bool,
}

impl Config {
    pub fn default() -> Self {
        Self {
            port: 8080,
            host: "localhost".to_string(),
            node_urls: vec!["http://localhost:3000".to_string()],
            request_timeout: 30, // seconds
            verbose: false,
        }
    }

    pub fn load() -> Result<Config, String> {
        let filepath = "src/config.toml".to_string(); // Path to your config file
        let content = fs::read_to_string(&filepath)
            .map_err(|err| format!("Failed to read config file: {}", err))?;
        let parsed_config: Config = toml::from_str(&content)
            .map_err(|err| format!("\nFailed to parse config file: {}", err))?;

        Ok(parsed_config)
    }
}
