use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, ParseError, Utc};
use rust_decimal::Decimal;
use std::{collections::HashMap, num::ParseFloatError, num::ParseIntError, str::ParseBoolError};

use crate::{Expression, Node};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Cannot parse to int: {0}")]
    ParseInt(#[from] ParseIntError),
    #[error("Cannot parse to float: {0}")]
    ParseFloat(#[from] ParseFloatError),
    #[error("Cannot parse to bool: {0}")]
    ParseBool(#[from] ParseBoolError),
    #[error("Cannot parse to chrono: {0}")]
    ParseChrono(#[from] ParseError),
    #[error("Cannot parse to decimal: {0}")]
    ParseDecimal(#[from] rust_decimal::Error),
    #[error("Cannot serialize: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Cannot find key {0} in types")]
    UnknownKey(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum MysqlType {
    BigInt(Option<i64>),
    BigUnsigned(Option<u64>),
    Binary(Option<Vec<u8>>),
    Bool(Option<bool>),
    Date(Option<NaiveDate>),
    DateTime(Option<NaiveDateTime>),
    Decimal(Option<Decimal>),
    Double(Option<f64>),
    Float(Option<f32>),
    Int(Option<i32>),
    Json(Option<serde_json::Value>),
    SmallInt(Option<i16>),
    SmallUnsigned(Option<u16>),
    StringLike(Option<String>),
    Time(Option<NaiveTime>),
    TimeStamp(Option<DateTime<Utc>>),
    TimeTamp(Option<DateTime<Local>>),
    TinyInt(Option<i8>),
    TinyUnsigned(Option<u8>),
    Unsigned(Option<u32>),
}
impl MysqlType {
    pub fn replace_and_return(&self, s: &str) -> Result<Self> {
        match self {
            MysqlType::BigInt(_) => Ok(MysqlType::BigInt(Some(s.parse()?))),
            MysqlType::BigUnsigned(_) => Ok(MysqlType::BigUnsigned(Some(s.parse()?))),
            MysqlType::Binary(_) => Ok(MysqlType::Binary(Some(s.as_bytes().into()))),
            MysqlType::Bool(_) => Ok(MysqlType::Bool(Some(s.parse()?))),
            MysqlType::Date(_) => Ok(MysqlType::Date(Some(s.parse()?))),
            MysqlType::DateTime(_) => Ok(MysqlType::DateTime(Some(s.parse()?))),
            MysqlType::Decimal(_) => Ok(MysqlType::Decimal(Some(s.parse()?))),
            MysqlType::Double(_) => Ok(MysqlType::Double(Some(s.parse()?))),
            MysqlType::Float(_) => Ok(MysqlType::Float(Some(s.parse()?))),
            MysqlType::Int(_) => Ok(MysqlType::Int(Some(s.parse()?))),
            MysqlType::Json(_) => Ok(MysqlType::Json(Some(s.parse()?))),
            MysqlType::SmallInt(_) => Ok(MysqlType::SmallInt(Some(s.parse()?))),
            MysqlType::SmallUnsigned(_) => Ok(MysqlType::SmallUnsigned(Some(s.parse()?))),
            MysqlType::StringLike(_) => Ok(MysqlType::StringLike(Some(s.into()))),
            MysqlType::Time(_) => Ok(MysqlType::Time(Some(s.parse()?))),
            MysqlType::TimeStamp(_) => Ok(MysqlType::TimeStamp(Some(s.parse()?))),
            MysqlType::TimeTamp(_) => Ok(MysqlType::TimeTamp(Some(s.parse()?))),
            MysqlType::TinyInt(_) => Ok(MysqlType::TinyInt(Some(s.parse()?))),
            MysqlType::TinyUnsigned(_) => Ok(MysqlType::TinyUnsigned(Some(s.parse()?))),
            MysqlType::Unsigned(_) => Ok(MysqlType::Unsigned(Some(s.parse()?))),
        }
    }
}

pub type MysqlRenames = HashMap<String, String>;
pub type MysqlTypes = HashMap<String, MysqlType>;

pub fn interpret_expression(
    expression: &Expression,
    renames: &MysqlRenames,
    types: &MysqlTypes,
) -> Result<(String, Vec<MysqlType>)> {
    Ok(match &expression.node {
        Node::And(left, right) => {
            let (left_clause, mut left_types) = interpret_expression(left, renames, types)?;
            let (right_clause, mut right_types) = interpret_expression(right, renames, types)?;
            let clause = format!("({} AND {})", left_clause, right_clause);
            left_types.append(&mut right_types);
            (clause, left_types)
        }
        Node::Or(left, right) => {
            let (left_clause, mut left_types) = interpret_expression(left, renames, types)?;
            let (right_clause, mut right_types) = interpret_expression(right, renames, types)?;
            let clause = format!("({} OR {})", left_clause, right_clause);
            left_types.append(&mut right_types);
            (clause, left_types)
        }
        Node::Not(expr) => {
            let (clause, types) = interpret_expression(expr, renames, types)?;
            (format!("(NOT {})", clause), types)
        }
        Node::Equal(key, target) => (
            format!("{} = ?", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .ok_or(Error::UnknownKey(key.to_string()))?
                .replace_and_return(target)?],
        ),
        Node::EqualCI(key, target) => (
            format!("{} LIKE ?", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .ok_or(Error::UnknownKey(key.to_string()))?
                .replace_and_return(target)?],
        ),
        Node::Greater(key, target) => (
            format!("{} > ?", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .ok_or(Error::UnknownKey(key.to_string()))?
                .replace_and_return(target)?],
        ),
        Node::Less(key, target) => (
            format!("{} < ?", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .ok_or(Error::UnknownKey(key.to_string()))?
                .replace_and_return(target)?],
        ),
        Node::Wildcard(key, target) => (
            format!("{} LIKE ?", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .ok_or(Error::UnknownKey(key.to_string()))?
                .replace_and_return(&target.replace("*", "%").replace("?", "_"))?],
        ),
        Node::Regex(key, target) => (
            format!("{} = ?", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .ok_or(Error::UnknownKey(key.to_string()))?
                .replace_and_return(target)?],
        ),
        Node::Any(key, targets) => {
            let sql = if targets.is_empty() {
                "FALSE".to_string()
            } else {
                format!(
                    "{} IN ({})",
                    renames.get(key).unwrap_or_else(|| key),
                    targets.iter().map(|_| "?").collect::<Vec<_>>().join(", ")
                )
            };
            let mut binds = Vec::with_capacity(targets.len());
            for target in targets.iter() {
                binds.push(
                    types
                        .get(key)
                        .ok_or(Error::UnknownKey(key.to_string()))?
                        .replace_and_return(target)?,
                );
            }
            (sql, binds)
        }
        Node::Null(key) => {
            if !types.contains_key(key) {
                return Err(Error::UnknownKey(key.to_string()));
            }
            (
                format!("{} IS NULL", renames.get(key).unwrap_or_else(|| key)),
                vec![],
            )
        }
    })
}

pub fn interpret(
    expression: &Expression,
    renames: &MysqlRenames,
    types: &MysqlTypes,
) -> Result<(String, Vec<MysqlType>)> {
    Ok(interpret_expression(expression, renames, types)?)
}
