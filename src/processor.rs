use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub enum Snack {
    INT(i64),
    UINT(u64),
    FLOAT(f64),
    STRING(String),
    BOOLEAN(bool)
}

impl fmt::Display for Snack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Snack::STRING(s) => {
                write!(f, "{}", s)
            },
            Snack::INT(i) => {
                write!(f, "{}", i)
            },
            Snack::UINT(u) => {
                write!(f, "{}", u)
            },
            Snack::FLOAT(n) => {
                write!(f, "{}", n) 
            },
            Snack::BOOLEAN(n) => {
                if *n {
                   write!(f, "1=1")
                } else {
                    write!(f, "1=2")
                }
            },
        }
    }
}

pub fn process_comments(lines: Vec<String>) -> Vec<String> {

    let mut keeping = Vec::<String>::new();
    let mut lines_iter = lines.iter();

    loop {
        match lines_iter.next() {
            Some(line) => {
                if line == "quiet" {
                    loop {
                        let l = lines_iter.next();
                        if l == Some(&"loud".to_string()) {
                            break
                        }
                    }
                } else {
                    let mut i = 0;                        
                    let tokens: Vec<&str> = line.split(" ").collect();
                    loop {
                        if i == tokens.len() { break }
                        if tokens[i] == "shh" {
                            break
                        }
                        i = i + 1;
                    }

                    keeping.push(tokens[0..i].join(" "));
                }
            },
            None => { break }
        }
    }

    return keeping;
}

fn local_scope_helper(mut lines: Vec<String>, line_number: usize, mut function_count: usize) -> (Vec<String>, usize) {
    let mut i: usize = line_number.clone();
    let mut new_lines: Vec<String> = Vec::<String>::new();        
    let mut args = HashMap::<String, String>::new();
    let original_function_count = function_count;
    let mut function_end = false;

    loop {
        
        if i == lines.len() {
            panic!("Missing delimiter!");
        }

        if lines[i].contains("much") {

            if original_function_count == function_count {
                let l: String = lines[i].clone();

                let split_line: Vec<&str> = l.split("much ").collect();

                if split_line.len() > 1 {
                    let split_args: Vec<&str> = split_line[1].split(" ").collect();

                    let mut k = 0;
                    for arg in split_args {
                        let new_arg_name: String = format!("{}_{}", original_function_count.to_string(), k.to_string());
                        if arg == "much" {
                            panic!("much is not allowed as an argument name!");
                        }
                        args.insert(arg.to_string(), new_arg_name);
                        k = k + 1;
                    }                        
                }
                
                function_count = function_count + 1;
                
            } else {
                let (returned_lines, new_function_count) = local_scope_helper(lines.clone(), i, function_count);

                for j in 0..returned_lines.len() {
                    let new_line: &str = &returned_lines[j];
                    lines[i+j] = new_line.to_string();
                }

                function_count = new_function_count;
            }

            let func_start: String = format!("FUNC_START_{}", original_function_count.to_string());

            lines[i] = lines[i].replace("much", &func_start);                
            
        }

        if lines[i].contains("wow") {
            let l: String = lines[i].clone();
            let split_line: Vec<&str> = l.split(" ").collect();
            if split_line.contains(&"wow") {
                lines[i] = lines[i].replace("wow", &format!("FUNC_END_{}", original_function_count.to_string()));
                function_end = true;                
            }
        }

        let mut split_line: Vec<&str> = lines[i].split(" ").collect();

        let mut t = 0;
        loop {
            if t == split_line.len() { break }

            for k in args.keys() {
                if split_line[t] == k {
                    split_line[t] = args.get(k).unwrap();;
                }
            }
            
            t = t + 1;
        }

        new_lines.push(split_line.join(" "));

        if function_end {
            break
        }

        i = i + 1;
    }

    return (new_lines, function_count);
}

