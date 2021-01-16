use std::collections::HashMap;

use crate::ast::*;

#[derive(Debug, Eq, PartialEq)]
pub enum MysqlType {
    StringLike(String),
    Int(String),
    Unsigned(String),
    Float(String),
    Double(String),
}
impl MysqlType {
    pub fn replace_and_return(&self, s: &str) -> Self {
        match self {
            MysqlType::StringLike(_) => MysqlType::StringLike(s.into()),
            MysqlType::Int(_) => MysqlType::Int(s.into()),
            MysqlType::Unsigned(_) => MysqlType::Unsigned(s.into()),
            MysqlType::Float(_) => MysqlType::Float(s.into()),
            MysqlType::Double(_) => MysqlType::Double(s.into()),
        }
    }
}

pub type MysqlRenames = HashMap<String, String>;
pub type MysqlTypes = HashMap<String, MysqlType>;

pub fn mysql_expression(
    expression: &Expression,
    renames: &MysqlRenames,
    types: &MysqlTypes,
) -> (String, Vec<MysqlType>) {
    let fallback_type = MysqlType::StringLike("".into());
    match &expression.node {
        Expr::And(left, right) => {
            let (left_clause, mut left_types) = mysql_expression(left, renames, types);
            let (right_clause, mut right_types) = mysql_expression(right, renames, types);
            let clause = format!("({} AND {})", left_clause, right_clause);
            left_types.append(&mut right_types);
            (clause, left_types)
        }
        Expr::Or(left, right) => {
            let (left_clause, mut left_types) = mysql_expression(left, renames, types);
            let (right_clause, mut right_types) = mysql_expression(right, renames, types);
            let clause = format!("({} OR {})", left_clause, right_clause);
            left_types.append(&mut right_types);
            (clause, left_types)
        }
        Expr::Not(expr) => {
            let (clause, types) = mysql_expression(expr, renames, types);
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

pub fn mysql(
    search: &Search,
    renames: &MysqlRenames,
    types: &MysqlTypes,
) -> Vec<(String, Vec<MysqlType>)> {
    search
        .stmts
        .iter()
        .map(|a| mysql_expression(a, renames, types))
        .collect()
}
