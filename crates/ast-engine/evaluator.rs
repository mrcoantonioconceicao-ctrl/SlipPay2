use crate::ast::*;
use crate::context::PaymentContext;

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
                    expr.value.parse().unwrap();

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
