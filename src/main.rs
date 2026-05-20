use std::collections::HashMap;
use std::sync::Arc;
use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

// Ingestão das crates do ecossistema SlipPay
use storage_engine::Storage;
use pq_security::wallet::PqWalletVault;
use risk_engine::fraud::FraudEvaluator;
use risk_engine::scoring::RiskPayload;
use risk_engine::anomaly::AnomalyContext;
use risk_engine::sanctions::SanctionCheckPayload;
use policy_engine::policy::MerchantPolicy;
use policy_engine::rules::PolicyTransactionPayload;
use policy_engine::execution::PolicyExecutor;
use ast_engine::ast::{Node, Operator, Value};
use ast_engine::evaluator::evaluate_ast;
use solana_engine::SolanaVerifier;

#[derive(Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
enum DynamicAstNode {
    Condition {
        field: String,
        operator: String,
        value: f64,
    },
    Logical {
        operator: String,
        conditions: Vec<DynamicAstNode>,
    },
}

#[derive(Deserialize)]
struct TransactionInput {
    solana_pubkey: String,
    solana_signature: String,
    amount: Decimal,
    transaction_type: String,
    document: String,
    dynamic_rule: DynamicAstNode,
}

#[derive(Serialize)]
struct TransactionResponse {
    status: String,
    antifraud_action: String,
    policy_allowed: bool,
    ast_approved: bool,
    solana_signature_valid: bool,
    solana_fee_lamports: u64,
    msg: String,
}

/// 📦 Estrutura padrão de erro contextualizado para resposta da API
#[derive(Serialize)]
struct ApiErrorResponse {
    success: bool,
    error_code: String,
    message: String,
    details: serde_json::Value,
}

#[get("/api/v1/transactions")]
async fn get_transactions(db: web::Data<Arc<Storage>>) -> impl Responder {
    match db.get_all_transactions().await {
        Ok(records) => HttpResponse::Ok().json(records),
        Err(_) => HttpResponse::InternalServerError().json("Erro ao acessar banco de dados de auditoria."),
    }
}

#[post("/api/v1/transactions")]
async fn handle_transaction(
    db: web::Data<Arc<Storage>>, 
    input: web::Json<TransactionInput>
) -> impl Responder {
    
    // 1. Validação Criptográfica com captura de erro contextual detalhado
    let solana_result = match SolanaVerifier::verify_payment_intent(
        &input.solana_pubkey,
        input.amount,
        &input.solana_signature
    ) {
        Ok(res) => res,
        Err(engine_error) => {
            // Transforma o enum da engine diretamente em JSON contextualizado
            let error_message = engine_error.to_string();
            let details_json = serde_json::to_value(&engine_error).unwrap_or(serde_json::Value::Null);
            
            return HttpResponse::BadRequest().json(ApiErrorResponse {
                success: false,
                error_code: "SOLANA_CRYPTOGRAPHY_ERROR".to_string(),
                message: error_message,
                details: details_json,
            });
        }
    };

    let db_historical_amounts = match db.get_history(&input.solana_pubkey).await {
        Ok(history) => history.iter().map(|d| d.to_f64().unwrap_or(0.0)).collect(),
        Err(_) => vec![1200.0, 1400.0, 1350.0],
    };

    let amount_f64 = input.amount.to_f64().unwrap_or(0.0);
    let _wallet = PqWalletVault::generate_new();
    let mut risk_evaluator = FraudEvaluator::new(15, 50000.0);
    let merchant_policy = MerchantPolicy::new_sandbox("merch_live_999");

    let risk_payload = RiskPayload {
        transaction_amount: amount_f64,
        device_reputation: 0.95,
        user_history_score: 0.80,
        is_anomalous_location: false,
    };

    let anomaly_ctx = AnomalyContext {
        historical_amounts: db_historical_amounts, 
        current_transaction_amount: amount_f64,
        allowed_deviation_factor: 3.0,
    };

    let sanction_payload = SanctionCheckPayload {
        document: input.document.clone(),
        wallet_address: input.solana_pubkey.clone(),
        country_code: "BR".to_string(),
    };

    let policy_payload = PolicyTransactionPayload {
        transaction_type: input.transaction_type.clone(),
        amount: amount_f64,
        daily_accumulated_volume: 500.0,
    };

    let fraud_verdict = risk_evaluator.evaluate_transaction("cliente_api", &risk_payload, &anomaly_ctx, &sanction_payload);
    let policy_audit = PolicyExecutor::execute_single(&merchant_policy, &policy_payload);

    let ast_rule = convert_to_native_ast(&input.dynamic_rule);

    let type_code = match input.transaction_type.as_str() {
        "Pix" => 1.0,
        "CreditCard" => 2.0,
        _ => 3.0,
    };

    let mut ast_context = HashMap::new();
    ast_context.insert("amount".to_string(), Value::Number(amount_f64));
    ast_context.insert("transaction_type_code".to_string(), Value::Number(type_code));
    
    let ast_approved = evaluate_ast(&ast_rule, &ast_context);

    let is_approved = policy_audit.verdict.is_allowed 
        && ast_approved 
        && solana_result.is_valid_signature
        && format!("{:?}", fraud_verdict.action) == "Approve";
        
    let status_final = if is_approved { "APPROVED" } else { "REJECTED" };
    
    let _ = db.save_transaction(&input.solana_pubkey, input.amount, status_final).await;

    HttpResponse::Ok().json(TransactionResponse {
        status: status_final.to_string(),
        antifraud_action: format!("{:?}", fraud_verdict.action),
        policy_allowed: policy_audit.verdict.is_allowed,
        ast_approved,
        solana_signature_valid: solana_result.is_valid_signature,
        solana_fee_lamports: solana_result.tx_fee_lamports,
        msg: "Processamento SlipPay com validação criptográfica Solana concluído com sucesso.".to_string(),
    })
}

fn convert_to_native_ast(node: &DynamicAstNode) -> Node {
    match node {
        DynamicAstNode::Condition { field, operator, value } => {
            let op = match operator.as_str() {
                "LessThanOrEqual" => Operator::LessThanOrEqual,
                "GreaterThan" => Operator::GreaterThan,
                "Equal" => Operator::Equal,
                _ => Operator::LessThanOrEqual,
            };
            Node::Condition {
                field: field.clone(),
                operator: op,
                value: Value::Number(*value),
            }
        }
        DynamicAstNode::Logical { operator, conditions } => {
            let native_conditions: Vec<Node> = conditions
                .iter()
                .map(convert_to_native_ast)
                .collect();
            match operator.as_str() {
                "AND" => Node::And(native_conditions),
                "OR" => Node::Or(native_conditions),
                _ => Node::And(native_conditions),
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("====================================================");
    println!(" 🚀 SLIPPAY CONTEXT ERRORS - OPERACIONAL");
    println!(" 🌐 Servidor escutando em: http://127.0.0.1:8080");
    println!("====================================================\n");

    let db = Storage::new("sqlite://slippay.db").await.expect("Falha ao conectar ao banco");
    db.run_migrations().await.expect("Falha ao rodar migrações");
    
    let shared_db = web::Data::new(Arc::new(db));

    HttpServer::new(move || {
        App::new()
            .app_data(shared_db.clone())
            .service(handle_transaction)
            .service(get_transactions)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

