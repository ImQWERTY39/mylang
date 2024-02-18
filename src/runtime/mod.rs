use std::collections::HashMap;

mod data;
use data::DataType;

mod functions;
mod variables;

mod parser;

type Scope = HashMap<String, DataType>;

pub fn run_code(byte_code: &str) {
    let mut global_scope: Scope = HashMap::new();
    let program: Vec<Vec<String>> = byte_code
        .split("\n")
        .map(|x| x.trim())
        .filter(|x| x.len() > 0)
        .map(|x| parser::parse_line(x).unwrap())
        .collect();

    let main_function_index = get_main_function_index(&program);
    functions::run_function(&program, main_function_index, None, &mut global_scope);
}

fn get_main_function_index(program: &[Vec<String>]) -> usize {
    for (i, v) in program.iter().enumerate() {
        if v.get(1).unwrap_or(&"".to_string()) == "main" {
            return i;
        }
    }

    panic!();
}
