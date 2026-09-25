#[derive(Debug)]
pub enum Action {
    Allow,
    Deny,
}

#[derive(Debug)]
pub enum Operator {
    Eq,
    Lt,
    Gt,
}

#[derive(Debug)]
pub struct Expression {
    pub field: ValidatedFieldName,
    pub operator: Operator,
    pub value: SafeValue,
}

#[derive(Debug)]
pub struct Rule {
    pub action: Action,
    pub expressions: Vec<Expression>,
}
