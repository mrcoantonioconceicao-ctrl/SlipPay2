use crate::ast::*;

pub fn parse_rule(input: &str) -> Rule {

    let action =
        if input.starts_with("ALLOW") {
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
