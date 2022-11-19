use crate::lexer::{Token, TokenType};

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    FunctionDeclaration(FunctionDeclaration),
}

#[derive(Debug)]
pub struct VariableDeclaration {
    pub name: String,
    pub type_: Type,
    pub value: Expression,
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub name: String,
    pub parameters: Vec<(String, Type)>,
    pub return_type: Type,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub enum Expression {
    ArrayLiteral(ArrayLiteral),
    NumberLiteral(NumberLiteral),
    StringLiteral(StringLiteral),
    CharacterLiteral(CharacterLiteral),
    BooleanLiteral(BooleanLiteral),
}

#[derive(Debug)]
pub struct ArrayLiteral {
    pub elements: Vec<Expression>,
}

#[derive(Debug)]
pub struct NumberLiteral {
    pub value: f64,
}

#[derive(Debug)]
pub struct StringLiteral {
    pub value: String,
}

#[derive(Debug)]
pub struct CharacterLiteral {
    pub value: char,
}

#[derive(Debug)]
pub struct BooleanLiteral {
    pub value: bool,
}

#[derive(Debug)]
pub enum Type {
    Array(Box<Type>),
    Int,
    String,
    Character,
    Boolean,
}

pub fn parse(tokens: Vec<Token>) -> Program {
    Program { statements: parse_statements(&tokens) }
}

// Parse methods
fn parse_type(tokens: &Vec<Token>, index: &mut usize) -> Type {
    let token: &Token = &tokens[*index];

    match token.token_type {
        TokenType::Array => {
            expect_tok(&tokens, index, TokenType::Array);
            expect_tok(&tokens, index, TokenType::LessThan);
            let type_ = parse_type(&tokens, index);
            expect_tok(&tokens, index, TokenType::GreaterThan);
            Type::Array(Box::new(type_))
        }
        TokenType::Int32 => {
            expect_tok(&tokens, index, TokenType::Int32);
            Type::Int
        }
        TokenType::String => {
            expect_tok(&tokens, index, TokenType::String);
            Type::String
        }
        TokenType::Char => {
            expect_tok(&tokens, index, TokenType::Char);
            Type::Character
        }
        TokenType::Boolean => {
            expect_tok(&tokens, index, TokenType::Boolean);
            Type::Boolean
        }
        _ => panic!("Unhandled token: {:?}", token),
    }
}
fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Expression {
    let token: &Token = &tokens[*index];

    match token.token_type {
        TokenType::OpenBracket => {
            expect_tok(&tokens, index, TokenType::OpenBracket);
            let mut elements: Vec<Expression> = Vec::new();
            while tokens[*index].token_type != TokenType::CloseBracket {
                elements.push(parse_expression(&tokens, index));
                if tokens[*index].token_type == TokenType::Comma {
                    expect_tok(&tokens, index, TokenType::Comma);
                }
            }
            expect_tok(&tokens, index, TokenType::CloseBracket);
            Expression::ArrayLiteral(ArrayLiteral { elements })
        }
        TokenType::NumberLiteral => {
            expect_tok(&tokens, index, TokenType::NumberLiteral);
            Expression::NumberLiteral(NumberLiteral {
                value: token.value.parse().unwrap(),
            })
        }
        TokenType::StringLiteral => {
            expect_tok(&tokens, index, TokenType::StringLiteral);
            Expression::StringLiteral(StringLiteral {
                value: token.value.clone(),
            })
        }
        TokenType::CharLiteral => {
            expect_tok(&tokens, index, TokenType::CharLiteral);
            Expression::CharacterLiteral(CharacterLiteral {
                value: token.value.chars().next().unwrap(),
            })
        }
        TokenType::BooleanLiteral => {
            expect_tok(&tokens, index, TokenType::BooleanLiteral);
            Expression::BooleanLiteral(BooleanLiteral {
                value: token.value.parse().unwrap(),
            })
        }
        _ => panic!("Unhandled token: {:?}", token),
    }
}
fn parse_parameters(tokens: &Vec<Token>, index: &mut usize) -> Vec<(String, Type)> {
    expect_tok(&tokens, index, TokenType::OpenParenthesis);
    let mut parameters: Vec<(String, Type)> = Vec::new();
    while tokens[*index].token_type != TokenType::CloseParenthesis {
        let name = tokens[*index].value.clone();
        expect_tok(&tokens, index, TokenType::IdentifierLiteral);
        expect_tok(&tokens, index, TokenType::Colon);
        let type_ = parse_type(&tokens, index);
        parameters.push((name, type_));
        if tokens[*index].token_type == TokenType::Comma {
            expect_tok(&tokens, index, TokenType::Comma);
        }
    }
    expect_tok(&tokens, index, TokenType::CloseParenthesis);
    parameters
}
fn parse_block(tokens: &Vec<Token>, index: &mut usize) -> Vec<Statement> {
    expect_tok(&tokens, index, TokenType::OpenBrace);
    let mut toks: Vec<Token> = Vec::new();
    while tokens[*index].token_type != TokenType::CloseBrace {
        toks.push(tokens[*index].clone());
        *index += 1;
    }
    let statements = parse_statements(&toks);
    expect_tok(&tokens, index, TokenType::CloseBrace);
    statements
}
fn parse_statements(tokens: &Vec<Token>) -> Vec<Statement> {
    let mut statements: Vec<Statement> = Vec::new();
    let mut index: usize = 0;

    while index < tokens.len() {
        let token: &Token = &tokens[index];

        match token.token_type {
            TokenType::IdentifierLiteral => {
                let name = token.value.clone();
                expect_tok(&tokens, &mut index, TokenType::IdentifierLiteral);
                expect_tok(&tokens, &mut index, TokenType::Colon);
                if (match_tok(&tokens, &mut index, &TokenType::Func)) {
                    expect_tok(&tokens, &mut index, TokenType::LessThan);
                    let return_type: Type = parse_type(&tokens, &mut index);
                    expect_tok(&tokens, &mut index, TokenType::GreaterThan);
                    expect_tok(&tokens, &mut index, TokenType::Assignment);
                    let mut parameters: Vec<(String, Type)> = parse_parameters(&tokens, &mut index);
                    expect_tok(&tokens, &mut index, TokenType::FatArrow);
                    let body: Vec<Statement> = parse_block(&tokens, &mut index);

                    statements.push(Statement::FunctionDeclaration(FunctionDeclaration {
                        name,
                        return_type,
                        parameters,
                        body,
                    }));
                } else {
                    let type_ = parse_type(&tokens, &mut index);
                    expect_tok(&tokens, &mut index, TokenType::Assignment);
                    let value = parse_expression(&tokens, &mut index);
                    expect_tok(&tokens, &mut index, TokenType::Semicolon);

                    statements.push(Statement::VariableDeclaration(VariableDeclaration {
                        name,
                        type_,
                        value,
                    }));
                }
            }
            _ => panic!("Unhandled token: {:?}", token),
        }
    }

    statements
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
        panic!("Expected token: {:?}, got {:?}", token_type.clone(), tokens[*index].token_type);
    }
}