use crate::lexer::{lex};
use crate::parser::{Program, Statement, Expression, Type, parse};
use std::fs::File;
use std::io::prelude::*;
use std::thread::current;

#[derive(Debug, PartialEq)]
pub struct Executor {
    pub functions: Vec<Function>,
    pub variables: Vec<Variable>,
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub body: Vec<Statement>,
    pub return_type: Type,
}

#[derive(Debug, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub _type: Type,
    pub optional: bool,
    pub default_value: Option<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct Variable {
    pub name: String,
    pub _type: Type,
    pub value: Option<Expression>,
    pub immutable: bool,
}

pub fn execute(program: Program) -> Executor {
    let mut i = 0;

    let mut functions: Vec<Function> = Vec::new();
    let mut global_variables: Vec<Variable> = Vec::new();

    while i < program.statements.len() {
        let statement = &program.statements[i];

        match statement {
            Statement::FunctionDeclaration(function) => {
                let name: String = function.name.clone();
                let mut parameters: Vec<Parameter> = Vec::new();
                for parameter in &function.parameters {
                    parameters.push(Parameter {
                        name: parameter.0.clone(),
                        _type: parameter.1.clone(),
                        optional: parameter.2,
                        default_value: Some(parameter.3.clone()),
                    });
                }

                let body: Vec<Statement> = function.body.clone();
                let return_type: Type = function.return_type.clone();

                let function = Function {
                    name,
                    parameters,
                    body,
                    return_type,
                };

                functions.push(function);

                i += 1;
            }
            Statement::FunctionCall(function) => {
                let name: String = function.name.clone();
                let mut parameters: Vec<Expression> = Vec::new();
                let mut local_variables: Vec<Variable> = Vec::new();
                for parameter in &function.arguments {
                    parameters.push(parameter.clone());
                }

                let function = functions.iter().find(|f| f.name == name);
                if function == None {
                    panic!("Function '{}' not found", name);
                }
                let function = function.unwrap().clone();

                let mut param_list = Vec::new();
                for (i, parameter) in function.parameters.iter().enumerate() {
                    let value = if i < parameters.len() {
                        parameters[i].clone()
                    } else if parameter.optional {
                        parameter.default_value.clone().unwrap()
                    } else {
                        panic!("Missing parameter '{}'", parameter.name);
                    };

                    param_list.push((parameter.name.clone(), value));
                }

                let mut j = 0;

                while j < function.parameters.len() {
                    let parameter = &function.parameters[j];

                    if parameter.optional && parameters.len() <= j {
                        parameters.push(parameter.default_value.clone().unwrap());
                    }

                    j += 1;
                }

                let mut k = 0;

                while k < function.body.len() {
                    let statement = &function.body[k];

                    match statement {
                        Statement::Return(expression) => {
                            break;
                        }
                        Statement::VariableDeclaration(variable) => {
                            let name: String = variable.name.clone();
                            let _type: Type = variable.type_.clone();
                            let value: Option<Expression> = Some(variable.value.clone());
                            let immutable: bool = variable.immutable;

                            let variable = Variable {
                                name,
                                _type,
                                value,
                                immutable,
                            };

                            local_variables.push(variable);

                            k += 1;
                        }
                        _ => panic!("Unhandled statement '{:?}'", statement),
                    }

                    k += 1;
                }

                i += 1;
            }
            Statement::VariableDeclaration(variable) => {
                let name: String = variable.name.clone();
                let _type: Type = variable.type_.clone();
                let value: Option<Expression> = Some(variable.value.clone());
                let immutable: bool = variable.immutable;

                let variable = Variable {
                    name,
                    _type,
                    value,
                    immutable,
                };

                global_variables.push(variable);

                i += 1;
            }
            Statement::Import(import) => {
                let methods = import.methods.clone();
                let path = import.path.clone();

                if path.ends_with(".wly") {
                    let current_dir = std::env::current_dir().unwrap();
                    let mut file = File::open(current_dir.join(path)).expect("File not found");
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)
                        .expect("Something went wrong reading the file");
                    let tokens = lex(contents);
                    let program = parse(&tokens);
                    let executor = execute(program);

                    for function in executor.functions {
                        functions.push(function);
                    }

                    for variable in executor.variables {
                        global_variables.push(variable);
                    }
                } else {
                    panic!("Unsupported file type");
                }

                i += 1;
            }
            _ => panic!("Unhandled statement '{:?}'", statement),
        }
    }

    println!("{:#?}", functions);
    println!("{:#?}", global_variables);

    return Executor {
        functions,
        variables: global_variables,
    };
}