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
                        s.push_str(word.trim_matches('\''));
                        s.push_str(" ");

                        loop {
                            let mut w = words.next().unwrap(); 
                            if w.contains("'") {
                                s.push_str(w.trim_matches('\''));
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
fn gen_ast(token_lines: &Vec<Vec<String>>) -> Vec<String> {
    let mut ast = Vec::new();

    for i in 0..token_lines.len() {
        let mut token_line = token_lines[i].iter();

        loop {
            match token_line.next() {
                Some(t) => {
                    let token : &str = t;
                    match token {
                        "very" => {
                            ast.push("ASSIGN".to_string());
                            let variable_name = token_line.next().unwrap();
                            ast.push(variable_name.to_string());

                            let next_token = token_line.next().unwrap();
                            if next_token != "is" {
                                panic!("Expecting keyword 'is' found {}", next_token);
                            }
                        },
                        "much" => {
                            ast.push("FUNCTION".to_string());
                            
                            ast.push(token_line.len().to_string());
                        },
                        "plz" => {
                            ast.push("CALL".to_string());
                        },
                        "with" => {
                            ()
                        },
                        "wow" => {
                            ()
                        },
                        _ => {
                            ast.push(token.to_string());
                        }
                    }
                },
                None => break
            }
        }
    }

    return ast;
}

fn main() {
    let filename = "testwow.mdg";

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let code_lines = preprocess(&contents);
    let token_lines = tokenize(&code_lines);
    let ast = gen_ast(&token_lines);

    for node in ast {
        println!("{}", node);
    }
}
