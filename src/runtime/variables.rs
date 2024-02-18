use std::collections::HashMap;

use super::data::{get_value_type, DataType, ValueType};

pub fn create_new_variable(scope: &mut HashMap<String, DataType>, type_: &str, name: &str) {
    scope.entry(name.to_string()).or_insert(match type_ {
        "int" => DataType::Integer(None),
        "float" => DataType::Float(None),
        "bool" => DataType::Boolean(None),
        "String" => DataType::String(None),
        "char" => DataType::Character(None),
        _ => unimplemented!(),
    });
}

pub fn set_var(
    global_scope: &mut HashMap<String, DataType>,
    local_scope: &mut HashMap<String, DataType>,
    name: &str,
    value: &str,
) {
    let value_type = get_value_type(value);

    match value_type {
        ValueType::Identifier(ident) => {
            let other_variable = match local_scope.get(&ident) {
                Some(i) => i.clone(),
                None => match global_scope.get(&ident) {
                    Some(i) => i.clone(),
                    None => panic!(),
                },
            };

            let variable_to_change = get_val_mut(global_scope, local_scope, name);

            if !variable_to_change.same_type(&other_variable) {
                panic!("{:?} | {:?}", variable_to_change, other_variable);
            }

            *variable_to_change = other_variable;
        }
        ValueType::Integer(i) => {
            let variable_to_change = get_val_mut(global_scope, local_scope, name);

            if !matches!(variable_to_change, DataType::Integer(_)) {
                panic!();
            }

            *variable_to_change = DataType::Integer(Some(i))
        }
        ValueType::Float(i) => {
            let variable_to_change = get_val_mut(global_scope, local_scope, name);

            if !matches!(variable_to_change, DataType::Float(_)) {
                panic!();
            }

            *variable_to_change = DataType::Float(Some(i))
        }
        ValueType::Boolean(i) => {
            let variable_to_change = get_val_mut(global_scope, local_scope, name);

            if !matches!(variable_to_change, DataType::Boolean(_)) {
                panic!();
            }

            *variable_to_change = DataType::Boolean(Some(i))
        }
        ValueType::Character(i) => {
            let variable_to_change = get_val_mut(global_scope, local_scope, name);

            if !matches!(variable_to_change, DataType::String(_)) {
                panic!();
            }

            *variable_to_change = DataType::Character(Some(i))
        }
        ValueType::String(i) => {
            let variable_to_change = get_val_mut(global_scope, local_scope, name);

            if !matches!(variable_to_change, DataType::String(_)) {
                panic!();
            }

            *variable_to_change = DataType::String(Some(i))
        }
    }
}

pub fn get_val<'a>(
    global_scope: &'a HashMap<String, DataType>,
    local_scope: &'a HashMap<String, DataType>,
    name: &'a str,
) -> &'a DataType {
    match local_scope.get(name) {
        Some(i) => return i,
        None => (),
    }
    match global_scope.get(name) {
        Some(i) => return i,
        None => panic!(),
    }
}

pub fn get_val_mut<'a>(
    global_scope: &'a mut HashMap<String, DataType>,
    local_scope: &'a mut HashMap<String, DataType>,
    name: &'a str,
) -> &'a mut DataType {
    match local_scope.get_mut(name) {
        Some(i) => return i,
        None => (),
    }

    match global_scope.get_mut(name) {
        Some(i) => return i,
        None => panic!(),
    }
}
