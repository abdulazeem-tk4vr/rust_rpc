use serde::Deserialize;
use std::fs;
use std::env;

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

    pub fn load_from_env(config: &mut Config) {
        if let Ok(port_var) = env::var("PORT") {
            if let Ok(port) = port_var.parse::<u64>() {
                println!("Setting port from ENV: {}", port);
                config.port = port;
            }
        }

        if let Ok(host) = env::var("HOST") {
            config.host = host;
        }

        if let Ok(node_urls) = env::var("NODE_URLS") {
            config.node_urls = node_urls.split(',').map(|url| url.to_string()).collect();
        }

        if let Ok(request_timeout_var) = env::var("REQUEST_TIMEOUT") {
            if let Ok(request_timeout) = request_timeout_var.parse::<u64>() {
                config.request_timeout = request_timeout;
            }
        }

        if let Ok(verbose_var) = env::var("VERBOSE") {
            if let Ok(verbose) = verbose_var.parse::<bool>() {
                config.verbose = verbose;
            }
        }
    }
}
