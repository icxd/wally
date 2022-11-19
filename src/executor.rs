use crate::parser::{Program, Statement, Expression};

pub fn execute(program: Program) {
    let mut i = 0;
    let mut stack: Vec<i32> = Vec::new();
    let mut memory: Vec<i32> = Vec::new();
    let mut output: String = String::new();
    
    while i < program.statements.len() {
        let statement = &program.statements[i];

        match statement {
            Statement::Import(import) => {
                let path: String = import.path.to_string();
                let methods: &Expression = &import.methods;

                println!("{} {:?}", path, methods);

                match methods {
                    Expression::Identifier(identifier) => {
                        
                    }
                    Expression::ArrayLiteral(array) => {
                        for item in &array.elements {
                            match item {
                                Expression::Identifier(identifier) => {
                                    
                                },
                                _ => {
                                    panic!("Invalid import method");
                                }
                            }
                        }
                    },
                    _ => panic!("Invalid import method")
                }

                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }
}