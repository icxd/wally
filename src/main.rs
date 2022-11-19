mod lexer;
mod parser;

use crate::lexer::*;
use crate::parser::*;

fn main() {
    let source: String = String::from("x: array<int> = [1, 2, 3];");
    let tokens: Vec<Token> = lex(source);
    let program: Program = parse(tokens);
    
    println!("{:?}", program);
}
