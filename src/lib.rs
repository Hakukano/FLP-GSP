pub mod ast;
pub mod lexer;
pub mod parser;

#[cfg(test)]
mod tests {
    use crate::{lexer, parser};
    #[test]
    fn it_works() {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let lexer = lexer::Lexer::new(&s).inspect(|tok| println!("tok: {:?}", tok));
        let search = parser::parse(lexer).unwrap();
        println!("{:?}", search);
        assert_eq!(2 + 2, 4);
    }
}
