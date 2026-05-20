use serde::{Deserialize, Serialize};

/// Níveis de classificação de risco para tomada de decisão no gateway
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Estrutura que define os parâmetros de entrada para o cálculo do Score de Risco
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RiskPayload {
    pub transaction_amount: f64,
    pub device_reputation: f64,    // 0.0 (Péssimo) a 1.0 (Confiável)
    pub user_history_score: f64,   // 0.0 (Novo/Suspeito) a 1.0 (Excelente)
    pub is_anomalous_location: bool,
}

/// Resultado consolidado da análise de score do Risk-Engine
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RiskScoreResult {
    pub score: f64, // Escala de 0.0 a 100.0
    pub level: RiskLevel,
    pub requires_mfa: bool,
}

/// Calcula a pontuação de risco matemática baseada em pesos ponderados de mercado financeiro
pub fn calculate_risk_score(payload: &RiskPayload) -> RiskScoreResult {
    let mut final_score = 0.0;

    // 1. Peso do Valor da Transação (Até 30 pontos)
    // Transações muito altas elevam o score de risco base de forma progressiva
    if payload.transaction_amount > 10000.0 {
        final_score += 30.0;
    } else if payload.transaction_amount > 5000.0 {
        final_score += 20.0;
    } else if payload.transaction_amount > 1000.0 {
        final_score += 10.0;
    }

    // 2. Peso da Reputação do Dispositivo (Até 25 pontos)
    // Quanto menor a reputação do hardware/IP, maior o risco
    final_score += (1.0 - payload.device_reputation) * 25.0;

    // 3. Peso do Histórico do Usuário (Até 25 pontos)
    // Usuários novos ou com histórico de estornos/problemas elevam o risco
    final_score += (1.0 - payload.user_history_score) * 25.0;

    // 4. Peso de Geolocalização Anômala (Até 20 pontos de punição direta)
    if payload.is_anomalous_location {
        final_score += 20.0;
    }

    // Garante que o score fique travado no limite matemático de 0 a 100
    if final_score > 100.0 {
        final_score = 100.0;
    } else if final_score < 0.0 {
        final_score = 0.0;
    }

    // 5. Classificação do Nível baseado no Score resultante
    let level = match final_score {
        s if s >= 85.0 => RiskLevel::Critical,
        s if s >= 60.0 => RiskLevel::High,
        s if s >= 30.0 => RiskLevel::Medium,
        _ => RiskLevel::Low,
    };

    // Determina se o comportamento exige Step-Up Authentication (MFA adicional)
    let requires_mfa = match level {
        RiskLevel::High | RiskLevel::Critical => true,
        RiskLevel::Medium if payload.transaction_amount > 2000.0 => true,
        _ => false,
    };

    RiskScoreResult {
        score: final_score,
        level,
        requires_mfa,
    }
}

