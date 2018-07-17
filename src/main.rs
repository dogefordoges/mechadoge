use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
mod processor;
use processor::Snack;

fn read_to_lines(filename: &str) -> Vec<String> {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut lines = Vec::<String>::new();

    contents.lines().for_each( |line| lines.push(line.to_string()));

    return lines;
}

fn interpret(tokens: Vec<Snack>, context: processor::Context) {
    let mut stack: Vec<Snack>  = tokens.clone();
    stack.reverse();
    
    let mut stack_pointer: usize = stack.len() - 1;
    let mut global_variables: HashMap<String, Snack> = HashMap::<String, Snack>::new();

    loop {

        let token: Snack = stack[stack_pointer].clone();

        match token {
            Snack::STRING(s) => {
                let t: &str = &s;

                match t {
                    "very" => {
                        let variable: Snack = stack.pop().unwrap();
                        //println!("{:?}", variable);

                        let variable_name: Snack = stack.pop().unwrap();
                        
                        //println!("{:?}", variable_name);

                        match variable_name {
                            Snack::STRING(s) => {
                                if s.contains("GLOBAL") {
                                    global_variables.insert(s, variable);
                                } else {
                                    panic!("Expecting global variable. Found {:?}", s);
                                }
                            },
                            _ => {
                                panic!("Expecting string. Found {:?}", variable_name);
                            }
                        }
                        stack.pop();//pop "very"
                    },
                    "plz" => {
                        let function_name: Snack = stack[stack_pointer+1].clone();

                        match function_name {
                            Snack::STRING(s) => {
                                let mut function_pointer: String = s.clone();

                                if !function_pointer.contains("FUNC_START") {
                                    if global_variables.contains_key(&function_pointer) {
                                        function_pointer = global_variables.get(&function_pointer).unwrap().to_string();                        
                                    } else {
                                        function_pointer = "NONE".to_string();
                                    }
                                }

                                if function_pointer != "NONE" {
                                    if context.function_heap.contains_key(&function_pointer) {
                                        let function: &processor::Function = context.function_heap.get(&function_pointer).unwrap();

                                        let mut local_scope: HashMap<String, Snack> = HashMap::<String, Snack>::new();
                                        
                                        for i in 0..function.num_args {
                                            let parameter_value: Snack = stack.pop().unwrap();
                                            local_scope.insert(function.parameter_names[i].clone(), parameter_value);
                                        }

                                        stack.pop();//pop off function name
                                        stack.pop();//pop off "plz"

                                        for code in processor::stackify(function.body.clone()).iter().rev() {
                                            match code {
                                                processor::Snack::STRING(s) => {
                                                    if local_scope.contains_key(s) {
                                                        let snack: Snack = local_scope.get(s).unwrap().clone();
                                                        stack.push(snack);
                                                    }
                                                },
                                                _ => {
                                                    stack.push(code.clone());
                                                }
                                            }
                                        }

                                        stack_pointer = stack.len() - 1;
                                        
                                    } else {
                                        panic!("function pointer: {} has no definition", function_pointer);   
                                    }
                                } else {
                                    let func: &str = &s;
                                    match func {
                                        "bark" => {
                                            let value: Snack = stack.pop().unwrap();
                                            stack.pop();//pop off "bark"
                                            stack.pop();//pop off "plz"

                                            match value {
                                                Snack::STRING(s2) => {
                                                    //if value == "bark" {
                                                    //    panic!("no input available for bark");
                                                    //}

                                                    if s2.contains("GLOBAL") {
                                                        println!("{}", global_variables.get(&s2).unwrap());
                                                    }

                                                    if s2.contains("STR") {
                                                        println!("{}", context.string_heap.get(&s2).unwrap());
                                                    }

                                                    println!("{}", s2);
                                                    
                                                },
                                                _ => {
                                                    panic!("Expecting STRING for function name, found {:?}", value);
                                                }
                                            }
                                        },
                                        _ => {
                                            panic!("function: {:?} not found", s);
                                        }
                                    }
                                }
                            },
                            _ => {
                                panic!("Expecting STRING, found {:?}", function_name.clone());
                            }
                        }
                    },
                    _ => {
                        ()
                    }
                }
            },
            _ => {
                //should panic?
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
    let (processed_code, context): (Vec<Snack>, processor::Context) = processor::preprocess_code(input_lines);
    
    interpret(processed_code, context);
}
