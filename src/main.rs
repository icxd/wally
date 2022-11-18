mod lexer;

use crate::lexer::*;

fn main() {
    let source: String = String::from("x: array<int> = [1, 2, 3];");
    let tokens: Vec<Token> = lex(source);
    
    for token in tokens {
        println!("{:?}", token);
    }

}
