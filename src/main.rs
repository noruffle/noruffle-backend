mod middlewares;
mod routes;
mod config;

use config::{Config, Server};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let (app, listener) = (
        Config::server_create_app(),
        Config::server_create_listener().await

    );
    
    axum::serve(listener, app).await.unwrap();
}