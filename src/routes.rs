use axum::{Router, routing::get, routing::post};
use crate::services::api;

// [SecOps Guard] Checked Signer & Authority Validation
    pub fn create_routes() -> Router {
    Router::new()
        .route("/health", get(|| async { "SlipPay OK" }))
        .route("/eval", post(api::handle_eval))
}
