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

    pub fn process_functions(mut lines: Vec<String>) -> (Vec<String>, HashMap<String, Vec<String>>) {
        let mut i = 0;
        let mut function_heap: HashMap<String, Vec<String>> = HashMap::new();

        loop {
            if i == lines.len() {
                break
            }           

            if lines[i].contains("FUNC_START") {

                let l: String = lines[i].clone();              

                let mut j = i;
                let mut function_start_counter = 0;
                let mut function_end_counter = 0;

                loop {
                    
                    if j == lines.len() {
                        panic!("missing delimiter!");
                    }

                    if lines[i].contains("FUNC_START") && function_start_counter == 0 {
                        let args: Vec<&str> = lines[i].split(" ").skip(0).collect();
                        let mut function_body: Vec<String> = Vec::<String>::new();

                        function_body.push(args.len().to_string());

                        for arg in args { function_body.push(arg.to_string()) }

                        function_start_counter = function_start_counter + 1;
                        
                    } else if lines[i].contains("FUNC_START") && function_start_counter == 1 {
                    }
                    
                    if lines[i].contains("FUNC_END") {
                        function_end_counter = function_end_counter + 1;
                    }

                    j = j + 1;

                    if function_start_counter == function_end_counter {
                        break
                    }                    
 
                }
            }
            
            i = i + 1;
            
        }

        return (lines, function_heap);
    }
    
}

#[cfg(test)]
mod processor_tests {

    use std::fs::File;
    use std::io::prelude::*;
    use processor::*;    

    fn read_to_lines(filename: &str) -> Vec<String> {
        let mut f = File::open(filename).expect("file not found");
        let mut contents = String::new();

        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        let mut lines = Vec::<String>::new();

        contents.lines().for_each( |line| lines.push(line.to_string()));

        return lines;
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
        let (output, _) = process_strings(input_lines);

        for i in 0..output.len() {
            assert_eq!(output[i], output_lines[i]);
        }
    }

}
