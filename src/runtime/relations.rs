use std::collections::HashMap;

use super::{
    data::{get_value_type, DataType, ValueType},
    variables::{get_val, get_val_mut},
};

pub fn is_equal(
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
                (DataType::Integer(i), DataType::Integer(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() == j.unwrap()));
                    }
                }
                (DataType::Integer(i), DataType::Float(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() as f64 == j.unwrap()));
                    }
                }
                (DataType::Float(i), DataType::Integer(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() == j.unwrap() as f64));
                    }
                }
                (DataType::Float(i), DataType::Float(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() == j.unwrap()));
                    }
                }
                (DataType::Boolean(i), DataType::Boolean(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() == j.unwrap()));
                    }
                }
                (DataType::String(i), DataType::String(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() == j.unwrap()));
                    }
                }
                (DataType::Character(i), DataType::Character(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() == j.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::Integer(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() == j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(j as f64 == k.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::Float(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() as f64 == j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(j == k.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::Boolean(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Boolean(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() == j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::Character(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Character(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() == j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::String(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::String(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() == j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Integer(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() == j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(j as f64 == k.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Float(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() as f64 == j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(j == k.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Boolean(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Boolean(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() == j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Character(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Character(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() == j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::String(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::String(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() == j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Integer(i), ValueType::Integer(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i == j));
            }
        }
        (ValueType::Integer(i), ValueType::Float(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i as f64 == j));
            }
        }
        (ValueType::Float(i), ValueType::Integer(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i == j as f64));
            }
        }
        (ValueType::Float(i), ValueType::Float(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i == j));
            }
        }
        (ValueType::Boolean(i), ValueType::Boolean(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i == j));
            }
        }
        (ValueType::Character(i), ValueType::Character(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i == j));
            }
        }
        (ValueType::String(i), ValueType::String(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i == j));
            }
        }
        _ => (),
    };
}

pub fn is_less_than(
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
                (DataType::Integer(i), DataType::Integer(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() < j.unwrap()));
                    }
                }
                (DataType::Integer(i), DataType::Float(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some((i.unwrap() as f64) < j.unwrap()));
                    }
                }
                (DataType::Float(i), DataType::Integer(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() < j.unwrap() as f64));
                    }
                }
                (DataType::Float(i), DataType::Float(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() < j.unwrap()));
                    }
                }
                (DataType::Boolean(i), DataType::Boolean(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() < j.unwrap()));
                    }
                }
                (DataType::String(i), DataType::String(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() < j.unwrap()));
                    }
                }
                (DataType::Character(i), DataType::Character(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() < j.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::Integer(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() < j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some((j as f64) < k.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::Float(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some((k.unwrap() as f64) < j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(j < k.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::Boolean(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Boolean(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() < j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::Character(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Character(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() < j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::String(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::String(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() < j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Integer(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() < j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some((j as f64) < k.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Float(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some((k.unwrap() as f64) < j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(j < k.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Boolean(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Boolean(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() < j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Character(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Character(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() < j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::String(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::String(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() < j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Integer(i), ValueType::Integer(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i < j));
            }
        }
        (ValueType::Integer(i), ValueType::Float(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some((i as f64) < j));
            }
        }
        (ValueType::Float(i), ValueType::Integer(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i < j as f64));
            }
        }
        (ValueType::Float(i), ValueType::Float(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i < j));
            }
        }
        (ValueType::Boolean(i), ValueType::Boolean(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i < j));
            }
        }
        (ValueType::Character(i), ValueType::Character(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i < j));
            }
        }
        (ValueType::String(i), ValueType::String(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i < j));
            }
        }
        _ => (),
    };
}

pub fn is_greater_than(
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
                (DataType::Integer(i), DataType::Integer(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() > j.unwrap()));
                    }
                }
                (DataType::Integer(i), DataType::Float(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some((i.unwrap() as f64) > j.unwrap()));
                    }
                }
                (DataType::Float(i), DataType::Integer(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() > j.unwrap() as f64));
                    }
                }
                (DataType::Float(i), DataType::Float(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() > j.unwrap()));
                    }
                }
                (DataType::Boolean(i), DataType::Boolean(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() > j.unwrap()));
                    }
                }
                (DataType::String(i), DataType::String(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() > j.unwrap()));
                    }
                }
                (DataType::Character(i), DataType::Character(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() > j.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::Integer(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() > j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some((j as f64) > k.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::Float(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some((k.unwrap() as f64) > j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(j > k.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::Boolean(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Boolean(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() > j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::Character(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Character(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() > j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::String(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::String(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() > j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Integer(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() > j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some((j as f64) > k.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Float(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some((k.unwrap() as f64) > j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(j > k.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Boolean(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Boolean(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() > j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Character(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Character(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() > j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::String(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::String(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() > j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Integer(i), ValueType::Integer(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i > j));
            }
        }
        (ValueType::Integer(i), ValueType::Float(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some((i as f64) > j));
            }
        }
        (ValueType::Float(i), ValueType::Integer(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i > j as f64));
            }
        }
        (ValueType::Float(i), ValueType::Float(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i > j));
            }
        }
        (ValueType::Boolean(i), ValueType::Boolean(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i > j));
            }
        }
        (ValueType::Character(i), ValueType::Character(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i > j));
            }
        }
        (ValueType::String(i), ValueType::String(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i > j));
            }
        }
        _ => (),
    };
}

pub fn is_not_equal(
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
                (DataType::Integer(i), DataType::Integer(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() != j.unwrap()));
                    }
                }
                (DataType::Integer(i), DataType::Float(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some((i.unwrap() as f64) != j.unwrap()));
                    }
                }
                (DataType::Float(i), DataType::Integer(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() != j.unwrap() as f64));
                    }
                }
                (DataType::Float(i), DataType::Float(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() != j.unwrap()));
                    }
                }
                (DataType::Boolean(i), DataType::Boolean(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() != j.unwrap()));
                    }
                }
                (DataType::String(i), DataType::String(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() != j.unwrap()));
                    }
                }
                (DataType::Character(i), DataType::Character(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() != j.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::Integer(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() != j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some((j as f64) != k.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::Float(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some((k.unwrap() as f64) != j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(j != k.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::Boolean(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Boolean(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() != j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::Character(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Character(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() != j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::String(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::String(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() != j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Integer(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() != j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some((j as f64) != k.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Float(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some((k.unwrap() as f64) != j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(j != k.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Boolean(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Boolean(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() != j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Character(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Character(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() != j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::String(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::String(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() != j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Integer(i), ValueType::Integer(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i != j));
            }
        }
        (ValueType::Integer(i), ValueType::Float(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some((i as f64) != j));
            }
        }
        (ValueType::Float(i), ValueType::Integer(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i != j as f64));
            }
        }
        (ValueType::Float(i), ValueType::Float(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i != j));
            }
        }
        (ValueType::Boolean(i), ValueType::Boolean(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i != j));
            }
        }
        (ValueType::Character(i), ValueType::Character(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i != j));
            }
        }
        (ValueType::String(i), ValueType::String(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i != j));
            }
        }
        _ => (),
    };
}

pub fn is_less_than_equal(
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
                (DataType::Integer(i), DataType::Integer(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() <= j.unwrap()));
                    }
                }
                (DataType::Integer(i), DataType::Float(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some((i.unwrap() as f64) <= j.unwrap()));
                    }
                }
                (DataType::Float(i), DataType::Integer(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() <= j.unwrap() as f64));
                    }
                }
                (DataType::Float(i), DataType::Float(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() <= j.unwrap()));
                    }
                }
                (DataType::Boolean(i), DataType::Boolean(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() <= j.unwrap()));
                    }
                }
                (DataType::String(i), DataType::String(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() <= j.unwrap()));
                    }
                }
                (DataType::Character(i), DataType::Character(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() <= j.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::Integer(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() <= j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some((j as f64) <= k.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::Float(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some((k.unwrap() as f64) <= j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(j <= k.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::Boolean(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Boolean(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() <= j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::Character(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Character(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() <= j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::String(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::String(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() <= j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Integer(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() <= j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some((j as f64) <= k.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Float(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some((k.unwrap() as f64) <= j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(j <= k.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Boolean(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Boolean(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() <= j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Character(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Character(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() <= j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::String(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::String(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() <= j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Integer(i), ValueType::Integer(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i <= j));
            }
        }
        (ValueType::Integer(i), ValueType::Float(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some((i as f64) <= j));
            }
        }
        (ValueType::Float(i), ValueType::Integer(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i <= j as f64));
            }
        }
        (ValueType::Float(i), ValueType::Float(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i <= j));
            }
        }
        (ValueType::Boolean(i), ValueType::Boolean(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i <= j));
            }
        }
        (ValueType::Character(i), ValueType::Character(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i <= j));
            }
        }
        (ValueType::String(i), ValueType::String(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i <= j));
            }
        }
        _ => (),
    };
}

pub fn is_greater_than_equal(
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
                (DataType::Integer(i), DataType::Integer(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() >= j.unwrap()));
                    }
                }
                (DataType::Integer(i), DataType::Float(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some((i.unwrap() as f64) >= j.unwrap()));
                    }
                }
                (DataType::Float(i), DataType::Integer(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() >= j.unwrap() as f64));
                    }
                }
                (DataType::Float(i), DataType::Float(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() >= j.unwrap()));
                    }
                }
                (DataType::Boolean(i), DataType::Boolean(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() >= j.unwrap()));
                    }
                }
                (DataType::String(i), DataType::String(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() >= j.unwrap()));
                    }
                }
                (DataType::Character(i), DataType::Character(j)) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(i.unwrap() >= j.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::Integer(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() >= j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some((j as f64) >= k.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::Float(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some((k.unwrap() as f64) >= j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(j >= k.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::Boolean(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Boolean(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() >= j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::Character(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Character(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() >= j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Identifier(i), ValueType::String(j)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::String(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() >= j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Integer(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() >= j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some((j as f64) >= k.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Float(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Integer(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some((k.unwrap() as f64) >= j));
                    }
                }
                DataType::Float(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(j >= k.unwrap()));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Boolean(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Boolean(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() >= j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Character(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::Character(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() >= j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::String(j), ValueType::Identifier(i)) => {
            let val1 = get_val(global_scope, local_scope, &i).clone();

            match val1 {
                DataType::String(k) => {
                    let operand3 = get_val_mut(global_scope, local_scope, assign_to);

                    if matches!(operand3, DataType::Boolean(_)) {
                        *operand3 = DataType::Boolean(Some(k.unwrap() >= j));
                    }
                }
                _ => (),
            }
        }
        (ValueType::Integer(i), ValueType::Integer(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i >= j));
            }
        }
        (ValueType::Integer(i), ValueType::Float(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some((i as f64) >= j));
            }
        }
        (ValueType::Float(i), ValueType::Integer(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i >= j as f64));
            }
        }
        (ValueType::Float(i), ValueType::Float(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i >= j));
            }
        }
        (ValueType::Boolean(i), ValueType::Boolean(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i >= j));
            }
        }
        (ValueType::Character(i), ValueType::Character(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i >= j));
            }
        }
        (ValueType::String(i), ValueType::String(j)) => {
            let operand3 = get_val_mut(global_scope, local_scope, assign_to);

            if matches!(operand3, DataType::Boolean(_)) {
                *operand3 = DataType::Boolean(Some(i >= j));
            }
        }
        _ => (),
    };
}