pub fn process_local_scope(mut lines: Vec<String>) -> Vec<String> {
    let mut i = 0;
    let mut function_count = 0;

    loop {
        if i == lines.len() {
            break
        }

        if lines[i].contains("much") {
            let (new_lines, new_function_count) = local_scope_helper(lines.clone(), i, function_count);

            let mut j = 0;
            loop {

                if j == new_lines.len() {
                    break
                }

                let new_line: &str = &new_lines[j];
                
                lines[i+j] = new_line.to_string();

                j = j + 1;
            }

            function_count = new_function_count;

            i = i + new_lines.len();
        } else {
            i = i + 1;
        }            
    }

    return lines;
}

pub fn process_global_scope(mut lines: Vec<String>) -> Vec<String> {
    let mut i = 0;

    let mut variable_count = 0;
    let mut global_variable_scope: HashMap<String, String> = HashMap::<String, String>::new();

    loop {
        if i == lines.len() {
            break
        }

        if lines[i].contains("very") {
            let l: String = lines[i].clone();

            let split_line: Vec<&str> = l.split("very ").collect();

            let split_line2: Vec<&str> = split_line[1].split(" ").collect();

            let variable_name: String = split_line2[0].to_string();

            global_variable_scope.insert(variable_name, format!("GLOBAL_{}", variable_count.to_string()));

            variable_count = variable_count + 1;
        }

        let l: String = lines[i].clone();
        let mut split_line: Vec<&str> = l.split(" ").collect();

        let mut t = 0;
        loop {
            if t == split_line.len() { break }

            for k in global_variable_scope.keys() {
                if split_line[t] == k {
                    split_line[t] = global_variable_scope.get(k).unwrap();;
                }
            }
            
            t = t + 1;
        }

        lines[i] = split_line.join(" ");
        
        i = i + 1;
    }

    return lines;
}

pub fn process_strings(mut lines: Vec<String>) -> (Vec<String>, HashMap<String, Snack>) {
    let mut i = 0;

    let mut string_count = 0;

    let mut string_heap: HashMap<String, Snack> = HashMap::<String, Snack>::new();

    loop {

        if i == lines.len() {
            break
        }

        if lines[i].contains("\"") {

            let chars: Vec<char> = lines[i].chars().collect();

            let mut mechadoge_str: Vec<char> = Vec::<char>::new();

            let mut in_string: bool = false;

            let mut j = 0;
            loop {
                
                if j == chars.len() {
                    if in_string {
                        panic!("line {} is missing string delimiter", i);
                    }
                    break
                }

                if chars[j] == '"' {
                    if !in_string {
                        in_string = true;
                    } else {
                        //in_string = false;
                        mechadoge_str.push('"');
                        break
                    }
                }

                if in_string {
                    mechadoge_str.push(chars[j]);
                }
                

                j = j + 1;
            }

            let mut new_str: String = mechadoge_str.into_iter().collect();

            let string_pointer: String = format!("STR_{}", string_count);

            lines[i] = lines[i].replace(&new_str, &string_pointer);

            new_str.retain(|c| c != '"');

            string_heap.insert(string_pointer, Snack::STRING(new_str));

            string_count = string_count + 1;
        } else {
            i = i + 1
        }
        
    }

    return (lines, string_heap);
}

pub fn stackify(tokens: Vec<Snack>) -> Vec<Snack> {
    let mut new_tokens: Vec<Snack> = Vec::<Snack>::new();
    let mut stack: Vec<Snack> = Vec::<Snack>::new();
    let mut i: usize = tokens.len() - 1;

    loop {

        match &tokens[i] {
            Snack::STRING(s) => {
                let t: &str = &s.clone();

                match t {
                    "very" => {
                        stack.push(tokens[i].clone());//push "very"
                        loop {
                            if stack.len() == 0 { break }
                            new_tokens.push(stack.pop().unwrap());
                        }
                    },
                    "rly" => {
                        stack.push(tokens[i].clone());//push "plz"
                        loop {
                            if stack.len() == 0 { break }
                            new_tokens.push(stack.pop().unwrap());
                        }                        
                    },
                    "many" => {
                        stack.push(tokens[i].clone());//push "plz"
                        loop {
                            if stack.len() == 0 { break }
                            new_tokens.push(stack.pop().unwrap());
                        }                        
                    },                    
                    "plz" => {
                        stack.push(tokens[i].clone());//push "plz"
                        loop {
                            if stack.len() == 0 { break }
                            new_tokens.push(stack.pop().unwrap());
                        }                        
                    },
                    _ => { stack.push(tokens[i].clone()) }
                }
                
            },
            _ => {
                stack.push(tokens[i].clone());
            }            
        }

        if i == 0 { break }
        
        i = i - 1;
    }

    for s in stack {
        new_tokens.push(s);
    }

    new_tokens.reverse();
    
    return new_tokens;
}

