use regex::Regex;
use std::collections::HashMap;
use wildmatch::WildMatch;

use crate::{Expression, Node};

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
            is_match_wildcard: |value, target| WildMatch::new(target).matches(value),
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
        Node::And(left, right) => {
            interpret_expression(left, rules, pairs) && interpret_expression(right, rules, pairs)
        }
        Node::Or(left, right) => {
            interpret_expression(left, rules, pairs) || interpret_expression(right, rules, pairs)
        }
        Node::Not(expr) => !interpret_expression(expr, rules, pairs),
        Node::Equal(key, target) => {
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
        Node::EqualCI(key, target) => {
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
        Node::Greater(key, target) => {
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
        Node::Less(key, target) => {
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
        Node::Wildcard(key, target) => {
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
        Node::Regex(key, target) => {
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
        Node::Any(key, targets) => {
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
        Node::Null(key) => {
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

pub fn interpret(expression: &Expression, rules: &EvaluateRules, pairs: &EvaluatePairs) -> bool {
    interpret_expression(expression, rules, pairs)
}
