use std::collections::HashMap;

use super::{
    data::{get_value_type, DataType, ValueType},
    variables::{get_val, get_val_mut},
};

pub fn add_values(
    global_scope: &mut HashMap<String, DataType>,
    local_scope: &mut HashMap<String, DataType>,
    var1: &str,
    var2: &str,
    assign_to: &str,
) {
    let operand1 = get_value_type(var1);
    let operand2 = get_value_type(var2);

    match (operand1, operand2) {
        (ValueType::Identifier(i), ValueType::Identifier(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();
            let val2 = get_val(global_scope, local_scope, &j).clone();

            match (val1, val2) {
                (DataType::Integer(k), DataType::Integer(l)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Integer(_)) {
                        *operand3 = DataType::Integer(Some(k.unwrap() + l.unwrap()));
                    }
                }
                (DataType::Integer(k), DataType::Float(l)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() as f64 + l.unwrap()));
                    }
                }
                (DataType::Float(k), DataType::Integer(l)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() + l.unwrap() as f64));
                    }
                }
                (DataType::Float(k), DataType::Float(l)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() + l.unwrap()));
                    }
                }
                (DataType::String(k), DataType::String(l)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::String(_)) {
                        *operand3 = DataType::String(Some(k.unwrap() + &l.unwrap()));
                    }
                }
                _ => panic!(),
            }
        }
        (ValueType::Identifier(i), ValueType::Integer(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Integer(_)) {
                        *operand3 = DataType::Integer(Some(k.unwrap() + j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() + j as f64));
                    }
                }
                _ => panic!(),
            }
        }
        (ValueType::Identifier(i), ValueType::Float(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() as f64 + j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() + j));
                    }
                }
                _ => panic!(),
            }
        }
        (ValueType::Identifier(i), ValueType::String(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::String(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::String(_)) {
                        *operand3 = DataType::String(Some(k.unwrap() + &j));
                    }
                }
                _ => panic!(),
            }
        }
        (ValueType::Integer(i), ValueType::Identifier(j)) => {
            let val1 = get_val(global_scope, local_scope, &j).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Integer(_)) {
                        *operand3 = DataType::Integer(Some(k.unwrap() + i));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() + i as f64));
                    }
                }
                _ => panic!(),
            }
        }
        (ValueType::Integer(i), ValueType::Integer(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Integer(_)) {
                *operand3 = DataType::Integer(Some(i + j));
            }
        }
        (ValueType::Integer(i), ValueType::Float(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Float(_)) {
                *operand3 = DataType::Float(Some(i as f64 + j));
            }
        }
        (ValueType::Float(i), ValueType::Identifier(j)) => {
            let val1 = get_val(global_scope, local_scope, &j).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() as f64 + i));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() + i as f64));
                    }
                }
                _ => panic!(),
            }
        }
        (ValueType::Float(i), ValueType::Integer(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Float(_)) {
                *operand3 = DataType::Float(Some(i + j as f64));
            }
        }
        (ValueType::Float(i), ValueType::Float(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Float(_)) {
                *operand3 = DataType::Float(Some(i + j));
            }
        }
        (ValueType::String(i), ValueType::String(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::String(_)) {
                *operand3 = DataType::String(Some(i + &j));
            }
        }
        _ => panic!(),
    }
}

