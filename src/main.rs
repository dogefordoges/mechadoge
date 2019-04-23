use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

mod processor;
use processor::Snack;

mod standard_library;

use std::env;

fn read_to_lines(filename: &str) -> Vec<String> {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut lines = Vec::<String>::new();

    contents.lines().for_each( |line| lines.push(line.to_string()));

    return lines;
}

fn globalize(snack: Snack, global_variables: &HashMap<String, Snack>) -> Snack {
    match snack {
        Snack::STRING(s) => {
            if s.contains("GLOBAL") {
                global_variables.get(&s).unwrap().clone()
            } else {
                Snack::STRING(s)
            }
        },
        _ => { snack }
    }
}

fn interpret(mut stack: Vec<Snack>, context: &mut processor::Context) {
    stack.reverse();
    
    let mut stack_pointer: usize = stack.len() - 1;
    let mut global_variables: HashMap<String, Snack> = HashMap::<String, Snack>::new();
    let mut loop_stack: Vec<Snack> = Vec::<Snack>::new();

    loop {

        let token: Snack = stack[stack_pointer].clone();

        //println!("{}", token);

        match token {
            Snack::STRING(s) => {
                let t: &str = &s;

                match t {
                    "very" => {
                        let variable: Snack = stack.pop().unwrap();

                        let variable_name: Snack = stack.pop().unwrap();                       

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
                    "rly" => {
                        let boolean: Snack = stack.pop().unwrap();

                        match boolean {
                            Snack::BOOLEAN(b) => {
                                if b {
                                    stack[stack_pointer] = Snack::STRING("plz".to_string());

                                    let function_pointer: String = stack[stack_pointer+1].to_string();                                    

                                    if function_pointer.contains("FUNC_START") {                                        
                                        if context.num_args_function(&function_pointer) > 0 {
                                            panic!("Function after `rly` must expect 0 arguments");
                                        }
                                    } else {
                                        panic!("Expecting function, found {}", function_pointer);
                                    }
                                        

                                    stack_pointer = stack_pointer + 1;
                                } else {
                                    stack.pop();//pop off "FUNC_START"
                                    stack.pop();//pop off "rly"
                                }
                            },
                            _ => { panic!("Expecting boolean, found {:?}", boolean) }
                        }
                    },
                    "many" => {
                        let block_pointer: String = globalize(stack.pop().unwrap(), &global_variables).to_string();

                        if block_pointer.contains("FUNC_START") {

                            let num_args = context.num_args_function(&block_pointer);
                            if num_args > 0 {
                                panic!("many expects a function with no arguments, provided function expects {} arguments", num_args);
                            } else {
                                loop_stack.push(Snack::STRING(block_pointer.clone()));
                                
                                stack.push(Snack::STRING("plz".to_string()));
                                stack.push(Snack::STRING(block_pointer));
                                stack_pointer = stack.len() - 1;
                            }
                        } else if block_pointer == "many" {
                            stack.push(Snack::STRING("many".to_string()));
                            stack.push(Snack::STRING("plz".to_string()));
                            stack.push(loop_stack.last().unwrap().clone());
                            stack_pointer = stack.len() - 1;
                        } else if block_pointer == "break" {
                            stack.pop();//pop off many
                            loop_stack.pop();
                        }else {
                            panic!("Expecting function, found: {}", block_pointer);
                        }
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
                                    if context.has_function(&function_pointer) {

                                        let mut local_scope: HashMap<String, Snack> = HashMap::<String, Snack>::new();

                                        for name in context.param_names_function(&function_pointer).iter() {
                                            let parameter_value = stack.pop().unwrap();
                                            local_scope.insert(name.to_string(), parameter_value);
                                        }

                                        stack.pop();// ???
                                        stack.pop();// ???

                                        let body: Vec<Snack> = processor::stackify(context.get_body_function(&function_pointer));
                                        
                                        for code in body.iter().rev() {
                                            match code {
                                                processor::Snack::STRING(s) => {
                                                    if local_scope.contains_key(s) {
                                                        let snack: Snack = local_scope.get(s).unwrap().clone();
                                                        stack.push(snack);
                                                    } else {
                                                        stack.push(Snack::STRING(s.clone()))
                                                    }
                                                },
                                                _ => {
                                                    stack.push(code.clone());
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

                                            standard_library::bark(value.to_string(), &global_variables, &context);
                                        },
                                        "add" => {
                                            let v2: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            let v1: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            stack.pop();//pop off add
                                            stack.pop();//pop off plz

                                            stack.push(standard_library::add(&v1, &v2));
                                        },
                                        "sub" => {
                                            let v2: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            let v1: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            stack.pop();//pop off sub
                                            stack.pop();//pop off plz

                                            stack.push(standard_library::sub(&v1, &v2));
                                        },
                                        "mul" => {
                                            let v2: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            let v1: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            stack.pop();//pop off mul
                                            stack.pop();//pop off plz

                                            stack.push(standard_library::mul(&v1, &v2));
                                        },
                                        "div" => {
                                            let v2: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            let v1: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            stack.pop();//pop off div
                                            stack.pop();//pop off plz

                                            stack.push(standard_library::div(&v1, &v2));
                                        },
                                        "float" => {
                                            let v: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            stack.pop();//pop off float
                                            stack.pop();//pop off plz

                                            match v {
                                                Snack::FLOAT(_) => { stack.push(v); },
                                                Snack::INT(i) => { stack.push(Snack::FLOAT(i as f64)); },
                                                Snack::UINT(u) => { stack.push(Snack::FLOAT(u as f64)); },
                                                _ => {   
                                                    panic!("Cannot convert {:?} to float");
                                                }
                                            }
                                        },
                                        "int" => {
                                            let v: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            stack.pop();//pop off int
                                            stack.pop();//pop off plz

                                            match v {
                                                Snack::INT(_) => { stack.push(v); },
                                                Snack::FLOAT(f) => { stack.push(Snack::INT(f as i64)); },
                                                Snack::UINT(u) => { stack.push(Snack::INT(u as i64)); },
                                                _ => {
                                                    panic!("Cannot convert {:?} to int");
                                                }
                                            }                                            
                                        }
                                        "uint" => {
                                            let v: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            stack.pop();//pop off uint
                                            stack.pop();//pop off plz

                                            match v {
                                                Snack::UINT(_) => { stack.push(v); },
                                                Snack::FLOAT(f) => { stack.push(Snack::UINT(f as u64)); },
                                                Snack::INT(i) => { stack.push(Snack::UINT(i as u64)); },
                                                _ => {
                                                    panic!("Cannot convert {:?} to uint");
                                                }
                                            }                                            
                                        },
                                        "umm" => {
                                            let v: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            stack.pop();//pop off uint
                                            stack.pop();//pop off plz

                                            match v {
                                                Snack::BOOLEAN(b) => { assert!(b, "{} is not 1=1", v); },
                                                _ => { panic!("Expecting boolean, found {}", v.to_string()); }
                                            }
                                        },
                                        "is" => {
                                            let v2: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            let v1: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            stack.pop();//pop off is
                                            stack.pop();//pop off plz

                                            stack.push(standard_library::equal(&v1, &v2));
                                        },
                                        "isnot" => {
                                            let v2: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            let v1: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            stack.pop();//pop off isnot
                                            stack.pop();//pop off plz

                                            stack.push(standard_library::isnot(&v1, &v2));
                                        },
                                        "not" => {
                                            let v: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            stack.pop();//pop off is
                                            stack.pop();//pop off plz

                                            stack.push(standard_library::not(&v));
                                        },
                                        "bigger" => {
                                            let v2: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            let v1: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            stack.pop();//pop off bigger
                                            stack.pop();//pop off plz

                                            stack.push(standard_library::bigger(&v1, &v2));
                                        },
                                        "smaller" => {
                                            let v2: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            let v1: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            stack.pop();//pop off smaller
                                            stack.pop();//pop off plz

                                            stack.push(standard_library::smaller(&v1, &v2));
                                        },
                                        "biggerish" => {
                                            let v2: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            let v1: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            stack.pop();//pop off biggerish
                                            stack.pop();//pop off plz

                                            stack.push(standard_library::biggerish(&v1, &v2));
                                        },                                        
                                        "smallerish" => {
                                            let v2: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            let v1: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            stack.pop();//pop off smallerish
                                            stack.pop();//pop off plz

                                            stack.push(standard_library::smallerish(&v1, &v2));
                                        },
                                        "and" => {
                                            let v2: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            let v1: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            stack.pop();//pop off and
                                            stack.pop();//pop off plz

                                            stack.push(standard_library::and(&v1, &v2));
                                        },                                        
                                        "or" => {
                                            let v2: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            let v1: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            stack.pop();//pop off or
                                            stack.pop();//pop off plz

                                            stack.push(standard_library::or(&v1, &v2));
                                        },
                                        "nand" => {
                                            let v2: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            let v1: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            stack.pop();//pop off nand
                                            stack.pop();//pop off plz

                                            stack.push(standard_library::nand(&v1, &v2));
                                        },
                                        "each" => {

                                            let block_pointer: Snack = stack.pop().unwrap();
                                            let mut function_pointer: String = block_pointer.to_string();

                                            match block_pointer {
                                                Snack::STRING(s) => {
                                                    if global_variables.contains_key(&function_pointer) {
                                                        function_pointer = global_variables.get(&function_pointer).unwrap().to_string();
                                                    }

                                                    if context.has_function(&function_pointer) {

                                                        assert!(context.num_args_function(&function_pointer) == 1, "function must contain up to, and no more than one argument");

                                                        let mut array_pointer: String = stack.pop().unwrap().to_string();

                                                        stack.pop();//pop off each
                                                        stack.pop();//pop off plz

                                                        if global_variables.contains_key(&array_pointer) {
                                                            array_pointer = global_variables.get(&array_pointer).unwrap().to_string();
                                                        }

                                                        assert!(array_pointer.contains("ARR"), "first argument to `each` must be an array");

                                                        let arr: &Vec<Snack> = context.get_array(&array_pointer);

                                                        for i in (0..(arr.len() - 1 )).rev() {
                                                            stack.push(Snack::STRING("plz".to_string()));
                                                            stack.push(Snack::STRING(function_pointer.clone()));
                                                            //Cloning for now, but should turn this into an iter type or have a way to use references to snacks
                                                            stack.push(arr[i].clone());
                                                        }

                                                        stack_pointer = stack.len() - 1;
                                                    } else {
                                                        panic!("Provided function has no definition {}", s);
                                                    }
                                                },
                                                _ => {
                                                    panic!("Expecting Function as second pointer to `each` found: {:?}", block_pointer);
                                                }
                                            }
                                        },
                                        "at" => {
                                            let index: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            let mut array_pointer: String = stack.pop().unwrap().to_string();
                                            stack.pop();//pop off "get"
                                            stack.pop();//pop off "plz"

                                            if array_pointer.contains("GLOBAL") {
                                                array_pointer = global_variables.get(&array_pointer).unwrap().to_string();
                                            }

                                            match index {
                                                Snack::UINT(n) => {
                                                    let i: usize = n as usize;
                                                    if context.has_array(&array_pointer) {
                                                        stack.push(context.array_at(&array_pointer, i));
                                                    } else {
                                                        panic!("Empty array pointer!");
                                                    }
                                                },
                                                _ => { panic!("Expecting unsigned integer, found: {:?}", index) }
                                            }
                                        },
                                        "push" => {
                                            let value: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            let mut array_pointer: String = stack.pop().unwrap().to_string();
                                            stack.pop();//pop off "push"
                                            stack.pop();//pop off "plz"

                                            if array_pointer.contains("GLOBAL") {
                                                array_pointer = global_variables.get(&array_pointer).unwrap().to_string();
                                            }

                                            if array_pointer.contains("ARR") {
                                                context.push_array(&array_pointer, value);
                                            } else {
                                                panic!("Expecting array pointer, found {}", array_pointer);
                                            }
                                        },
                                        "pop" => {
                                            let mut array_pointer: String = stack.pop().unwrap().to_string();
                                            stack.pop();//pop off "pop"
                                            stack.pop();//pop off "plz"

                                            if array_pointer.contains("GLOBAL") {
                                                array_pointer = global_variables.get(&array_pointer).unwrap().to_string();
                                            }

                                            if array_pointer.contains("ARR") {
                                                let value: Option<Snack> = context.pop_array(&array_pointer);
                                                match value {
                                                    Some(v) => {
                                                        stack.push(v);
                                                    },
                                                    None => {
                                                        panic!("Pop on {} failed to return a value", array_pointer);
                                                    }
                                                }
                                            } else {
                                                panic!("Expecting array pointer, found {}", array_pointer);
                                            }                                            
                                        },
                                        "prepend" => {
                                            let value: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            let mut array_pointer: String = stack.pop().unwrap().to_string();
                                            stack.pop();//pop off "prepend"
                                            stack.pop();//pop off "plz"

                                            if array_pointer.contains("GLOBAL") {
                                                array_pointer = global_variables.get(&array_pointer).unwrap().to_string();
                                            }

                                            if array_pointer.contains("ARR") {
                                                context.prepend_array(&array_pointer, value);
                                            } else {
                                                panic!("Expecting array pointer, found {}", array_pointer);
                                            }
                                        },
                                        "len" => {
                                            let mut array_pointer: String = stack.pop().unwrap().to_string();
                                            stack.pop();//pop off "prepend"
                                            stack.pop();//pop off "plz"

                                            if array_pointer.contains("GLOBAL") {
                                                array_pointer = global_variables.get(&array_pointer).unwrap().to_string();
                                            }

                                            if array_pointer.contains("ARR") {
                                                stack.push(Snack::UINT(context.len_array(&array_pointer)));
                                            } else {
                                                panic!("Expecting array pointer, found {}", array_pointer);
                                            }
                                        },                                        
                                        "free" => {                                            
                                            let mut pointer: String = stack.pop().unwrap().to_string();
                                            stack.pop();//pop off "free"
                                            stack.pop();//pop off "plz"

                                            if pointer.contains("GLOBAL") {
                                                pointer = global_variables.get(&pointer).unwrap().to_string();
                                            }

                                            if pointer.contains("ARR") {
                                                context.delete_array(&pointer);
                                            } else if pointer.contains("STR") {
                                                context.delete_string(&pointer);
                                            } else if pointer.contains("FUNC") {
                                                context.delete_function(&pointer);
                                            } else {
                                                panic!("{} is not a valid pointer", pointer);
                                            }
                                        },
                                        "args" => {
                                            stack.pop();//pop off "args"
                                            stack.pop();//pop off "plz"
                                            
                                            let args = env::args();

                                            let mut mecha_args: Vec<Snack> = Vec::<Snack>::new();

                                            args.for_each(|a| mecha_args.push(Snack::STRING(a.to_string())));

                                            let array_pointer: String = format!("ARR_{}", context.num_arrays());

                                            context.new_array(array_pointer.clone(), mecha_args);

                                            stack.push(Snack::STRING(array_pointer));
                                        },
                                        "range" => {
                                            let end: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            let start: Snack = globalize(stack.pop().unwrap(), &global_variables);
                                            stack.pop();//pop off range
                                            stack.pop();//pop off plz

                                            let mut start_i: i64;
                                            let mut end_i: i64;                                            

                                            match start {
                                                Snack::INT(i) => {
                                                   start_i = i;
                                                },
                                                Snack::UINT(u) => {
                                                   start_i = u as i64;
                                                },
                                                _ => {
                                                    panic!("Expecting start of range to be int or uint, found {:?}", start); 
                                                }
                                            }

                                            match end {
                                                Snack::INT(i) => {
                                                   end_i = i;
                                                },
                                                Snack::UINT(u) => {
                                                   end_i = u as i64;
                                                },
                                                _ => {
                                                    panic!("Expecting start of range to be int or uint, found {:?}", start); 
                                                }
                                            }

                                            let mut snacks: Vec<Snack> = Vec::<Snack>::new();

                                            loop {
                                                if start_i == end_i {
                                                    break
                                                }

                                                snacks.push(Snack::INT(start_i));

                                                start_i = start_i + 1;
                                            }

                                            let array_pointer: String = format!("ARR_{}", context.num_arrays());
                                            context.new_array(array_pointer.clone(), snacks);

                                            stack.push(Snack::STRING(array_pointer));
                                            
                                        },
                                        "copy" => {
                                            let array_pointer: String = globalize(stack.pop().unwrap(), &global_variables).to_string();
                                            stack.pop();//pop off copy
                                            stack.pop();//pop off plz

                                            let new_array_pointer: String = format!("ARR_{}", context.num_arrays());
                                            let new_array = context.copy_array(&array_pointer);
                                            context.new_array(new_array_pointer.clone(), new_array);

                                            stack.push(Snack::STRING(new_array_pointer));
                                        },
                                        "reverse" => {
                                            let array_pointer: String = globalize(stack.pop().unwrap(), &global_variables).to_string();
                                            stack.pop();//pop off reverse
                                            stack.pop();//pop off plz

                                            context.reverse_array(&array_pointer);
                                        },
                                        "join" => {
                                            let join_string: String = context.get_string(&stack.pop().unwrap().to_string()).to_string();
                                            let array_pointer: String = globalize(stack.pop().unwrap(), &global_variables).to_string();
                                            stack.pop();//pop off reverse
                                            stack.pop();//pop off plz

                                            let snacks: Vec<Snack> = context.copy_array(&array_pointer);

                                            let string_pointer: String = format!("STR_{}", context.num_strings());

                                            let strings: Vec<String> = snacks.iter().map(|s| s.to_string()).collect();

                                            let new_string: String = strings.join(&join_string);

                                            context.new_string(string_pointer.clone(), new_string);

                                            stack.push(Snack::STRING(string_pointer));
                                        },
                                        "swap" => {
                                            let v1: Snack = stack.pop().unwrap();
                                            let v2: Snack = stack.pop().unwrap();
                                            stack.pop();//pop off swap
                                            stack.pop();//pop off plz

                                            stack.push(v1);
                                            stack.push(v2);                                            
                                        },
                                        "set" => {
                                            let value: Snack = stack.pop().unwrap();
                                            let variable_pointer: String = stack.pop().unwrap().to_string();
                                            stack.pop();//pop off set
                                            stack.pop();//pop off plz

                                            if variable_pointer.contains("GLOBAL") {
                                                global_variables.insert(variable_pointer, value);
                                            } else {
                                                panic!("Expecting global variable, found {}", variable_pointer);
                                            }
                                        },
                                        "setat" => {
                                            let value: Snack = stack.pop().unwrap();
                                            let index: Snack = stack.pop().unwrap();
                                            let array_pointer: String = globalize(stack.pop().unwrap(), &global_variables).to_string();
                                            stack.pop();//pop off setat
                                            stack.pop();//pop off plz

                                            if array_pointer.contains("ARR") {                                               

                                                match index {
                                                    Snack::UINT(u) => {
                                                        let n: usize = u as usize;
                                                        context.set_at_array(&array_pointer, n, value);
                                                    },
                                                    _ => {
                                                        panic!("Expecting unsigned integer, found {:?}", index);
                                                    }
                                                }
                                                
                                            } else {
                                                panic!("Expecting array, found {}", array_pointer);
                                            }
                                        },
                                        _ => {
                                            panic!("function_pointer: {} has no definition", s);
                                        },                                        
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

fn handle_input() -> Vec<String> {
    assert!(env::args().len() > 1, "No input file given");

    let filename: String = env::args().nth(1).unwrap().to_string();

    let filename_split: Vec<&str> = filename.split(".").collect();

    assert!(filename_split.len() > 1, "Input file has no file extension");

    assert_eq!(filename_split[1], "mdg", "Wrong file type, expecting `.mdg` found: {}", filename_split[1]);
    
    read_to_lines(&filename)
}

fn execute_code(lines: Vec<String>) {
    let (processed_code, mut context): (Vec<Snack>, processor::Context) = processor::preprocess_code(lines);
    interpret(processed_code, &mut context);
}

fn main() {
    execute_code(handle_input());
}

#[cfg(test)]
mod interpreter_tests {
    use super::*;

    #[test]
    fn test_fun_program() {
        execute_code(read_to_lines("data/fun_program.mdg"));
    }

    #[test]
    fn test_many_array_mutate() {
        execute_code(read_to_lines("data/many_array_mutate.mdg"));
    }

    #[test]
    fn test_much_array() {
        execute_code(read_to_lines("data/much_array.mdg"));
    }
    
    #[test]
    fn test_much_control() {
        execute_code(read_to_lines("data/much_control.mdg"));
    }

    #[test]
    fn test_much_fun() {
        execute_code(read_to_lines("data/much_fun.mdg"));
    }

    #[test]
    fn test_much_loops() {
        execute_code(read_to_lines("data/much_loops.mdg"));
    }
    
}
