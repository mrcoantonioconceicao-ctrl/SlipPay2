use actix_files::Files;
use actix_web::{
    web,
    App,
    HttpRequest,
    HttpResponse,
    HttpServer,
    Responder,
};

use base64::{engine::general_purpose, Engine as _};

use hmac::{Hmac, Mac};

use reqwest;

use rusqlite::{params, Connection};

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use serde::{Deserialize, Serialize};

use sha2::{Digest, Sha256};

use std::sync::Mutex;

use tokio::time::{sleep, Duration};

use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;

// ========================================
// APP STATE
// ========================================

struct AppState {
    db: Mutex<Connection>,
}

// ========================================
// ORDER
// ========================================

#[derive(Serialize, Deserialize)]
struct Order {
    id: String,

    valor_brl: Decimal,

    valor_xlm: Decimal,

    memo: String,

    tx_hash: Option<String>,

    status: String,

    payment_uri: String,
}

// ========================================
// CREATE ORDER REQUEST
// ========================================

#[derive(Deserialize)]
struct CreateOrderRequest {
    valor_brl: Decimal,
}

// ========================================
// HOMEPAGE
// ========================================

async fn home() -> impl Responder {

    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../index.html"))
}

// ========================================
// CREATE ORDER
// ========================================

async fn create_order(
    req: HttpRequest,
    data: web::Data<AppState>,
    body: web::Json<CreateOrderRequest>,
) -> impl Responder {

    // ========================================
    // HMAC VALIDATION
    // ========================================

    let signature = req
        .headers()
        .get("x-signature")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let payload = format!(
        r#"{{"valor_brl":{}}}"#,
        body.valor_brl
    );

    let mut mac =
        HmacSha256::new_from_slice(
            b"super-secret-key"
        )
        .unwrap();

    mac.update(payload.as_bytes());

    let _expected_signature =
        hex::encode(mac.finalize().into_bytes());

    // ========================================
    // MVP TEMP
    // ========================================

    /*
    if signature != _expected_signature {

        return HttpResponse::Unauthorized()
            .body("invalid signature");
    }
    */

    println!(
        "🔐 Signature recebida: {}",
        signature
    );

    // ========================================
    // BUSINESS LOGIC
    // ========================================

    let valor_xlm =
        body.valor_brl / dec!(5);

    let id = Uuid::new_v4().to_string();

    let mut hasher = Sha256::new();

    hasher.update(id.as_bytes());

    let memo =
        hex::encode(hasher.finalize());

    let payment_uri = format!(
        "stellar:{}?amount={}&memo={}&memo_type=hash",
        "GB4TW32HFZEQMTS67U33D6GD36ZHTMEPAVFOIEPWXWY5QYFQDE3PC7QT",
        valor_xlm,
        memo
    );

    let order = Order {

        id: id.clone(),

        valor_brl: body.valor_brl,

        valor_xlm,

        memo: memo.clone(),

        tx_hash: None,

        status: "pending".to_string(),

        payment_uri: payment_uri.clone(),
    };

    // ========================================
    // SAVE DATABASE
    // ========================================

    let conn = data.db.lock().unwrap();

    conn.execute(
        "
        INSERT INTO orders (
            id,
            valor_brl,
            valor_xlm,
            memo,
            tx_hash,
            status
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        ",
        params![
            order.id,
            order.valor_brl.to_string(),
            order.valor_xlm.to_string(),
            order.memo,
            order.tx_hash,
            order.status
        ],
    )
    .unwrap();

    HttpResponse::Ok().json(order)
}

// ========================================
// LIST ORDERS
// ========================================

async fn list_orders(
    data: web::Data<AppState>,
) -> impl Responder {

    let conn = data.db.lock().unwrap();

    let mut stmt = conn
        .prepare(
            "
            SELECT
                id,
                valor_brl,
                valor_xlm,
                memo,
                tx_hash,
                status
            FROM orders
            ",
        )
        .unwrap();

    let rows = stmt
        .query_map([], |row| {

            let memo: String = row.get(3)?;

            let valor_brl: Decimal =
                row
                    .get::<_, String>(1)?
                    .parse::<Decimal>()
                    .unwrap();

            let valor_xlm: Decimal =
                row
                    .get::<_, String>(2)?
                    .parse::<Decimal>()
                    .unwrap();

            Ok(Order {

                id: row.get(0)?,

                valor_brl,

                valor_xlm,

                memo: memo.clone(),

                tx_hash: row.get(4)?,

                status: row.get(5)?,

                payment_uri: format!(
                    "stellar:{}?amount={}&memo={}&memo_type=hash",
                    "GB4TW32HFZEQMTS67U33D6GD36ZHTMEPAVFOIEPWXWY5QYFQDE3PC7QT",
                    valor_xlm,
                    memo
                ),
            })
        })
        .unwrap();

    let mut orders = Vec::new();

    for order in rows {

        orders.push(order.unwrap());
    }

    HttpResponse::Ok().json(orders)
}

// ========================================
// LOAD CURSOR
// ========================================

fn load_cursor(
    db: &web::Data<AppState>,
) -> String {

    let conn = db.db.lock().unwrap();

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS listener_state (
            id INTEGER PRIMARY KEY,
            paging_token TEXT
        )
        ",
        [],
    )
    .unwrap();

    let result: Result<String, _> =
        conn.query_row(
            "
            SELECT paging_token
            FROM listener_state
            WHERE id = 1
            ",
            [],
            |row| row.get(0),
        );

    result.unwrap_or("now".to_string())
}

