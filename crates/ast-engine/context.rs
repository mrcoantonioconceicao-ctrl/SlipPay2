#[derive(Debug, Clone)]
pub struct PaymentContext {
    pub amount: u64, // Consider a dedicated decimal type or integer for smallest unit
    pub country: String,
    pub asset: String,
    pub risk_score: rust_decimal::Decimal,
}
