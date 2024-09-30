mod middlewares;
mod routes;
mod config;

use config::Config;
use config::Server;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
 
    let app = Config::server_create_app();
    let listener = Config::server_create_listener().await;
    
    axum::serve(listener, app).await.unwrap();
}