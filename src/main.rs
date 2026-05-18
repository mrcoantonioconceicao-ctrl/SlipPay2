mod routes;
mod services;
mod utils;

use axum::Router;
use routes::create_routes;

#[tokio::main]
async fn main() {
    let app = create_routes();
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
