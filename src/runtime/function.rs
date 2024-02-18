use std::collections::HashMap;

use super::data::{get_value_type, DataType, ValueType};
use super::parser::parse_line;
use super::relations::*;
use super::variables::*;

use super::math::*;

pub fn get_function(instruction_set: &[Vec<String>], function_name: &str) -> usize {
    for (i, v) in instruction_set.iter().enumerate() {
        if *v.get(0).unwrap_or(&"".to_string()) == "FUNC"
            && *v.get(1).unwrap_or(&"".to_string()) == function_name
        {
            return i;
        }
    }

    panic!()
}

fn get_block(instruction_set: &[Vec<String>], block_name: &str) -> usize {
    for (i, v) in instruction_set.iter().enumerate() {
        if *v.get(0).unwrap_or(&"".to_string()) == "BLOCK"
            && *v.get(1).unwrap_or(&"".to_string()) == block_name
        {
            return i;
        }
    }

    panic!()
}

pub fn run_function(
    program: &[Vec<String>],
    function_index: usize,
    arguments: Option<HashMap<String, DataType>>,
    global_scope: &mut HashMap<String, DataType>,
) {
    let mut local_scope: HashMap<String, DataType> = match arguments {
        Some(i) => i,
        None => HashMap::new(),
    };
    let mut current_index = function_index + 1;

    while current_index < program.len() {
        let line = &program[current_index];
        let instruction = line[0].as_str();

        match instruction {
            "CREATE_VAR" => create_new_variable(&mut local_scope, &line[1], &line[2]),
            "SET_VAR" => set_var(global_scope, &mut local_scope, &line[1], &line[2]),
            "JUMP" => {
                if should_jump(global_scope, &local_scope, &line[1]) {
                    let mut global_scope_copy = global_scope.clone();

                    for (k, v) in local_scope.clone().into_iter() {
                        global_scope_copy.insert(k, v);
                    }

                    run_function(
                        program,
                        get_block(program, &line[2]),
                        None,
                        &mut global_scope_copy,
                    );

                    for (k, v) in local_scope.iter_mut() {
                        match global_scope_copy.get(k) {
                            Some(i) => *v = i.clone(),
                            None => (),
                        }
                    }

                    for (k, v) in global_scope.iter_mut() {
                        match global_scope_copy.get(k) {
                            Some(i) => *v = i.clone(),
                            None => (),
                        }
                    }
                }
            }
            "ADD" => add_values(global_scope, &mut local_scope, &line[1], &line[2], &line[3]),
            "SUB" => subtract_values(global_scope, &mut local_scope, &line[1], &line[2], &line[3]),
            "MUL" => multiply_values(global_scope, &mut local_scope, &line[1], &line[2], &line[3]),
            "DIV" => divide_values(global_scope, &mut local_scope, &line[1], &line[2], &line[3]),
            "MOD" => modulus_values(global_scope, &mut local_scope, &line[1], &line[2], &line[3]),
            "EQ" => is_equal(global_scope, &mut local_scope, &line[1], &line[2], &line[3]),
            "NE" => is_not_equal(global_scope, &mut local_scope, &line[1], &line[2], &line[3]),
            "LT" => is_less_than(global_scope, &mut local_scope, &line[1], &line[2], &line[3]),
            "LE" => {
                is_less_than_equal(global_scope, &mut local_scope, &line[1], &line[2], &line[3])
            }
            "GT" => is_greater_than(global_scope, &mut local_scope, &line[1], &line[2], &line[3]),
            "GE" => {
                is_greater_than_equal(global_scope, &mut local_scope, &line[1], &line[2], &line[3])
            }
            "RESTART" => {
                if should_jump(global_scope, &local_scope, &line[1]) {
                    current_index = function_index;
                }
            }
            "END" => return,
            i => panic!("Invalid instruction: {}", i),
        }

        current_index += 1;
    }
}

fn should_jump(
    global_scope: &HashMap<String, DataType>,
    local_scope: &HashMap<String, DataType>,
    value: &str,
) -> bool {
    match get_value_type(value) {
        ValueType::Identifier(i) => match get_val(global_scope, local_scope, &i) {
            DataType::Integer(i) => i.unwrap() != 0,
            DataType::Float(i) => i.unwrap() != 0.0,
            DataType::Boolean(i) => i.unwrap(),
            _ => false,
        },
        ValueType::Boolean(i) => i,
        _ => false,
    }
}
