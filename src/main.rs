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
fn gen_ast<'a, I>(token_lines: &mut I) -> Vec<String>
where
    I: Iterator<Item = &'a Vec<String>>
{
    let mut ast = Vec::new();

    loop {
        match token_lines.next() {
            Some(line) => {
                let mut token_line = line.iter();
                let mut line_eval = Vec::new();
                loop {
                    match token_line.next() {
                        Some(t) => {
                            let token : &str = t;
                            match token {
                                "very" => {
                                    line_eval.insert(0, "ASSIGN".to_string());
                                    let variable_name = token_line.next().unwrap();
                                    line_eval.insert(0, variable_name.to_string());                                                                                                                                             
                                },
                                "much" => {
                                    line_eval.insert(0, "FUNCTION".to_string());
                                    
                                    line_eval.insert(0, token_line.len().to_string());

                                    for _j in 0..token_line.len() {
                                        line_eval.insert(0, token_line.next().unwrap().to_string());
                                    }

                                    ast.append(&mut gen_ast(token_lines));
                                },
                                "plz" => {
                                    line_eval.insert(0, "CALL".to_string());
                                },                               
                                "wow" => {
                                    ast.insert(0, "FUNCTION_END".to_string());
                                    return ast;
                                },
                                _ => {
                                    line_eval.insert(0, token.to_string());
                                }
                            }
                        },
                        None => break
                    }
                }
                ast.append(&mut line_eval);
            },
            None => break
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
                    panic!("FUNCTION DELIMITER NOT FOUND!");
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
                    let function_reference = &stack[stack_pointer + 1];

                    println!("{}", function_reference);                    
                }
            },
            _ => {
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
    let mut token_lines_iter = token_lines.iter();    
    let ast = gen_ast(&mut token_lines_iter);

    //Reverse the AST for stack operations
    let mut stack: Vec<String> = Vec::<String>::new();
    for code in ast.iter().rev() {
        let token: String = code.to_string();
        stack.push(token);
    }
    
    interpret(stack);
}