#[derive(Clone, Debug)]
pub struct Function {
    pub num_args: usize,
    pub parameter_names: Vec<String>,
    pub body: Vec<Snack>
}


fn function_helper(lines: Vec<String>, line_number: usize) -> (Vec<String>, HashMap<String, Function>) {
    let mut i = line_number;

    let mut function_heap: HashMap<String, Function> = HashMap::<String, Function>::new();
    let mut new_lines: Vec<String> = Vec::<String>::new();

    let mut function_body: Function = Function { num_args: 0, parameter_names: Vec::<String>::new(), body: Vec::<Snack>::new() };
    let mut func_pointer: &str = "";
    let mut start_number: &str = "";

    loop {
        
        if i == lines.len() {
            panic!("Missing delimiter!");
        }

        if lines[i].contains("FUNC_START") && func_pointer != "" {
            let (newest_lines, newest_heap) = function_helper(lines.clone(), i);
            function_body.body.push(snackify(newest_lines[0].clone()));

            let len: usize = newest_lines.len();

            for j in 1..newest_lines.len() { new_lines.push(newest_lines[j].clone()) }

            new_lines.push("".to_string());

            function_heap.extend(newest_heap);

            i = i + len;

        } else if lines[i].contains("FUNC_START") {

            let split_line: Vec<&str> = lines[i].split(" ").filter(|t| t != &"" && t != &" ").collect();

            let mut j = 0;
            loop {
                if j == split_line.len() {
                    panic!("FUNC_START not found, something terrible has happened!");
                }

                if split_line[j].contains("FUNC_START") {
                    func_pointer = split_line[j];
                    j = j + 1;
                    break
                }

                j = j + 1;
            }

            let split_pointer: Vec<&str> = func_pointer.split("_").collect();

            start_number = split_pointer[2];

            let args: Vec<&str> = split_line[j..split_line.len()].to_vec();

            function_body.num_args = args.len();

            for arg in args.iter().rev() { function_body.parameter_names.push(arg.to_string()) }

            let mut new_line: String = String::new();

            for k in 0..j {
                new_line.push_str(split_line[k]);
                new_line.push_str(" ");
            }

            new_lines.push(new_line.trim().to_string());
            
            i = i + 1;
            
        } else if lines[i].contains("FUNC_END") {
            let line2: String = lines[i].clone();
            let split_line2: Vec<&str> = line2.split("_").collect();

            let mut end_number: &str = split_line2[2];

            if split_line2[2].contains(" ") {
                let split_space: Vec<&str> = split_line2[2].split(" ").collect();
                let new_line: String = split_space[1..split_space.len()].join(" ");
                new_lines.push(new_line);
                end_number = split_space[0];
            } else {
                new_lines.push("".to_string());
            }           

            function_heap.insert(func_pointer.to_string(), function_body.clone());

            if end_number == start_number {
                break
            }
            
        } else {
            let tokens: Vec<&str> = lines[i].split(" ").filter(|t| t != &"" && t != &" ").collect();

            for t in tokens { function_body.body.push(snackify(t.to_string())) }

            new_lines.push("".to_string());
            i = i + 1;
        }            
    }

    return (new_lines, function_heap);
}

