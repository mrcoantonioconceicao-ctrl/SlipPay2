#[derive(Debug, Clone)]
pub struct PaymentContext {
    pub amount: f64,
    pub country: String,
    pub asset: String,
    pub risk_score: f32,
}
