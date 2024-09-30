use axum::{extract::Query, Extension};
use redis::Commands;

#[axum_macros::debug_handler]
pub async fn hello_world() -> &'static str {
    "Hello, World!"
}

#[axum_macros::debug_handler]
pub async fn list_api() -> &'static str {
    "Api List"
}

#[axum_macros::debug_handler]
pub async fn user_create
(
    Query(query): Query<User>,
    Extension(redis_client): Extension<std::sync::Arc<tokio::sync::Mutex<redis::Client>>>
)
-> String 
{
    let mut connection = redis_client.lock().await.get_connection().unwrap();

    let key = format!("user:{}", query.username);

    let _: () = connection.set(&key, query.age).unwrap();

    // http://127.0.0.1:3000/user_create?name=John&age=25
    format!("Пользователь {} с возрастом {} был добавлен в Redis", query.username, query.age)
}

#[derive(serde::Deserialize)]
pub struct User {
    pub username: String,
    pub age: u32
}