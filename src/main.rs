use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

//For now this simply deletes any lines between quiet and loud
fn preprocess(contents: &str) -> Vec<&str> {

    let mut lines = contents.lines();
    let mut keeping = Vec::new();

    loop {
        match lines.next() {
            Some(line) => {
                if line == "quiet" {
                    loop {
                        let l = lines.next();
                        if l == Some("loud") { break }
                    }
                } else {
                    keeping.push(line);
                }
            },
            None => { break }
        }
    }

    return keeping;
}


fn tokenize(lines: &Vec<&str>) -> Vec<Vec<String>> {

    let mut tokens = Vec::new();

    for line in lines {
        let mut words = line.split_whitespace();
        let mut line_tokens = Vec::new();

        loop {
            match words.next() {
                Some(word) => {
                    if word.contains("'") {
                        
                        let mut s = String::new();
                        s.push_str(word);
                        s.push_str(" ");

                        loop {
                            let mut w = words.next().unwrap(); 
                            if w.contains("'") {
                                s.push_str(w);
                                break
                            } else {
                                s.push_str(w);
                                s.push_str(" ");
                            }
                        }

                        line_tokens.push(s);
                    } else if word == "shh" {
                        break
                    } else {
                        line_tokens.push(word.to_string());
                    }
                },
                None => break
            }
        }

        if line_tokens.len() > 0 {
            tokens.push(line_tokens);
        }
    }
    return tokens;
}

//TODO: Switch insert 0 to push, and then don't do the reverse

//Translate into IR stack
fn gen_ast2(mut token_lines: Vec<Vec<String>>) -> Vec<String> {

    let mut ast: Vec<String> = Vec::<String>::new();

    let mut temp: Vec<String> = Vec::<String>::new();

    let mut function_count = 0;

    loop {
        
        if token_lines.len() == 0 {
            break
        }

        let mut token_line = token_lines.pop().unwrap();

        loop {

            if token_line.len() == 0 {
                break
            }

            let token: &str = &token_line.pop().unwrap();

            match token {
                "very" => {
                    ast.push("ASSIGN".to_string());

                    loop {
                        if temp.len() == 0 {
                            break
                        }
                        ast.push(temp.pop().unwrap());
                    }
                },
                "wow" => {

                    let mut function_end: String = "FUNCTION_END ".to_string();

                    function_end.push_str(&function_count.to_string());
                    
                    ast.push(function_end);
                },
                "plz" => {
                    temp.push("CALL".to_string());

                    loop {
                        if temp.len() == 0 {
                            break
                        }
                        ast.push(temp.pop().unwrap());
                    }
                    
                },
                "much" => {

                    let num_args = temp.len();

                    let mut function_start: String = "FUNCTION ".to_string();

                    function_start.push_str(&function_count.to_string());

                    ast.push(function_start);
                    
                    ast.push(num_args.to_string());
                    
                    loop {
                       if temp.len() == 0 {
                           break
                       }
                        ast.push(temp.pop().unwrap());
                    }

                    let mut function_pointer: String = "FUNC ".to_string();

                    function_pointer.push_str(&function_count.to_string());

                    temp.push(function_pointer);
                    
                },
                _ => {
                    temp.push(token.to_string());
                }
            }
            
        }
    }

    return ast;
    
}

fn interpret(mut stack: Vec<String>) {

    let mut stack_pointer: usize = stack.len() - 1;
    let mut variable_context = HashMap::<String, String>::new();
    let mut function_context = Vec::new();

    let mut dont_call = false;
    
    loop {

        let token: &str = &stack[stack_pointer].clone();

        //println!("{} {} {}", token, stack_pointer, stack.len());
        
        match token {
            "ASSIGN" => {
                let variable = stack.pop().unwrap();
                let variable_name = stack.pop().unwrap();                

                //pop ASSIGN off stack
                stack.pop();

                variable_context.insert(variable_name, variable);
            },
            "FUNCTION_END" => {
                dont_call = true;
            },
            "FUNCTION" => {
                let function_end = stack.pop().unwrap();

                if function_end != "FUNCTION_END".to_string() {
                    panic!("function delimiter not found");
                }

                let mut function: Vec<String> = Vec::<String>::new();

                loop {
                    if stack.len() == stack_pointer {
                        function.pop();
                        break
                    } else {
                        function.push(stack.pop().unwrap().to_string());
                    }
                }

                let mut function_pointer: String = String::new();
                function_pointer.push_str("FUNC ");
                function_pointer.push_str(&function_context.len().to_string());
                
                function_context.push(function);

                stack.push(function_pointer);

                dont_call = false;
            },
            "CALL" => {
                if !dont_call {
                    let function_identifier = &stack[stack_pointer + 1].clone();

                    let function_pointer = match function_identifier.contains("FUNC") {
                        true => {
                            function_identifier
                        },
                        false => {
                            match variable_context.get(function_identifier) {
                                Some(func) => {
                                    func
                                },
                                None => {
                                    "NONE"
                                }
                            }
                        }
                    };

                    if function_pointer != "NONE" {
                        let (_, index) = function_pointer.split_at(5);
                        let pointer: usize = index.parse().unwrap();

                        let mut function_body = function_context[pointer].clone();

                        let num_args: i32 = function_body.pop().unwrap().parse().unwrap();

                        let mut function_scope = HashMap::<String, String>::new();
                        
                        for _i in 0..num_args {
                            let param_name: String = function_body.pop().unwrap();
                            let param: String = stack.pop().unwrap();

                            function_scope.insert(param_name, param);
                        }

                        //replace param names with param values
                        let mut i: usize = 0;
                        loop {

                            if i == function_body.len() {
                                break
                            }
                            
                            let value = function_scope.get(&function_body[i]);

                            match value {
                                Some(v) => {
                                    function_body[i] = v.to_string();
                                },
                                None => {
                                    ()
                                }
                            }
                            
                            i = i + 1;
                        }

                        stack.pop();//pop off remaining function identifier
                        stack.pop();//pop off CALL token

                        // for op in function_body.clone() {
                        //     println!("{}", op);
                        // }

                        // println!(" ");

                        //TODO: Fix executing backwards part
                        //add function body to stack
                        for _i in 0..function_body.len() {
                            stack.push(function_body.pop().unwrap());
                        }

                        // for op in stack.clone() {
                        //     println!("{}", op);
                        // }

                        //stack_pointer = stack.len() - 1;

                    } else {
                        let func: &str = function_identifier;
                        match func {
                            "bark" => {
                                println!("{}", stack.pop().unwrap());
                            },
                            _ => {
                                panic!("function identifier: {} not found in scope", function_identifier); 
                            }
                        }
                        stack.pop();//pop off remaining function identifier
                        stack.pop();//pop off CALL token                        
                    }
                }
            },
            _ => {
                //Should be doing assignment here
                ()
            }
        };

        if stack_pointer > 0 {
            stack_pointer = stack_pointer - 1;
        } else {
            break
        }
        
    }
}

fn main() {
    let filename = "testwow.mdg";

    let mut f = File::open(filename).expect("file not found");
    
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let code_lines = preprocess(&contents);
    let token_lines = tokenize(&code_lines);
    let ast = gen_ast2(token_lines);
    
    //let mut token_lines_iter = token_lines.iter();
    //let ast = gen_ast(&mut token_lines_iter);

    for op in ast {
        println!("{}", op);
    }

    //Reverse the AST for stack operations
    //for code in ast {
    //    let token: String = code.to_string();
    //    println!("{}", code);
        //stack.push(token);
    //}

    
    //interpret(stack);
}
