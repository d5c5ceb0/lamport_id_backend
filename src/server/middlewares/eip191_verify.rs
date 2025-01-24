use crate::app::SharedState;
use axum::{
    body::Body, extract::State, http::Request, http::StatusCode, middleware::Next,
    response::Response,
};
pub async fn eip191_middleware(
    State(_state): State<SharedState>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    tracing::info!("EIP191 middleware: {:?}", req);
    //extract body

    Ok(next.run(req).await)
}
