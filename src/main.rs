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

//TODO: Handle strings with no spaces, single words

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
fn gen_ast(mut token_lines: Vec<Vec<String>>) -> Vec<String> {

    let mut ast: Vec<String> = Vec::<String>::new();

    let mut temp: Vec<String> = Vec::<String>::new();

    let mut function_count: i32 = 1;

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

                    loop {
                        if temp.len() == 0 {
                            break
                        }
                        ast.push(temp.pop().unwrap());
                    }

                    ast.push("ASSIGN".to_string());
                },
                "wow" => {

                    let mut function_end: String = "FUNCTION_END ".to_string();

                    function_end.push_str(&function_count.to_string());
                    
                    ast.push(function_end);                    
                },
                "with" => {
                    loop {
                        if temp.len() == 0 {
                            break
                        }
                        ast.push(temp.pop().unwrap());
                    }
                },
                "plz" => {                    

                    loop {
                        if temp.len() == 0 {
                            break
                        }
                        ast.push(temp.pop().unwrap());
                    }

                    ast.push("CALL".to_string());
                    
                },
                "much" => {

                    let num_args = temp.len();

                    let mut function_start: String = "FUNCTION_START ".to_string();

                    function_start.push_str(&function_count.to_string());
                    
                    loop {
                       if temp.len() == 0 {
                           break
                       }
                        ast.push(temp.pop().unwrap());
                    }

                    ast.push(num_args.to_string());

                    ast.push(function_start);                    

                    let mut function_pointer: String = "FUNCPTR ".to_string();

                    function_pointer.push_str(&function_count.to_string());

                    ast.push(function_pointer);

                    function_count = function_count + 1;
                    
                },
                _ => {
                    temp.insert(0, token.to_string());
                }
            }
            
        }
    }

    return ast;
    
}

fn interpret(mut stack: Vec<String>) {

    let mut stack_pointer = stack.len() - 1;
    let mut variable_context = HashMap::<String, String>::new();
    let mut function_context = HashMap::<String, Vec<String>>::new();
    
    loop {

        let opcode: &str = &stack[stack_pointer].clone();

        //println!("{} {} {}", opcode, stack_pointer, stack.len());

        match opcode {

            "ASSIGN" => {
                stack.pop();//Pop off ASSIGN

                let variable_name: String = stack.pop().unwrap();
                stack_pointer = stack_pointer - 1;
                
                let variable: String = stack.pop().unwrap();
                stack_pointer = stack_pointer - 1;

                variable_context.insert(variable_name, variable);                
            },
            "CALL" => {
                stack.pop();//Pop CALL off of stack

                let function_name: String = stack.pop().unwrap();
                stack_pointer = stack_pointer - 1;

                let mut function_pointer: String = function_name.clone();

                if !function_pointer.contains("FUNCPTR") {
                    match variable_context.get(&function_pointer) {
                        Some(f_ptr) => {
                            function_pointer = f_ptr.to_string();
                        },
                        None => {
                            function_pointer = "NONE".to_string();
                        }
                    }                    
                }

                if function_pointer != "NONE" {

                    //Push function body onto stack
                    
                    let (_, function_id) = function_pointer.split_at(9);

                    match function_context.get(function_id) {
                        Some(body) => {
                            
                            let mut func_body: Vec<String> = body.to_vec().clone();

                            let num_args: i32 = func_body.pop().unwrap().parse().unwrap();
                            let mut function_scope: HashMap<String, String> = HashMap::<String, String>::new();
                            //define scope
                            for _i in 0..num_args {
                                function_scope.insert(func_body.pop().unwrap(), stack.pop().unwrap());
                                stack_pointer = stack_pointer - 1;
                            }

                            //replace param names with param values
                            let mut i: usize = 0;
                            loop {

                                if i == func_body.len() {
                                    break
                                }
                                
                                let value = function_scope.get(&func_body[i]);

                                match value {
                                    Some(v) => {
                                        func_body[i] = v.to_string();
                                    },
                                    None => {
                                        ()
                                    }
                                }
                                
                                i = i + 1;
                            }

                            //Add function body to stack
                            for op in func_body {
                                stack.push(op);
                            }                            

                            stack_pointer = stack.len();
                            
                        },
                        None => {
                            panic!("function {} not found in scope", function_name);
                        }
                    }                    
                } else {
                    let name: &str = &function_name;
                    match name {
                        "bark" => {
                            
                            println!("{}", stack.pop().unwrap());
                            stack_pointer = stack_pointer - 1;

                        }
                        _ => {
                            panic!("{} is not defined", function_name); 
                        }
                    }
                }
                
            },
            _ => {

                if opcode.contains("FUNCTION_START") {

                    let op = stack.pop().unwrap();

                    let (_, ptr) = op.split_at(16);

                    let mut function_body = Vec::<String>::new();

                    loop {
                        let token: String = stack.pop().unwrap();
                        stack_pointer = stack_pointer - 1;

                        if token.contains("FUNCTION_END") {
                            let (_, end_ptr) = token.split_at(14);

                           if ptr == end_ptr {
                                break
                            }
                        }

                        function_body.insert(0, token);
                    }

                    function_context.insert(ptr.to_string(), function_body);

                } else {
                    ()
                }
                
            }
            
        }

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
    let ast = gen_ast(token_lines);    
    interpret(ast);
}
