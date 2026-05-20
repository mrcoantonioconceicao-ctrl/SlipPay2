use crate::policy::{MerchantPolicy, PolicyStatus, PolicySeverity};
use crate::rules::{self, PolicyTransactionPayload, RuleEvaluation};
use serde::{Deserialize, Serialize};

/// Resultado consolidado de todas as políticas aplicadas sobre a operação
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyVerdict {
    pub is_allowed: bool,
    pub severity: Option<PolicySeverity>,
    pub evaluations: Vec<RuleEvaluation>,
    pub summary: String,
}

/// Avalia uma transação contra a política comercial de um lojista
pub fn validate_merchant_policy(policy: &MerchantPolicy, payload: &PolicyTransactionPayload) -> PolicyVerdict {
    // 1. Se a política estiver inativa, ela é ignorada pelo gateway (Short-circuit amigável)
    if policy.status == PolicyStatus::Inactive {
        return PolicyVerdict {
            is_allowed: true,
            severity: None,
            evaluations: Vec::new(),
            summary: "Políticas comerciais inativas para este lojista. Liberação automática.".to_string(),
        };
    }

    // 2. Executa a esteira de validações sequenciais do rules.rs
    let mut evaluations = Vec::new();
    
    evaluations.push(rules::check_transaction_amount(policy, payload));
    evaluations.push(rules::check_daily_volume_limit(policy, payload));
    evaluations.push(rules::check_allowed_methods(policy, payload));

    // 3. Analisa se houve alguma violação contratual nas regras executadas
    let has_violations = evaluations.iter().any(|eval| !eval.passed);

    // 4. Monta o veredito baseado no status operacional (Active ou Testing)
    let is_allowed = if policy.status == PolicyStatus::Testing {
        // Modo Sandbox/Testing: Mesmo se falhar nas regras, permite a transação mas gera o alerta no log
        true
    } else {
        !has_violations
    };

    let severity = if has_violations { Some(policy.severity.clone()) } else { None };

    let summary = match (has_violations, &policy.status) {
        (true, PolicyStatus::Testing) => {
            "Transação violou regras de limites, mas foi permitida devido ao modo Sandbox/Testing.".to_string()
        }
        (true, PolicyStatus::Active) => {
            "Transação REJEITADA. Violação estrita de políticas de governança comercial.".to_string()
        }
        (false, _) => {
            "Transação aprovada e em total conformidade com os limites comerciais.".to_string()
        }
        _ => "Análise concluída.".to_string(),
    };

    PolicyVerdict {
        is_allowed,
        severity,
        evaluations,
        summary,
    }
}

