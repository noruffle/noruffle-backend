use crate::routes::*;
use crate::middlewares::my_middleware;
use axum::{Router, middleware, routing::get};
use super::{Config, Redis};

pub trait Server {
    fn server_get_address() -> String;
    fn server_create_app() -> Router;
    async fn server_create_listener() -> tokio::net::TcpListener;
}

impl Server for Config {
    fn server_get_address() -> String {
        let (ip, port) = (
            dotenv::var("SERVER_IP")
                .expect("IP сервера не указан в env"),
            dotenv::var("SERVER_PORT")
                .expect("PORT сервера не указан в env"),
        );
        
        println!("Server going on http://localhost:{port}");
        
        Config::format_address(&ip, &port)
    }
    
    fn server_create_app() -> Router {
        let redis_client = Config::redis_create_client();
        
        Router::new()
            .route("/", get(hello_world))
            .route("/api", get(list_api))
            .route("/api/users", get(user_create))
            .layer(middleware::from_fn(my_middleware))
            .layer(axum::Extension(redis_client))    
    }

    async fn server_create_listener() -> tokio::net::TcpListener {
        let server_address = Config::server_get_address();

        tokio::net::TcpListener::bind(server_address).await.unwrap()
    }
}