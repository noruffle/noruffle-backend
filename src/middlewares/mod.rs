#[axum_macros::debug_middleware]
pub async fn my_middleware(
    request: Request,
    next: Next
) -> Response {
    next.run(request).await
}

use crate::*;