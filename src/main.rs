use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn tokenize(lines: &Vec<&str>) -> Vec<String> {

    let mut tokens = Vec::new();

    for line in lines {
        let mut words = line.split_whitespace();

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

                        tokens.push(s);
                    } else if word == "shh" {
                        break
                    } else {
                        tokens.push(word.to_string());
                    }
                },
                None => break
            }
        }
    }
    return tokens;
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


fn main() {
    let filename = "testwow.mdg";

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let code_lines = preprocess(&contents);
    let tokens = tokenize(&code_lines);

    for token in tokens {
        println!("{}", token);
    }
    // let ir = parse(&tokens);
}
