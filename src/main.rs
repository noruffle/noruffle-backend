use axum::{extract::Request, middleware::{self, Next}, response::Response, routing::get, Router};
use dotenv::dotenv;

mod middlewares;
use middlewares::my_middleware;
mod routes;
use routes::hello_world;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .route("/", get(hello_world))
        .layer(middleware::from_fn(my_middleware));

    let address = {
        let get_address = |ip: &str, port: &str| {
            format!("{ip}:{port}")
        };
        let ip = dotenv::var("IP").expect("IP-адрес не указан в env");
        let port = dotenv::var("PORT").expect("PORT не указан в env");

        println!("Server started on http://localhost:{port}");

        get_address(&ip, &port)
    };
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}