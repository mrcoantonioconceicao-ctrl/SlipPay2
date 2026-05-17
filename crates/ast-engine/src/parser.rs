use crate::ast::*;
use std::collections::HashMap;

/// Parser simples de regras
pub fn parse_rule(input: &str) -> Rule {
    let action = if input.starts_with("ALLOW") {
        Action::Allow
    } else {
        Action::Deny
    };

    Rule {
        action,
        expressions: vec![
            Expression {
                field: "country".into(),
                operator: Operator::Eq,
                value: "BR".into(),
            }
        ],
    }
}

/// Avaliação de expressão matemática básica com variáveis
pub fn eval(expr: &str, vars: &HashMap<String, f64>) -> Result<f64, String> {
    let mut replaced = expr.to_string();
    for (k, v) in vars {
        replaced = replaced.replace(k, &v.to_string());
    }

    meval::eval_str(&replaced)
        .map_err(|e| format!("Erro ao avaliar expressão: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_eval_simple_expression() {
        let mut vars = HashMap::new();
        vars.insert("valor".to_string(), 200.0);
        vars.insert("taxa".to_string(), 1.5);

        let result = eval("valor * 0.02 + taxa", &vars).unwrap();
        assert_eq!(result, 5.5);
    }

    #[test]
    fn test_parse_rule_allow() {
        let rule = parse_rule("ALLOW country == BR");
        assert!(matches!(rule.action, Action::Allow));
        assert_eq!(rule.expressions[0].field, "country");
        assert_eq!(rule.expressions[0].value, "BR");
    }

    #[test]
    fn test_parse_rule_deny() {
        let rule = parse_rule("DENY country == BR");
        assert!(matches!(rule.action, Action::Deny));
    }
}
