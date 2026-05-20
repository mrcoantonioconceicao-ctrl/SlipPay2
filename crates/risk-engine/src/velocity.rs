use std::collections::HashMap;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

/// Define os limites de velocidade permitidos pelo gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VelocityLimits {
    pub max_transactions_per_minute: usize,
    pub max_volume_per_minute: f64,
}

/// Registro histórico das transações recentes de um determinado usuário/merchant
/// Nota: Mantemos Instant fora da serialização direta pois ele mede tempo de CPU local.
#[derive(Debug, Clone)]
pub struct UserTransactionHistory {
    pub timestamps: Vec<Instant>,
    pub total_volume_in_window: f64,
}

/// Gerenciador de memória volátil para análise estatística de velocidade em tempo real
pub struct VelocityTracker {
    pub registry: HashMap<String, UserTransactionHistory>,
    pub limits: VelocityLimits,
}

impl VelocityTracker {
    /// Inicializa o rastreador com limites padrão de segurança bancária
    pub fn new(max_txs: usize, max_vol: f64) -> Self {
        VelocityTracker {
            registry: HashMap::new(),
            limits: VelocityLimits {
                max_transactions_per_minute: max_txs,
                max_volume_per_minute: max_vol,
            },
        }
    }

    /// Avalia se a nova transação viola os limites de velocidade.
    /// Retorna `true` se a transação for permitida, ou `false` se estourar o limite.
    pub fn evaluate_and_track(&mut self, user_id: &str, amount: f64) -> bool {
        let now = Instant::now();
        let one_minute_ago = now - Duration::from_secs(60);

        // Busca ou cria o histórico do usuário
        let history = self.registry.entry(user_id.to_string()).or_insert(UserTransactionHistory {
            timestamps: Vec::new(),
            total_volume_in_window: 0.0,
        });

        // 1. Limpa registros mais velhos que 60 segundos (janela deslizante)
        history.timestamps.retain(|&time| time > one_minute_ago);

        // 2. Verifica se a quantidade de transações na última janela estoura o limite
        if history.timestamps.len() >= self.limits.max_transactions_per_minute {
            return false;
        }

        // 3. Verifica se o volume financeiro acumulado no último minuto estoura o teto
        if history.total_volume_in_window + amount > self.limits.max_volume_per_minute {
            return false;
        }

        // Se passou em todas as regras de velocidade, registra a transação atual
        history.timestamps.push(now);
        history.total_volume_in_window += amount;
        
        true
    }
}