pub fn subtract_values(
    global_scope: &mut HashMap<String, DataType>,
    local_scope: &mut HashMap<String, DataType>,
    var1: &str,
    var2: &str,
    assign_to: &str,
) {
    let operand1 = get_value_type(var1);
    let operand2 = get_value_type(var2);

    match (operand1, operand2) {
        (ValueType::Identifier(i), ValueType::Identifier(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();
            let val2 = get_val(global_scope, local_scope, &j).clone();

            match (val1, val2) {
                (DataType::Integer(k), DataType::Integer(l)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Integer(_)) {
                        *operand3 = DataType::Integer(Some(k.unwrap() - l.unwrap()));
                    }
                }
                (DataType::Integer(k), DataType::Float(l)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() as f64 - l.unwrap()));
                    }
                }
                (DataType::Float(k), DataType::Integer(l)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() - l.unwrap() as f64));
                    }
                }
                (DataType::Float(k), DataType::Float(l)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() - l.unwrap()));
                    }
                }
                _ => panic!(),
            }
        }
        (ValueType::Identifier(i), ValueType::Integer(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Integer(_)) {
                        *operand3 = DataType::Integer(Some(k.unwrap() - j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() - j as f64));
                    }
                }
                _ => panic!(),
            }
        }
        (ValueType::Identifier(i), ValueType::Float(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() as f64 - j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() - j));
                    }
                }
                _ => panic!(),
            }
        }
        (ValueType::Integer(i), ValueType::Identifier(j)) => {
            let val1 = get_val(global_scope, local_scope, &j).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Integer(_)) {
                        *operand3 = DataType::Integer(Some(k.unwrap() - i));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() - i as f64));
                    }
                }
                _ => panic!(),
            }
        }
        (ValueType::Integer(i), ValueType::Integer(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Integer(_)) {
                *operand3 = DataType::Integer(Some(i - j));
            }
        }
        (ValueType::Integer(i), ValueType::Float(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Float(_)) {
                *operand3 = DataType::Float(Some(i as f64 - j));
            }
        }
        (ValueType::Float(i), ValueType::Identifier(j)) => {
            let val1 = get_val(global_scope, local_scope, &j).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() as f64 - i));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() - i as f64));
                    }
                }
                _ => panic!(),
            }
        }
        (ValueType::Float(i), ValueType::Integer(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Float(_)) {
                *operand3 = DataType::Float(Some(i - j as f64));
            }
        }
        (ValueType::Float(i), ValueType::Float(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Float(_)) {
                *operand3 = DataType::Float(Some(i - j));
            }
        }
        _ => panic!(),
    }
}

pub fn multiply_values(
    global_scope: &mut HashMap<String, DataType>,
    local_scope: &mut HashMap<String, DataType>,
    var1: &str,
    var2: &str,
    assign_to: &str,
) {
    let operand1 = get_value_type(var1);
    let operand2 = get_value_type(var2);

    match (operand1, operand2) {
        (ValueType::Identifier(i), ValueType::Identifier(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();
            let val2 = get_val(global_scope, local_scope, &j).clone();

            match (val1, val2) {
                (DataType::Integer(k), DataType::Integer(l)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Integer(_)) {
                        *operand3 = DataType::Integer(Some(k.unwrap() * l.unwrap()));
                    }
                }
                (DataType::Integer(k), DataType::Float(l)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() as f64 * l.unwrap()));
                    }
                }
                (DataType::Float(k), DataType::Integer(l)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() * l.unwrap() as f64));
                    }
                }
                (DataType::Float(k), DataType::Float(l)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() * l.unwrap()));
                    }
                }
                _ => panic!(),
            }
        }
        (ValueType::Identifier(i), ValueType::Integer(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Integer(_)) {
                        *operand3 = DataType::Integer(Some(k.unwrap() * j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() * j as f64));
                    }
                }
                _ => panic!(),
            }
        }
        (ValueType::Identifier(i), ValueType::Float(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() as f64 * j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() * j));
                    }
                }
                _ => panic!(),
            }
        }
        (ValueType::Integer(i), ValueType::Identifier(j)) => {
            let val1 = get_val(global_scope, local_scope, &j).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Integer(_)) {
                        *operand3 = DataType::Integer(Some(k.unwrap() * i));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() * i as f64));
                    }
                }
                _ => panic!(),
            }
        }
        (ValueType::Integer(i), ValueType::Integer(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Integer(_)) {
                *operand3 = DataType::Integer(Some(i * j));
            }
        }
        (ValueType::Integer(i), ValueType::Float(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Float(_)) {
                *operand3 = DataType::Float(Some(i as f64 * j));
            }
        }
        (ValueType::Float(i), ValueType::Identifier(j)) => {
            let val1 = get_val(global_scope, local_scope, &j).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() as f64 * i));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() * i as f64));
                    }
                }
                _ => panic!(),
            }
        }
        (ValueType::Float(i), ValueType::Integer(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Float(_)) {
                *operand3 = DataType::Float(Some(i * j as f64));
            }
        }
        (ValueType::Float(i), ValueType::Float(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Float(_)) {
                *operand3 = DataType::Float(Some(i * j));
            }
        }
        _ => panic!(),
    }
}

