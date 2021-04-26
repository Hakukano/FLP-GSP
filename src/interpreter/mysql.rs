use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, ParseError, Utc};
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
pub enum MysqlType {
    BigInt(Option<i64>),
    BigUnsigned(Option<u64>),
    Binary(Option<Vec<u8>>),
    Bool(Option<bool>),
    Date(Option<NaiveDate>),
    DateTime(Option<NaiveDateTime>),
    Double(Option<f64>),
    Float(Option<f32>),
    Int(Option<i32>),
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
        let is_null = s.to_ascii_lowercase() == "null";
        match self {
            MysqlType::BigInt(_) => {
                if is_null {
                    return Ok(MysqlType::BigInt(None));
                }
                Ok(MysqlType::BigInt(Some(s.parse()?)))
            }
            MysqlType::BigUnsigned(_) => {
                if is_null {
                    return Ok(MysqlType::BigUnsigned(None));
                }
                Ok(MysqlType::BigUnsigned(Some(s.parse()?)))
            }
            MysqlType::Binary(_) => {
                if is_null {
                    return Ok(MysqlType::Binary(None));
                }
                Ok(MysqlType::Binary(Some(s.as_bytes().into())))
            }
            MysqlType::Bool(_) => {
                if is_null {
                    return Ok(MysqlType::Bool(None));
                }
                Ok(MysqlType::Bool(Some(s.parse()?)))
            }
            MysqlType::Date(_) => {
                if is_null {
                    return Ok(MysqlType::Date(None));
                }
                Ok(MysqlType::Date(Some(s.parse()?)))
            }
            MysqlType::DateTime(_) => {
                if is_null {
                    return Ok(MysqlType::DateTime(None));
                }
                Ok(MysqlType::DateTime(Some(s.parse()?)))
            }
            MysqlType::Double(_) => {
                if is_null {
                    return Ok(MysqlType::Double(None));
                }
                Ok(MysqlType::Double(Some(s.parse()?)))
            }
            MysqlType::Float(_) => {
                if is_null {
                    return Ok(MysqlType::Float(None));
                }
                Ok(MysqlType::Float(Some(s.parse()?)))
            }
            MysqlType::Int(_) => {
                if is_null {
                    return Ok(MysqlType::Int(None));
                }
                Ok(MysqlType::Int(Some(s.parse()?)))
            }
            MysqlType::SmallInt(_) => {
                if is_null {
                    return Ok(MysqlType::SmallInt(None));
                }
                Ok(MysqlType::SmallInt(Some(s.parse()?)))
            }
            MysqlType::SmallUnsigned(_) => {
                if is_null {
                    return Ok(MysqlType::SmallUnsigned(None));
                }
                Ok(MysqlType::SmallUnsigned(Some(s.parse()?)))
            }
            MysqlType::StringLike(_) => {
                if is_null {
                    return Ok(MysqlType::StringLike(None));
                }
                Ok(MysqlType::StringLike(Some(s.into())))
            }
            MysqlType::Time(_) => {
                if is_null {
                    return Ok(MysqlType::Time(None));
                }
                Ok(MysqlType::Time(Some(s.parse()?)))
            }
            MysqlType::TimeStamp(_) => {
                if is_null {
                    return Ok(MysqlType::TimeStamp(None));
                }
                Ok(MysqlType::TimeStamp(Some(s.parse()?)))
            }
            MysqlType::TimeTamp(_) => {
                if is_null {
                    return Ok(MysqlType::TimeTamp(None));
                }
                Ok(MysqlType::TimeTamp(Some(s.parse()?)))
            }
            MysqlType::TinyInt(_) => {
                if is_null {
                    return Ok(MysqlType::TinyInt(None));
                }
                Ok(MysqlType::TinyInt(Some(s.parse()?)))
            }
            MysqlType::TinyUnsigned(_) => {
                if is_null {
                    return Ok(MysqlType::TinyUnsigned(None));
                }
                Ok(MysqlType::TinyUnsigned(Some(s.parse()?)))
            }
            MysqlType::Unsigned(_) => {
                if is_null {
                    return Ok(MysqlType::Unsigned(None));
                }
                Ok(MysqlType::Unsigned(Some(s.parse()?)))
            }
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
    let fallback_type = MysqlType::StringLike(Some("".into()));
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
            format!("`{}` <=> ?", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .unwrap_or_else(|| &fallback_type)
                .replace_and_return(target)?],
        ),
        Expr::EqualCI(key, target) => (
            format!("`{}` LIKE ?", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .unwrap_or_else(|| &fallback_type)
                .replace_and_return(target)?],
        ),
        Expr::Greater(key, target) => (
            format!("`{}` > ?", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .unwrap_or_else(|| &fallback_type)
                .replace_and_return(target)?],
        ),
        Expr::Less(key, target) => (
            format!("`{}` < ?", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .unwrap_or_else(|| &fallback_type)
                .replace_and_return(target)?],
        ),
        Expr::Wildcard(key, target) => (
            format!("`{}` LIKE ?", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .unwrap_or_else(|| &fallback_type)
                .replace_and_return(&target.replace("*", "%").replace("?", "_"))?],
        ),
        Expr::Regex(key, target) => (
            format!("`{}` = ?", renames.get(key).unwrap_or_else(|| key)),
            vec![types
                .get(key)
                .unwrap_or_else(|| &fallback_type)
                .replace_and_return(target)?],
        ),
    })
}

pub fn interpret(
    search: &Search,
    renames: &MysqlRenames,
    types: &MysqlTypes,
) -> Result<Vec<(String, Vec<MysqlType>)>> {
    let mut binds = Vec::with_capacity(search.stmts.len());
    for stmt in search.stmts.iter() {
        binds.push(interpret_expression(stmt, renames, types)?);
    }
    Ok(binds)
}
