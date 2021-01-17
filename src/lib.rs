pub mod ast;
pub mod interpreter;
pub mod lexer;
pub mod parser;

/// Parse a general search string (GSS) to an AST
pub fn parse(
    text: &str,
) -> Result<ast::Search, (Option<(lexer::Token, lexer::Span)>, &'static str)> {
    let lexer = lexer::Lexer::new(text);
    parser::parse(lexer)
}
