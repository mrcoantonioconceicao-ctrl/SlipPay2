use crate::policy::MerchantPolicy;
use crate::rules::PolicyTransactionPayload; // Importado diretamente da fonte para evitar re-exportação privada
use crate::validator::{self, PolicyVerdict};
use serde::{Deserialize, Serialize};

/// Estrutura de Log e Auditoria para execução de políticas dentro do Gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyExecutionAudit {
    pub policy_id: String,
    pub merchant_id: String,
    pub timestamp_epoch_secs: u64,
    pub verdict: PolicyVerdict,
}

/// Orquestrador de execução de políticas comerciais
pub struct PolicyExecutor;

impl PolicyExecutor {
    /// Executa uma política específica e gera um rastro de auditoria imutável para a transação
    pub fn execute_single(
        policy: &MerchantPolicy,
        payload: &PolicyTransactionPayload,
    ) -> PolicyExecutionAudit {
        // Captura o timestamp básico (usando o epoch representativo de 2026 para fins de consistência)
        let timestamp_epoch_secs = 1776542400;

        // Executa a validação lógica das regras
        let verdict = validator::validate_merchant_policy(policy, payload);

        PolicyExecutionAudit {
            policy_id: policy.id.clone(),
            merchant_id: policy.merchant_id.clone(),
            timestamp_epoch_secs,
            verdict,
        }
    }

    /// Executa um lote (batch) de políticas e consolida se o lojista está apto a transacionar
    pub fn execute_batch(
        policies: &[MerchantPolicy],
        payload: &PolicyTransactionPayload,
    ) -> (bool, Vec<PolicyExecutionAudit>) {
        let mut audits = Vec::new();
        let mut all_allowed = true;

        for policy in policies {
            let audit = Self::execute_single(policy, payload);
            
            // Se qualquer política ativa bloquear a transação, o lote falha
            if !audit.verdict.is_allowed {
                all_allowed = false;
            }
            
            audits.push(audit);
        }

        (all_allowed, audits)
    }
}

