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

fn bark(value: String, global_variables: &HashMap<String, Snack>, string_heap: &HashMap<String, Snack>) {

    let mut print_value: String = value.clone();

    if print_value.contains("GLOBAL") {
        print_value = global_variables.get(&print_value).unwrap().to_string();
    }

    if print_value.contains("STR") {
        println!("{}", print_value);
        print_value = string_heap.get(&print_value).unwrap().to_string();
    }

    println!("{}", print_value);
}

fn add(v1: &Snack, v2: &Snack) -> Snack {
    match v1 {
        Snack::INT(i1) => {
            match v2 {
                Snack::INT(i2) => {
                    Snack::INT(i1 + i2)
                },
                _ => {
                    panic!("Numeric values given to add must be of same type, found: {:?} {:?}", v1, v2);
                }
            }
        },
        Snack::UINT(u1) => {
            match v2 {
                Snack::UINT(u2) => {
                    Snack::UINT(u1 + u2)
                },
                _ => {
                    panic!("Numeric values given to add must be of same type, found: {:?} {:?}", v1, v2);
                }
            }
        },
        Snack::FLOAT(f1) => {
            match v2 {
                Snack::FLOAT(f2) => {
                    Snack::FLOAT(f1 + f2)
                },
                _ => {
                    panic!("Numeric values given to add must be of same type, found: {:?} {:?}", v1, v2);
                }
            }
        },
        _ => {
            panic!("Only numeric values allowed as input to add");
        }
    }
}

fn interpret(tokens: Vec<Snack>, context: processor::Context) {
    let mut stack: Vec<Snack>  = tokens.clone();
    stack.reverse();
    
    let mut stack_pointer: usize = stack.len() - 1;
    let mut global_variables: HashMap<String, Snack> = HashMap::<String, Snack>::new();

    loop {

        let token: Snack = stack[stack_pointer].clone();

        //println!("{}", token);

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

                                        stack.pop();
                                        stack.pop();

                                        for code in processor::stackify(function.body.clone()).iter().rev() {
                                            match code {
                                                processor::Snack::STRING(local_var) => {
                                                    if local_scope.contains_key(local_var) {
                                                        let snack: Snack = local_scope.get(local_var).unwrap().clone();
                                                        stack.push(snack);
                                                    } else {
                                                        stack.push(code.clone())
                                                    }
                                                },
                                                _ => {
                                                    stack.push(code.clone())
                                                }
                                            }
                                        }

                                        stack_pointer = stack.len() - 1;
                                        
                                    }
                                } else {
                                    let func: &str = &s;

                                    match func {
                                        "bark" => {
                                            let value: Snack = stack.pop().unwrap();
                                            stack.pop();//pop off bark
                                            stack.pop();//pop off plz

                                            bark(value.to_string(), &global_variables, &context.string_heap);
                                        },
                                        "add" => {
                                            let v1: Snack = stack.pop().unwrap();
                                            let v2: Snack = stack.pop().unwrap();
                                            stack.pop();//pop off add
                                            stack.pop();//pop off plz

                                            stack.push(add(&v1, &v2));
                                        },
                                        _ => {
                                            panic!("function_pointer: {} has no definition", s);
                                        }                                        
                                    }                                    
                                }
                            },
                            _ => {
                                panic!("Expecting {} to be string", function_name);
                            }
                        }
                    },
                    _ => {
                        //should panic?
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
    let input_lines: Vec<String> = read_to_lines("data/fun_program.mdg");
    let (processed_code, context): (Vec<Snack>, processor::Context) = processor::preprocess_code(input_lines);
    
    interpret(processed_code, context);
}
