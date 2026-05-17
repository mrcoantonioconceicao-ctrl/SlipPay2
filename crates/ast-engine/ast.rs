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
    pub field: String,
    pub operator: Operator,
    pub value: String,
}

#[derive(Debug)]
pub struct Rule {
    pub action: Action,
    pub expressions: Vec<Expression>,
}
