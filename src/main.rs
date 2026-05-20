use std::collections::HashMap;

use pq_security::wallet::PqWalletVault;
use risk_engine::fraud::FraudEvaluator;
use risk_engine::scoring::RiskPayload;
use risk_engine::anomaly::AnomalyContext;
use risk_engine::sanctions::SanctionCheckPayload;
use policy_engine::policy::MerchantPolicy;
use policy_engine::rules::PolicyTransactionPayload;
use policy_engine::execution::PolicyExecutor;

// 🔗 Importação milimétrica baseada no real código da sua crate AST
use ast_engine::ast::{Node, Operator, Value};
use ast_engine::evaluator::evaluate_ast;

fn main() {
    println!("====================================================");
    println!(" 🚀 INICIANDO SLIPPAY CORE - INTEGRAÇÃO TOTAL (4/4)");
    println!("====================================================\n");

    // ---------------------------------------------------------
    // 1. GERANDO IDENTIDADE HÍBRIDA (CLÁSSICA + QUÂNTICA)
    // ---------------------------------------------------------
    println!("[1/5] 🔐 Gerando Cofre Criptográfico SlipPay...");
    let _wallet = PqWalletVault::generate_new();
    println!("      ✅ Chave Clássica (Ed25519) gerada.");
    println!("      ✅ Chave Kyber-1024 gerada para blindagem.");
    println!("      ✅ Chave Dilithium5 gerada para assinatura pós-quântica.\n");

    // ---------------------------------------------------------
    // 2. CONFIGURAÇÃO DE MOTORES E CONTRATOS
    // ---------------------------------------------------------
    println!("[2/5] ⚙️  Inicializando Motores e Políticas...");
    
    let mut risk_evaluator = FraudEvaluator::new(15, 50000.0);
    let merchant_policy = MerchantPolicy::new_sandbox("merch_live_999");
    
    println!("      ✅ Motor Antifraude calibrado.");
    println!("      ✅ Política Comercial [{}] carregada.\n", merchant_policy.name);

    // ---------------------------------------------------------
    // 3. SIMULAÇÃO DE UMA TRANSAÇÃO ENTRANTE (R$ 1.500 via PIX)
    // ---------------------------------------------------------
    println!("[3/5] 💸 Processando nova transação (Pix: R$ 1.500,00)...");
    let transaction_amount = 1500.0;
    
    let risk_payload = RiskPayload {
        transaction_amount,
        device_reputation: 0.95,
        user_history_score: 0.80,
        is_anomalous_location: false,
    };

    let anomaly_ctx = AnomalyContext {
        historical_amounts: vec![1200.0, 1400.0, 1350.0, 1600.0],
        current_transaction_amount: transaction_amount,
        allowed_deviation_factor: 3.0,
    };

    let sanction_payload = SanctionCheckPayload {
        document: "01234567890".to_string(),
        wallet_address: "G_CLEAN_STELLAR_WALLET".to_string(),
        country_code: "BR".to_string(),
    };

    let policy_payload = PolicyTransactionPayload {
        transaction_type: "Pix".to_string(),
        amount: transaction_amount,
        daily_accumulated_volume: 500.0,
    };

    // ---------------------------------------------------------
    // 4. EXECUÇÃO DA ESTEIRA (RISCO + POLÍTICA + AST)
    // ---------------------------------------------------------
    println!("\n[4/5] ⚖️  Calculando Vereditos...");

    // Avaliação de Fraude (risk-engine)
    let fraud_verdict = risk_evaluator.evaluate_transaction(
        "cliente_abc",
        &risk_payload,
        &anomaly_ctx,
        &sanction_payload
    );

    // Avaliação de Política (policy-engine)
    let policy_audit = PolicyExecutor::execute_single(&merchant_policy, &policy_payload);

    // Avaliação Dinâmica (ast-engine)
    // Regra estruturada: "amount <= 2000.0"
    let ast_rule = Node::Condition {
        field: "amount".to_string(),
        operator: Operator::LessThanOrEqual,
        value: Value::Number(2000.0),
    };

    // Mapeamento do contexto transacional em tempo real
    let mut ast_context = HashMap::new();
    ast_context.insert("amount".to_string(), Value::Number(transaction_amount));

    // O compilador AST processa a regra contra o cenário
    let ast_verdict = evaluate_ast(&ast_rule, &ast_context);

    // ---------------------------------------------------------
    // 5. RESULTADOS MENSURÁVEIS
    // ---------------------------------------------------------
    println!("\n================ RESULTADOS ========================");
    println!("🛡️  ANTIFRAUDE (Risk-Engine)");
    println!("   Ação Recomendada : {:?}", fraud_verdict.action);
    println!("   Score de Risco   : {:.2} (0 a 100)", fraud_verdict.risk_score);
    println!("   Motivo           : {}", fraud_verdict.reason);
    
    println!("\n📜 GOVERNANÇA (Policy-Engine)");
    println!("   Permitido?       : {}", if policy_audit.verdict.is_allowed { "SIM ✅" } else { "NÃO ❌" });
    
    println!("\n🧠 REGRAS DINÂMICAS (AST-Engine)");
    println!("   Regra AST Lida   : amount <= 2000.0");
    println!("   Valor Analisado  : {}", transaction_amount);
    println!("   Resultado AST    : {}", if ast_verdict { "APROVADO ✅" } else { "BLOQUEADO ❌" });
    println!("====================================================\n");
}

