#[derive(Debug, Clone)]
pub struct PaymentContext {
    pub amount: rust_decimal::Decimal,
    pub country: String,
    pub asset: String,
    pub risk_score: rust_decimal::Decimal,
}
