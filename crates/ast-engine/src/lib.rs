pub mod ast;
pub mod parser;
pub mod evaluator;

/// Função de diagnóstico para expor o status de prontidão do motor lógico.
pub fn info() {
    println!("AST-Engine: Motor lógico determinístico carregado e operacional.");
}

