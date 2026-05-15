use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};

use serde_json::{json, Value};
use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use chrono::Utc;

use crate::models::CreateChargeRequest;

// ─── GET / ───────────────────────────────────────────────
pub async fn root() -> &'static str {
    "SlipPay Online"
}

// ─── GET /payments ───────────────────────────────────────
pub async fn list_payments(
    State(db): State<SqlitePool>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {

    let rows = sqlx::query(
        "SELECT id, tx_hash, amount, asset_type, sender, receiver, created_at
         FROM payments ORDER BY id DESC LIMIT 100"
    )
    .fetch_all(&db)
    .await
    .map_err(|e| internal_error(e.to_string()))?;

    let payments: Vec<Value> = rows.iter().map(|r| json!({
        "id":         r.get::<i64,   _>("id"),
        "tx_hash":    r.get::<String, _>("tx_hash"),
        "amount":     r.get::<String, _>("amount"),
        "asset_type": r.get::<String, _>("asset_type"),
        "sender":     r.get::<String, _>("sender"),
        "receiver":   r.get::<String, _>("receiver"),
        "created_at": r.get::<String, _>("created_at"),
    })).collect();

    Ok(Json(json!({
        "success": true,
        "count":   payments.len(),
        "data":    payments
    })))
}

// ─── GET /payments/:tx_hash ───────────────────────────────
pub async fn get_payment(
    State(db): State<SqlitePool>,
    Path(tx_hash): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {

    let row = sqlx::query(
        "SELECT id, tx_hash, amount, asset_type, sender, receiver, created_at
         FROM payments WHERE tx_hash = ?"
    )
    .bind(&tx_hash)
    .fetch_optional(&db)
    .await
    .map_err(|e| internal_error(e.to_string()))?;

    match row {
        Some(r) => Ok(Json(json!({
            "success": true,
            "data": {
                "id":         r.get::<i64,    _>("id"),
                "tx_hash":    r.get::<String, _>("tx_hash"),
                "amount":     r.get::<String, _>("amount"),
                "asset_type": r.get::<String, _>("asset_type"),
                "sender":     r.get::<String, _>("sender"),
                "receiver":   r.get::<String, _>("receiver"),
                "created_at": r.get::<String, _>("created_at"),
            }
        }))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "error": "Pagamento não encontrado"
            })),
        )),
    }
}

// ─── POST /charges ────────────────────────────────────────
pub async fn create_charge(
    State(db): State<SqlitePool>,
    Json(body): Json<CreateChargeRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {

    if body.amount.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": "amount é obrigatório"
            })),
        ));
    }

    let id         = Uuid::new_v4().to_string();
    let memo       = generate_memo();
    let created_at = Utc::now().to_rfc3339();
    let status     = "pending";
    let asset      = if body.asset.is_empty() {
        "native".to_string()
    } else {
        body.asset.clone()
    };

    sqlx::query(
        "INSERT INTO charges (id, memo, amount, asset, status, created_at)
         VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&memo)
    .bind(&body.amount)
    .bind(&asset)
    .bind(status)
    .bind(&created_at)
    .execute(&db)
    .await
    .map_err(|e| internal_error(e.to_string()))?;

    Ok(Json(json!({
        "success": true,
        "data": {
            "id":         id,
            "memo":       memo,
            "amount":     body.amount,
            "asset":      asset,
            "status":     status,
            "created_at": created_at,
        }
    })))
}

// ─── GET /charges ─────────────────────────────────────────
pub async fn list_charges(
    State(db): State<SqlitePool>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {

    let rows = sqlx::query(
        "SELECT id, memo, amount, asset, status, created_at, tx_hash
         FROM charges ORDER BY rowid DESC LIMIT 100"
    )
    .fetch_all(&db)
    .await
    .map_err(|e| internal_error(e.to_string()))?;

    let charges: Vec<Value> = rows.iter().map(|r| json!({
        "id":         r.get::<String, _>("id"),
        "memo":       r.get::<String, _>("memo"),
        "amount":     r.get::<String, _>("amount"),
        "asset":      r.get::<String, _>("asset"),
        "status":     r.get::<String, _>("status"),
        "created_at": r.get::<String, _>("created_at"),
        "tx_hash":    r.get::<Option<String>, _>("tx_hash"),
    })).collect();

    Ok(Json(json!({
        "success": true,
        "count":   charges.len(),
        "data":    charges
    })))
}

// ─── GET /charges/:id ─────────────────────────────────────
pub async fn get_charge(
    State(db): State<SqlitePool>,
    Path(charge_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {

    let row = sqlx::query(
        "SELECT id, memo, amount, asset, status, created_at, tx_hash
         FROM charges WHERE id = ?"
    )
    .bind(&charge_id)
    .fetch_optional(&db)
    .await
    .map_err(|e| internal_error(e.to_string()))?;

    match row {
        Some(r) => Ok(Json(json!({
            "success": true,
            "data": {
                "id":         r.get::<String, _>("id"),
                "memo":       r.get::<String, _>("memo"),
                "amount":     r.get::<String, _>("amount"),
                "asset":      r.get::<String, _>("asset"),
                "status":     r.get::<String, _>("status"),
                "created_at": r.get::<String, _>("created_at"),
                "tx_hash":    r.get::<Option<String>, _>("tx_hash"),
            }
        }))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "error": "Cobrança não encontrada"
            })),
        )),
    }
}

// ─── Helpers ──────────────────────────────────────────────

fn generate_memo() -> String {
    let uid = Uuid::new_v4().to_string();
    let short = &uid.replace('-', "")[..8];
    format!("SPY{}", short.to_uppercase())
}

fn internal_error(msg: String) -> (StatusCode, Json<Value>) {
    eprintln!("❌ DB error: {}", msg);
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({
            "success": false,
            "error": "Erro interno do servidor"
        })),
    )
}

