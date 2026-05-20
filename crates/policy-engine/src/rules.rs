use crate::policy::MerchantPolicy;

use serde::{Deserialize, Serialize};

/// Resultado individual de uma regra de governança aplicada
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleEvaluation {
    pub rule_name: String,
    pub passed: bool,
    pub current_value: String,
    pub limit_value: String,
    pub violation_message: Option<String>,
}

/// Payload de dados brutos da transação atual para validação de limites comerciais
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyTransactionPayload {
    pub transaction_type: String, // Ex: "Pix", "Stellar.USDC"
    pub amount: f64,
    pub daily_accumulated_volume: f64,
}

/// Valida se a transação atende ao limite individual configurado na política
pub fn check_transaction_amount(policy: &MerchantPolicy, payload: &PolicyTransactionPayload) -> RuleEvaluation {
    let passed = payload.amount <= policy.max_limit_per_transaction;
    
    let violation_message = if !passed {
        Some(format!(
            "O valor da transação (R${:.2}) excede o limite máximo individual permitido de R${:.2}",
            payload.amount, policy.max_limit_per_transaction
        ))
    } else {
        None
    };

    RuleEvaluation {
        rule_name: "Limitação de Valor Unitário".to_string(),
        passed,
        current_value: format!("R${:.2}", payload.amount),
        limit_value: format!("R${:.2}", policy.max_limit_per_transaction),
        violation_message,
    }
}

/// Valida se o volume acumulado do dia (somado com a transação atual) estoura o teto diário do lojista
pub fn check_daily_volume_limit(policy: &MerchantPolicy, payload: &PolicyTransactionPayload) -> RuleEvaluation {
    let projected_volume = payload.daily_accumulated_volume + payload.amount;
    let passed = projected_volume <= policy.max_daily_limit;

    let violation_message = if !passed {
        Some(format!(
            "O volume diário projetado (R${:.2}) excede o teto diário autorizado de R${:.2}",
            projected_volume, policy.max_daily_limit
        ))
    } else {
        None
    };

    RuleEvaluation {
        rule_name: "Limitação de Volume Acumulado Diário".to_string(),
        passed,
        current_value: format!("R${:.2}", projected_volume),
        limit_value: format!("R${:.2}", policy.max_daily_limit),
        violation_message,
    }
}

/// Valida se a rota financeira/método de pagamento escolhido é suportada pelo contrato do lojista
pub fn check_allowed_methods(policy: &MerchantPolicy, payload: &PolicyTransactionPayload) -> RuleEvaluation {
    let passed = policy.allowed_transaction_types.contains(&payload.transaction_type);

    let violation_message = if !passed {
        Some(format!(
            "O método/moeda '{}' não está habilitado para este lojista. Métodos aceitos: {:?}",
            payload.transaction_type, policy.allowed_transaction_types
        ))
    } else {
        None
    };

    RuleEvaluation {
        rule_name: "Método de Liquidação Autorizado".to_string(),
        passed,
        current_value: payload.transaction_type.clone(),
        limit_value: format!("{:?}", policy.allowed_transaction_types),
        violation_message,
    }
}

