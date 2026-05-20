use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Status operacional de uma política de governança no gateway
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PolicyStatus {
    Active,
    Inactive,
    Testing, // Roda em modo "Dry-Run" (apenas loga, não bloqueia)
}

/// Nível de criticidade caso a política seja violada
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PolicySeverity {
    Low,
    Medium,
    High,
    Critical, // Bloqueio imediato da conta/merchant
}

/// Estrutura principal que define uma Política de Governança Comercial no SlipPay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerchantPolicy {
    pub id: String,
    pub merchant_id: String,
    pub name: String,
    pub status: PolicyStatus,
    pub severity: PolicySeverity,
    pub max_limit_per_transaction: f64,
    pub max_daily_limit: f64,
    pub allowed_transaction_types: Vec<String>, // Ex: ["Pix", "Stellar.USDC", "Stellar.XLM"]
    pub custom_metadata: HashMap<String, String>,
}

impl MerchantPolicy {
    /// Cria uma política padrão restritiva para novos lojistas em fase de onboarding
    pub fn new_sandbox(merchant_id: &str) -> Self {
        let mut metadata = HashMap::new();
        metadata.insert("tier".to_string(), "sandbox".to_string());
        metadata.insert("kyc_status".to_string(), "pending".to_string());

        MerchantPolicy {
            id: format!("pol_sb_{}", merchant_id),
            merchant_id: merchant_id.to_string(),
            name: "Política Padrão de Onboarding (Sandbox)".to_string(),
            status: PolicyStatus::Testing,
            severity: PolicySeverity::Medium,
            max_limit_per_transaction: 500.0, // Limite baixo inicial para testes
            max_daily_limit: 2500.0,
            allowed_transaction_types: vec!["Pix".to_string(), "Stellar.USDC".to_string()],
            custom_metadata: metadata,
        }
    }
}

