use crate::parser::{Program, Statement, Expression, Type};

#[derive(Debug, PartialEq)]
struct Function {
    name: String,
    parameters: Vec<Parameter>,
    body: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
struct Parameter {
    name: String,
    _type: Type,
    optional: bool,
    default_value: Option<Expression>,
}

pub fn execute(program: Program) {
    let mut i = 0;

    let mut functions = Vec::new();
    
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

                let function = Function {
                    name,
                    parameters,
                    body,
                };

                functions.push(function);

                i += 1;
            }
            Statement::FunctionCall(function) => {
                let name: String = function.name.clone();
                let mut parameters: Vec<Expression> = Vec::new();
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

                    let prog = Program {
                        statements: vec![statement.clone()],
                    };

                    execute(prog);

                    k += 1;
                }

                i += 1;
            }
            Statement::Return(expression) => {
                println!("{:?}", expression);
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }
}