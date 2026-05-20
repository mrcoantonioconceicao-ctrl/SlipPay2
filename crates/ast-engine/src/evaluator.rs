use crate::ast::{Node, Operator, Value};
use std::collections::HashMap;

/// Avalia um nó da árvore (AST) contra um contexto dinâmico de dados.
/// Retorna `true` se a transação passar nas condições da regra ou `false` se falhar.
pub fn evaluate_ast(node: &Node, context: &HashMap<String, Value>) -> bool {
    match node {
        Node::And(conditions) => {
            // Se houver um AND vazio, por segurança determinística, falha.
            if conditions.is_empty() { return false; }
            conditions.iter().all(|cond| evaluate_ast(cond, context))
        }
        Node::Or(conditions) => {
            if conditions.is_empty() { return false; }
            conditions.iter().any(|cond| evaluate_ast(cond, context))
        }
        Node::Condition { field, operator, value } => {
            // Busca a variável dentro do contexto da transação
            let context_value = match context.get(field) {
                Some(val) => val,
                None => return false, // Campo ausente invalida a condição automaticamente
            };

            // Processa a regra baseando-se no operador lógico correspondente
            match (context_value, value) {
                (Value::Number(ctx_num), Value::Number(rule_num)) => match operator {
                    Operator::Equal => (ctx_num - rule_num).abs() < f64::EPSILON,
                    Operator::NotEqual => (ctx_num - rule_num).abs() >= f64::EPSILON,
                    Operator::GreaterThan => ctx_num > rule_num,
                    Operator::LessThan => ctx_num < rule_num,
                    Operator::GreaterThanOrEqual => ctx_num >= rule_num,
                    Operator::LessThanOrEqual => ctx_num <= rule_num,
                },
                (Value::String(ctx_str), Value::String(rule_str)) => match operator {
                    Operator::Equal => ctx_str == rule_str,
                    Operator::NotEqual => ctx_str != rule_str,
                    _ => false, // Strings não aceitam operadores relacionais de grandeza (<, >, etc.)
                },
                (Value::Boolean(ctx_bool), Value::Boolean(rule_bool)) => match operator {
                    Operator::Equal => ctx_bool == rule_bool,
                    Operator::NotEqual => ctx_bool != rule_bool,
                    _ => false,
                },
                _ => false, // Tipos incompatíveis em tempo de execução falham na checagem
            }
        }
    }
}

