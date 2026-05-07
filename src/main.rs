use actix_web::{
    web,
    App,
    HttpRequest,
    HttpResponse,
    HttpServer,
    Responder,
};

use base64::{
    engine::general_purpose,
    Engine as _,
};

use hmac::{Hmac, Mac};

use rusqlite::{
    params,
    Connection,
};

use serde::{
    Deserialize,
    Serialize,
};

use sha2::Sha256;

use std::sync::Mutex;

use tokio::time::{
    sleep,
    Duration,
};

use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;

const HMAC_SECRET: &str =
    "super-secret-key";

const STELLAR_ACCOUNT: &str =
    "GB4TW32HFZEQMTS67U33D6GD36ZHTMEPAVFOIEPWXWY5QYFQDE3PC7QT";

// 📦 estado app
struct AppState {

    db: Mutex<Connection>,
}

// 📦 request
#[derive(Deserialize)]
struct CreateOrderRequest {

    valor_brl: f64,
}

// 📦 resposta order
#[derive(Serialize)]
struct OrderResponse {

    id: String,

    valor_brl: f64,

    valor_xlm: f64,

    memo: String,

    payment_uri: String,

    status: String,
}

// 📦 listagem
#[derive(Serialize)]
struct OrderListItem {

    id: String,

    status: String,

    valor_brl: f64,

    tx_hash: Option<String>,
}

// 🔐 gerar hmac
fn generate_hmac(
    payload: &str,
) -> String {

    let mut mac =
        HmacSha256::new_from_slice(
            HMAC_SECRET.as_bytes()
        )
        .unwrap();

    mac.update(
        payload.as_bytes()
    );

    let result =
        mac.finalize()
            .into_bytes();

    hex::encode(result)
}

// 🌐 homepage
async fn index()
-> impl Responder {

    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
            <html>

                <head>
                    <title>SlipPay</title>
                </head>

                <body
                    style="
                        font-family: Arial;
                        padding: 40px;
                    "
                >

                    <h1>
                        🚀 SlipPay
                    </h1>

                    <p>
                        Gateway Stellar funcionando.
                    </p>

                </body>

            </html>
            "#
        )
}

// 💳 criar pedido
async fn create_order(

    req: HttpRequest,

    body: String,

    data: web::Data<AppState>,

) -> impl Responder {

    // 🔐 assinatura
    let signature =
        req.headers()
            .get("x-signature")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

    let expected =
        generate_hmac(&body);

    if signature != expected {

        return HttpResponse::Unauthorized()
            .body("invalid signature");
    }

    // 📦 json
    let payload:
        CreateOrderRequest =
        match serde_json::from_str(&body) {

            Ok(v) => v,

            Err(e) => {

                return HttpResponse::BadRequest()
                    .body(
                        format!(
                            "Json deserialize error: {}",
                            e
                        )
                    );
            }
        };

    // 🆔 id
    let id =
        Uuid::new_v4()
            .to_string();

    // 💰 conversão fake
    let valor_xlm =
        payload.valor_brl / 5.0;

    // 🔐 memo hash
    let memo =
        generate_hmac(&id);

    // 🌌 payment uri
    let payment_uri =
        format!(
            "stellar:{}?amount={}&memo={}&memo_type=hash",
            STELLAR_ACCOUNT,
            valor_xlm,
            memo
        );

    // 💾 salvar db
    let conn =
        data.db.lock()
            .unwrap();

    conn.execute(
        "
        INSERT INTO orders (

            id,
            valor_brl,
            valor_xlm,
            memo,
            status

        )
        VALUES (?1, ?2, ?3, ?4, ?5)
        ",
        params![
            id,
            payload.valor_brl,
            valor_xlm,
            memo,
            "pending"
        ],
    )
    .unwrap();

    HttpResponse::Ok()
        .json(
            OrderResponse {

                id,

                valor_brl:
                    payload.valor_brl,

                valor_xlm,

                memo,

                payment_uri,

                status:
                    "pending".to_string(),
            }
        )
}

// 📋 listar pedidos
async fn list_orders(
    data: web::Data<AppState>,
)
-> impl Responder {

    let conn =
        data.db.lock()
            .unwrap();

    let mut stmt =
        conn.prepare(
            "
            SELECT
                id,
                status,
                valor_brl,
                tx_hash
            FROM orders
            ORDER BY rowid DESC
            "
        )
        .unwrap();

    let rows =
        stmt.query_map(
            [],
            |row| {

                Ok(
                    OrderListItem {

                        id:
                            row.get(0)?,

                        status:
                            row.get(1)?,

                        valor_brl:
                            row.get(2)?,

                        tx_hash:
                            row.get(3)?,
                    }
                )
            },
        )
        .unwrap();

    let mut orders =
        vec![];

    for row in rows {

        orders.push(
            row.unwrap()
        );
    }

    HttpResponse::Ok()
        .json(orders)
}

