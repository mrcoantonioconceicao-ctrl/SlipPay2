use serde::{Deserialize, Serialize};

/// Representa os operadores de comparação suportados pelo motor lógico.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Operator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

/// Representa os valores primitivos que podem ser processados na árvore.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
}

/// A Árvore de Sintaxe Abstrata (AST) propriamente dita.
/// Define nós de condição composta (AND, OR) ou nós folha de comparação direta.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "data")]
pub enum Node {
    /// Conector lógico AND (Ex: Condição A AND Condição B)
    And(Vec<Node>),
    
    /// Conector lógico OR (Ex: Condição A OR Condição B)
    Or(Vec<Node>),
    
    /// Condição Folha: Compara uma variável do contexto com um valor estático
    /// Ex: "amount" > 5000.0
    Condition {
        field: String,
        operator: Operator,
        value: Value,
    },
}

impl Node {
    /// Função utilitária para criar uma condição folha de forma limpa via código
    pub fn new_condition(field: &str, operator: Operator, value: Value) -> Self {
        Node::Condition {
            field: field.to_string(),
            operator,
            value,
        }
    }
}

