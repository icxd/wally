use crate::lexer::{Token, TokenType};

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
}

#[derive(Debug)]
pub struct VariableDeclaration {
    pub name: String,
    pub type_: Type,
    pub value: Expression,
}

#[derive(Debug)]
pub enum Expression {
    ArrayLiteral(ArrayLiteral),
}

#[derive(Debug)]
pub struct ArrayLiteral {
    pub elements: Vec<Expression>,
}

#[derive(Debug)]
pub enum Type {
    Array(Box<Type>),
    Int,
}

pub fn parse(tokens: Vec<Token>) -> Program {
    let mut statements: Vec<Statement> = Vec::new();
    let mut index: usize = 0;

    while index < tokens.len() {
        let token = &tokens[index];

        match token.token_type {
            TokenType::IdentifierLiteral => {

            }
            _ => panic!("Unhandled token: {:?}", token),
        }
    }

    Program { statements }
}

// Util methods
fn match_tok(tokens: &Vec<Token>, index: &mut usize, token_type: &TokenType) -> bool {
    if &tokens[*index].token_type == token_type {
        *index += 1;
        return true;
    }

    false
}
fn expect_tok(tokens: &Vec<Token>, index: &mut usize, token_type: TokenType) {
    if !match_tok(tokens, index, &token_type) {
        panic!("Expected token: {:?}", token_type.clone());
    }
}