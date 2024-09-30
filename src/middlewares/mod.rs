use axum::{response::Response, middleware::Next, extract::Request};
use axum_macros::debug_middleware;

#[debug_middleware]
pub async fn my_middleware(
    request: Request,
    next: Next
) -> Response {
    next.run(request).await
}