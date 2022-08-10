pub mod functions;
use functions::variable::*;
use functions::*;

#[derive(Debug)]
pub struct Interpreter {
    pub var_stack: Vec<Variable>,
    pub fn_stack: Vec<Function>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let var_stack: Vec<Variable> = Vec::new();
        let fn_stack: Vec<Function> = Vec::new();
        let i = Interpreter {
            var_stack: var_stack,
            fn_stack: fn_stack,
        };
        return i;
    }

    pub fn input(&mut self, input: &str)  -> Result<Option<f32>, String> {
        // First we check if the input is empty, if it is we return an error
        println!("Given Input: {}", input);
        println!("Current State of Interpreter: {:?}", self);

        if input.is_empty() || input == " " {
            return Ok(None);
        }


        let mut tokens:Vec<&str> = input.split(" ").collect();
        println!("Tokens: {:?}", tokens);
        //println!("Tokens: {:?}", tokens);
        if tokens.len() > 1 && !input.contains(|c:char| c.is_ascii_alphabetic() || c.is_ascii_punctuation()){
            return Err("Invalid Input".to_string());
        }


        // If input is a single token and it is a non numeric input, then we check if it is a previous variable
        if tokens.len() == 1 && tokens[0].contains(|c:char| c.is_ascii_alphabetic()){
            let result = self.check_stack_for_variable(tokens[0]);
            if result.is_none() {
                let self_copy = self.fn_stack.clone();
                for function in self_copy.iter() {
                    if input.starts_with(&function.name) {
                        let operation_for_solve = function.prep_functions(&input);
                        if operation_for_solve.is_none() {
                            return Err("Invalid Function Call".to_string())
                        } else {
                            let op = operation_for_solve.unwrap();
                            let r = self.solve_math(&op);
                            //println!("FunctionResult: {:?}", r);
                            let ans = r.unwrap();
                            let num = ans.parse::<f32>();
                            return Ok(Some(num.unwrap()))

                        }
                    }
                }
                return Err("Variable Not Found".to_string());
            } else {
                let value = result.unwrap();
                let final_return = value.parse::<f32>();
                let final_value = final_return.unwrap();
                return Ok(Some(final_value));
            }
        }

        // Second we check if input is trying to create a function, if valid we store it.
        if input.contains("fn") {
            if input.contains("(") && input.contains(")") {
                let mut total_input = input.to_string();
                total_input.retain(|c:char| c != ' ');
                let index_of_par = total_input.find("(");
                let sec_index_of_par = total_input.rfind(")");
                if index_of_par == Some(0) && sec_index_of_par == Some(total_input.len()-1) {
                    return Err("Fn Inside Expression".to_string());
                }
                
            }
            let possible_function = self.check_function_validity(input);
            if possible_function.is_none() {
                return Err(String::from("Function is not valid"));
            } else {
                let new_function = possible_function.unwrap();
                //println!("Saved Function: {:?}", new_function);
                // Here we check if function with same name already exists. if it does, 
                // we need to redefine it
                let fn_stack_copy = self.fn_stack.clone();
                let new_function_copy = new_function.clone();
                for function in fn_stack_copy.iter() {
                    if function.name == new_function_copy.name {
                        self.fn_stack.retain(|f| f.name != new_function_copy.name);
                        self.fn_stack.push(new_function); 
                        return Ok(None);
                    }
                }
                // If function was found, we push it anyways
                self.fn_stack.push(new_function);
                return Ok(None);
            }
        }

        //Now we check if the whole statment is an assignment
        if input.contains("=") && !input.contains("fn") && !input.contains("=>") {
            /*
            If second token is a sole = operator, we know its an assignment and token should look like this
            tokens = ["var_name", "=", "operations to perform"];
            */
            //let new_var = Variable::new(tokens[0].to_string(), tokens[])
            let mut ass = input.to_string();
            ass.retain(|c:char| c != ' ');
            let assign: Vec<&str> = ass.split("=").collect();
            //println!("assign: {:?}", assign);
            // First we check if right hand expression can be solved, if not then we know we have 
            // chained assignments
            let assignment: Vec<&str> = ass.splitn(2, "=").collect();
            let operation_to_solve = assignment[1];
            
            let result = self.solve_math(operation_to_solve);
            if result.is_some() {
                if assignment[0].contains(|c:char| c.is_alphabetic()) {
                    let var_name = assignment[0].to_string();
                        // check to see if variable trying to be assigned is already a function name
                    for function in self.fn_stack.iter() {
                        if function.name == var_name {
                            return Err("Already a function".to_string())
                        }
                    }
                    // No function named variable found so we continue
    
                    let new_var = Variable::new(var_name, result.clone());
                    let var_stack_copy = self.var_stack.clone();
                    let new_variable_copy = new_var.clone();
                    for vari in var_stack_copy.iter() {
                        if vari.name == new_variable_copy.name {
                            self.var_stack.retain(|f| f.name != new_variable_copy.name);
                            self.var_stack.push(new_var); 
                            let value = result.unwrap();
                            let new_value = value.parse::<f32>();
                            return Ok(Some(new_value.unwrap()));
                        }
                    }
                    // Regardless, we push it into the var_stack
                    self.var_stack.push(new_var);
                    let value = result.unwrap();
                    let new_value = value.parse::<f32>();
                    return Ok(Some(new_value.unwrap()));
                } else {
                    return Err("Invalid variable identifier".to_string());
                }
            }


            let mut ans: String = String::new();
            let mut finished = false;
            let mut i: usize = 0;
            while !finished {
                if i < assign.len() - 1 {
                    if assign[i].contains(|c:char| c.is_alphabetic()) {
                        let var_name = assign[i].to_string();
                        // check to see if variable trying to be assigned is already a function name
                        for function in self.fn_stack.iter() {
                            if function.name == var_name {
                                return Err("Already a function".to_string())
                            }
                        }
                        // check to see if already a function
                        let new_var = Variable::new(var_name, None);
                        self.var_stack.push(new_var);
                        i += 1;
                        continue;

                    } else {
                        return Err("Invalid Variable Name".to_string());
                    }
                } else {
                    let operation = assign[i].to_string();
                    let result = self.solve_math(&operation);
                    if result.is_none() {
                        return Err("Invalid assigment".to_string());
                    } else {
                        ans = result.unwrap();
                    }
                }  
                if i == assign.len() - 1 {
                    finished = true;
                }
                i += 1;
            }
            let var_stack_copy = self.var_stack.clone();
            for i in 0..var_stack_copy.len() {
                if var_stack_copy[i].value.is_none() {
                    self.var_stack[i].value = Some(ans.clone());
                }
            }
            //println!("i after new var assignment: {:?}", self);
            let final_value = ans.parse::<f32>();
            return Ok(Some(final_value.unwrap()));
        }


        //Now we must check if the first word is a function itself
        let self_copy = self.fn_stack.clone();
        for function in self_copy.iter() {
            if input.starts_with(&function.name) {
                let operation_for_solve = function.prep_functions(&input);
                if operation_for_solve.is_none() {
                    // if operation goes wrong, we may have recursive function calls
                    let rr = self.function_calls(&tokens);
                    if rr.is_some() {
                        let rrr = rr.unwrap();
                        return Ok(Some(rrr.parse::<f32>().unwrap()))
                    }
                    return Err("Invalid Function Call".to_string())
                } else {
                    let op = operation_for_solve.unwrap();
                    let r = self.solve_math(&op);
                    // println!("FunctionResult: {:?}", r);
                    let ans = r.unwrap();
                    let num = ans.parse::<f32>();
                    return Ok(Some(num.unwrap()))

                }
            }
        }

        // Now we prepare the variable with its correct values
        for i in 0..tokens.len() {
            if tokens[i].contains(|c:char| c.is_ascii_alphabetic()) {
                let result = self.check_stack_for_variable(tokens[i]);
                if result.is_none() {
                    //return None;
                } else {
                    tokens.remove(i);
                    let value = result.unwrap();
                    let value_str = Interpreter::string_to_str(value);
                    tokens.insert(i,value_str);
                }
            }
        }
        for token in tokens.iter() {
            if token.contains(|c:char| c.is_ascii_alphabetic()) {
                return Err("Invalid Input, unrecognized variable".to_string())
            } 
        }

        /*
        At this point it isn't any of the following...
        - Function Making Call 
        - Assignment to new variable(s)
        - Function call to previously made function 
        - Contains no, or any, unrecognized variables

        Therefore, it must be pure math at this point
        */
        let tokens_string = tokens.join("");
        let final_ans = self.solve_math(&tokens_string);
        if final_ans.is_none() {
            return Err("Invalid input".to_string());
        }
        let final_result = final_ans.unwrap();
        let num = final_result.parse::<f32>();
        return Ok(Some(num.unwrap()));
    }

    fn check_stack_for_variable(&self, name: &str) -> Option<String> {
        for var in &self.var_stack {
            if var.name == name {
                return var.value.clone();
            }
        }
        return None;
    }

    pub fn check_function_validity(&self, input: &str) -> Option<Function> {
        let function_setup: &str;
        let operation: &str;
        let tokens: Vec<&str>;
        let function_name: &str;
        let mut function_parameters: Vec<&str> = Vec::new();

        if input.find("=>").is_none() {
            return None;
        } else {
            let ending_index = input.find("=>").unwrap();
            function_setup = input.get(3..ending_index + 2).unwrap(); //start from 3 to exclude fn keyword and add two to include the arrow itself!
            operation = input.get(ending_index + 2..).unwrap();
            //println!("{}", function_setup);
            // println!("{}", operation);
            tokens = function_setup.split_whitespace().collect();
            // Here is how the tokens vector should look like
            /*
            tokens = ["function_name", "parameters" -how every many, "=>"]
            */
            function_name = tokens[0];
            for i in 1..tokens.len() - 1 {
                function_parameters.push(tokens[i]);
            }

            // If function name is already variable name, we should return None 
            let var_stack = self.var_stack.clone();
            for var in var_stack.iter() {
                if var.name == function_name {
                    return None;
                }
            }

            // if parameters having duplicate arguments, should return none
            let parameters_num = function_parameters.len(); 
            function_parameters.dedup();
            if parameters_num != function_parameters.len() {
                return None;
            }

            //println!("Function Name: {}", function_name);
            //println!("Parameters: {:?}", function_parameters);
        }

        if tokens.len() < 2 {
            return None; // Function has no name, therefore invalid
        }

        // Code below creates clone of operation string in order to get variables in operation string only, this way we can
        // can check for invalid variables.
        let mut op_clone = operation.clone().to_string();
        op_clone.retain(|c: char| c.is_ascii_alphanumeric() || c == ' ' || c == '_');
        let mut final_op: Vec<&str> = op_clone.split_whitespace().collect();
        //println!("{:?}", final_op);
        let copy_op: Vec<&str> = final_op.clone();
        for var in copy_op.iter() {
            if !var.contains(|c: char| c.is_alphabetic()) {
                final_op.retain(|v| v != var);
                continue;
            }
            if !function_parameters.contains(var) {
                return None; // if a varaible is in operation but isn't a paramter, its an invalid function!!
            }
        }

        /*  If we reach this point, then we have valid:
            -Paramters
            -Function Name and arrow
            -Valid variables in Operations
        */
        //First we can make the function type
        let mut function_para_var: Vec<Variable> = Vec::new();
        for para in function_parameters.iter() {
            let new_para = Variable::new(para.to_string(), None);
            function_para_var.push(new_para);
        }
        let new_function = Function::new(
            function_name.to_string(),
            function_para_var,
            operation.to_string(),
        );

        return Some(new_function);
    }

    pub fn solve_math(&mut self, input: &str) -> Option<String> {
        // Turn input into vector with all tokens
        println!("Solve Math Input: {} ", input);
        let operation = input.clone();
        let mut operation = operation.to_string();
        operation.retain(|c| c != ' ');
        let mut tokens = operation.split("").collect::<Vec<&str>>();
        tokens.remove(0);
        tokens.pop();
        //println!("Tokens: {:?}", tokens)



        let mut stack: Vec<&str> = Vec::new();
        stack.push(tokens[0]);
        let mut finished = false;

        while !finished {
            let current: &str = tokens.first().unwrap();
            if !tokens.contains(&"(")
                && !tokens.contains(&")")
                && !stack.contains(&"(")
                && !stack.contains(&")")
            {
                tokens.remove(0);
                stack.append(&mut tokens);
                let eq_string = stack.join("");
                let var = self.check_for_assignment(&eq_string);
                stack.clear();
                let var_unwrapped = var.unwrap();
                if var_unwrapped.name != "" {
                    let new_vari = var_unwrapped.clone();
                    self.var_stack.push(new_vari);
                }
                let value_str = Interpreter::string_to_str(var_unwrapped.value.unwrap());
                stack.push(value_str);
                
                
            }
            if current == ")" {
                let mut curr = stack.pop().unwrap();
                let mut temp: String = String::new();
                while curr != "(" {
                    temp = curr.to_string() + &temp;
                    curr = stack.pop().unwrap();
                }
                temp.insert(0, '(');
                //println!("Temp: {}", temp);
                //println!("Stack: {:?}", stack);
                let var = self.check_for_assignment(&temp);
                // Inner Expression is invalid, therefore whole expression is invalid
                if var.is_none() {
                    return None;
                } else {
                    let var_unwrapped = var.unwrap();
                    let var_value_option = var_unwrapped.value.clone();
                    let var_value = var_value_option.unwrap();
                    let value_str = Interpreter::string_to_str(var_value);
                    if var_unwrapped.name == "" {
                        //just value, not variable
                        stack.push(value_str);
                    } else {
                        //variable too, need to store it in the variable Stack
                        self.var_stack.push(var_unwrapped);
                        stack.push(value_str);
                    }
                }
                // Only value returned so no assignment but there was a math expression solved
            }
            //Test to see if we are done
            let result = stack.join("");
            let final_result = result.parse::<f64>();
            if final_result.is_err() {
                finished = false;
            } else if final_result.is_ok() && tokens.is_empty() {
                finished = true;
                let final_ans = final_result.unwrap();
                return Some(final_ans.to_string());
            }

            // if not continue
            tokens.remove(0);
            if !tokens.is_empty() {
                stack.push(tokens[0]);
            } else {
                if !tokens.contains(&"(")
                && !tokens.contains(&")")
                && !stack.contains(&"(")
                && !stack.contains(&")")
            {
                // tokens.remove(0);
                stack.append(&mut tokens);
                let eq_strings = stack.join("");
                let vars = self.pemdas(&eq_strings);
                stack.clear();
                let var_unwrappeds = vars.unwrap();
                let value_strs = Interpreter::string_to_str(var_unwrappeds);
                stack.push(value_strs);
                let results = stack.join("");
                let final_results = results.parse::<f64>();
                if final_results.is_err() {
                    finished = false;
                } else if final_results.is_ok() && tokens.is_empty() {
                    finished = true;
                    let final_ans = final_results.unwrap();
                    return Some(final_ans.to_string());
                }
            }
            }
            //println!("Stack: {:?}", stack);
            
        }
        None
    }

    pub fn pemdas(&self, input: &str) -> Option<String> {
        /*
        Example Input:
        input = "0 + 9 / 10 * 9"
        MUST NOT HAVE
        - Parentheses
        - Anything other than operators or numbers

        CAN HAVE
        -Decimal numbers
        -Unlimited Spaces in between operators and numbers
        -Negative Numbers

        Example Output:
        output = 8.1
        */

        //println!("Input: {} ", input);
        let mut operation = input.clone();
        let mut op = operation.to_string();
        if op.contains("(") && op.contains(")") {
            op.remove(0);
            op.pop();
        }
        op.retain(|c: char| c != ' ');
        //println!("Input WWS:{}t", op);
        let mut tokens: Vec<&str> = op
            .split(|c: char| c == '/' || c == '-' || c == '+' || c == '*' || c == '%')
            .collect();
        let operators: Vec<&str> = op
            .matches(|c: char| c == '/' || c == '-' || c == '+' || c == '*' || c == '%')
            .collect();
        if operators.is_empty() && tokens.len() == 1 {
            let number = tokens.join("").parse::<f64>();
            if number.is_ok() {
                let num = number.unwrap(); 
                return Some(num.to_string());
            }
        }

        for i in 0..tokens.len() {
            if tokens[i].contains(|c:char| c.is_ascii_alphabetic()) {
                let result = self.check_stack_for_variable(tokens[i]);
                if result.is_none() {
                    //return None;
                } else {
                    tokens.remove(i);
                    let value = result.unwrap();
                    let value_str = Interpreter::string_to_str(value);
                    tokens.insert(i,value_str);
                }
            }
        }
        for token in tokens.iter() {
            if token.contains(|c:char| c.is_ascii_alphabetic()) {
                return None;
            } 
        }


        // Reinsert operators
        for i in 0..operators.len() {
            tokens.insert(i * 2 + 1, operators[i]);
        }
        //println!("Tokens: {:?}", tokens);

        // Fix negative numbers
        if tokens.contains(&"") {
            for i in 0..tokens.len() {
                if i > tokens.len() - 1 {
                    break;
                }
                let c = tokens[i];
                if c == "/" || c == "-" || c == "+" || c == "*" {
                    if tokens[i + 1] != "" {
                        continue;
                    }
                    if tokens[i + 2] == "-" && tokens[i + 1] == "" {
                        tokens.remove(i + 1);
                        tokens.remove(i + 1);
                        let number = tokens[i + 1];
                        tokens.remove(i + 1);
                        let mut num = number.to_string();
                        num.insert(0, '-');
                        let num_str = Interpreter::string_to_str(num);
                        tokens.insert(i + 1, num_str);
                    }
                }
            }
        }

        let mut finished = false;

        let mut first_num: &str = "0";
        let mut second_num: &str = "0";
        let mut operator: &str = "0";

        let mut first_index: usize = 0;
        let mut second_index: usize = 0;

        while !finished {
            // Following pemdas, if no parentheses or exponent, division and multiplication are first in order from left to right
            if tokens.contains(&"/") || tokens.contains(&"*") || tokens.contains(&"%"){
                for i in 0..tokens.len() {
                    if tokens[i] == "/" || tokens[i] == "*" || tokens[i] == "%" {
                        operator = tokens[i];
                        first_num = tokens[i - 1].clone();
                        second_num = tokens[i + 1].clone();
                        first_index = i - 1;
                        second_index = i + 1;
                        break;
                    }
                }
            } else {
                //Else we do addition and subtraction starting from left to right
                for i in 0..tokens.len() {
                    if tokens[i] == "+" || tokens[i] == "-" {
                        operator = tokens[i];
                        first_num = tokens[i - 1];
                        second_num = tokens[i + 1];
                        first_index = i - 1;
                        second_index = i + 1;
                        break;
                    }
                }
            }
            // Now here we have the operation we should do and the operation
            let first_number = first_num.parse::<f64>().unwrap();
            let second_number = second_num.parse::<f64>().unwrap();
            let mut final_result: f64 = 0.0;
            match operator {
                "+" => {
                    final_result = first_number + second_number;
                }
                "-" => {
                    final_result = first_number - second_number;
                }
                "*" => {
                    final_result = first_number * second_number;
                }
                "/" => {
                    final_result = first_number / second_number;
                }
                "%" => {
                    final_result = first_number % second_number;
                }
                _ => {
                    println!("Something went wrong")
                }
            };
            // Now we modify the vector
            for i in first_index..second_index + 1 {
                tokens.remove(first_index);
            }
            let final_str = final_result.to_string();
            let string = Interpreter::string_to_str(final_str);
            tokens.insert(first_index, string);
            //println!("Tokens: {:?}", tokens);

            // Now we check if we can have the final result
            let tokens_as_string = tokens.join("");
            let final_ans = tokens_as_string.parse::<f64>();
            if final_ans.is_ok() {
                finished = true;
                // print!("Final ans: {}", final_ans.unwrap());
                return Some(final_ans.unwrap().to_string());
            }
        }
        None
    }

    pub fn string_to_str(s: String) -> &'static str {
        Box::leak(s.into_boxed_str())
    }


    pub fn check_for_assignment(&mut self, input: &str) -> Option<Variable> {
        let mut returned_tuple: Option<Variable> = None;
        let mut input = input.to_string();
        if input.contains("(") {
            input.remove(0);
            input.pop();
        }

        if input.contains("=") && !input.contains("fn") {
            let mut var_name = String::new();
            let mut expression = String::new();
            let mut operator = String::from("=");
            let mut operator_found = false;
            for c in input.chars() {
                if (c.is_alphanumeric() && !operator_found) || c == '_' {
                    var_name.push(c);
                } else if (c.is_digit(10) && operator_found) || c == '.' {
                    expression.push(c);
                } else if c == '=' {
                    operator_found = true;
                }
            }
            let mut expression_result = expression.parse::<f64>();
            let mut expression_num: f64;
            if expression_result.is_err() {
                return None;
            } else {
                
                let new_var = Variable::new(var_name, Some(expression.clone()));
                returned_tuple = Some(new_var);
                return returned_tuple;
            }
        } else {

            let expression_result = self.pemdas(&input);
            let new_var = Variable::new(String::from(""), Some(expression_result.unwrap()));
            returned_tuple = Some(new_var);
            return returned_tuple;
        }
    }

    pub fn function_calls(&mut self, tokens: &Vec<&str>) -> Option<String>{
        
        let mut tokens_copy = tokens.clone();
        let mut i: usize = 0;
        let mut finished = false;
        let mut parameters: Vec<&str> = Vec::new();

        
            if tokens_copy[i].contains(|c:char| c.is_alphabetic()) {
                let possible_function = self.check_stack_for_function(tokens_copy[i]);
                if possible_function.is_none() {
                    //Function doesn't even exist in the stack so return None
                    return None;
                }
                let function = possible_function.unwrap();
                let num_of_parameters = function.parameters.len();
                tokens_copy.remove(0);
                let matches = tokens_copy.iter().filter(|s| s.contains(|c: char| c.is_alphabetic())).collect::<Vec<&&str>>();
                if matches.len() != num_of_parameters {
                    // Big Function Call Doesn't Contain Correct number of parameters, return None
                    return None;
                }
                let mut j: usize = 0;
                while j < tokens_copy.len() {
                    if tokens_copy[j].contains(|c:char| c.is_alphabetic()) {
                        let function_name = tokens_copy[j];
                        let possible_function = self.check_stack_for_function(&function_name);
                        if possible_function.is_none() {
                            //Function doesn't even exist in the stack so return None
                            return None;
                        }
                        let function = possible_function.unwrap();
                        let num_of_parameters = function.parameters.len();
                        j += 1;
                        while !tokens_copy[j].contains(|c:char| c.is_alphabetic()) {
                            parameters.push(tokens_copy[j]);
                            j += 1;
                            if j > tokens_copy.len()-1 {
                                break;
                            }
                        }
                        if parameters.len() != num_of_parameters {
                            return None;
                        }
                        let input = function_name.to_string() + " " + &parameters.join(" ");
                        let operation_for_solve = function.prep_functions(&input);
                        if operation_for_solve.is_none() {
                            return None;
                        }
                        let input_for_solve_math = operation_for_solve.unwrap();
                        let result = self.solve_math(&input_for_solve_math);
                        if result.is_none() {
                            return None;
                        }
                        let value_string = result.unwrap();
                        let value = Interpreter::string_to_str(value_string);
                        j -= (parameters.len() + 1);
                        for w in 0..parameters.len()+1 {
                            tokens_copy.remove(j);
                        }
                        tokens_copy.insert(j, value);
                        parameters.clear();
                        j += 1;
                    }
                }
                let final_input = function.name.to_string() + " " + &tokens_copy.join(" ");
                let operation_for_solve = function.prep_functions(&final_input);
                let final_result = self.solve_math(&operation_for_solve.unwrap());
                if final_result.is_none() {
                    return None;
                }
                let final_value_string = final_result.unwrap();
                return Some(final_value_string);
             }
             None
        
    }

    pub fn check_stack_for_function(&self, function_name: &str) -> Option<Function> {
        let function_stack = self.fn_stack.clone();
        for function in function_stack.iter() {
            if function.name == function_name {
                return Some(function.clone());
            }
        }
        return None;
    }
}
