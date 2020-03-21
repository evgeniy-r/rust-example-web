use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use toml;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_host")]
    pub host: String,
    #[serde(default = "Config::default_port")]
    pub port: u16,
    #[serde(default = "Config::default_root")]
    pub root: String,
    #[serde(default = "Config::default_read_timeout")]
    pub read_timeout: u64,
    #[serde(default = "Config::default_worker_number")]
    pub worker_number: usize,
}

impl Config {
    fn default_host() -> String {
        "127.0.0.1".to_string()
    }

    fn default_port() -> u16 {
        80
    }

    fn default_root() -> String {
        "public".to_string()
    }

    fn default_read_timeout() -> u64 {
        5
    }

    fn default_worker_number() -> usize {
        4
    }

    pub fn from_file(filename: &str) -> Config {
        println!("Loading a configuration from {}...", filename);
        let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
            println!("The configuration file not found, a default configuration was used.");
            "".to_string()
        });
        toml::from_str(&file_contents).unwrap()
    }

    pub fn address(&self) -> String {
        format!("{}:{}", &self.host, &self.port)
    }

    pub fn parsed_root(&self) -> PathBuf {
        fs::canonicalize(&self.root).unwrap()
    }
}