pub fn process_functions(mut lines: Vec<String>) -> (Vec<String>, HashMap<String, Function>) {
    let mut i = 0;
    let mut function_heap: HashMap<String, Function> = HashMap::new();

    loop {
        if i == lines.len() {
            break
        }

        if lines[i].contains("FUNC_START") {
            let (new_lines, new_heap) = function_helper(lines.clone(), i);

            let len = new_lines.len();

            for j in 0..new_lines.len() {
                lines[i+j] = new_lines[j].clone();
            }

            function_heap.extend(new_heap);

            i = i + len;
            
        } else {
            i = i + 1;         
        }            
        
    }

    let final_lines: Vec<String> = lines.iter().filter(|l| l != &"").map(|l| l.to_string()).collect();

    return (final_lines, function_heap);
}

fn array_helper(tokens: Vec<Snack>, token_number: usize, mut array_count: usize) -> (Vec<Snack>, HashMap<String, Vec<Snack>>, usize) {
    let mut new_tokens: Vec<Snack> = Vec::<Snack>::new();
    let mut new_heap: HashMap<String, Vec<Snack>> = HashMap::new();
    let mut array_body: Vec<Snack> = Vec::<Snack>::new();

    let mut i = token_number;
    let mut array_pointer: String = "".to_string();

    loop {

        if i == tokens.len() {
            panic!("Missing array delimiter!");
        }

        let token: Snack = tokens[i].clone();

        match token {
            Snack::STRING(s) => {
                if s == "long" && array_pointer != "" {
                    let (newest_tokens, newest_heap, new_array_count) = array_helper(tokens.clone(), i, array_count);

                    array_body.push(newest_tokens[0].clone());

                    new_heap.extend(newest_heap.clone());

                    array_count = new_array_count;

                    let mut num_skip_tokens: usize = 0;

                    for k in newest_heap.keys() {
                        let len: usize = newest_heap.get(k).unwrap().len();

                        num_skip_tokens = num_skip_tokens + (len + 2);
                    }

                    for _ in 0..num_skip_tokens { new_tokens.push(Snack::STRING("".to_string())) }

                    i = i + num_skip_tokens;
                    
                } else if s == "long" {
                    array_pointer = format!("ARR_START_{}", array_count);
                    new_tokens.push(Snack::STRING(array_pointer.to_string()));
                    array_count = array_count + 1;
                    i = i + 1;
                } else if s == "boi" {
                    new_heap.insert(array_pointer, array_body);
                    new_tokens.push(Snack::STRING("".to_string()));  
                    break
                } else {
                    array_body.push(tokens[i].clone());
                    new_tokens.push(Snack::STRING("".to_string()));
                    i = i + 1;
                }            
                
            },
            _ => {
                array_body.push(tokens[i].clone());
                new_tokens.push(Snack::STRING("".to_string()));
                i = i + 1;
            }
        }
    }

    return (new_tokens, new_heap, array_count);
}

pub fn process_arrays(mut tokens: Vec<Snack>) -> (Vec<Snack>, HashMap<String, Vec<Snack>>) {
    let mut i: usize = 0;
    let mut array_heap: HashMap<String, Vec<Snack>> = HashMap::<String, Vec<Snack>>::new();
    let mut array_count: usize = 0;

    loop {

        if i == tokens.len() {
            break
        }

        let token: Snack = tokens[i].clone();

        match token {
            Snack::STRING(s) => {
                if s == "long" {
                    let (new_tokens, new_heap, new_array_count) = array_helper(tokens.clone(), i, array_count);

                    let len: usize = new_tokens.len();

                    for j in 0..len {
                        tokens[i+j] = new_tokens[j].clone();
                    }

                    array_heap.extend(new_heap);

                    array_count = new_array_count;

                    i = i + len;
                } else {
                    i = i + 1;
                }
                
            },
            _ => {
                i = i + 1;
            }
        }

    }

    let final_tokens: Vec<Snack> = tokens.iter().filter(|t| match t {
        Snack::STRING(s) => {
            s != &"" && s != &" "
        },
        _ => {
            true
        }
    }).map(|snack| snack.clone()).collect();

    return (final_tokens, array_heap);
}

