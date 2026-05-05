use actix_web::{web, App, HttpServer, HttpResponse, Responder, HttpRequest};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, FromRow};
use uuid::Uuid;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use tokio::time::{sleep, Duration};

type HmacSha256 = Hmac<Sha256>;

const SECRET: &str = "super-secret-key";

// ✅ SUA CHAVE TESTNET
const MERCHANT_ADDRESS: &str = "GBIMDQQWTUZVVMMGHWNCEUJROWKJA726FDWV2EGUDXCBR2BINYLNKTWR";

// ✅ HORIZON TESTNET
const HORIZON: &str = "https://horizon-testnet.stellar.org";

// ------------------ MODEL ------------------

#[derive(Serialize, Deserialize, FromRow)]
struct Order {
    id: String,
    valor_brl: f64,
    valor_xlm: Option<f64>,
    status: String,
    memo: Option<String>,
    tx_hash: Option<String>,
}

// ------------------ HMAC ------------------

fn verify_signature(payload: &str, signature: &str) -> bool {
    let mut mac = HmacSha256::new_from_slice(SECRET.as_bytes()).unwrap();
    mac.update(payload.as_bytes());
    let result = mac.finalize();
    let expected = hex::encode(result.into_bytes());
    expected == signature
}

// ------------------ CREATE ORDER ------------------

async fn create_order(
    pool: web::Data<SqlitePool>,
    body: String,
    req: HttpRequest,
) -> impl Responder {

    let signature = req.headers()
        .get("x-signature")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if !verify_signature(&body, signature) {
        return HttpResponse::Unauthorized().body("Assinatura inválida");
    }

    let data: serde_json::Value = serde_json::from_str(&body).unwrap();
    let valor_brl = data["valor_brl"].as_f64().unwrap();

    let id = Uuid::new_v4().to_string();
    let valor_xlm = valor_brl / 5.0;

    sqlx::query(
        "INSERT INTO orders (id, valor_brl, valor_xlm, status, memo)
         VALUES (?, ?, ?, 'pending', ?)"
    )
    .bind(&id)
    .bind(valor_brl)
    .bind(valor_xlm)
    .bind(&id)
    .execute(pool.get_ref())
    .await
    .unwrap();

    let payment_uri = format!(
        "stellar:{}?amount={}&memo={}",
        MERCHANT_ADDRESS, valor_xlm, id
    );

    HttpResponse::Ok().json(serde_json::json!({
        "id": id,
        "valor_brl": valor_brl,
        "valor_xlm": valor_xlm,
        "memo": id,
        "address": MERCHANT_ADDRESS,
        "payment_uri": payment_uri,
        "status": "pending"
    }))
}

// ------------------ LIST ORDERS ------------------

async fn list_orders(pool: web::Data<SqlitePool>) -> impl Responder {
    let orders = sqlx::query_as::<_, Order>(
        "SELECT id, valor_brl, valor_xlm, status, memo, tx_hash FROM orders"
    )
    .fetch_all(pool.get_ref())
    .await;

    match orders {
        Ok(o) => HttpResponse::Ok().json(o),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao listar"),
    }
}

// ------------------ STELLAR LISTENER ------------------

async fn stellar_listener(pool: SqlitePool) {
    loop {
        println!("🔎 Escutando TESTNET...");

        let url = format!(
            "{}/accounts/{}/payments?limit=10&order=desc",
            HORIZON, MERCHANT_ADDRESS
        );

        if let Ok(resp) = reqwest::get(&url).await {
            if let Ok(json) = resp.json::<serde_json::Value>().await {

                if let Some(records) = json["_embedded"]["records"].as_array() {
                    for payment in records {

                        if payment["type"] != "payment" {
                            continue;
                        }

                        let to = payment["to"].as_str().unwrap_or("");
                        if to != MERCHANT_ADDRESS {
                            continue;
                        }

                        let tx_hash = payment["transaction_hash"].as_str().unwrap_or("");
                        let amount: f64 = payment["amount"]
                            .as_str()
                            .unwrap_or("0")
                            .parse()
                            .unwrap_or(0.0);

                        // buscar memo
                        let tx_url = format!("{}/transactions/{}", HORIZON, tx_hash);

                        let tx_resp = match reqwest::get(&tx_url).await {
                            Ok(r) => r,
                            Err(_) => continue,
                        };

                        let tx_json: serde_json::Value = match tx_resp.json().await {
                            Ok(j) => j,
                            Err(_) => continue,
                        };

                        let memo = tx_json["memo"].as_str().unwrap_or("");

                        let order = sqlx::query_as::<_, Order>(
                            "SELECT * FROM orders WHERE id = ?"
                        )
                        .bind(memo)
                        .fetch_optional(&pool)
                        .await
                        .unwrap();

                        if let Some(order) = order {

                            if order.status == "confirmed" {
                                continue;
                            }

                            if (order.valor_xlm.unwrap_or(0.0) - amount).abs() > 0.01 {
                                continue;
                            }

                            println!("💰 TESTNET pagamento detectado: {}", memo);

                            sqlx::query(
                                "UPDATE orders 
                                 SET status = 'confirmed', tx_hash = ? 
                                 WHERE id = ?"
                            )
                            .bind(tx_hash)
                            .bind(memo)
                            .execute(&pool)
                            .await
                            .unwrap();
                        }
                    }
                }
            }
        }

        sleep(Duration::from_secs(10)).await;
    }
}

// ------------------ MAIN ------------------

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = SqlitePool::connect("sqlite:slippay.db").await
        .expect("Erro DB");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS orders (
            id TEXT PRIMARY KEY,
            valor_brl REAL,
            valor_xlm REAL,
            status TEXT,
            memo TEXT,
            tx_hash TEXT UNIQUE
        )"
    )
    .execute(&pool)
    .await
    .unwrap();

    let listener_pool = pool.clone();
    tokio::spawn(async move {
        stellar_listener(listener_pool).await;
    });

    println!("🚀 SlipPay TESTNET rodando + listener ativo");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/orders", web::post().to(create_order))
            .route("/orders", web::get().to(list_orders))
    })
    .bind(("127.0.0.1", 8081))? // 🔥 PORTA ALTERADA
    .run()
    .await
}

