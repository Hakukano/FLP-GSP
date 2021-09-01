use lalrpop_util::lalrpop_mod;

pub mod ast;
pub mod interpreter;

lalrpop_mod!(pub gsp);

/// Parse a general search string (GSS) to an AST
/// Fake error right now for backwarding.
pub fn parse(text: &str) -> Result<ast::Search, (Option<(ast::Symbol, ast::Span)>, &'static str)> {
    gsp::GSSsParser::new()
        .parse(text)
        .map_err(|_| (None, "Cannot parse gss"))
}
