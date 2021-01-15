use crate::lexer::Span;

#[derive(Debug)]
pub enum Expr {
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Not(Box<Expression>),
    Equal(String, String),
    EqualCI(String, String),
    Greater(String, String),
    Less(String, String),
    Wildcard(String, String),
    Regex(String, String),
}

#[derive(Debug)]
pub struct Expression {
    pub span: Span,
    pub node: Expr,
}

#[derive(Debug)]
pub struct Search {
    pub stmts: Vec<Expression>,
}
