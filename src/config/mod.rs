#[derive(Debug)]
pub struct Config {
    pub network: Network,
    pub status: Status,
}

#[derive(Debug)]
pub struct Network {
    pub ip: String,
    pub port: String,
    pub stage: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Status {
    pub is_enabled: Option<bool>,
    pub is_active: Option<bool>,
    pub is_suspended: Option<bool>,
}

impl Config {
    pub fn format_address(ip: &str, port: &str) -> String {
        format!("{ip}:{port}")
    }
}

//pub use config_redis::Redis;
pub use config_server::Server;
pub mod config_redis;
pub mod config_server;