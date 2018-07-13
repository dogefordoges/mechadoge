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

fn interpret(mut tokens: Vec<String>, context: processor::Context) {
    tokens.reverse();
    let mut stack: Vec<String> = tokens.clone();

    let mut global_variables: HashMap<String, String> = HashMap::<String, String>::new();

    let mut stack_pointer = stack.len() - 1;

    loop {
        if stack_pointer == 0 { break }

        let token: &str = &stack[stack_pointer].clone();

        //println!("{}", token);

        match token {
            "very" => {
                stack.pop();//pop "very"
                
                let variable_name: String = stack.pop().unwrap();
                stack_pointer = stack_pointer - 1;
                
                let variable: String = stack.pop().unwrap();
                stack_pointer = stack_pointer - 1;
                
                global_variables.insert(variable_name, variable);
            },
            "plz" => {
                stack.pop();//pop "plz"
                
                let function_name: String = stack.pop().unwrap();
                stack_pointer = stack_pointer - 1;
                let mut function_pointer: String = function_name.clone();

                if !function_pointer.contains("FUNC_START") {
                    if global_variables.contains_key(&function_pointer) {
                        function_pointer = global_variables.get(&function_pointer).unwrap().to_string();
                    } else {
                        function_pointer = "NONE".to_string();
                    }
                    
                }

                if function_pointer != "NONE".to_string() {
                    let function_body: Vec<String> = context.function_heap.get(&function_pointer).unwrap().to_vec();

                    let num_args: usize = function_body[0].parse().unwrap();

                    let mut local_scope: HashMap<String, String> = HashMap::<String, String>::new();

                    for i in 0..num_args {
                        let parameter_name: String = function_body[1+i].clone();
                        let parameter: String = stack.pop().unwrap();
                        stack_pointer = stack_pointer - 1;
                        local_scope.insert(parameter_name, parameter);
                    }

                    for i in (1+num_args..function_body.len()).rev() {
                        if local_scope.contains_key(&function_body[i]) {
                            stack.push(local_scope.get(&function_body[i]).unwrap().to_string());
                        } else {
                            stack.push(function_body[i].clone());
                        }
                    }

                    //for s in stack.clone().iter().rev() { println!("{}", s); }
                    //println!(" ");

                    stack_pointer = stack.len() - 1;
                } else {
                    let name: &str = &function_name; 
                    match name {
                        "bark" => {
                            let value: String = stack.pop().unwrap();
                            stack_pointer = stack_pointer - 1;

                            if value.contains("STR") {
                                if context.string_heap.contains_key(&value) {
                                    let str_value: String = context.string_heap.get(&value).unwrap().to_string();
                                    println!("{}", str_value);                                    
                                } else {
                                    panic!("string: {} not found", value);
                                }                              
                            } else if global_variables.contains_key(&value) {
                                let global_variable: String = global_variables.get(&value).unwrap().to_string();

                                if global_variable.contains("STR") {
                                    if context.string_heap.contains_key(&global_variable) {
                                        let str_value: String = context.string_heap.get(&global_variable).unwrap().to_string();
                                        println!("{}", str_value);                                    
                                    } else {
                                        panic!("string: {} not found", global_variable);
                                    }
                                }
                                
                            } else {
                                panic!("mechadoge can't bark: {}", value);
                            }                            
                        },
                        _ => {
                            panic!("function: {} not found", function_name);
                        }
                    }
                }
                
            },
            _ => {
                //what to do here? panic?!
                
                //println!("{}", token);
            }
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
