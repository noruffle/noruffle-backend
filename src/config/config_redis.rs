use super::Config;

pub trait Redis {
    fn redis_get_address() -> String;
    fn redis_create_client() -> std::sync::Arc<tokio::sync::Mutex<redis::Client>>;
}

impl Redis for Config {
    fn redis_get_address() -> String {
        let ip = dotenv::var("REDIS_IP")
            .expect("IP redis не указан в env");
        let port = dotenv::var("REDIS_PORT")
            .expect("PORT redis не указан в env");
        
        println!("Redis going on http://{ip}:{port}");
        
        Config::format_address(&ip, &port)
    }

    fn redis_create_client() -> std::sync::Arc<tokio::sync::Mutex<redis::Client>> {
        let redis_address = Config::redis_get_address();

        std::sync::Arc::new(
            tokio::sync::Mutex::new(
                redis::Client::open(redis_address)
                    .expect("Не удалось создать клиент redis")
            )
        )
    }
}