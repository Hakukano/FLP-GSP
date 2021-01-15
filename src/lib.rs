pub mod ast;
pub mod lexer;
mod parser;

/// Parse a search string to an AST
///
/// # LALR(1) Grammer
///
/// ## Search:
///
/// * Statements
///
/// ## Statements:
///
/// * []
/// * Statements Relation
///
/// ## Relation:
///
/// * GroupStart Comparison GroupEnd
/// * GroupStart Relation And Relation GroupEnd
/// * GroupStart Relation And Comparison GroupEnd
/// * GroupStart Comparison And Relation GroupEnd
/// * GroupStart Comparison And Comparison GroupEnd
/// * GroupStart Relation Or Relation GroupEnd
/// * GroupStart Relation Or Comparison GroupEnd
/// * GroupStart Comparison Or Relation GroupEnd
/// * GroupStart Comparison Or Comparison GroupEnd
/// * GroupStart Not Relation GroupEnd
/// * GroupStart Not Comparison GroupEnd
///
/// ## Comparison
///
/// * Str Equal Str
/// * Str EqualCI Str
/// * Str Greater Str
/// * Str Less Str
/// * Str Wildcard Str
/// * Str Regex Str
///
/// ## Str
/// 
/// * \`\[^\`\]\*\`
/// 
/// ## GroupStart
/// 
/// * (
/// 
/// ## GroupEnd
/// 
/// * )
/// 
/// ## And
/// 
/// * &
/// 
/// ## Or
/// 
/// * |
/// 
/// ## Not
/// 
/// * !
/// 
/// ## Equal
/// 
/// * =
/// 
/// ## EqualCI
/// 
/// * ~
/// 
/// ## Greater
/// 
/// * >
/// 
/// ## Less
/// 
/// * <
/// 
/// ## Wildcard
/// 
/// * \*
/// 
/// ## Regex
/// 
/// * $
pub fn parse(
    text: &str,
    inspect: bool,
) -> Result<ast::Search, (Option<(lexer::Token, lexer::Span)>, &'static str)> {
    if inspect {
        let lexer = lexer::Lexer::new(text).inspect(|tok| eprintln!("tok: {:?}", tok));
        parser::parse(lexer)
    } else {
        let lexer = lexer::Lexer::new(text);
        parser::parse(lexer)
    }
}

#[cfg(test)]
mod tests {
    use crate::parse;
    #[test]
    fn it_works() {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let search = parse(&s, false).unwrap();
        println!("{:?}", search);
        assert_eq!(2 + 2, 4);
    }
}
