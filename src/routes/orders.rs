use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use rusqlite::Connection;
use crate::services::payment_service::PaymentService;

#[derive(Deserialize)]
pub struct OrderRequest {
    amount_brl: f64,
    amount_xlm: f64,
    destination: String,
    memo: String,
}

#[post("/orders")]
async fn create_order(req: web::Json<OrderRequest>) -> HttpResponse {
    let conn = Connection::open("slippay.db").expect("Erro ao abrir banco");
    let service = PaymentService::new(&conn);

    let payment = service.create_payment(
        req.amount_brl,
        req.amount_xlm,
        &req.destination,
        &req.memo,
    );

    HttpResponse::Ok().json(payment)
}
