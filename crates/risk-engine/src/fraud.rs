use crate::scoring::{self, RiskPayload, RiskLevel};
use crate::velocity::{VelocityTracker};
use crate::anomaly::{self, AnomalyContext};
use crate::sanctions::{SanctionsChecker, SanctionCheckPayload};
use serde::{Deserialize, Serialize};

/// Ações recomendadas após processamento completo do motor de fraude
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FraudAction {
    Approve,
    ChallengeMfa,
    ReviewManual,
    Deny,
}

/// Resposta consolidada final do Antifraude do SlipPay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudVerdict {
    pub action: FraudAction,
    pub risk_score: f64,
    pub risk_level: RiskLevel,
    pub reason: String,
}

/// Orquestrador central que roda todas as sub-camadas de análise de fraude
pub struct FraudEvaluator {
    pub sanctions_checker: SanctionsChecker,
    pub velocity_tracker: VelocityTracker,
}

impl FraudEvaluator {
    /// Inicializa o orquestrador configurando as regras e caches locais
    pub fn new(max_txs_per_min: usize, max_vol_per_min: f64) -> Self {
        FraudEvaluator {
            sanctions_checker: SanctionsChecker::new_with_defaults(),
            velocity_tracker: VelocityTracker::new(max_txs_per_min, max_vol_per_min),
        }
    }

    /// Executa a esteira completa de avaliação de risco contra uma tentativa de transação
    pub fn evaluate_transaction(
        &mut self,
        user_id: &str,
        risk_payload: &RiskPayload,
        anomaly_context: &AnomalyContext,
        sanction_payload: &SanctionCheckPayload,
    ) -> FraudVerdict {
        
        // 1. Barreira Criminosa / Compliance (Sanções Internacionais)
        let sanction_res = self.sanctions_checker.check_compliance(sanction_payload);
        if sanction_res.is_blocked {
            return FraudVerdict {
                action: FraudAction::Deny,
                risk_score: 100.0,
                risk_level: RiskLevel::Critical,
                reason: sanction_res.explanation,
            };
        }

        // 2. Barreira Comportamental Rápida (Velocity / Rate Limit Financeiro)
        let velocity_ok = self.velocity_tracker.evaluate_and_track(user_id, risk_payload.transaction_amount);
        if !velocity_ok {
            return FraudVerdict {
                action: FraudAction::Deny,
                risk_score: 90.0,
                risk_level: RiskLevel::High,
                reason: "Bloqueio por estouro de limite de velocidade transacional por minuto.".to_string(),
            };
        }

        // 3. Análise Estatística de Curva Histórica (Desvio Padrão)
        let anomaly_res = anomaly::detect_amount_anomaly(anomaly_context);
        
        // 4. Pontuação Base Ponderada (Scoring Engine)
        let mut score_res = scoring::calculate_risk_score(risk_payload);

        // Se houver anomalia estatística severa, penaliza o score base adicionando 25 pontos
        if anomaly_res.is_anomalous {
            score_res.score = (score_res.score + 25.0).min(100.0);
            score_res.requires_mfa = true;
        }

        // 5. Tomada de Decisão Baseada no Score Final Consolidado
        let action = match score_res.level {
            RiskLevel::Critical => FraudAction::Deny,
            RiskLevel::High => FraudAction::ChallengeMfa,
            RiskLevel::Medium => {
                if score_res.requires_mfa {
                    FraudAction::ChallengeMfa
                } else {
                    FraudAction::ReviewManual
                }
            }
            RiskLevel::Low => FraudAction::Approve,
        };

        let reason = if anomaly_res.is_anomalous {
            format!("Análise concluída. Punição aplicada: {}", anomaly_res.message)
        } else {
            "Análise concluída sem desvios severos identificados.".to_string()
        };

        FraudVerdict {
            action,
            risk_score: score_res.score,
            risk_level: score_res.level,
            reason,
        }
    }
}

