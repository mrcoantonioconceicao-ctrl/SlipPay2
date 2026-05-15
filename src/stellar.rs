use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Debug, Deserialize)]
pub struct HorizonResponse {
    pub _embedded: Embedded,
}

#[derive(Debug, Deserialize)]
pub struct Embedded {
    pub records: Vec<PaymentRecord>,
}

#[derive(Debug, Deserialize)]
pub struct PaymentRecord {

    pub paging_token: Option<String>,

    pub transaction_hash: Option<String>,

    pub amount: Option<String>,

    pub asset_type: Option<String>,

    pub from: Option<String>,

    pub to: Option<String>,

    pub created_at: Option<String>,

    #[serde(rename = "type")]
    pub operation_type: Option<String>,
}

pub async fn start_listener(
    db: SqlitePool,
    horizon_url: String,
    public_key: String,
) {

    let client = reqwest::Client::new();

    let mut cursor = "now".to_string();

    loop {

        let url = format!(
            "{}/accounts/{}/payments?cursor={}&limit=10&order=asc",
            horizon_url,
            public_key,
            cursor
        );

        println!("🔍 Consultando Horizon...");
        println!("🌐 URL: {}", url);

        let response = client
            .get(&url)
            .send()
            .await;

        match response {

            Ok(resp) => {

                let text = resp.text().await.unwrap();

                println!("📦 Horizon RAW: {}", text);

                let parsed: Result<HorizonResponse, _> =
                    serde_json::from_str(&text);

                match parsed {

                    Ok(data) => {

                        println!(
                            "✅ {} pagamentos encontrados",
                            data._embedded.records.len()
                        );

                        for payment in data._embedded.records {

                            println!(
                                "📦 TX: {}",
                                payment.transaction_hash
                                    .clone()
                                    .unwrap_or_default()
                            );

                            println!(
                                "💰 Valor: {} {}",
                                payment.amount
                                    .clone()
                                    .unwrap_or_default(),
                                payment.asset_type
                                    .clone()
                                    .unwrap_or_default()
                            );

                            let tx_hash =
                                payment.transaction_hash
                                    .clone()
                                    .unwrap_or_default();

                            let exists =
                                sqlx::query(
                                    "SELECT tx_hash FROM payments WHERE tx_hash = ?"
                                )
                                .bind(&tx_hash)
                                .fetch_optional(&db)
                                .await
                                .unwrap();

                            if exists.is_none() {

                                sqlx::query(
                                    r#"
                                    INSERT INTO payments (
                                        tx_hash,
                                        amount,
                                        asset_type,
                                        sender,
                                        receiver,
                                        created_at
                                    )
                                    VALUES (?, ?, ?, ?, ?, ?)
                                    "#
                                )
                                .bind(&tx_hash)
                                .bind(
                                    payment.amount.unwrap_or_default()
                                )
                                .bind(
                                    payment.asset_type.unwrap_or_default()
                                )
                                .bind(
                                    payment.from.unwrap_or_default()
                                )
                                .bind(
                                    payment.to.unwrap_or_default()
                                )
                                .bind(
                                    payment.created_at.unwrap_or_default()
                                )
                                .execute(&db)
                                .await
                                .unwrap();

                                println!("✅ reconciliado");
                            }

                            if let Some(token) =
                                payment.paging_token {

                                cursor = token;
                            }
                        }
                    }

                    Err(err) => {
                        println!("❌ parse error: {}", err);
                    }
                }
            }

            Err(err) => {
                println!("❌ request error: {}", err);
            }
        }

        tokio::time::sleep(
            tokio::time::Duration::from_secs(5)
        )
        .await;
    }
}
