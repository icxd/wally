mod lexer;
mod parser;

use crate::lexer::*;
use crate::parser::*;

fn main() {
    let source: String = String::from("x: int = 123;");
    let tokens: Vec<Token> = lex(source);
    let program: Program = parse(tokens);
    
    println!("{:#?}", program);
}
