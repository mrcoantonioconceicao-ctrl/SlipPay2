// =========================================================================
// ARCHITECTURE CORE MODULES (SLIPPAY APPLICATION LAYER)
// =========================================================================

/// Camada de Configurações Globais do Sistema (Stellar, DB, Segurança, Observabilidade)
pub mod config;

/// Camada de Domínio (Entidades e Regras de Negócio Puras e Não-Custodiais)
pub mod domain {
    pub mod payment;
    pub mod invoice;
    pub mod settlement;
    pub mod merchant;
}

/// Data Transfer Objects (Contratos de Entrada e Saída das APIs e Webhooks)
pub mod dto;

/// Adaptadores de Infraestrutura Externa (Rede Stellar/Horizon, SQLite, Pix, Compliance)
pub mod adapters {
    pub mod stellar;
    pub mod database;
    pub mod external;
}

/// Interfaces de Entrada de Dados (Rotas HTTP REST da API e Listeners de Webhooks)
pub mod interfaces {
    pub mod api;
    pub mod webhook;
}

/// Orquestradores e Serviços de Fluxo de Caixa, Reconciliação e Ingestão de Dados
pub mod services {
    pub mod ingestion;
    pub mod reconciliation;
    pub mod orchestration;
    pub mod security;
}

/// Repositórios de Acesso a Dados e Persistência de Estado
pub mod repositories;

/// Tratamento de Erros Tipados e Tratados por Camada
pub mod errors;

/// Observabilidade (Métricas Prometheus, Tracing Distribuído e Logs Estruturados)
pub mod observability;

/// Utilitários Globais (Criptografia Auxiliar, Manipulação de Tempo e Encodings)
pub mod utils;

// =========================================================================
// WORKSPACE INTEGRATION VERIFIER
// =========================================================================

/// Função de inicialização e diagnóstico para validar o acoplamento correto 
/// das crates do Workspace com o Core Engine.
pub fn initialize_application_core() {
    tracing::info!("Inicializando o Core do SlipPay...");
    
    // Testando o acoplamento e prontidão das sub-crates
    ast_engine::info();
    pq_security::info();
    risk_engine::info();
    policy_engine::info();
    
    tracing::info!("Todos os motores auxiliares do Workspace foram acoplados com sucesso.");
}

