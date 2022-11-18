mod lexer;

use crate::lexer::*;

fn main() {
    let source: String = String::from("'agasg'");
    let tokens = lex(source);
    
    for token in tokens {
        println!("{:?}", token);
    }

}
