use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

extern crate regex;
use regex::Regex;

struct Token {
    token_type: String,
    value: String,
}

fn tokenize(contents: &str) -> Vec<Token> {
    let mut tokens = contents.split_whitespace();

    let mut vec = Vec::new();

    loop {
        match tokens.next() {
            Some(token) => {
                if token.contains("'") {
                    
                    let mut s = String::new();
                    s.push_str(token.trim_matches('\''));

                    loop {
                        let mut t = tokens.next().unwrap(); 
                        if t.contains("'") {
                            s.push_str(t.trim_matches('\''));
                            break
                        } else {
                            s.push_str(t);
                        }
                    }
                    
                    vec.push(Token {
                        token_type: "String".to_string(),
                        value: s
                    });
                } else {                 
                    vec.push(Token {
                        token_type: "Atom".to_string(),
                        value: token.to_string()
                    });
                }
            },
            None => { break }
        }
    }

    return vec;
}

//Translate into IR stack
// fn parse(tokens: &Vec<Token>) -> Vec<&str> {
//     let mut tokens_iter = tokens.iter();
//     let mut ast = Vec::new();
//     loop {
//         match tokens_iter.next() {
//             Some(token) => {
//                 let v: &str = token.unwrap().value;
//                 match v {
//                     "very" => {
//                         ast.push("ASSIGN");
//                     },
//                     "is" => {
//                         ()
//                     },
//                     _ => {
//                         ast.push(v);
//                     }
//                 }
//             },
//             None => {
//                 break
//             }
//         }
//     }
//     return Vec::new();
// }

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


fn main() {
    let filename = "testwow.mdg";

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let code_lines = preprocess(&contents);

    for line in code_lines {
        println!("{}", line);
    }


    // let tokens = tokenize(&contents);
    // let ir = parse(&tokens);
}
