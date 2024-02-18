use std::collections::HashMap;

mod constants;

mod data;
use data::DataType;

mod parser;
use parser::parse_line;

mod function;
use function::{get_function, run_function};

mod math;
mod relations;
mod variables;

pub fn run_code(byte_code: &str) {
    let mut global_frame: HashMap<String, DataType> = HashMap::new();
    let program: Vec<Vec<String>> = byte_code
        .split("\n")
        .map(|x| x.trim())
        .filter(|x| x.len() > 0)
        .map(|x| parse_line(x).unwrap())
        .collect();

    let main_function_index = get_function(&program, "main");

    run_function(&program, main_function_index, None, &mut global_frame);
}
