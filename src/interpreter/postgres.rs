use bit_vec::BitVec;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, ParseError, Utc};
use ipnetwork::{IpNetwork, IpNetworkError};
use rust_decimal::Decimal;
use std::{
    collections::HashMap, fmt, num::ParseFloatError, num::ParseIntError, str::ParseBoolError,
};
use uuid::Uuid;

use crate::ast::*;

#[derive(Debug)]
pub struct Error {
    target: String,
    error: String,
}
impl Error {
    fn new<E>(target: &str, error: E) -> Self
    where
        E: fmt::Display,
    {
        Self {
            target: target.into(),
            error: error.to_string(),
        }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cannot parse to {}: {}", self.target, self.error)
    }
}
impl std::error::Error for Error {}
impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Self::new("int", err)
    }
}
impl From<ParseFloatError> for Error {
    fn from(err: ParseFloatError) -> Self {
        Self::new("float", err)
    }
}
impl From<ParseBoolError> for Error {
    fn from(err: ParseBoolError) -> Self {
        Self::new("bool", err)
    }
}
impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Self::new("chrono", err)
    }
}
impl From<IpNetworkError> for Error {
    fn from(err: IpNetworkError) -> Self {
        Self::new("ipnetwork", err)
    }
}
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::new("serde_json", err)
    }
}
impl From<uuid::Error> for Error {
    fn from(err: uuid::Error) -> Self {
        Self::new("uuid", err)
    }
}
impl From<rust_decimal::Error> for Error {
    fn from(err: rust_decimal::Error) -> Self {
        Self::new("decimal", err)
    }
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
    let fallback_type = PostgresType::StringLike(Some("".into()));
    Ok(match &expression.node {
        Expr::And(left, right) => {
            let (left_clause, mut left_types) = interpret_expression(left, renames, types)?;
            let (right_clause, mut right_types) = interpret_expression(right, renames, types)?;
            let clause = format!("({} AND {})", left_clause, right_clause);
            left_types.append(&mut right_types);
            (clause, left_types)
        }
        Expr::Or(left, right) => {
            let (left_clause, mut left_types) = interpret_expression(left, renames, types)?;
            let (right_clause, mut right_types) = interpret_expression(right, renames, types)?;
            let clause = format!("({} OR {})", left_clause, right_clause);
            left_types.append(&mut right_types);
            (clause, left_types)
        }
        Expr::Not(expr) => {
            let (clause, types) = interpret_expression(expr, renames, types)?;
            (format!("(NOT {})", clause), types)
        }
        Expr::Equal(key, target) => (
            format!("{} = ??", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .unwrap_or_else(|| &fallback_type)
                .replace_and_return(target)?],
        ),
        Expr::EqualCI(key, target) => (
            format!("{} ILIKE ??", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .unwrap_or_else(|| &fallback_type)
                .replace_and_return(target)?],
        ),
        Expr::Greater(key, target) => (
            format!("{} > ??", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .unwrap_or_else(|| &fallback_type)
                .replace_and_return(target)?],
        ),
        Expr::Less(key, target) => (
            format!("{} < ??", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .unwrap_or_else(|| &fallback_type)
                .replace_and_return(target)?],
        ),
        Expr::Wildcard(key, target) => (
            format!("{} ILIKE ??", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .unwrap_or_else(|| &fallback_type)
                .replace_and_return(&target.replace("*", "%").replace("?", "_"))?],
        ),
        Expr::Regex(key, target) => (
            format!("{} = ??", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .unwrap_or_else(|| &fallback_type)
                .replace_and_return(target)?],
        ),
        Expr::In(key, targets) => {
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
                        .unwrap_or_else(|| &fallback_type)
                        .replace_and_return(target)?,
                );
            }
            (sql, binds)
        }
        Expr::IsNone(key) => (
            format!("{} IS NULL", renames.get(key).unwrap_or_else(|| key)),
            vec![],
        ),
    })
}

pub fn interpret(
    search: &Search,
    renames: &PostgresRenames,
    types: &PostgresTypes,
    index: usize,
) -> Result<Vec<(String, Vec<PostgresType>)>> {
    let mut binds = Vec::with_capacity(search.stmts.len());
    for stmt in search.stmts.iter() {
        let (tmp_sql, params) = interpret_expression(stmt, renames, types)?;
        let mut buffer = String::new();
        let splitted = tmp_sql.split("??").collect::<Vec<_>>();
        for (i, s) in splitted.iter().enumerate() {
            buffer.push_str(s);
            if i < splitted.len() - 1 {
                buffer.push_str(format!("${}", index + i).as_str());
            }
        }
        binds.push((buffer, params));
    }
    Ok(binds)
}