pub fn divide_values(
    global_scope: &mut HashMap<String, DataType>,
    local_scope: &mut HashMap<String, DataType>,
    var1: &str,
    var2: &str,
    assign_to: &str,
) {
    let operand1 = get_value_type(var1);
    let operand2 = get_value_type(var2);

    match (operand1, operand2) {
        (ValueType::Identifier(i), ValueType::Identifier(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();
            let val2 = get_val(global_scope, local_scope, &j).clone();

            match (val1, val2) {
                (DataType::Integer(k), DataType::Integer(l)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Integer(_)) {
                        *operand3 = DataType::Integer(Some(k.unwrap() / l.unwrap()));
                    }
                }
                (DataType::Integer(k), DataType::Float(l)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() as f64 / l.unwrap()));
                    }
                }
                (DataType::Float(k), DataType::Integer(l)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() / l.unwrap() as f64));
                    }
                }
                (DataType::Float(k), DataType::Float(l)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() / l.unwrap()));
                    }
                }
                _ => panic!(),
            }
        }
        (ValueType::Identifier(i), ValueType::Integer(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Integer(_)) {
                        *operand3 = DataType::Integer(Some(k.unwrap() / j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() / j as f64));
                    }
                }
                _ => panic!(),
            }
        }
        (ValueType::Identifier(i), ValueType::Float(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() as f64 / j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() / j));
                    }
                }
                _ => panic!(),
            }
        }
        (ValueType::Integer(i), ValueType::Identifier(j)) => {
            let val1 = get_val(global_scope, local_scope, &j).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Integer(_)) {
                        *operand3 = DataType::Integer(Some(k.unwrap() / i));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() / i as f64));
                    }
                }
                _ => panic!(),
            }
        }
        (ValueType::Integer(i), ValueType::Integer(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Integer(_)) {
                *operand3 = DataType::Integer(Some(i / j));
            }
        }
        (ValueType::Integer(i), ValueType::Float(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Float(_)) {
                *operand3 = DataType::Float(Some(i as f64 / j));
            }
        }
        (ValueType::Float(i), ValueType::Identifier(j)) => {
            let val1 = get_val(global_scope, local_scope, &j).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() as f64 / i));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Float(_)) {
                        *operand3 = DataType::Float(Some(k.unwrap() / i as f64));
                    }
                }
                _ => panic!(),
            }
        }
        (ValueType::Float(i), ValueType::Integer(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Float(_)) {
                *operand3 = DataType::Float(Some(i / j as f64));
            }
        }
        (ValueType::Float(i), ValueType::Float(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Float(_)) {
                *operand3 = DataType::Float(Some(i / j));
            }
        }
        _ => panic!(),
    }
}

pub fn modulus_values(
    global_scope: &mut HashMap<String, DataType>,
    local_scope: &mut HashMap<String, DataType>,
    var1: &str,
    var2: &str,
    assign_to: &str,
) {
    let operand1 = get_value_type(var1);
    let operand2 = get_value_type(var2);

    match (operand1, operand2) {
        (ValueType::Identifier(i), ValueType::Identifier(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();
            let val2 = get_val(global_scope, local_scope, &j).clone();

            match (val1, val2) {
                (DataType::Integer(k), DataType::Integer(l)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Integer(_)) {
                        *operand3 = DataType::Integer(Some(k.unwrap() % l.unwrap()));
                    }
                }
                _ => panic!(),
            }
        }
        (ValueType::Identifier(i), ValueType::Integer(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Integer(_)) {
                        *operand3 = DataType::Integer(Some(k.unwrap() % j));
                    }
                }
                _ => panic!(),
            }
        }
        (ValueType::Integer(i), ValueType::Identifier(j)) => {
            let val1 = get_val(global_scope, local_scope, &j).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Integer(_)) {
                        *operand3 = DataType::Integer(Some(k.unwrap() % i));
                    }
                }
                _ => panic!(),
            }
        }
        (ValueType::Integer(i), ValueType::Integer(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Integer(_)) {
                *operand3 = DataType::Integer(Some(i % j));
            }
        }
        _ => panic!(),
    }
}
