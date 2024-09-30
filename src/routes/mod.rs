#[axum_macros::debug_handler]
pub async fn hello_world() -> &'static str {
    "Hello, World!"
}