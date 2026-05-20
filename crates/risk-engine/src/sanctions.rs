use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Define os tipos de restrições ou órgãos emissores da sanção
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SanctionType {
    OfacTerrorism,
    OnuSecurityCouncil,
    InternalBlacklist,
    PldComplianceRisk,
}

/// Payload contendo os dados do indivíduo ou nó de rede a ser verificado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanctionCheckPayload {
    pub document: String,        // CPF, CNPJ ou identificador nacional sanitizado
    pub wallet_address: String,  // Chave pública Stellar do nó transacionador
    pub country_code: String,    // Código ISO do país (Ex: "BR", "US", "IR")
}

/// Resposta analítica do motor de sanções e compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanctionMatchResult {
    pub is_blocked: bool,
    pub matches: Vec<SanctionType>,
    pub explanation: String,
}

/// Motor de conformidade regulatória para barramento de entidades sancionadas
pub struct SanctionsChecker {
    blocked_documents: HashSet<String>,
    blocked_wallets: HashSet<String>,
    embargoed_countries: HashSet<String>,
}

impl SanctionsChecker {
    /// Inicializa o validador populando as listas negras estáticas de teste (Mocks)
    pub fn new_with_defaults() -> Self {
        let mut docs = HashSet::new();
        let mut wallets = HashSet::new();
        let mut countries = HashSet::new();

        // Exemplos simulados para testes de mesa do gateway
        docs.insert("00000000000".to_string()); // CPF mock de teste bloqueado internamente
        docs.insert("99999999999999".to_string()); // CNPJ mock de teste
        
        // Simulação de endereço Stellar de hacker ou nó sancionado
        wallets.insert("GBLACKHOLEMAXWELLSTRUCTUREDED25519STELLARSUITE".to_string());

        // Países sob forte embargo internacional (padrão GAFI/FATF)
        countries.insert("KP".to_string()); // Coreia do Norte
        countries.insert("IR".to_string()); // Irã

        SanctionsChecker {
            blocked_documents: docs,
            blocked_wallets: wallets,
            embargoed_countries: countries,
        }
    }

    /// Executa a triagem estrita contra múltiplos vetores de sanções internacionais
    pub fn check_compliance(&self, payload: &SanctionCheckPayload) -> SanctionMatchResult {
        let mut matches = Vec::new();

        // 1. Verifica bloqueio de documento (CPF/CNPJ)
        if self.blocked_documents.contains(&payload.document) {
            matches.push(SanctionType::InternalBlacklist);
        }

        // 2. Verifica bloqueio na camada blockchain (Stellar Public Key)
        if self.blocked_wallets.contains(&payload.wallet_address) {
            matches.push(SanctionType::OfacTerrorism);
        }

        // 3. Verifica se a origem geográfica pertence a um país sob embargo econômico
        if self.embargoed_countries.contains(&payload.country_code) {
            matches.push(SanctionType::OnuSecurityCouncil);
        }

        let is_blocked = !matches.is_empty();
        let explanation = if is_blocked {
            format!(
                "Transação negada imediatamente. A entidade acionou alertas de conformidade em: {:?}",
                matches
            )
        } else {
            "Nenhuma restrição ou sanção encontrada para os dados fornecidos.".to_string()
        };

        SanctionMatchResult {
            is_blocked,
            matches,
            explanation,
        }
    }
}

