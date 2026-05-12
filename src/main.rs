mod stellar;
mod reconciler;
mod db;
mod webhook;
mod listener;

use actix_web::{
    web,
    App,
    HttpResponse,
    HttpServer,
    Responder,
};

use rusqlite::Connection;

use serde::{Deserialize, Serialize};

use std::sync::Mutex;

use uuid::Uuid;

use rust_decimal::Decimal;

use rust_decimal_macros::dec;

use tokio::spawn;

use db::{
    Charge,
    create_charge,
    get_charge_by_memo,
};

#[derive(Deserialize)]
struct CreateChargeRequest {

    brl_amount: Decimal,
}

#[derive(Serialize)]
struct CreateChargeResponse {

    id: String,

    memo: String,

    xlm_amount: Decimal,

    status: String,
}

#[derive(Deserialize)]
struct ConfirmPaymentRequest {

    tx_hash: String,
}

struct AppState {

    db: Mutex<Connection>,
}

async fn health() -> impl Responder {

    HttpResponse::Ok().body("ok")
}

async fn create_charge_handler(
    data: web::Data<AppState>,
    body: web::Json<CreateChargeRequest>,
) -> impl Responder {

    let conn =
        data.db.lock().unwrap();

    let id =
        Uuid::new_v4().to_string();

    let memo =
        Uuid::new_v4()
            .simple()
            .to_string();

    let rate =
        dec!(5.0);

    let xlm_amount =
        body.brl_amount / rate;

    match create_charge(
        &conn,
        body.brl_amount,
        &memo,
        &id,
    ) {

        Ok(_) => {

            let response =
                CreateChargeResponse {

                    id,

                    memo,

                    xlm_amount,

                    status:
                        "pending".to_string(),
                };

            HttpResponse::Ok().json(
                response
            )
        }

        Err(err) => {

            HttpResponse::InternalServerError()
                .body(
                    err.to_string()
                )
        }
    }
}

async fn get_charge_handler(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {

    let memo =
        path.into_inner();

    let conn =
        data.db.lock().unwrap();

    match get_charge_by_memo(
        &conn,
        &memo,
    ) {

        Ok(charge) => {

            HttpResponse::Ok()
                .json(charge)
        }

        Err(_) => {

            HttpResponse::NotFound()
                .body("charge not found")
        }
    }
}

async fn confirm_payment_handler(
    body: web::Json<ConfirmPaymentRequest>,
) -> impl Responder {

    let conn =
        match Connection::open(
            "slippay.db"
        ) {

            Ok(c) => c,

            Err(err) => {

                return HttpResponse::InternalServerError()
                    .body(
                        err.to_string()
                    );
            }
        };

    let result =
        reconciler::reconcile_payment(
            &conn,
            &body.tx_hash,
        ).await;

    match result {

        Ok(response) => {

            HttpResponse::Ok()
                .json(response)
        }

        Err(err) => {

            HttpResponse::BadRequest()
                .body(err)
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!(
        "================================"
    );

    println!(
        " SlipPay API Starting"
    );

    println!(
        "================================"
    );

    let conn =
        Connection::open(
            "slippay.db"
        )
        .expect("failed db");

    spawn(async {

        listener::start_listener().await;

    });

    let state =
        web::Data::new(
            AppState {
                db: Mutex::new(conn),
            }
        );

    HttpServer::new(move || {

        App::new()

            .app_data(
                state.clone()
            )

            .route(
                "/health",
                web::get().to(health),
            )

            .route(
                "/create-charge",
                web::post()
                    .to(
                        create_charge_handler
                    ),
            )

            .route(
                "/charge/{memo}",
                web::get()
                    .to(
                        get_charge_handler
                    ),
            )

            .route(
                "/confirm-payment",
                web::post()
                    .to(
                        confirm_payment_handler
                    ),
            )
    })

    .bind((
        "0.0.0.0",
        8081,
    ))?

    .run()

    .await
}
