use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Charge {
    pub id: String,
    pub memo: String,
    pub amount: String,
    pub asset: String,
    pub status: String,
    pub created_at: String,
    pub tx_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateChargeRequest {
    pub amount: String,
    pub asset: String,
}
