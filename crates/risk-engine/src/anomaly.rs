use serde::{Deserialize, Serialize};

/// Payload contendo o histórico do cliente necessário para o cálculo estatístico de anomalias
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyContext {
    pub historical_amounts: Vec<f64>,
    pub current_transaction_amount: f64,
    pub allowed_deviation_factor: f64, // Quantidade de desvios padrões tolerados (Ex: 2.5 ou 3.0)
}

/// Resposta analítica do motor de anomalias comportamentais
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyResult {
    pub is_anomalous: bool,
    pub average_amount: f64,
    pub deviation_calculated: f64,
    pub message: String,
}

/// Analisa se o valor atual representa uma anomalia estatística severa com base no histórico fornecido
pub fn detect_amount_anomaly(context: &AnomalyContext) -> AnomalyResult {
    let history = &context.historical_amounts;

    // Se o usuário não tem histórico suficiente (menos de 3 transações), não aplicamos o desvio estatístico por segurança
    if history.len() < 3 {
        return AnomalyResult {
            is_anomalous: false,
            average_amount: history.iter().sum::<f64>() / history.len().max(1) as f64,
            deviation_calculated: 0.0,
            message: "Histórico insuficiente para análise estatística de desvio padrão.".to_string(),
        };
    }

    // 1. Calcula a Média Aritmética
    let count = history.len() as f64;
    let sum: f64 = history.iter().sum();
    let mean = sum / count;

    // 2. Calcula a Variância
    let variance: f64 = history.iter()
        .map(|&amount| {
            let diff = amount - mean;
            diff * diff
        })
        .sum::<f64>() / count;

    // 3. Calcula o Desvio Padrão
    let std_deviation = variance.sqrt();

    // 4. Determina o teto máximo aceitável para esta transação
    let upper_bound = mean + (context.allowed_deviation_factor * std_deviation);

    // Se o desvio padrão for extremamente baixo (valores idênticos no histórico),
    // evita falsos positivos adicionando uma tolerância mínima absoluta de segurança
    let is_anomalous = if std_deviation < 1.0 {
        context.current_transaction_amount > (mean * 5.0) // Dispara se for 5x maior que a média linear estável
    } else {
        context.current_transaction_amount > upper_bound
    };

    let message = if is_anomalous {
        format!(
            "Anomalia detectada: O valor de R${:.2} excede o limite estatístico de R${:.2} (Média: R${:.2}, Desvio: {:.2})",
            context.current_transaction_amount, upper_bound, mean, std_deviation
        )
    } else {
        "Comportamento financeiro dentro da curva de normalidade do perfil.".to_string()
    };

    AnomalyResult {
        is_anomalous,
        average_amount: mean,
        deviation_calculated: std_deviation,
        message,
    }
}

