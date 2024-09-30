pub struct Config;

impl Config {
    pub fn format_address(ip: &str, port: &str) -> String {
        format!("{ip}:{port}")
    }
}
pub use config_redis::Redis;
pub use config_server::Server;
pub mod config_redis;
pub mod config_server;