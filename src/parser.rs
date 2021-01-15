#![allow(unused_braces)]

use plex::parser;

use crate::{
    ast::*,
    lexer::{Token::*, *},
};

parser! {
    fn parse_(Token, Span);

    (a, b) {
        Span {
            lo: a.lo,
            hi: b.hi,
        }
    }

    search: Search {
        statements[st] => Search { stmts: st }
    }

    statements: Vec<Expression> {
        => vec![],
        statements[mut st] relation[e] => {
            st.push(e);
            st
        }
    }

    relation: Expression {
        GroupStart comparison[a] GroupEnd => a,
        GroupStart relation[a] And relation[b] GroupEnd => Expression {
            span: span!(),
            node: Expr::And(Box::new(a), Box::new(b)),
        },
        GroupStart relation[a] And comparison[b] GroupEnd => Expression {
            span: span!(),
            node: Expr::And(Box::new(a), Box::new(b)),
        },
        GroupStart comparison[a] And relation[b] GroupEnd => Expression {
            span: span!(),
            node: Expr::And(Box::new(a), Box::new(b)),
        },
        GroupStart comparison[a] And comparison[b] GroupEnd => Expression {
            span: span!(),
            node: Expr::And(Box::new(a), Box::new(b)),
        },

        GroupStart relation[a] Or relation[b] GroupEnd => Expression {
            span: span!(),
            node: Expr::Or(Box::new(a), Box::new(b)),
        },
        GroupStart relation[a] Or comparison[b] GroupEnd => Expression {
            span: span!(),
            node: Expr::Or(Box::new(a), Box::new(b)),
        },
        GroupStart comparison[a] Or relation[b] GroupEnd => Expression {
            span: span!(),
            node: Expr::Or(Box::new(a), Box::new(b)),
        },
        GroupStart comparison[a] Or comparison[b] GroupEnd => Expression {
            span: span!(),
            node: Expr::Or(Box::new(a), Box::new(b)),
        },

        GroupStart Not relation[a] GroupEnd => Expression {
            span: span!(),
            node: Expr::Not(Box::new(a)),
        },
        GroupStart Not comparison[a] GroupEnd => Expression {
            span: span!(),
            node: Expr::Not(Box::new(a)),
        },
    }

    comparison: Expression {
        Str(a) Equal Str(b) => Expression {
            span: span!(),
            node: Expr::Equal(a, b),
        },
        Str(a) EqualCI Str(b) => Expression {
            span: span!(),
            node: Expr::EqualCI(a, b),
        },
        Str(a) Greater Str(b) => Expression {
            span: span!(),
            node: Expr::Greater(a, b),
        },
        Str(a) Less Str(b) => Expression {
            span: span!(),
            node: Expr::Less(a, b),
        },
        Str(a) Wildcard Str(b) => Expression {
            span: span!(),
            node: Expr::Wildcard(a, b),
        },
        Str(a) Regex Str(b) => Expression {
            span: span!(),
            node: Expr::Regex(a, b),
        },
    }
}

pub fn parse<I: Iterator<Item = (Token, Span)>>(
    i: I,
) -> Result<Search, (Option<(Token, Span)>, &'static str)> {
    parse_(i)
}
