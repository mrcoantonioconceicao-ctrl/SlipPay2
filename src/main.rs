use axum::{routing::{post, get}, Json, Router};
use serde::Deserialize;
use std::collections::HashMap;
use std::net::SocketAddr;

#[derive(Deserialize)]
struct EvalRequest {
    expr: String,
    vars: HashMap<String, f64>,
}

async fn eval_handler(Json(req): Json<EvalRequest>) -> Json<f64> {
    // Chama o crate local ast-engine
    let result = ast_engine::eval(&req.expr, &req.vars).unwrap_or(0.0);
    Json(result)
}

async fn health_handler() -> &'static str {
    "AST Engine OK"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/eval", post(eval_handler))
        .route("/health", get(health_handler));

    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    println!("Servidor AST rodando em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
