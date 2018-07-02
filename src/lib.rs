mod processor {
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

    pub fn process_local_scope(mut lines: Vec<String>) -> Vec<String> {
        let mut i = 0;

        loop {
            if i == lines.len() {
                break
            }           

            if lines[i].contains("much") {

                let l: String = lines[i].clone();
               
                let split_line: Vec<&str> = l.split("much ").collect();
                let split_args: Vec<&str> = split_line[1].split(" ").collect();
                let args: Vec<&str> = split_args.to_vec();            

                let mut j = i;
                let mut function_start_counter = 0;
                let mut function_end_counter = 0;

                loop {
                    
                    if j == lines.len() {
                        panic!("missing delimiter!");
                    }
                    
                    let mut l2: String = lines[j].clone();

                    if l2.contains("much") {
                        function_start_counter = function_start_counter + 1;
                    }                

                    if l2.contains("wow") {
                        function_end_counter = function_end_counter + 1;
                    }

                    for index in 0..args.len() {
                        if l2.contains(args[index]) {
                            l2 = l2.replace(args[index], &index.to_string());
                            lines[j] = l2.clone();
                        }
                    }

                    j = j + 1;

                    if function_start_counter == function_end_counter {
                        break
                    }                    
 
                }
            }
            
            i = i + 1;
            
        }

        return lines;
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
}
