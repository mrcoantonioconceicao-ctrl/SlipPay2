use serde::{Deserialize, Serialize};

const HORIZON_URL: &str =
    "https://horizon-testnet.stellar.org";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentRecord {

    pub id: String,

    #[serde(rename = "transaction_hash")]
    pub tx_hash: String,

    pub memo: Option<String>,

    pub successful: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Embedded {

    pub records: Vec<PaymentRecord>,
}

#[derive(Debug, Deserialize)]
pub struct PaymentsResponse {

    #[serde(rename = "_embedded")]
    pub embedded: Embedded,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {

    pub successful: bool,

    pub memo: String,
}

pub async fn fetch_transaction(
    tx_hash: &str,
) -> Result<TransactionResponse, String> {

    let url = format!(
        "{}/transactions/{}",
        HORIZON_URL,
        tx_hash
    );

    let response =
        reqwest::get(&url)
            .await
            .map_err(|e| e.to_string())?;

    if !response.status().is_success() {

        return Err(format!(
            "stellar tx not found: {}",
            tx_hash
        ));
    }

    let tx =
        response
            .json::<TransactionResponse>()
            .await
            .map_err(|e| e.to_string())?;

    Ok(tx)
}

pub async fn fetch_latest_payments(
) -> Result<Vec<PaymentRecord>, String> {

    let url = format!(
        "{}/payments?limit=10&order=desc",
        HORIZON_URL
    );

    let response =
        reqwest::get(&url)
            .await
            .map_err(|e| e.to_string())?;

    if !response.status().is_success() {

        return Err(
            "failed fetching payments"
                .to_string()
        );
    }

    let data =
        response
            .json::<PaymentsResponse>()
            .await
            .map_err(|e| e.to_string())?;

    Ok(data.embedded.records)
}
