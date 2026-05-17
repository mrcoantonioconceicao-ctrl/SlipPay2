use axum::{
    extract::Path,
    response::Json,
};
use serde_json::{json, Value};

pub async fn get_payments() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "message": "payments endpoint"
    }))
}

pub async fn get_payment(
    Path(tx_hash): Path<String>,
) -> Json<Value> {
    Json(json!({
        "tx_hash": tx_hash
    }))
}

pub async fn get_charges() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "message": "charges endpoint"
    }))
}

pub async fn create_charge() -> Json<Value> {
    Json(json!({
        "status": "created"
    }))
}

pub async fn get_charge(
    Path(id): Path<String>,
) -> Json<Value> {
    Json(json!({
        "charge_id": id
    }))
}
