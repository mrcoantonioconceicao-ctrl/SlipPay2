use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, FromRow};
use uuid::Uuid;
use sha2::{Sha256, Digest};

const MERCHANT_ADDRESS: &str = "GB4TW32HFZEQMTS67U33D6GD36ZHTMEPAVFOIEPWXWY5QYFQDE3PC7QT";

#[derive(Serialize, Deserialize, FromRow)]
struct Order {
    id: String,
    valor_brl: f64,
    valor_xlm: Option<f64>,
    status: String,
    memo: Option<String>,
    tx_hash: Option<String>,
}

#[derive(Deserialize)]
struct CreateOrder {
    valor_brl: f64,
}

// ---------------- HASH ----------------

fn generate_hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

// ---------------- CREATE ORDER ----------------

async fn create_order(
    pool: web::Data<SqlitePool>,
    data: web::Json<CreateOrder>,
) -> impl Responder {

    let id = Uuid::new_v4().to_string();
    let valor_xlm = data.valor_brl / 5.0;

    let memo_hash = generate_hash(&id);

    sqlx::query(
        "INSERT INTO orders (id, valor_brl, valor_xlm, status, memo)
         VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(data.valor_brl)
    .bind(valor_xlm)
    .bind("pending")
    .bind(&memo_hash)
    .execute(pool.get_ref())
    .await
    .unwrap();

    let payment_uri = format!(
        "stellar:{}?amount={}&memo={}",
        MERCHANT_ADDRESS, valor_xlm, memo_hash
    );

    HttpResponse::Ok().json(serde_json::json!({
        "id": id,
        "valor_brl": data.valor_brl,
        "valor_xlm": valor_xlm,
        "address": MERCHANT_ADDRESS,
        "memo": memo_hash,
        "memo_type": "hash",
        "payment_uri": payment_uri,
        "status": "pending"
    }))
}

// ---------------- LIST ORDERS ----------------

async fn list_orders(pool: web::Data<SqlitePool>) -> impl Responder {
    let orders = sqlx::query_as::<_, Order>(
        "SELECT id, valor_brl, valor_xlm, status, memo, tx_hash FROM orders"
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap();

    HttpResponse::Ok().json(orders)
}

// ---------------- LISTENER ----------------

async fn stellar_listener(pool: SqlitePool) {
    loop {
        println!("🔎 Escutando TESTNET...");

        let url = format!(
            "https://horizon-testnet.stellar.org/accounts/{}/payments?limit=20&order=desc",
            MERCHANT_ADDRESS
        );

        if let Ok(resp) = reqwest::get(&url).await {
            if let Ok(json) = resp.json::<serde_json::Value>().await {

                if let Some(records) = json["_embedded"]["records"].as_array() {

                    for payment in records {

                        let tx_hash = payment["transaction_hash"].as_str().unwrap_or("");
                        let amount = payment["amount"].as_str().unwrap_or("0");
                        let to = payment["to"].as_str().unwrap_or("");

                        if to != MERCHANT_ADDRESS {
                            continue;
                        }

                        // Buscar dados da transação
                        let tx_url = format!(
                            "https://horizon-testnet.stellar.org/transactions/{}",
                            tx_hash
                        );

                        if let Ok(tx_resp) = reqwest::get(&tx_url).await {
                            if let Ok(tx_json) = tx_resp.json::<serde_json::Value>().await {

                                let memo = tx_json["memo"].as_str().unwrap_or("");
                                let memo_type = tx_json["memo_type"].as_str().unwrap_or("");

                                // 🔥 AGORA ACEITA APENAS HASH
                                if memo_type != "hash" {
                                    continue;
                                }

                                println!("Memo HASH recebido: {}", memo);

                                let order = sqlx::query_as::<_, Order>(
                                    "SELECT id, valor_xlm, memo FROM orders WHERE memo = ? AND status = 'pending'"
                                )
                                .bind(memo)
                                .fetch_optional(&pool)
                                .await
                                .unwrap();

                                if let Some(order) = order {

                                    let expected = order.valor_xlm.unwrap_or(0.0);
                                    let amount_f: f64 = amount.parse().unwrap_or(0.0);

                                    if (amount_f - expected).abs() > 0.01 {
                                        continue;
                                    }

                                    println!("💰 PAGAMENTO DETECTADO: {}", order.id);

                                    sqlx::query(
                                        "UPDATE orders SET status = 'confirmed', tx_hash = ? WHERE id = ?"
                                    )
                                    .bind(tx_hash)
                                    .bind(order.id)
                                    .execute(&pool)
                                    .await
                                    .unwrap();
                                }
                            }
                        }
                    }
                }
            }
        }

        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}

// ---------------- MAIN ----------------

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = SqlitePool::connect("sqlite:slippay.db").await.unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS orders (
            id TEXT PRIMARY KEY,
            valor_brl REAL,
            valor_xlm REAL,
            status TEXT,
            memo TEXT,
            tx_hash TEXT
        )"
    )
    .execute(&pool)
    .await
    .unwrap();

    let listener_pool = pool.clone();
    tokio::spawn(async move {
        stellar_listener(listener_pool).await;
    });

    println!("🚀 SlipPay HASH (PROD) rodando + listener ativo");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/orders", web::post().to(create_order))
            .route("/orders", web::get().to(list_orders))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
