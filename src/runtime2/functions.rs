use std::collections::HashMap;

use super::data::DataType;
use super::variables;

pub fn run_function(
    program: &[Vec<String>],
    function_index: usize,
    arguments: Option<HashMap<String, DataType>>,
    global_scope: &mut HashMap<String, DataType>,
) {
    let mut local_scope = match arguments {
        Some(i) => i,
        None => HashMap::new(),
    };
    let mut current_index = function_index + 1;

    while current_index < program.len() {
        let line = &program[current_index];
        let instruction = line[0].as_str();

        match instruction {
            "CREATE_VAR" => variables::create_var(&mut local_scope, &line[1], &line[2]),
            "SET_VAR" => {
                variables::set_var(global_scope, &mut local_scope, &line[1], &line[2]);
            }
            "EQ" => {
                variables::is_equal(global_scope, &mut local_scope, &line[1], &line[2], &line[3]);
            }
            "NE" => {
                variables::is_not_equal(
                    global_scope,
                    &mut local_scope,
                    &line[1],
                    &line[2],
                    &line[3],
                );
            }
            "LT" => {
                variables::is_less_than(
                    global_scope,
                    &mut local_scope,
                    &line[1],
                    &line[2],
                    &line[3],
                );
            }
            "LE" => {
                variables::is_less_than_equal(
                    global_scope,
                    &mut local_scope,
                    &line[1],
                    &line[2],
                    &line[3],
                );
            }
            "GT" => {
                variables::is_greater_than(
                    global_scope,
                    &mut local_scope,
                    &line[1],
                    &line[2],
                    &line[3],
                );
            }
            "GE" => {
                variables::is_greater_than_equal(
                    global_scope,
                    &mut local_scope,
                    &line[1],
                    &line[2],
                    &line[3],
                );
            }

            "SHOW_ALL" => println!(
                "----------\nLocal scope: {:#?}\n\nGlobal scope: {:#?}\n----------",
                local_scope, global_scope
            ),
            "END" => return,
            _ => panic!(),
        }

        current_index += 1;
    }
}
