use chrono::{DateTime, ParseError, Utc};
use std::{
    collections::HashMap, fmt, num::ParseFloatError, num::ParseIntError, str::ParseBoolError,
};

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

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum SqliteType {
    BigInt(Option<i64>),
    Blob(Option<Vec<u8>>),
    Boolean(Option<bool>),
    DateTime(Option<DateTime<Utc>>),
    Integer(Option<i32>),
    Real(Option<f64>),
    Text(Option<String>),
}
impl SqliteType {
    pub fn replace_and_return(&self, s: &str) -> Result<Self> {
        match self {
            SqliteType::BigInt(_) => Ok(SqliteType::BigInt(Some(s.parse()?))),
            SqliteType::Blob(_) => Ok(SqliteType::Blob(Some(s.as_bytes().to_vec()))),
            SqliteType::Boolean(_) => Ok(SqliteType::Boolean(Some(s.parse()?))),
            SqliteType::DateTime(_) => Ok(SqliteType::DateTime(Some(s.parse()?))),
            SqliteType::Integer(_) => Ok(SqliteType::Integer(Some(s.parse()?))),
            SqliteType::Real(_) => Ok(SqliteType::Real(Some(s.parse()?))),
            SqliteType::Text(_) => Ok(SqliteType::Text(Some(s.to_string()))),
        }
    }
}

pub type SqliteRenames = HashMap<String, String>;
pub type SqliteTypes = HashMap<String, SqliteType>;

pub fn interpret_expression(
    expression: &Expression,
    renames: &SqliteRenames,
    types: &SqliteTypes,
) -> Result<(String, Vec<SqliteType>)> {
    let fallback_type = SqliteType::Text(Some("".into()));
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
            format!("{} = ?", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .unwrap_or_else(|| &fallback_type)
                .replace_and_return(target)?],
        ),
        Expr::EqualCI(key, target) => (
            format!("{} LIKE ?", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .unwrap_or_else(|| &fallback_type)
                .replace_and_return(target)?],
        ),
        Expr::Greater(key, target) => (
            format!("{} > ?", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .unwrap_or_else(|| &fallback_type)
                .replace_and_return(target)?],
        ),
        Expr::Less(key, target) => (
            format!("{} < ?", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .unwrap_or_else(|| &fallback_type)
                .replace_and_return(target)?],
        ),
        Expr::Wildcard(key, target) => (
            format!("{} LIKE ?", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .unwrap_or_else(|| &fallback_type)
                .replace_and_return(&target.replace("*", "%").replace("?", "_"))?],
        ),
        Expr::Regex(key, target) => (
            format!("{} = ?", renames.get(key).unwrap_or_else(|| key)),
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
                    targets.iter().map(|_| "?").collect::<Vec<_>>().join(", ")
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
    renames: &SqliteRenames,
    types: &SqliteTypes,
) -> Result<Vec<(String, Vec<SqliteType>)>> {
    let mut binds = Vec::with_capacity(search.stmts.len());
    for stmt in search.stmts.iter() {
        binds.push(interpret_expression(stmt, renames, types)?);
    }
    Ok(binds)
}