pub fn snackify(token: String) -> Snack {    

    if token.parse::<u64>().is_ok() {
        return Snack::UINT(token.parse().unwrap());
    }

    if token.parse::<i64>().is_ok() {
        return Snack::INT(token.parse().unwrap());
    }

    if token.parse::<f64>().is_ok() {
        return Snack::FLOAT(token.parse().unwrap());
    }

    if token.contains("=") {
        let split_tokens: Vec<&str> = token.split("=").collect();

        assert_eq!(split_tokens.len(), 2, "{} is not correct Ex: `1=1`", token);

        if split_tokens[0] == split_tokens[1] {
            return Snack::BOOLEAN(true);
        } else {
            return Snack::BOOLEAN(false);
        }
    }
    
    return Snack::STRING(token);
}

pub struct Context {
    pub string_heap: HashMap<String, Snack>,
    pub function_heap: HashMap<String, Function>,
    pub array_heap: HashMap<String, Vec<Snack>>,
}

pub fn preprocess_code(lines: Vec<String>) -> (Vec<Snack>, Context) {
    let processed_comments: Vec<String> = process_comments(lines);
    let (processed_strings, string_heap) = process_strings(processed_comments);
    let processed_local_scope: Vec<String> = process_local_scope(processed_strings);
    let processed_global_scope: Vec<String> = process_global_scope(processed_local_scope);
    let (processed_functions, function_heap) = process_functions(processed_global_scope);

    let mut tokens: Vec<String> = Vec::<String>::new();

    for l in processed_functions {
        let line_tokens: Vec<String> = l.split(" ").map(|t| t.trim_matches('\n').to_string()).collect();
        for t in line_tokens {
            tokens.push(t);
        }
    }
    
    let (processed_arrays, array_heap) = process_arrays(tokens.iter().map(|t| snackify(t.to_string())).collect());

    let context = Context {
        string_heap: string_heap,
        function_heap: function_heap,
        array_heap: array_heap
    };

    return (stackify(processed_arrays), context);
}

#[cfg(test)]
mod processor_tests {

    use std::fs::File;
    use std::io::prelude::*;
    use std::collections::HashMap;
    use super::*;

    fn read_to_lines(filename: &str) -> Vec<String> {
        let mut f = File::open(filename).expect("file not found");
        let mut contents = String::new();

        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        let mut lines = Vec::<String>::new();

        contents.lines().for_each( |line| lines.push(line.to_string()));

        return lines;
    }

    fn read_to_tokens(filename: &str) -> Vec<String> {
        let mut f = File::open(filename).expect("file not found");
        let mut contents = String::new();

        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        let mut tokens = Vec::<String>::new();
        
        contents.lines().for_each( |line| line.split(" ").for_each( |token| tokens.push(token.to_string())));
        return tokens;
    }    
    
    #[test]
    fn test_comment_processor() {
        let input_lines = read_to_lines("data/comment_test_input.mdg");
        let output_lines = read_to_lines("data/comment_test_output.mdg");
        let output = process_comments(input_lines);

        for i in 0..output.len() {
            assert_eq!(output[i], output_lines[i]);
        }        
    }

    #[test]
    fn test_local_scope_processor() {
        let input_lines = read_to_lines("data/local_scope_test_input.mdg");
        let output_lines = read_to_lines("data/local_scope_test_output.mdg");
        let output = process_local_scope(input_lines);

        for i in 0..output.len() {
            assert_eq!(output[i], output_lines[i]);
        }
    }

    #[test]
    fn test_global_scope_processor() {
        let input_lines = read_to_lines("data/global_scope_test_input.mdg");
        let output_lines = read_to_lines("data/global_scope_test_output.mdg");
        let output = process_global_scope(input_lines);

        for i in 0..output.len() {
            assert_eq!(output[i], output_lines[i]);
        }
    }

