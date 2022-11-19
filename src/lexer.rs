#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Data types
    Void,
    String,
    Char,
    Int32,
    Int64,
    UInt32,
    UInt64,
    Float,
    Double,
    Short,
    Long,
    Byte,
    Boolean,

    // Literal values
    IdentifierLiteral,
    StringLiteral,
    CharLiteral,
    NumberLiteral,
    FloatLiteral,
    BooleanLiteral,

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Increment,
    Decrement,
    AddByValue,
    SubtractByValue,
    MultiplyByValue,
    DivideByValue,
    ModuloByValue,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    BitwiseAnd,
    BitwiseOr,
    BitwiseNot,
    BitwiseXor,
    BitwiseLeftShift,
    BitwiseRightShift,
    BitwiseUnsignedRightShift,
    BitwiseAndByValue,
    BitwiseOrByValue,
    BitwiseXorByValue,
    BitwiseNotByValue,
    BitwiseLeftShiftByValue,
    BitwiseRightShiftByValue,
    BitwiseUnsignedRightShiftByValue,
    Assignment,
    FatArrow,
    Arrow,
    QuestionMark,
    Colon,
    Comma,
    Semicolon,
    Dot,
    OpenParenthesis,
    CloseParenthesis,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,

    // Keywords
    Const,
    Import,
    Func,
    Lambda,
    Array,
    Map,
    Pointer,
    Return,

    // Special
    EndOfFile,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: usize,
    pub column: usize,
}
pub fn lex(source: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut i: usize = 0;
    let mut line: usize = 1;
    let mut column: usize = 1;

    while i < source.len() {
        let c: char = source.chars().nth(i).unwrap();

        match c {
            ' ' => {
                i += 1;
            }
            '\t' => {
                i += 1;
            }
            '\r' => {
                i += 1;
            }
            '\n' => {
                i += 1;
                line += 1;
                column = 1;
            }
            '+' => {
                if source.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token {
                        token_type: TokenType::AddByValue,
                        value: "+=".to_string(),
                        line,
                        column,
                    });
                    i += 2;
                } else if source.chars().nth(i + 1).unwrap() == '+' {
                    tokens.push(Token {
                        token_type: TokenType::Increment,
                        value: "++".to_string(),
                        line,
                        column,
                    });
                    i += 2;
                } else {
                    tokens.push(Token {
                        token_type: TokenType::Plus,
                        value: "+".to_string(),
                        line,
                        column,
                    });
                    i += 1;
                }
            }
            '-' => {
                if source.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token {
                        token_type: TokenType::SubtractByValue,
                        value: "-=".to_string(),
                        line,
                        column,
                    });
                    i += 2;
                } else if source.chars().nth(i + 1).unwrap() == '-' {
                    tokens.push(Token {
                        token_type: TokenType::Decrement,
                        value: "--".to_string(),
                        line,
                        column,
                    });
                    i += 2;
                } else if source.chars().nth(i + 1).unwrap() == '>' {
                    tokens.push(Token {
                        token_type: TokenType::Arrow,
                        value: "->".to_string(),
                        line,
                        column,
                    });
                    i += 2;
                } else {
                    tokens.push(Token {
                        token_type: TokenType::Minus,
                        value: "-".to_string(),
                        line,
                        column,
                    });
                    i += 1;
                }
            }
            '*' => {
                if source.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token {
                        token_type: TokenType::MultiplyByValue,
                        value: "*=".to_string(),
                        line,
                        column,
                    });
                    i += 2;
                } else {
                    tokens.push(Token {
                        token_type: TokenType::Multiply,
                        value: "*".to_string(),
                        line,
                        column,
                    });
                    i += 1;
                }
            }
            '/' => {
                if source.chars().nth(i + 1).unwrap() == '/' {
                    while source.chars().nth(i).unwrap() != '\n' {
                        i += 1;
                    }
                } else if source.chars().nth(i + 1).unwrap() == '*' {
                    i += 2;
                    while source.chars().nth(i).unwrap() != '*' && source.chars().nth(i + 1).unwrap() != '/' {
                        i += 1;
                    }
                    i += 2;
                } else if source.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token {
                        token_type: TokenType::DivideByValue,
                        value: "/=".to_string(),
                        line,
                        column,
                    });
                    i += 2;
                } else {
                    tokens.push(Token {
                        token_type: TokenType::Divide,
                        value: "/".to_string(),
                        line,
                        column,
                    });
                    i += 1;
                }
            }
            '%' => {
                if source.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token {
                        token_type: TokenType::ModuloByValue,
                        value: "%=".to_string(),
                        line,
                        column,
                    });
                    i += 2;
                } else {
                    tokens.push(Token {
                        token_type: TokenType::Modulo,
                        value: "%".to_string(),
                        line,
                        column,
                    });
                    i += 1;
                }
            }
            '=' => {
                if source.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token {
                        token_type: TokenType::Equal,
                        value: "==".to_string(),
                        line,
                        column,
                    });
                    i += 2;
                } else if source.chars().nth(i + 1).unwrap() == '>' {
                    tokens.push(Token {
                        token_type: TokenType::FatArrow,
                        value: "=>".to_string(),
                        line,
                        column,
                    });
                    i += 2;
                } else {
                    tokens.push(Token {
                        token_type: TokenType::Assignment,
                        value: "=".to_string(),
                        line,
                        column,
                    });
                    i += 1;
                }
            }
            '!' => {
                if source.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token {
                        token_type: TokenType::NotEqual,
                        value: "!=".to_string(),
                        line,
                        column,
                    });
                    i += 2;
                } else {
                    tokens.push(Token {
                        token_type: TokenType::LogicalNot,
                        value: "!".to_string(),
                        line,
                        column,
                    });
                    i += 1;
                }
            }
            '>' => {
                if source.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token {
                        token_type: TokenType::GreaterThanOrEqual,
                        value: ">=".to_string(),
                        line,
                        column,
                    });
                    i += 2;
                } else {
                    tokens.push(Token {
                        token_type: TokenType::GreaterThan,
                        value: ">".to_string(),
                        line,
                        column,
                    });
                    i += 1;
                }
            }
            '<' => {
                if source.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token {
                        token_type: TokenType::LessThanOrEqual,
                        value: "<=".to_string(),
                        line,
                        column,
                    });
                    i += 2;
                } else if source.chars().nth(i + 1).unwrap() == '<' {
                    if source.chars().nth(i + 2).unwrap() == '=' {
                        tokens.push(Token {
                            token_type: TokenType::BitwiseLeftShiftByValue,
                            value: "<<=".to_string(),
                            line,
                            column,
                        });
                        i += 3;
                    } else {
                        tokens.push(Token {
                            token_type: TokenType::BitwiseLeftShift,
                            value: "<<".to_string(),
                            line,
                            column,
                        });
                        i += 2;
                    }
                } else {
                    tokens.push(Token {
                        token_type: TokenType::LessThan,
                        value: "<".to_string(),
                        line,
                        column,
                    });
                    i += 1;
                }
            }
            '&' => {
                if source.chars().nth(i + 1).unwrap() == '&' {
                    tokens.push(Token {
                        token_type: TokenType::LogicalAnd,
                        value: "&&".to_string(),
                        line,
                        column,
                    });
                    i += 2;
                } else if source.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token {
                        token_type: TokenType::BitwiseAndByValue,
                        value: "&=".to_string(),
                        line,
                        column,
                    });
                    i += 2;
                } else {
                    tokens.push(Token {
                        token_type: TokenType::BitwiseAnd,
                        value: "&".to_string(),
                        line,
                        column,
                    });
                    i += 1;
                }
            }
            '|' => {
                if source.chars().nth(i + 1).unwrap() == '|' {
                    tokens.push(Token {
                        token_type: TokenType::LogicalOr,
                        value: "||".to_string(),
                        line,
                        column,
                    });
                    i += 2;
                } else if source.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token {
                        token_type: TokenType::BitwiseOrByValue,
                        value: "|=".to_string(),
                        line,
                        column,
                    });
                    i += 2;
                } else {
                    tokens.push(Token {
                        token_type: TokenType::BitwiseOr,
                        value: "|".to_string(),
                        line,
                        column,
                    });
                    i += 1;
                }
            }
            '^' => {
                if source.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token {
                        token_type: TokenType::BitwiseXorByValue,
                        value: "^=".to_string(),
                        line,
                        column,
                    });
                    i += 2;
                } else {
                    tokens.push(Token {
                        token_type: TokenType::BitwiseXor,
                        value: "^".to_string(),
                        line,
                        column,
                    });
                    i += 1;
                }
            }
            '~' => {
                if source.chars().nth(i + 1).unwrap() == '=' {
                    tokens.push(Token {
                        token_type: TokenType::BitwiseNotByValue,
                        value: "~=".to_string(),
                        line,
                        column,
                    });
                    i += 2;
                } else {
                    tokens.push(Token {
                        token_type: TokenType::BitwiseNot,
                        value: "~".to_string(),
                        line,
                        column,
                    });
                    i += 1;
                }
            }
            '(' => {
                tokens.push(Token {
                    token_type: TokenType::OpenParenthesis,
                    value: "(".to_string(),
                    line,
                    column,
                });
                i += 1;
            }
            ')' => {
                tokens.push(Token {
                    token_type: TokenType::CloseParenthesis,
                    value: ")".to_string(),
                    line,
                    column,
                });
                i += 1;
            }
            '{' => {
                tokens.push(Token {
                    token_type: TokenType::OpenBrace,
                    value: "{".to_string(),
                    line,
                    column,
                });
                i += 1;
            }
            '}' => {
                tokens.push(Token {
                    token_type: TokenType::CloseBrace,
                    value: "}".to_string(),
                    line,
                    column,
                });
                i += 1;
            }
            '[' => {
                tokens.push(Token {
                    token_type: TokenType::OpenBracket,
                    value: "[".to_string(),
                    line,
                    column,
                });
                i += 1;
            }
            ']' => {
                tokens.push(Token {
                    token_type: TokenType::CloseBracket,
                    value: "]".to_string(),
                    line,
                    column,
                });
                i += 1;
            }
            ',' => {
                tokens.push(Token {
                    token_type: TokenType::Comma,
                    value: ",".to_string(),
                    line,
                    column,
                });
                i += 1;
            }
            '.' => {
                tokens.push(Token {
                    token_type: TokenType::Dot,
                    value: ".".to_string(),
                    line,
                    column,
                });
                i += 1;
            }
            ';' => {
                tokens.push(Token {
                    token_type: TokenType::Semicolon,
                    value: ";".to_string(),
                    line,
                    column,
                });
                i += 1;
            }
            ':' => {
                tokens.push(Token {
                    token_type: TokenType::Colon,
                    value: ":".to_string(),
                    line,
                    column,
                });
                i += 1;
            }
            '?' => {
                tokens.push(Token {
                    token_type: TokenType::QuestionMark,
                    value: "?".to_string(),
                    line,
                    column,
                });
                i += 1;
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut value = String::new();
                value.push(source.chars().nth(i).unwrap());
                i += 1;
                while source.chars().nth(i).unwrap().is_alphanumeric() || source.chars().nth(i).unwrap() == '_' {
                    value.push(source.chars().nth(i).unwrap());
                    i += 1;
                }

                let token_type = match value.as_str() {
                    "void" => TokenType::Void,
                    "string" => TokenType::String,
                    "char" => TokenType::Char,
                    "int" => TokenType::Int32,
                    "int64" => TokenType::Int64,
                    "uint" => TokenType::UInt32,
                    "uint64" => TokenType::UInt64,
                    "float" => TokenType::Float,
                    "double" => TokenType::Double,
                    "short" => TokenType::Short,
                    "long" => TokenType::Long,
                    "byte" => TokenType::Byte,
                    "boolean" => TokenType::Boolean,
                    "const" => TokenType::Const,
                    "import" => TokenType::Import,
                    "func" => TokenType::Func,
                    "lambda" => TokenType::Lambda,
                    "array" => TokenType::Array,
                    "map" => TokenType::Map,
                    "ptr" => TokenType::Pointer,
                    "return" => TokenType::Return,
                    "true" | "false" => TokenType::BooleanLiteral,
                    _ => TokenType::IdentifierLiteral,
                };

                tokens.push(Token {
                    token_type,
                    value,
                    line,
                    column,
                });
            }
            '0'..='9' => {
                let mut value = String::new();
                value.push(source.chars().nth(i).unwrap());
                i += 1;
                while source.chars().nth(i).unwrap().is_numeric() || source.chars().nth(i).unwrap() == '.' {
                    value.push(source.chars().nth(i).unwrap());
                    i += 1;
                }

                let token_type = match value.contains('.') {
                    true => TokenType::FloatLiteral,
                    false => TokenType::NumberLiteral,
                };

                tokens.push(Token {
                    token_type,
                    value,
                    line,
                    column,
                });                
            }
            '"' => {
                let mut value = String::new();
                i += 1;
                while source.chars().nth(i).unwrap() != '"' {
                    value.push(source.chars().nth(i).unwrap());
                    i += 1;
                }
                i += 1;

                tokens.push(Token {
                    token_type: TokenType::StringLiteral,
                    value,
                    line,
                    column,
                });
            }
            '\'' => {
                let mut value = String::new();
                i += 1;
                while source.chars().nth(i).unwrap() != '\'' {
                    value.push(source.chars().nth(i).unwrap());
                    i += 1;
                }
                i += 1;

                if value.len() != 1 {
                    panic!("Invalid char literal");
                }

                tokens.push(Token {
                    token_type: TokenType::CharLiteral,
                    value,
                    line,
                    column,
                });
            }
            _ => panic!("Unexpected character: {}", source.chars().nth(i).unwrap()),
        }
    }

    tokens.push(Token {
        token_type: TokenType::EndOfFile,
        value: "".to_string(),
        line,
        column,
    });

    tokens
}