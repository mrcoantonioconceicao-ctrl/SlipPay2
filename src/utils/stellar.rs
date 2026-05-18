use reqwest;
use serde_json::Value;
use tokio::time::{sleep, Duration};

pub async fn monitor_payments(wallet: &str) {

    println!("🚀 Monitor Stellar iniciado");

    let client = reqwest::Client::new();

    // cursor persistente
    let mut cursor = String::from("now");

    loop {

        println!("🧠 CURSOR ATUAL: {}", cursor);

        let url = format!(
            "https://horizon-testnet.stellar.org/accounts/{}/operations?cursor={}&limit=10&order=asc",
            wallet,
            cursor
        );

        println!("🔍 Consultando Horizon...");
        println!("🌐 URL: {}", url);

        match client.get(&url).send().await {

            Ok(response) => {

                match response.text().await {

                    Ok(text) => {

                        println!("📦 Horizon RAW: {}", text);

                        let parsed: Result<Value, _> =
                            serde_json::from_str(&text);

                        match parsed {

                            Ok(json) => {

                                let records =
                                    json["_embedded"]["records"]
                                        .as_array()
                                        .unwrap_or(&vec![])
                                        .to_vec();

                                println!(
                                    "✅ {} operações encontradas",
                                    records.len()
                                );

                                for record in records {

                                    // salva cursor NOVO
                                    if let Some(token) =
                                        record["paging_token"]
                                            .as_str()
                                    {
                                        println!(
                                            "💾 NOVO CURSOR: {}",
                                            token
                                        );

                                        cursor =
                                            token.to_string();
                                    }

                                    let op_type =
                                        record["type"]
                                            .as_str()
                                            .unwrap_or("");

                                    // filtra apenas payments
                                    if op_type != "payment" {
                                        continue;
                                    }

                                    println!(
                                        "💸 PAGAMENTO DETECTADO"
                                    );

                                    println!(
                                        "🧾 HASH: {}",
                                        record["transaction_hash"]
                                            .as_str()
                                            .unwrap_or("")
                                    );

                                    println!(
                                        "💰 VALOR: {}",
                                        record["amount"]
                                            .as_str()
                                            .unwrap_or("")
                                    );

                                    println!(
                                        "📤 FROM: {}",
                                        record["from"]
                                            .as_str()
                                            .unwrap_or("")
                                    );

                                    println!(
                                        "📥 TO: {}",
                                        record["to"]
                                            .as_str()
                                            .unwrap_or("")
                                    );

                                    // busca memo/hash da tx
                                    if let Some(tx_hash) =
                                        record["transaction_hash"]
                                            .as_str()
                                    {
                                        fetch_transaction(
                                            &client,
                                            tx_hash
                                        ).await;
                                    }

                                    println!(
                                        "────────────────────────"
                                    );
                                }
                            }

                            Err(err) => {
                                println!(
                                    "❌ Erro parse JSON: {}",
                                    err
                                );
                            }
                        }
                    }

                    Err(err) => {
                        println!(
                            "❌ Erro lendo body: {}",
                            err
                        );
                    }
                }
            }

            Err(err) => {
                println!(
                    "❌ Erro HTTP: {}",
                    err
                );
            }
        }

        sleep(Duration::from_secs(5)).await;
    }
}

async fn fetch_transaction(
    client: &reqwest::Client,
    tx_hash: &str,
) {

    let url = format!(
        "https://horizon-testnet.stellar.org/transactions/{}",
        tx_hash
    );

    println!("🔎 Buscando TX...");
    println!("🌐 TX URL: {}", url);

    match client.get(&url).send().await {

        Ok(response) => {

            match response.text().await {

                Ok(text) => {

                    let parsed: Result<Value, _> =
                        serde_json::from_str(&text);

                    match parsed {

                        Ok(json) => {

                            println!(
                                "✅ TX SUCCESS: {}",
                                json["successful"]
                            );

                            println!(
                                "🧠 MEMO TYPE: {}",
                                json["memo_type"]
                                    .as_str()
                                    .unwrap_or("")
                            );

                            let memo_base64 =
                                json["memo"]
                                    .as_str()
                                    .unwrap_or("");

                            println!(
                                "📝 MEMO BASE64: {}",
                                memo_base64
                            );

                            match base64::decode(memo_base64) {

                                Ok(bytes) => {

                                    let memo_hex =
                                        hex::encode(bytes);

                                    println!(
                                        "🔐 MEMO HEX: {}",
                                        memo_hex
                                    );
                                }

                                Err(err) => {
                                    println!(
                                        "❌ Erro convertendo memo: {}",
                                        err
                                    );
                                }
                            }
                        }

                        Err(err) => {
                            println!(
                                "❌ Erro parse TX JSON: {}",
                                err
                            );
                        }
                    }
                }

                Err(err) => {
                    println!(
                        "❌ Erro lendo TX: {}",
                        err
                    );
                }
            }
        }

        Err(err) => {
            println!(
                "❌ Erro HTTP TX: {}",
                err
            );
        }
    }
}
