use crate::lexer::{Token, TokenType};

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    FunctionCall(FunctionCall),
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
pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    ArrayLiteral(ArrayLiteral),
    MapLiteral(MapLiteral),
    NumberLiteral(NumberLiteral),
    StringLiteral(StringLiteral),
    CharacterLiteral(CharacterLiteral),
    BooleanLiteral(BooleanLiteral),
    Identifier(Identifier),
}

#[derive(Debug)]
pub struct ArrayLiteral {
    pub elements: Vec<Expression>,
}

#[derive(Debug)]
pub struct MapLiteral {
    pub elements: Vec<(Expression, Expression)>,
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
pub struct Identifier {
    pub name: String,
}

#[derive(Debug)]
pub enum Type {
    Array(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Int,
    String,
    Character,
    Boolean,
    Pointer(Box<Type>),
}

pub fn parse(tokens: &Vec<Token>) -> Program {
    Program { statements: parse_statements(&tokens) }
}

// Parse methods
fn parse_statements(tokens: &Vec<Token>) -> Vec<Statement> {
    let mut statements: Vec<Statement> = Vec::new();
    let mut index: usize = 0;

    while index < tokens.len() {
        let token: &Token = &tokens[index];

        match token.token_type {
            TokenType::IdentifierLiteral => {
                let name = token.value.clone();
                expect_tok(&tokens, &mut index, TokenType::IdentifierLiteral);
                if match_tok(&tokens, &mut index, &TokenType::OpenParenthesis) {
                    let arguments = parse_arguments(&tokens, &mut index);
                    expect_tok(&tokens, &mut index, TokenType::CloseParenthesis);
                    expect_tok(&tokens, &mut index, TokenType::Semicolon);
                    statements.push(Statement::FunctionCall(FunctionCall { name, arguments }));
                    continue;
                }
                expect_tok(&tokens, &mut index, TokenType::Colon);
                if match_tok(&tokens, &mut index, &TokenType::Func) {
                    expect_tok(&tokens, &mut index, TokenType::LessThan);
                    let return_type: Type = parse_type(&tokens, &mut index);
                    expect_tok(&tokens, &mut index, TokenType::GreaterThan);
                    expect_tok(&tokens, &mut index, TokenType::Assignment);
                    let parameters: Vec<(String, Type)> = parse_parameters(&tokens, &mut index);
                    expect_tok(&tokens, &mut index, TokenType::FatArrow);
                    expect_tok(&tokens, &mut index, TokenType::OpenBrace);
                    let body: Vec<Statement> = parse_block(&tokens, &mut index);
                    expect_tok(&tokens, &mut index, TokenType::CloseBrace);

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
            TokenType::EndOfFile => break,
            _ => panic!("Unhandled token: {:?}", token),
        }
    }

    statements
}
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
        TokenType::Map => {
            expect_tok(&tokens, index, TokenType::Map);
            expect_tok(&tokens, index, TokenType::LessThan);
            let key_type = parse_type(&tokens, index);
            expect_tok(&tokens, index, TokenType::Comma);
            let value_type = parse_type(&tokens, index);
            expect_tok(&tokens, index, TokenType::GreaterThan);
            Type::Map(Box::new(key_type), Box::new(value_type))
        }
        TokenType::Pointer => {
            expect_tok(&tokens, index, TokenType::Pointer);
            expect_tok(&tokens, index, TokenType::LessThan);
            let type_ = parse_type(&tokens, index);
            expect_tok(&tokens, index, TokenType::GreaterThan);
            Type::Pointer(Box::new(type_))
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
        TokenType::OpenBrace => {
            expect_tok(&tokens, index, TokenType::OpenBrace);
            let mut elements: Vec<(Expression, Expression)> = Vec::new();
            while tokens[*index].token_type != TokenType::CloseBrace {
                let key = parse_expression(&tokens, index);
                expect_tok(&tokens, index, TokenType::Colon);
                let value = parse_expression(&tokens, index);
                elements.push((key, value));
                if *index + 1 > tokens.len() {
                    break;
                }
                // this shit is broken asf, but only if you define a map inside of a function body???????
                if tokens[*index].token_type != TokenType::CloseBrace {
                    expect_tok(&tokens, index, TokenType::Comma);
                }
            }
            expect_tok(&tokens, index, TokenType::CloseBrace);
            Expression::MapLiteral(MapLiteral { elements })
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
        TokenType::IdentifierLiteral => {
            expect_tok(&tokens, index, TokenType::IdentifierLiteral);
            Expression::Identifier(Identifier {
                name: token.value.clone(),
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
    let mut toks: Vec<Token> = Vec::new();
    let mut depth: usize = 0;
    while tokens[*index].token_type != TokenType::CloseBrace || depth != 0 {
        if tokens[*index].token_type == TokenType::OpenBrace {
            depth += 1;
        } else if tokens[*index].token_type == TokenType::CloseBrace {
            depth -= 1;
        }
        toks.push(tokens[*index].clone());
        *index += 1;
    }
    let statements = parse_statements(&toks);
    statements
}
fn parse_arguments(tokens: &Vec<Token>, index: &mut usize) -> Vec<Expression> {
    let mut arguments: Vec<Expression> = Vec::new();
    while tokens[*index].token_type != TokenType::CloseParenthesis {
        arguments.push(parse_expression(&tokens, index));
        
        if tokens[*index].token_type == TokenType::Comma {
            expect_tok(&tokens, index, TokenType::Comma);
        }
    }
    arguments
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