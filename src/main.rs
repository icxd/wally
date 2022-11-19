mod lexer;
mod parser;

use crate::lexer::*;
use crate::parser::*;
use std::fs::File;
use std::io::prelude::*;
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    let filename: &String = &args[1];
    let mut file: File = File::open(filename).expect("File not found");
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let tokens: Vec<Token> = lex(contents);
    let program: Program = parse(&tokens);

    for token in tokens {
        println!("{:?}", token);
    }
    
    println!("{:#?}", program);
}
