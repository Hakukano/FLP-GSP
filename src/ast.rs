#[derive(Debug)]
pub enum Symbol {
    GroupStart,
    GroupEnd,
    And,
    Or,
    Not,
    Equal,
    EqualCI,
    Greater,
    Less,
    Wildcard,
    Regex,
    In,
    IsNone,
}

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
    In(String, Vec<String>),
    IsNone(String),
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

#[derive(Debug)]
pub struct Expression {
    /// Fake span for now
    pub span: Span,
    pub node: Expr,
}

#[derive(Debug)]
pub struct Search {
    pub stmts: Vec<Expression>,
}