// 🔎 listener
async fn listener(
    data: web::Data<AppState>,
) {

    loop {

        println!(
            "🔎 Escutando TESTNET..."
        );

        // 🔎 cursor salvo
        let paging_token:
            Option<String> = {

            let conn =
                data.db.lock()
                    .unwrap();

            conn.query_row(
                "
                SELECT paging_token
                FROM listener_state
                WHERE id = 1
                ",
                [],
                |row| row.get(0),
            )
            .ok()
        };

        // 🌌 URL
        let url =
            match paging_token {

                Some(token) => {

                    format!(
                        "https://horizon-testnet.stellar.org/accounts/{}/transactions?cursor={}&limit=10&order=asc",
                        STELLAR_ACCOUNT,
                        token
                    )
                }

                None => {

                    format!(
                        "https://horizon-testnet.stellar.org/accounts/{}/transactions?limit=10&order=desc",
                        STELLAR_ACCOUNT
                    )
                }
            };

        match reqwest::get(&url).await {

            Ok(resp) => {

                match resp
                    .json::<serde_json::Value>()
                    .await
                {

                    Ok(json) => {

                        if let Some(records) =
                            json["_embedded"]["records"]
                                .as_array()
                        {

                            for tx in records {

                                let memo_type =
                                    tx["memo_type"]
                                        .as_str()
                                        .unwrap_or("");

                                let raw_memo =
                                    tx["memo"]
                                        .as_str()
                                        .unwrap_or("");

                                let memo =

                                    if memo_type == "hash" {

                                        match general_purpose::STANDARD
                                            .decode(raw_memo)
                                        {

                                            Ok(bytes) => {

                                                hex::encode(bytes)
                                            }

                                            Err(_) => {

                                                continue;
                                            }
                                        }

                                    } else {

                                        raw_memo.to_string()
                                    };

                                let hash =
                                    tx["hash"]
                                        .as_str()
                                        .unwrap_or("");

                                let paging =
                                    tx["paging_token"]
                                        .as_str()
                                        .unwrap_or("");

                                let successful =
                                    tx["successful"]
                                        .as_bool()
                                        .unwrap_or(false);

                                if memo.is_empty() {

                                    continue;
                                }

                                if !successful {

                                    continue;
                                }

                                println!(
                                    "💰 Memo detectado: {}",
                                    memo
                                );

                                let conn =
                                    data.db.lock()
                                        .unwrap();

                                let result =
                                    conn.execute(
                                        "
                                        UPDATE orders
                                        SET
                                            status='confirmed',
                                            tx_hash=?1
                                        WHERE memo=?2
                                        AND status='pending'
                                        ",
                                        params![
                                            hash,
                                            memo
                                        ],
                                    );

                                match result {

                                    Ok(updated) => {

                                        if updated > 0 {

                                            println!(
                                                "✅ Pedido confirmado!"
                                            );

                                            println!(
                                                "🔗 TX HASH: {}",
                                                hash
                                            );
                                        }
                                    }

                                    Err(e) => {

                                        println!(
                                            "❌ Erro DB: {:?}",
                                            e
                                        );
                                    }
                                }

                                // 💾 salvar cursor SEMPRE
                                conn.execute(
                                    "
                                    INSERT OR REPLACE INTO listener_state (
                                        id,
                                        paging_token
                                    )
                                    VALUES (1, ?1)
                                    ",
                                    params![
                                        paging
                                    ],
                                )
                                .unwrap();
                            }
                        }
                    }

                    Err(e) => {

                        println!(
                            "❌ JSON error: {:?}",
                            e
                        );
                    }
                }
            }

            Err(e) => {

                println!(
                    "❌ Horizon error: {:?}",
                    e
                );
            }
        }

        sleep(
            Duration::from_secs(5)
        ).await;
    }
}

// 🚀 main
#[actix_web::main]
async fn main()
-> std::io::Result<()> {

    // 💾 sqlite
    let conn =
        Connection::open(
            "slippay.db"
        )
        .unwrap();

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS orders (

            id TEXT PRIMARY KEY,

            valor_brl REAL NOT NULL,

            valor_xlm REAL NOT NULL,

            memo TEXT NOT NULL,

            tx_hash TEXT,

            status TEXT NOT NULL
        )
        ",
        [],
    )
    .unwrap();

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

    let data =
        web::Data::new(
            AppState {

                db:
                    Mutex::new(conn),
            }
        );

    // 🔎 listener
    let listener_data =
        data.clone();

    tokio::spawn(
        async move {

            listener(
                listener_data
            ).await;
        }
    );

    println!(
        "🚀 SlipPay SEGURO rodando + listener ativo"
    );

    // 🌐 servidor
    HttpServer::new(
        move || {

            App::new()

                .app_data(
                    data.clone()
                )

                .route(
                    "/",
                    web::get()
                        .to(index),
                )

                .route(
                    "/orders",
                    web::post()
                        .to(create_order),
                )

                .route(
                    "/orders",
                    web::get()
                        .to(list_orders),
                )
        }
    )
    .bind((
        "127.0.0.1",
        8081
    ))?
    .run()
    .await
}
