use std::collections::HashMap;

use crate::ast::*;

#[derive(Clone, Debug, PartialEq)]
pub enum MysqlType {
    BigInt(i64),
    BigUnsigned(u64),
    Binary(Vec<u8>),
    Bool(bool),
    Double(f64),
    Float(f32),
    Int(i32),
    SmallInt(i16),
    SmallUnsigned(u16),
    StringLike(String),
    TinyInt(i8),
    TinyUnsigned(u8),
    Unsigned(u32),
}
impl MysqlType {
    pub fn replace_and_return(&self, s: &str) -> Self {
        match self {
            MysqlType::BigInt(_) => MysqlType::BigInt(s.parse().unwrap_or_else(|_| 0)),
            MysqlType::BigUnsigned(_) => MysqlType::BigUnsigned(s.parse().unwrap_or_else(|_| 0)),
            MysqlType::Binary(_) => MysqlType::Binary(s.as_bytes().into()),
            MysqlType::Bool(_) => MysqlType::Bool(s.parse().unwrap_or_else(|_| false)),
            MysqlType::Double(_) => MysqlType::Double(s.parse().unwrap_or_else(|_| 0.0)),
            MysqlType::Float(_) => MysqlType::Float(s.parse().unwrap_or_else(|_| 0.0)),
            MysqlType::Int(_) => MysqlType::Int(s.parse().unwrap_or_else(|_| 0)),
            MysqlType::SmallInt(_) => MysqlType::SmallInt(s.parse().unwrap_or_else(|_| 0)),
            MysqlType::SmallUnsigned(_) => {
                MysqlType::SmallUnsigned(s.parse().unwrap_or_else(|_| 0))
            }
            MysqlType::StringLike(_) => MysqlType::StringLike(s.into()),
            MysqlType::TinyInt(_) => MysqlType::TinyInt(s.parse().unwrap_or_else(|_| 0)),
            MysqlType::TinyUnsigned(_) => MysqlType::TinyUnsigned(s.parse().unwrap_or_else(|_| 0)),
            MysqlType::Unsigned(_) => MysqlType::Unsigned(s.parse().unwrap_or_else(|_| 0)),
        }
    }
}

pub type MysqlRenames = HashMap<String, String>;
pub type MysqlTypes = HashMap<String, MysqlType>;

pub fn interpret_expression(
    expression: &Expression,
    renames: &MysqlRenames,
    types: &MysqlTypes,
) -> (String, Vec<MysqlType>) {
    let fallback_type = MysqlType::StringLike("".into());
    match &expression.node {
        Expr::And(left, right) => {
            let (left_clause, mut left_types) = interpret_expression(left, renames, types);
            let (right_clause, mut right_types) = interpret_expression(right, renames, types);
            let clause = format!("({} AND {})", left_clause, right_clause);
            left_types.append(&mut right_types);
            (clause, left_types)
        }
        Expr::Or(left, right) => {
            let (left_clause, mut left_types) = interpret_expression(left, renames, types);
            let (right_clause, mut right_types) = interpret_expression(right, renames, types);
            let clause = format!("({} OR {})", left_clause, right_clause);
            left_types.append(&mut right_types);
            (clause, left_types)
        }
        Expr::Not(expr) => {
            let (clause, types) = interpret_expression(expr, renames, types);
            (format!("(NOT {})", clause), types)
        }
        Expr::Equal(key, target) => (
            format!("`{}` = ?", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .unwrap_or_else(|| &fallback_type)
                .replace_and_return(target)],
        ),
        Expr::EqualCI(key, target) => (
            format!("`{}` = ?", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .unwrap_or_else(|| &fallback_type)
                .replace_and_return(target)],
        ),
        Expr::Greater(key, target) => (
            format!("`{}` > ?", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .unwrap_or_else(|| &fallback_type)
                .replace_and_return(target)],
        ),
        Expr::Less(key, target) => (
            format!("`{}` < ?", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .unwrap_or_else(|| &fallback_type)
                .replace_and_return(target)],
        ),
        Expr::Wildcard(key, target) => (
            format!("`{}` LIKE ?", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .unwrap_or_else(|| &fallback_type)
                .replace_and_return(&target.replace("*", "%").replace("?", "_"))],
        ),
        Expr::Regex(key, target) => (
            format!("`{}` = ?", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .unwrap_or_else(|| &fallback_type)
                .replace_and_return(target)],
        ),
    }
}

pub fn interpret(
    search: &Search,
    renames: &MysqlRenames,
    types: &MysqlTypes,
) -> Vec<(String, Vec<MysqlType>)> {
    search
        .stmts
        .iter()
        .map(|a| interpret_expression(a, renames, types))
        .collect()
}
