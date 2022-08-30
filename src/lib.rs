pub mod interpreter;
mod parser;

use parser::comparison::Comparison;
use parser::relation::Relation;

#[derive(Debug)]
pub enum Node {
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Not(Box<Expression>),
    Equal(String, String),
    EqualCI(String, String),
    Greater(String, String),
    Less(String, String),
    Wildcard(String, String),
    Regex(String, String),
    Any(String, Vec<String>),
    Null(String),
}

#[derive(Debug)]
pub struct Expression {
    pub node: Node,
}

impl From<Comparison> for Expression {
    fn from(c: Comparison) -> Self {
        match c {
            Comparison::IsEqual(c) => Self {
                node: Node::Equal(c.left.0, c.right.0),
            },
            Comparison::IsEqualCI(c) => Self {
                node: Node::EqualCI(c.left.0, c.right.0),
            },
            Comparison::IsGreater(c) => Self {
                node: Node::Greater(c.left.0, c.right.0),
            },
            Comparison::IsLess(c) => Self {
                node: Node::Less(c.left.0, c.right.0),
            },
            Comparison::IsWildcard(c) => Self {
                node: Node::Wildcard(c.left.0, c.right.0),
            },
            Comparison::IsRegex(c) => Self {
                node: Node::Regex(c.left.0, c.right.0),
            },
            Comparison::IsAny(c) => Self {
                node: Node::Any(c.left.0, c.right.0),
            },
            Comparison::IsNull(c) => Self {
                node: Node::Null(c.0 .0),
            },
        }
    }
}

impl From<Box<Relation>> for Expression {
    fn from(relation: Box<Relation>) -> Self {
        match *relation {
            Relation::C(c) => c.into(),
            Relation::RAR { left, right } => Self {
                node: Node::And(Box::new(left.into()), Box::new(right.into())),
            },
            Relation::RAC { left, right } => Self {
                node: Node::And(Box::new(left.into()), Box::new(right.into())),
            },
            Relation::CAR { left, right } => Self {
                node: Node::And(Box::new(left.into()), Box::new(right.into())),
            },
            Relation::CAC { left, right } => Self {
                node: Node::And(Box::new(left.into()), Box::new(right.into())),
            },
            Relation::ROR { left, right } => Self {
                node: Node::Or(Box::new(left.into()), Box::new(right.into())),
            },
            Relation::ROC { left, right } => Self {
                node: Node::Or(Box::new(left.into()), Box::new(right.into())),
            },
            Relation::COR { left, right } => Self {
                node: Node::Or(Box::new(left.into()), Box::new(right.into())),
            },
            Relation::COC { left, right } => Self {
                node: Node::Or(Box::new(left.into()), Box::new(right.into())),
            },
            Relation::NR(r) => Self {
                node: Node::Not(Box::new(r.into())),
            },
            Relation::NC(c) => Self {
                node: Node::Not(Box::new(c.into())),
            },
        }
    }
}

impl Expression {
    pub fn try_from_str(s: &str) -> Result<Self, String> {
        Ok(parser::relation::relation(s)
            .map_err(|err| err.to_string())?
            .1
            .into())
    }
}
