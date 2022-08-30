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
    #[error("Cannot find key {0} in types")]
    UnknownKey(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum HasuraType {
    Boolean,
    Integer,
    Float,
    StringLike,
}
impl HasuraType {
    pub fn to_hasura_string(&self, s: &str) -> Result<String> {
        match self {
            HasuraType::Boolean => Ok(s.parse::<bool>()?.to_string()),
            HasuraType::Integer => Ok(s.parse::<i64>()?.to_string()),
            HasuraType::Float => Ok(s.parse::<f64>()?.to_string()),
            HasuraType::StringLike => Ok(format!("\"{}\"", s)),
        }
    }
}

pub type HasuraTypes = HashMap<String, HasuraType>;

pub fn interpret_expression(expression: &Expression, types: &HasuraTypes) -> Result<String> {
    Ok(match &expression.node {
        Node::And(left, right) => {
            let left_clause = interpret_expression(left, types)?;
            let right_clause = interpret_expression(right, types)?;
            let clause = format!("{{_and:[{},{}]}}", left_clause, right_clause);
            clause
        }
        Node::Or(left, right) => {
            let left_clause = interpret_expression(left, types)?;
            let right_clause = interpret_expression(right, types)?;
            let clause = format!("{{_or:[{},{}]}}", left_clause, right_clause);
            clause
        }
        Node::Not(expr) => {
            let clause = interpret_expression(expr, types)?;
            format!("{{_not:{}}}", clause)
        }
        Node::Equal(key, target) => format!(
            "{{{}:{{_eq:{}}}}}",
            key,
            types
                .get(key)
                .ok_or(Error::UnknownKey(key.to_string()))?
                .to_hasura_string(target)?
        ),
        Node::EqualCI(key, target) => format!(
            "{{{}:{{_ilike:{}}}}}",
            key,
            types
                .get(key)
                .ok_or(Error::UnknownKey(key.to_string()))?
                .to_hasura_string(target)?
        ),
        Node::Greater(key, target) => format!(
            "{{{}:{{_gt:{}}}}}",
            key,
            types
                .get(key)
                .ok_or(Error::UnknownKey(key.to_string()))?
                .to_hasura_string(target)?
        ),
        Node::Less(key, target) => format!(
            "{{{}:{{_lt:{}}}}}",
            key,
            types
                .get(key)
                .ok_or(Error::UnknownKey(key.to_string()))?
                .to_hasura_string(target)?
        ),
        Node::Wildcard(key, target) => format!(
            "{{{}:{{_ilike:{}}}}}",
            key,
            types
                .get(key)
                .ok_or(Error::UnknownKey(key.to_string()))?
                .to_hasura_string(&target.replace("*", "%").replace("?", "_"))?
        ),
        Node::Regex(key, target) => format!(
            "{{{}:{{_regex:{}}}}}",
            key,
            types
                .get(key)
                .ok_or(Error::UnknownKey(key.to_string()))?
                .to_hasura_string(target)?
        ),
        Node::Any(key, targets) => {
            let mut values = Vec::with_capacity(targets.len());
            for target in targets.iter() {
                values.push(
                    types
                        .get(key)
                        .ok_or(Error::UnknownKey(key.to_string()))?
                        .to_hasura_string(target)?,
                );
            }
            format!("{{{}:{{_in:[{}]}}}}", key, values.join(","))
        }
        Node::Null(key) => {
            if !types.contains_key(key) {
                return Err(Error::UnknownKey(key.to_string()));
            }
            format!("{{{}:{{_is_null:true}}}}", key)
        }
    })
}

pub fn interpret(expression: &Expression, types: &HasuraTypes) -> Result<String> {
    Ok(interpret_expression(expression, types)?)
}
