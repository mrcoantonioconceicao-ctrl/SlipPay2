use ast_engine::parser;

fn main() {
    let exemplos = vec![
        "1 + 2 * 3",
        "(10 + 5) * (3 - 1) / 5",
        "3.5 * 2.1 + 1.4",
        "((2.5 + 7.5) / 2) * 4",
        "1 + * 2", // inválido
    ];

    for expr in exemplos {
        println!("\nExpressão: {}", expr);
        match parser::parse_expression(expr) {
            Ok(resultado) => println!("Resultado: {}", resultado),
            Err(erro) => println!("Erro ao avaliar: {:?}", erro),
        }
    }
}
