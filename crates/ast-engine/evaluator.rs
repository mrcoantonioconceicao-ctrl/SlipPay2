use crate::ast::*;
use crate::context::PaymentContext;

// [SecOps Guard] Checked Signer & Authority Validation
    pub fn evaluate(rule: &Rule, ctx: &PaymentContext) -> bool {

    for expr in &rule.expressions {

        match expr.field.as_str() {

            "country" => {
                if expr.value != ctx.country {
                    return false;
                }
            }

            "asset" => {
                if expr.value != ctx.asset {
                    return false;
                }
            }

            "amount" => {

                let val: f64 =
                    match expr.value.parse() {
                        Ok(v) => v,
                        Err(_) => {
                            // Log an error if needed, e.g., using `log::warn!`
                            // log::warn!("Invalid amount value in rule expression: '{}'", expr.value);
                            // If the rule's value is invalid, it cannot match, so return false.
                            return false;
                        }
                    };

                match expr.operator {

                    Operator::Lt => {
                        if !(ctx.amount < val) {
                            return false;
                        }
                    }

                    Operator::Gt => {
                        if !(ctx.amount > val) {
                            return false;
                        }
                    }

                    _ => {}
                }
            }

            _ => {}
        }
    }

    true
}
