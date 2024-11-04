mod routes;
mod config;

#[allow(unused_imports)]
use config::{Config, Network, Server, Status};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let mut application = Config {
        network: Network {
            ip: 
                dotenv::var("SERVER_IP")
                    .expect("IP сервера не указан в env"),
            port: 
                dotenv::var("SERVER_PORT")
                    .expect("PORT сервера не указан в env"),
            stage: 
                dotenv::var("STAGE")
                    .expect("STAGE приложения не указан в env"),
        },
        status: Status { 
            is_enabled: None, 
            is_active: None, 
            is_suspended: None 
        }
    };

    println!("{:?}", application);

    application.server_create_app().await;
    
    
}