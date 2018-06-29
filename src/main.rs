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
                                    ast.insert(0, "DONT_EVAL".to_string());
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

//Runs through ast and returns stack
fn interpret(code_ast: &Vec<String>) -> Vec<String> {
    let mut stack = Vec::<String>::new();
    let mut context = HashMap::new();

    let mut eval = true;

    for c in code_ast {
        let opcode: &str = c;
        
        if opcode == "FUNCTION" {
            eval = true;
        }
        
        if eval {
            match opcode {
                "ASSIGN" => {
                    let variable_name = stack.pop();
                    let value = stack.pop();
                    context.insert(variable_name, value);
                },
                "DONT_EVAL" => {
                    eval = false;
                    stack.push("FUNCTION_END".to_string());                   
                },
                "CALL" => {
                    let function_name = stack.pop();

                    if context.contains_key(&function_name) {
                        let function_body = context.get(&function_name).unwrap();
                        println!("{:?}", function_body);                        
                    } else {
                        panic!("Function not found {}", function_name.unwrap());
                    }
                },
                "FUNCTION" => {
                    let num_args_str = stack.pop().unwrap();
                    let num_args: i32 = num_args_str.parse().unwrap();

                    let mut fun = String::new();

                    for _i in 0..num_args {
                        let arg: String = stack.pop().unwrap();
                        fun.push_str(&arg);
                        fun.push_str(" ");
                    }

                    fun.push_str("| ");

                    loop {
                        if stack.len() > 0 {
                            let val: &str = &stack.pop().unwrap();
                            match val {
                                "FUNCTION" => {
                                    fun.push_str(val);
                                },
                                "FUNCTION_END" => {                                                              },
                                _ => {
                                    fun.push_str(val);
                                }                           
                            }
                            fun.push_str(" ");
                        } else {
                            break
                        }
                    }

                    stack.push(fun);                                        
                },
                _ => {
                    stack.push(opcode.to_string());
                }
            }
        } else {
            stack.push(opcode.to_string());
        }
    }
    
    return stack;
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
    let stack = interpret(&ast);

    for value in stack {
        println!("{}", value);
    }
}