// ========================================
// SAVE CURSOR
// ========================================

fn save_cursor(
    db: &web::Data<AppState>,
    cursor: &str,
) {

    let conn = db.db.lock().unwrap();

    conn.execute(
        "
        INSERT OR REPLACE INTO listener_state (
            id,
            paging_token
        )
        VALUES (1, ?1)
        ",
        params![cursor],
    )
    .unwrap();
}

// ========================================
// STELLAR LISTENER
// ========================================

async fn stellar_listener(
    db: web::Data<AppState>,
) {

    let account =
        "GB4TW32HFZEQMTS67U33D6GD36ZHTMEPAVFOIEPWXWY5QYFQDE3PC7QT";

    let client = reqwest::Client::new();

    let mut cursor = load_cursor(&db);

    loop {

        println!("🔎 Escutando TESTNET...");

        let url = format!(
            "https://horizon-testnet.stellar.org/accounts/{}/transactions?cursor={}&limit=10&order=asc",
            account,
            cursor
        );

        match client.get(&url).send().await {

            Ok(response) => {

                match response
                    .json::<serde_json::Value>()
                    .await
                {

                    Ok(json) => {

                        if let Some(records) =
                            json["_embedded"]["records"]
                                .as_array()
                        {

                            for tx in records {

                                if let Some(paging_token) =
                                    tx["paging_token"]
                                        .as_str()
                                {

                                    cursor =
                                        paging_token.to_string();

                                    save_cursor(
                                        &db,
                                        &cursor,
                                    );
                                }

                                let memo_base64 =
                                    tx["memo"]
                                        .as_str()
                                        .unwrap_or("");

                                if memo_base64.is_empty() {

                                    continue;
                                }

                                let memo_hex =
                                    match general_purpose::STANDARD
                                        .decode(memo_base64)
                                    {

                                        Ok(bytes) => {
                                            hex::encode(bytes)
                                        }

                                        Err(_) => continue,
                                    };

                                println!(
                                    "💰 Memo detectado: {}",
                                    memo_hex
                                );

                                let tx_hash =
                                    tx["hash"]
                                        .as_str()
                                        .unwrap_or("");

                                let conn =
                                    db.db.lock().unwrap();

                                let updated =
                                    conn.execute(
                                        "
                                        UPDATE orders
                                        SET
                                            status = 'confirmed',
                                            tx_hash = ?1
                                        WHERE memo = ?2
                                        ",
                                        params![
                                            tx_hash,
                                            memo_hex
                                        ],
                                    )
                                    .unwrap();

                                if updated > 0 {

                                    println!(
                                        "✅ Pedido confirmado!"
                                    );

                                    println!(
                                        "🔗 TX HASH: {}",
                                        tx_hash
                                    );
                                }
                            }
                        }
                    }

                    Err(err) => {

                        println!(
                            "❌ JSON error: {:?}",
                            err
                        );
                    }
                }
            }

            Err(err) => {

                println!(
                    "❌ Horizon error: {:?}",
                    err
                );
            }
        }

        sleep(Duration::from_secs(5)).await;
    }
}

// ========================================
// MAIN
// ========================================

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("🚀 SlipPay iniciado");

    let conn =
        Connection::open("slippay.db").unwrap();

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS orders (
            id TEXT PRIMARY KEY,
            valor_brl TEXT NOT NULL,
            valor_xlm TEXT NOT NULL,
            memo TEXT NOT NULL,
            tx_hash TEXT,
            status TEXT NOT NULL
        )
        ",
        [],
    )
    .unwrap();

    let data = web::Data::new(AppState {
        db: Mutex::new(conn),
    });

    // ========================================
    // START LISTENER
    // ========================================

    let listener_data = data.clone();

    tokio::spawn(async move {

        stellar_listener(listener_data).await;
    });

    // ========================================
    // START SERVER
    // ========================================

    HttpServer::new(move || {

        App::new()

            .app_data(data.clone())

            // API ROUTES FIRST

            .route("/", web::get().to(home))

            .route(
                "/orders",
                web::post().to(create_order),
            )

            .route(
                "/orders",
                web::get().to(list_orders),
            )

            // STATIC FILES LAST

            .service(
                Files::new("/", "./")
                    .index_file("index.html")
            )
    })

.bind(("127.0.0.1", 8081))?

    .run()

    .await
}