    #[test]
    fn test_string_processor() {
        let input_lines = read_to_lines("data/string_test_input.mdg");
        let output_lines = read_to_lines("data/string_test_output.mdg");
        let (output, string_heap) = process_strings(input_lines);

        assert_eq!("all your base belong to us", string_heap.get("STR_0").unwrap().to_string());
        assert_eq!("- mechadoge", string_heap.get("STR_1").unwrap().to_string());

        for i in 0..output.len() {
            assert_eq!(output[i], output_lines[i]);
        }
    }

    #[test]
    fn test_function_processor() {
        let input_lines = read_to_lines("data/function_test_input.mdg");
        let output_lines = read_to_lines("data/function_test_output.mdg");
        
        let (output, output_heap): (Vec<String>, HashMap<String, Function>) = process_functions(input_lines);

        let function_body_lines = read_to_lines("data/function_body_test.txt");

        let func: &Function = output_heap.get("FUNC_START_0").unwrap();
        assert_eq!(func.num_args, 1);
        assert_eq!(func.parameter_names[0], "0_0");

        assert_eq!(func.body.len(), function_body_lines.len());

        for i in 0..func.body.len() {
            assert_eq!(func.body[i].to_string(), function_body_lines[i]);
        }

        assert_eq!(output.len(), output_lines.len());

        for i in 0..output.len() {
            assert_eq!(output[i], output_lines[i]);
        }
    }

    #[test]
    fn test_array_processor() {
        let mut input_tokens = read_to_tokens("data/array_test_input.mdg");
        let output_tokens = read_to_tokens("data/array_test_output.mdg");

        input_tokens =
            input_tokens
            .iter()
            .filter(|t| t != &"" && t != &" ")
            .map(|t| t.to_string())
            .collect();

        match File::create("data/array_tokens_out.txt") {
            Ok(f) => {
                let mut file = f;
                    match write!(file, "{}", input_tokens.join(" ")) {
                        Err(e) => { panic!("{}", e) },
                        _ => { () }
                    }
            },
            Err(e) => { panic!("{}", e) }
        }        
                    
        let (output, output_heap): (Vec<Snack>, HashMap<String, Vec<Snack>>) = process_arrays(input_tokens.iter().map(|t| snackify(t.to_string())).collect());
        
        let array_body_lines = read_to_lines("data/array_body_test.txt");
        let array_body: Vec<Snack> = output_heap.get("ARR_START_0").unwrap().to_vec();

        assert_eq!(array_body.len(), array_body_lines.len());

        for i in 0..array_body.len() {
            assert_eq!(array_body[i].to_string(), array_body_lines[i]);
        }

        assert_eq!(output.len(), output_tokens.len());

        for i in 0..output.len() {
            assert_eq!(output[i].to_string(), output_tokens[i]);
        }
        
    }

    #[test]
    fn test_preprocessor() {
        let input_lines = read_to_lines("data/preprocessor_test_input.mdg");
        let output_tokens = read_to_lines("data/preprocessor_test_output.txt");
        let (output, _context): (Vec<Snack>, Context) = preprocess_code(input_lines);

        match File::create("data/preprocessor_out.txt") {
            Ok(f) => {
                let mut file = f;
                for t in output.clone() {
                    match write!(file, "{}\n", t) {
                        Err(e) => { panic!("{}", e) },
                        _ => { () }
                    }
                }
            },
            Err(e) => { panic!("{}", e) }
        }
        
        assert_eq!(output.len(), output_tokens.len());

        for i in 0..output.len() {
            assert_eq!(output[i].to_string(), output_tokens[i]);
        }
    }

    #[test]
    fn test_snackify() {
        match snackify("1".to_string()) {
            Snack::UINT(_u) => { () },
            _ => { panic!("not an unsigned int!"); }
        }

        match snackify("-1".to_string()) {
            Snack::INT(_i) => { () },
            _ => { panic!("not an unsigned int!"); }
        }

        match snackify("1.5".to_string()) {
            Snack::FLOAT(_f) => { () },
            _ => { panic!("not an unsigned int!"); }
        }

        match snackify("foo.-4".to_string()) {
            Snack::STRING(_s) => { () },
            _ => { panic!("not an unsigned int!"); }
        }
                
    }
}
