mod api;
mod db;
mod listener;
mod models;
mod reconciler;
mod stellar;
mod webhook;

use axum::{
    routing::{get, post},
    Router,
};

use std::net::SocketAddr;

#[tokio::main]
async fn main() {

    // ── INIT DB ──────────────────────────────────────────
    let db = db::init_db().await;

    // ── CONFIG ───────────────────────────────────────────
    let horizon_url =
        "https://horizon-testnet.stellar.org".to_string();

    let public_key =
        "GB4TW32HFZEQMTS67U33D6GD36ZHTMEPAVFOIEPWXWY5QYFQDE3PC7QT"
            .to_string();

    // ── START LISTENER ───────────────────────────────────
    let db_listener = db.clone();

    tokio::spawn(async move {
        stellar::start_listener(
            db_listener,
            horizon_url,
            public_key,
        )
        .await;
    });

    // ── ROUTES ───────────────────────────────────────────
    let app = Router::new()
        // Health
        .route("/", get(api::root))

        // Payments (recebidos pelo listener)
        .route("/payments",          get(api::list_payments))
        .route("/payments/:tx_hash", get(api::get_payment))

        // Charges (cobranças criadas via API)
        .route("/charges",     get(api::list_charges))
        .route("/charges",     post(api::create_charge))
        .route("/charges/:id", get(api::get_charge))

        // Compartilha o pool com todos os handlers
        .with_state(db);

    // ── SERVER ───────────────────────────────────────────
    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));

    println!("🚀 SlipPay iniciado na porta 8081");
    println!("📡 Endpoints disponíveis:");
    println!("   GET  /payments");
    println!("   GET  /payments/:tx_hash");
    println!("   GET  /charges");
    println!("   POST /charges");
    println!("   GET  /charges/:id");

    let listener =
        tokio::net::TcpListener::bind(addr)
            .await
            .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}

