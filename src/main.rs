use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
mod processor;

fn read_to_lines(filename: &str) -> Vec<String> {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut lines = Vec::<String>::new();

    contents.lines().for_each( |line| lines.push(line.to_string()));

    return lines;
}

fn interpret(tokens: Vec<String>, context: processor::Context) {
    let mut stack: Vec<String>  = tokens.clone();
    stack.reverse();
    //for s in stack.clone() { println!("{}", s); }
    let mut stack_pointer: usize = stack.len() - 1;
    let mut global_variables: HashMap<String, String> = HashMap::<String, String>::new();

    loop {

        let token: &str = &stack[stack_pointer].clone();

        //println!("{}", token);

        match token {
            "very" => {
                let variable: String = stack.pop().unwrap();
                let variable_name: String = stack.pop().unwrap();
                stack.pop();//pop "very"

                global_variables.insert(variable_name, variable);
            },
            "plz" => {
                let function_name: String = stack[stack_pointer+1].clone();
                let mut function_pointer: String = function_name.clone();

                if !function_pointer.contains("FUNC_START") {
                    if global_variables.contains_key(&function_pointer) {
                        function_pointer = global_variables.get(&function_pointer).unwrap().to_string();                        
                    } else {
                        function_pointer = "NONE".to_string();
                    }
                }

                if function_pointer != "NONE" {
                    if context.function_heap.contains_key(&function_pointer) {
                        let function_body: Vec<String> = context.function_heap.get(&function_pointer).unwrap().to_vec();

                        let num_args: usize = function_body[0].parse().unwrap();

                        let mut local_scope: HashMap<String, String> = HashMap::<String, String>::new();
                        
                        for i in 0..num_args {
                            let parameter_name: String = function_body[1+i].clone();
                            let parameter_value: String = stack.pop().unwrap();
                            local_scope.insert(parameter_name, parameter_value);
                        }

                        stack.pop();//pop off function name
                        stack.pop();//pop off "plz"

                        let function: Vec<String> = function_body[num_args+1..function_body.len()].to_vec();

                        for code in processor::stackify(function).iter().rev() {
                            if local_scope.contains_key(code) {
                                stack.push(local_scope.get(code).unwrap().to_string());
                            } else {
                                stack.push(code.to_string());
                            }                            
                        }

                        stack_pointer = stack.len() - 1;
                        
                    } else {                        
                        panic!("function pointer: {} has no definition", function_pointer);   
                    }
                } else {
                    let func: &str = &function_name;
                    match func {
                        "bark" => {
                            let mut value: String = stack.pop().unwrap();
                            stack.pop();//pop off "bark"
                            stack.pop();//pop off "plz"

                            if value == "bark" {
                                panic!("no input available for bark");
                            }

                            if value.contains("GLOBAL") {
                                value = global_variables.get(&value).unwrap().to_string();
                            }

                            if value.contains("STR") {
                                value = context.string_heap.get(&value).unwrap().to_string();
                            }

                            println!("{}", value);
                        },
                        _ => {
                            panic!("function: {} not found", function_name);
                        }
                    }                    
                }
            },
            _ => {
                ()
            }
        }

        if stack_pointer == 0 {
            break
        }
        stack_pointer = stack_pointer - 1;
    }
}

fn main() {
    let input_lines: Vec<String> = read_to_lines("data/preprocessor_test_input.mdg");
    let (processed_code, context): (Vec<String>, processor::Context) = processor::preprocess_code(input_lines);
    //for t in processed_code.clone() { println!("{}", t); }
    interpret(processed_code, context);
}
