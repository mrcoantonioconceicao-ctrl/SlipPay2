use crate::ast::{Node, Value};
use serde_json::Value as JsonValue;

/// Transforma uma string JSON estruturada na árvore de nós da AST.
pub fn parse_json_to_ast(json_str: &str) -> Result<Node, serde_json::Error> {
    let node: Node = serde_json::from_str(json_str)?;
    Ok(node)
}

/// Converte um valor JSON genérico para o enum de valores fortemente tipados da nossa AST.
pub fn convert_json_value(json_val: &JsonValue) -> Value {
    match json_val {
        JsonValue::Number(num) => {
            if let Some(f) = num.as_f64() {
                Value::Number(f)
            } else {
                Value::Number(0.0)
            }
        }
        JsonValue::String(s) => Value::String(s.clone()),
        JsonValue::Bool(b) => Value::Boolean(*b),
        _ => Value::String(json_val.to_string()),
    }
}

/// Avalia expressões matemáticas customizadas de formato texto em tempo de execução.
pub fn evaluate_math_expression(expression: &str, context_value: f64) -> Result<f64, String> {
    let sanitized: String = expression
        .chars()
        .filter(|c| c.is_alphanumeric() || "+-*/(). ".contains(*c))
        .collect();

    let replaced = sanitized.replace("x", &context_value.to_string());
    
    meval::eval_str(&replaced)
        .map_err(|e| format!("Falha ao processar expressão analítica: {}", e))
}

