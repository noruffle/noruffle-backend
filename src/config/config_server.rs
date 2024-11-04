use crate::routes::*;
use axum::{Router, routing::get};
use super::*;

pub trait Server {
    fn server_get_address(&mut self) -> String;
    fn server_create_route() -> Router;
    async fn server_create_app(&mut self);
    async fn server_create_listener(&mut self) -> tokio::net::TcpListener;
}

impl Server for Config {
    fn server_get_address(&mut self) -> String {

        if self.network.stage == "dev" {
            
            let ip = "localhost";
            self.status.is_enabled = Some(true);
            
            println!("
                [Success]: Server started on: http://{ip}:{}/api",
                self.network.port
            )
            
        } else if self.network.stage == "prod" {
            
            self.status.is_enabled = Some(true);
            print!("
                [Success]: Server started on: http://{}:{}/api", 
                self.network.ip, 
                self.network.port
            )

        } else {
            
            self.status.is_enabled = Some(false);
            println!(
                "[Error]: Server is not started. Issue with IP/PORT of server"
            )
        }
        
        Config::format_address(
            &self.network.ip, 
            &self.network.port
        )
    }
    
    fn server_create_route() -> Router {
        //let redis_client = Config::redis_create_client();
        
        Router::new()
            .route("/", get(hello_world))
            .route("/api", get(list_api))  
    }

    async fn server_create_listener(&mut self) -> tokio::net::TcpListener {
        
        let server_address = self.server_get_address();

        tokio::net::TcpListener::bind(server_address).await.unwrap()
    }

    async fn server_create_app(&mut self) {
        axum::serve(
            self.server_create_listener().await,
            Config::server_create_route(),
        ).await.unwrap();
    }
}