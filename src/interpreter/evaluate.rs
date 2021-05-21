use regex::Regex;
use std::collections::HashMap;
use wildmatch::WildMatch;

use crate::ast::*;

pub struct EvaluateRule {
    pub is_equal: fn(value: &str, target: &str) -> bool,
    pub is_equal_ci: fn(value: &str, target: &str) -> bool,
    pub is_greater_than: fn(value: &str, target: &str) -> bool,
    pub is_less_than: fn(value: &str, target: &str) -> bool,
    pub is_match_wildcard: fn(value: &str, target: &str) -> bool,
    pub is_match_regex: fn(value: &str, target: &str) -> bool,
    pub is_in: fn(value: &str, target: &[String]) -> bool,
    pub is_none: fn(value: &str) -> bool,
}
impl Default for EvaluateRule {
    fn default() -> Self {
        Self {
            is_equal: |value, target| value == target,
            is_equal_ci: |value, target| value.to_lowercase() == target.to_lowercase(),
            is_greater_than: |value, target| value > target,
            is_less_than: |value, target| value < target,
            is_match_wildcard: |value, target| WildMatch::new(target).is_match(value),
            is_match_regex: |value, target| {
                let reg = Regex::new(target);
                if reg.is_err() {
                    return false;
                }
                let reg = reg.unwrap();
                reg.is_match(value)
            },
            is_in: |value, target| target.contains(&value.to_string()),
            is_none: |value| {
                value.to_ascii_lowercase() == "none" || value.to_ascii_lowercase() == "null"
            },
        }
    }
}

pub type EvaluateRules = HashMap<String, EvaluateRule>;
pub type EvaluatePairs = HashMap<String, String>;

pub fn interpret_expression(
    expression: &Expression,
    rules: &EvaluateRules,
    pairs: &EvaluatePairs,
) -> bool {
    match &expression.node {
        Expr::And(left, right) => {
            interpret_expression(left, rules, pairs) && interpret_expression(right, rules, pairs)
        }
        Expr::Or(left, right) => {
            interpret_expression(left, rules, pairs) || interpret_expression(right, rules, pairs)
        }
        Expr::Not(expr) => !interpret_expression(expr, rules, pairs),
        Expr::Equal(key, target) => {
            let rule = rules.get(key);
            if rule.is_none() {
                return false;
            }
            let rule = rule.unwrap();
            let value = pairs.get(key);
            if value.is_none() {
                return false;
            }
            let value = value.unwrap();
            (rule.is_equal)(value, target)
        }
        Expr::EqualCI(key, target) => {
            let rule = rules.get(key);
            if rule.is_none() {
                return false;
            }
            let rule = rule.unwrap();
            let value = pairs.get(key);
            if value.is_none() {
                return false;
            }
            let value = value.unwrap();
            (rule.is_equal_ci)(value, target)
        }
        Expr::Greater(key, target) => {
            let rule = rules.get(key);
            if rule.is_none() {
                return false;
            }
            let rule = rule.unwrap();
            let value = pairs.get(key);
            if value.is_none() {
                return false;
            }
            let value = value.unwrap();
            (rule.is_greater_than)(value, target)
        }
        Expr::Less(key, target) => {
            let rule = rules.get(key);
            if rule.is_none() {
                return false;
            }
            let rule = rule.unwrap();
            let value = pairs.get(key);
            if value.is_none() {
                return false;
            }
            let value = value.unwrap();
            (rule.is_less_than)(value, target)
        }
        Expr::Wildcard(key, target) => {
            let rule = rules.get(key);
            if rule.is_none() {
                return false;
            }
            let rule = rule.unwrap();
            let value = pairs.get(key);
            if value.is_none() {
                return false;
            }
            let value = value.unwrap();
            (rule.is_match_wildcard)(value, target)
        }
        Expr::Regex(key, target) => {
            let rule = rules.get(key);
            if rule.is_none() {
                return false;
            }
            let rule = rule.unwrap();
            let value = pairs.get(key);
            if value.is_none() {
                return false;
            }
            let value = value.unwrap();
            (rule.is_match_regex)(value, target)
        }
        Expr::In(key, targets) => {
            let rule = rules.get(key);
            if rule.is_none() {
                return false;
            }
            let rule = rule.unwrap();
            let value = pairs.get(key);
            if value.is_none() {
                return false;
            }
            let value = value.unwrap();
            (rule.is_in)(value, targets)
        }
        Expr::IsNone(key) => {
            let rule = rules.get(key);
            if rule.is_none() {
                return false;
            }
            let rule = rule.unwrap();
            let value = pairs.get(key);
            if value.is_none() {
                return false;
            }
            let value = value.unwrap();
            (rule.is_none)(value)
        }
    }
}

pub fn interpret(search: &Search, rules: &EvaluateRules, pairs: &EvaluatePairs) -> Vec<bool> {
    search
        .stmts
        .iter()
        .map(|a| interpret_expression(a, rules, pairs))
        .collect()
}
