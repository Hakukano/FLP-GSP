use bit_vec::BitVec;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, ParseError, Utc};
use ipnetwork::{IpNetwork, IpNetworkError};
use rust_decimal::Decimal;
use std::{collections::HashMap, num::ParseFloatError, num::ParseIntError, str::ParseBoolError};
use uuid::Uuid;

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
    #[error("Cannot parse to ipnetwork: {0}")]
    ParseIpNetwork(#[from] IpNetworkError),
    #[error("Cannot parse to uuid: {0}")]
    ParseUuid(#[from] uuid::Error),
    #[error("Cannot serialize: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Cannot find key {0} in types")]
    UnknownKey(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum PostgresType {
    BigInt(Option<i64>),
    Bool(Option<bool>),
    Bytea(Option<Vec<u8>>),
    Char(Option<i8>),
    Date(Option<NaiveDate>),
    Double(Option<f64>),
    INet(Option<IpNetwork>),
    Int(Option<i32>),
    Json(Option<serde_json::Value>),
    Numeric(Option<Decimal>),
    Real(Option<f32>),
    SmallInt(Option<i16>),
    StringLike(Option<String>),
    Time(Option<NaiveTime>),
    TimeStamp(Option<NaiveDateTime>),
    TimeStampTz(Option<DateTime<Utc>>),
    Uuid(Option<Uuid>),
    VarBit(Option<BitVec>),
}
impl PostgresType {
    pub fn replace_and_return(&self, s: &str) -> Result<Self> {
        match self {
            PostgresType::BigInt(_) => Ok(PostgresType::BigInt(Some(s.parse()?))),
            PostgresType::Bool(_) => Ok(PostgresType::Bool(Some(s.parse()?))),
            PostgresType::Bytea(_) => Ok(PostgresType::Bytea(Some(s.as_bytes().into()))),
            PostgresType::Char(_) => Ok(PostgresType::Char(Some(s.parse()?))),
            PostgresType::Date(_) => Ok(PostgresType::Date(Some(s.parse()?))),
            PostgresType::Double(_) => Ok(PostgresType::Double(Some(s.parse()?))),
            PostgresType::INet(_) => Ok(PostgresType::INet(Some(s.parse()?))),
            PostgresType::Int(_) => Ok(PostgresType::Int(Some(s.parse()?))),
            PostgresType::Json(_) => Ok(PostgresType::Json(Some(s.parse()?))),
            PostgresType::Numeric(_) => Ok(PostgresType::Numeric(Some(s.parse()?))),
            PostgresType::Real(_) => Ok(PostgresType::Real(Some(s.parse()?))),
            PostgresType::SmallInt(_) => Ok(PostgresType::SmallInt(Some(s.parse()?))),
            PostgresType::StringLike(_) => Ok(PostgresType::StringLike(Some(s.into()))),
            PostgresType::Time(_) => Ok(PostgresType::Time(Some(s.parse()?))),
            PostgresType::TimeStamp(_) => Ok(PostgresType::TimeStamp(Some(s.parse()?))),
            PostgresType::TimeStampTz(_) => Ok(PostgresType::TimeStampTz(Some(s.parse()?))),
            PostgresType::Uuid(_) => Ok(PostgresType::Uuid(Some(s.parse()?))),
            PostgresType::VarBit(_) => Ok(PostgresType::VarBit(Some(BitVec::from_bytes(
                &s.parse::<u64>()?.to_be_bytes(),
            )))),
        }
    }
}

pub type PostgresRenames = HashMap<String, String>;
pub type PostgresTypes = HashMap<String, PostgresType>;

pub fn interpret_expression(
    expression: &Expression,
    renames: &PostgresRenames,
    types: &PostgresTypes,
) -> Result<(String, Vec<PostgresType>)> {
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
            format!("{} = ??", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .ok_or(Error::UnknownKey(key.to_string()))?
                .replace_and_return(target)?],
        ),
        Node::EqualCI(key, target) => (
            format!("{} ILIKE ??", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .ok_or(Error::UnknownKey(key.to_string()))?
                .replace_and_return(target)?],
        ),
        Node::Greater(key, target) => (
            format!("{} > ??", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .ok_or(Error::UnknownKey(key.to_string()))?
                .replace_and_return(target)?],
        ),
        Node::Less(key, target) => (
            format!("{} < ??", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .ok_or(Error::UnknownKey(key.to_string()))?
                .replace_and_return(target)?],
        ),
        Node::Wildcard(key, target) => (
            format!("{} ILIKE ??", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .ok_or(Error::UnknownKey(key.to_string()))?
                .replace_and_return(&target.replace("*", "%").replace("?", "_"))?],
        ),
        Node::Regex(key, target) => (
            format!("{} = ??", renames.get(key).unwrap_or_else(|| key)),
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
                    targets.iter().map(|_| "??").collect::<Vec<_>>().join(", ")
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
    renames: &PostgresRenames,
    types: &PostgresTypes,
    index: usize,
) -> Result<(String, Vec<PostgresType>)> {
    let (tmp_sql, params) = interpret_expression(expression, renames, types)?;
    let mut buffer = String::new();
    let splitted = tmp_sql.split("??").collect::<Vec<_>>();
    for (i, s) in splitted.iter().enumerate() {
        buffer.push_str(s);
        if i < splitted.len() - 1 {
            buffer.push_str(format!("${}", index + i).as_str());
        }
    }
    Ok((buffer, params))
}
