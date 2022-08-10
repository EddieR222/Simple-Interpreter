pub mod variable;
use variable::*;
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Variable>,
    pub operation: String
}

impl Function {
    pub fn new(name: String, parameters: Vec<Variable>, operation: String) -> Function {
        let function = Function { name: name, parameters: parameters, operation: operation};
        return function;
    }

    pub fn print_function(&self) {
        //println!("Funtion: {}", self.name);
        //println!("Parameters: {:?}", self.parameters);
        //println!("Operation: {:?}", self.operation);
    }

    pub fn prep_functions(&self, input: &str) -> Option<String> {
        let num_of_parameters = self.parameters.len();
        let mut tokens: Vec<&str> = input.split_whitespace().collect();
        tokens.remove(0);
        if tokens.len() != num_of_parameters {
           return None;
        }
        for token in tokens.iter() {
            if token.contains(|c:char| c.is_alphabetic()) {
                return None;
            }
        }
        // At this point we know we have the correct number of input numbers 
        // so we can begin to prepare the operation for solving
        let mut operation_for_solve = self.operation.clone();
        for i in 0..num_of_parameters {
            operation_for_solve =  operation_for_solve.replace(&self.parameters[i].name, tokens[i]);
            //println!("Operation after change: {:?}", operation_for_solve);
        }

        // Now we can simply call the eval equivalent function I wrote to solve the 
        // function operation since its just now is pure numbers and tokens
        return Some(operation_for_solve);
        
    }

    
    
}

