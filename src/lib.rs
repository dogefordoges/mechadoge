mod processor {

    use std::collections::HashMap;
    
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
                        if line.contains("shh") {
                            let split_line: Vec<&str> = line.split(" shh").collect();

                            keeping.push(split_line[0].to_string());
                        } else {
                            keeping.push(line.to_string());
                        }                    
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
                lines[i] = lines[i].replace("wow", &format!("FUNC_END_{}", original_function_count.to_string()));

                function_end = true;
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

    //TODO: optimize by not passing in lines by value
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
                    
                    lines[j] = new_line.to_string();

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

                global_variable_scope.insert(variable_name, variable_count.to_string());

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

    pub fn process_strings(mut lines: Vec<String>) -> (Vec<String>, HashMap<String, String>) {
        let mut i = 0;

        let mut string_count = 0;

        let mut string_heap: HashMap<String, String> = HashMap::<String, String>::new();

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
                            in_string = false;
                            mechadoge_str.push('"');
                            break
                        }
                    }

                    if in_string {
                        mechadoge_str.push(chars[j]);
                    }
                    

                    j = j + 1;
                }

                let new_str: String = mechadoge_str.into_iter().collect();

                lines[i] = lines[i].replace(&new_str, &string_count.to_string());

                string_heap.insert(string_count.to_string(), new_str);

                string_count = string_count + 1;
            } else {
                i = i + 1
            }
           
        }

        return (lines, string_heap);
    }

    fn function_helper(lines: Vec<String>, line_number: usize) -> (Vec<String>, HashMap<String, Vec<String>>) {
        let mut i = line_number;

        let mut function_heap: HashMap<String, Vec<String>> = HashMap::new();
        let mut new_lines: Vec<String> = Vec::<String>::new();

        let mut function_body: Vec<String> = Vec::<String>::new();
        let mut func_pointer: &str = "";
        let mut start_number: &str = "";

        loop {
            
            if i == lines.len() {
                panic!("Missing delimiter!");
            }

            if lines[i].contains("FUNC_START") {

                let split_line: Vec<&str> = lines[i].split(" ").collect();

                func_pointer = split_line[0];

                let split_pointer: Vec<&str> = func_pointer.split("_").collect();

                start_number = split_pointer[2];

                let args: Vec<&str> = split_line[1..split_line.len()].to_vec();

                function_body.push(args.len().to_string());
                for arg in args { function_body.push(arg.to_string()) }
                
                new_lines.push(split_line[0].to_string());
                
            } else if lines[i].contains("FUNC_END") {
                let line2: String = lines[i].clone();
                let split_line2: Vec<&str> = line2.split("_").collect();
                let end_number: &str = split_line2[2];

                function_heap.insert(func_pointer.to_string(), function_body.clone());

                new_lines.push("".to_string());

                if end_number == start_number {
                    break
                }                     
                
            } else {
                let tokens: Vec<&str> = lines[i].split(" ").filter(|t| t != &"" && t != &" ").collect();

                for t in tokens { function_body.push(t.to_string()) }

                new_lines.push("".to_string());
            }            

            i = i + 1;
        }

        return (new_lines, function_heap);
    }

    pub fn process_functions(mut lines: Vec<String>) -> (Vec<String>, HashMap<String, Vec<String>>) {
        let mut i = 0;
        let mut function_heap: HashMap<String, Vec<String>> = HashMap::new();

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

    fn array_helper(tokens: Vec<String>, token_number: usize, mut array_count: usize) -> (Vec<String>, HashMap<String, Vec<String>>, usize) {
        let mut new_tokens: Vec<String> = Vec::<String>::new();
        let mut new_heap: HashMap<String, Vec<String>> = HashMap::new();
        let mut array_body: Vec<String> = Vec::<String>::new();

        let mut i = token_number;
        let mut array_pointer: String = "".to_string();

        loop {

            if i == tokens.len() {
                panic!("Missing array delimiter!");
            }

            if tokens[i] == "long" && array_pointer != "" {
                let (newest_tokens, newest_heap, new_array_count) = array_helper(tokens.clone(), i, array_count);

                array_body.push(newest_tokens[0].clone());

                new_heap.extend(newest_heap.clone());

                array_count = new_array_count;

                let mut num_skip_tokens: usize = 0;

                for k in newest_heap.keys() {
                    let len: usize = newest_heap.get(k).unwrap().len();

                    num_skip_tokens = num_skip_tokens + (len + 2);
                }

                for _ in 0..num_skip_tokens { new_tokens.push("".to_string()) }

                i = i + num_skip_tokens;
                
            } else if tokens[i] == "long" {
                array_pointer = format!("ARR_START_{}", array_count);
                new_tokens.push(array_pointer.to_string());
                array_count = array_count + 1;
                i = i + 1;
            } else if tokens[i] == "boi" {
                new_heap.insert(array_pointer, array_body);
                new_tokens.push("".to_string());                
                break
            } else {
                array_body.push(tokens[i].clone());
                new_tokens.push("".to_string());
                i = i + 1;
            }            
        }

        return (new_tokens, new_heap, array_count);
    }

    pub fn process_arrays(mut tokens: Vec<String>) -> (Vec<String>, HashMap<String, Vec<String>>) {
        let mut i: usize = 0;
        let mut array_heap: HashMap<String, Vec<String>> = HashMap::new();
        let mut array_count: usize = 0;

        loop {

            if i == tokens.len() {
                break
            }

            if tokens[i] == "long" {
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
        }

        let final_tokens: Vec<String> = tokens.iter().filter(|t| t != &"" && t != &" ").map(|t| t.to_string()).collect();

        return (final_tokens, array_heap);
    }
    
}


#[cfg(test)]
mod processor_tests {

    use std::fs::File;
    use std::io::prelude::*;
    use processor::*;
    use std::collections::HashMap;    

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

        contents.split(" ").for_each( |token| tokens.push(token.to_string()));

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
        let (output, _) = process_strings(input_lines);

        for i in 0..output.len() {
            assert_eq!(output[i], output_lines[i]);
        }
    }

    #[test]
    fn test_function_processor() {
        let input_lines = read_to_lines("data/function_test_input.mdg");
        let output_lines = read_to_lines("data/function_test_output.mdg");
        
        let (output, output_heap): (Vec<String>, HashMap<String, Vec<String>>) = process_functions(input_lines);

        let function_body_lines = read_to_lines("data/function_body_test.txt");

        let func_body: Vec<String> = output_heap.get("FUNC_START_0").unwrap().to_vec();

        assert_eq!(func_body.len(), function_body_lines.len());

        for i in 0..func_body.len() {
            assert_eq!(func_body[i], function_body_lines[i]);
        }

        assert_eq!(output.len(), output_lines.len());

        for i in 0..output.len() {
            assert_eq!(output[i], output_lines[i]);
        }
    }

    #[test]
    fn test_array_processor() {
        let input_tokens = read_to_tokens("data/array_test_input.mdg");
        let output_tokens = read_to_tokens("data/array_test_output.mdg");
        let (output, output_heap): (Vec<String>, HashMap<String, Vec<String>>) = process_arrays(input_tokens);

        let array_body_lines = read_to_lines("data/array_body_test.txt");
        let array_body: Vec<String> = output_heap.get("ARR_START_0").unwrap().to_vec();

        assert_eq!(array_body.len(), array_body_lines.len());

        for i in 0..array_body.len() {
            assert_eq!(array_body[i], array_body_lines[i]);
        }

        assert_eq!(output.len(), output_tokens.len());

        for i in 0..output.len() {
            assert_eq!(output[i], output_tokens[i]);
        }
        
    }

}
