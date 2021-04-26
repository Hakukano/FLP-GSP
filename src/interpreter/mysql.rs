use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use std::collections::HashMap;

use crate::ast::*;

#[derive(Clone, Debug, PartialEq)]
pub enum MysqlType {
    BigInt(i64),
    BigUnsigned(u64),
    Binary(Vec<u8>),
    Bool(bool),
    Date(NaiveDate),
    DateTime(NaiveDateTime),
    Double(f64),
    Float(f32),
    Int(i32),
    Null,
    SmallInt(i16),
    SmallUnsigned(u16),
    StringLike(String),
    Time(NaiveTime),
    TimeStamp(DateTime<Utc>),
    TimeTamp(DateTime<Local>),
    TinyInt(i8),
    TinyUnsigned(u8),
    Unsigned(u32),
}
impl MysqlType {
    pub fn replace_and_return(&self, s: &str) -> Self {
        if s.to_ascii_lowercase() == "null" {
            return MysqlType::Null;
        }
        match self {
            MysqlType::BigInt(a) => MysqlType::BigInt(s.parse().unwrap_or_else(|_| *a)),
            MysqlType::BigUnsigned(a) => MysqlType::BigUnsigned(s.parse().unwrap_or_else(|_| *a)),
            MysqlType::Binary(_) => MysqlType::Binary(s.as_bytes().into()),
            MysqlType::Bool(a) => MysqlType::Bool(s.parse().unwrap_or_else(|_| *a)),
            MysqlType::Date(a) => MysqlType::Date(s.parse().unwrap_or_else(|_| *a)),
            MysqlType::DateTime(a) => MysqlType::DateTime(s.parse().unwrap_or_else(|_| *a)),
            MysqlType::Double(a) => MysqlType::Double(s.parse().unwrap_or_else(|_| *a)),
            MysqlType::Float(a) => MysqlType::Float(s.parse().unwrap_or_else(|_| *a)),
            MysqlType::Int(a) => MysqlType::Int(s.parse().unwrap_or_else(|_| *a)),
            MysqlType::Null => MysqlType::Null,
            MysqlType::SmallInt(a) => MysqlType::SmallInt(s.parse().unwrap_or_else(|_| *a)),
            MysqlType::SmallUnsigned(a) => {
                MysqlType::SmallUnsigned(s.parse().unwrap_or_else(|_| *a))
            }
            MysqlType::StringLike(_) => MysqlType::StringLike(s.into()),
            MysqlType::Time(a) => MysqlType::Time(s.parse().unwrap_or_else(|_| *a)),
            MysqlType::TimeStamp(a) => MysqlType::TimeStamp(s.parse().unwrap_or_else(|_| *a)),
            MysqlType::TimeTamp(a) => MysqlType::TimeTamp(s.parse().unwrap_or_else(|_| *a)),
            MysqlType::TinyInt(a) => MysqlType::TinyInt(s.parse().unwrap_or_else(|_| *a)),
            MysqlType::TinyUnsigned(a) => MysqlType::TinyUnsigned(s.parse().unwrap_or_else(|_| *a)),
            MysqlType::Unsigned(a) => MysqlType::Unsigned(s.parse().unwrap_or_else(|_| *a)),
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
