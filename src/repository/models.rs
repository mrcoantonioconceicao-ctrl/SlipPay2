use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    pub id: String,
    pub memo: String,
    pub amount_brl: f64,
    pub amount_xlm: f64,
    pub destination: String,
    pub tx_hash: Option<String>,
    pub status: String,
    pub created_at: String,
    pub confirmed_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentEvent {
    pub id: i64,
    pub payment_id: String,
    pub event_type: String,
    pub payload: Option<String>,
    pub created_at: String,
}
