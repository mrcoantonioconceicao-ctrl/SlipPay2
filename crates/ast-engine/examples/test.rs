use ast_engine::parser;

fn main() {
    // Testando parse_rule
    let rule = parser::parse_rule("ALLOW country == BR");
    println!("Rule: {:?}", rule);

    // Testando eval
    let mut vars = std::collections::HashMap::new();
    vars.insert("valor".to_string(), 200.0);
    vars.insert("taxa".to_string(), 1.5);

    let result = parser::eval("valor * 0.02 + taxa", &vars).unwrap();
    println!("Eval result: {}", result);
}
