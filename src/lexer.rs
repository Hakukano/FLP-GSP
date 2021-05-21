use plex::lexer;

#[derive(Debug)]
pub enum Token {
    Whitespace,
    Array(Vec<String>),
    Str(String),
    GroupStart,
    GroupEnd,
    And,
    Or,
    Not,
    Equal,
    EqualCI,
    Greater,
    Less,
    Wildcard,
    Regex,
    In,
    IsNone,
}

lexer! {
    fn take_token(tok: 'a) -> Token;
    r#"[ \t\r\n]"# => Token::Whitespace,
    r#"\[[^\[,]*\w*(,[^\[,]*)*\]"# => {
        Token::Array(tok[1..tok.len() - 1]
            .split(",")
            .map(|a| a.trim().into())
            .collect())
    },
    r#"`[^`]*`"# => Token::Str(tok[1..tok.len() - 1].into()),
    r#"\("# => Token::GroupStart,
    r#"\)"# => Token::GroupEnd,
    r#"\&"# => Token::And,
    r#"\|"# => Token::Or,
    r#"\!"# => Token::Not,
    r#"="# => Token::Equal,
    r#"\~"# => Token::EqualCI,
    r#">"# => Token::Greater,
    r#"<"# => Token::Less,
    r#"\*"# => Token::Wildcard,
    r#"\$"# => Token::Regex,
    r#"\?"# => Token::In,
    r#"-"# => Token::IsNone,
    "." => Token::Whitespace
}

pub struct Lexer<'a> {
    original: &'a str,
    remaining: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Lexer<'a> {
        Lexer {
            original: s,
            remaining: s,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token, Span);
    fn next(&mut self) -> Option<(Token, Span)> {
        loop {
            let (tok, span) = if let Some((tok, new_remaining)) = take_token(self.remaining) {
                let lo = self.original.len() - self.remaining.len();
                let hi = self.original.len() - new_remaining.len();
                self.remaining = new_remaining;
                (tok, Span { lo, hi })
            } else {
                return None;
            };
            match tok {
                Token::Whitespace => {
                    continue;
                }
                tok => {
                    return Some((tok, span));
                }
            }
        }
    }
}
