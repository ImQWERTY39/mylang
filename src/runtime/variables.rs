use super::{data::DataType, Scope};

pub fn create_var(local_scope: &mut Scope, type_: &str, name: &str) {
    let data = match type_ {
        "int" => DataType::Integer(None),
        "bool" => DataType::Boolean(None),
        "float" => DataType::Float(None),
        "char" => DataType::Character(None),
        "String" => DataType::String(None),
        _ => unimplemented!(),
    };

    local_scope.insert(name.to_owned(), data);
}

pub fn set_var(global_scope: &mut Scope, local_scope: &mut Scope, name: &str, value: &str) {
    get_from_scopes_mut(local_scope, global_scope, name).set_value(value);
}

pub fn is_equal(
    global_scope: &mut Scope,
    local_scope: &mut Scope,
    var1: &str,
    var2: &str,
    assign_to: &str,
) {
    let variable1 = get_from_scopes(local_scope, global_scope, var1).clone();
    let variable2 = get_from_scopes(local_scope, global_scope, var2).clone();

    get_from_scopes_mut(local_scope, global_scope, assign_to).set_value(
        if variable1 == variable2 {
            "true"
        } else {
            "false"
        },
    );
}

pub fn is_not_equal(
    global_scope: &mut Scope,
    local_scope: &mut Scope,
    var1: &str,
    var2: &str,
    assign_to: &str,
) {
    let variable1 = get_from_scopes(local_scope, global_scope, var1).clone();
    let variable2 = get_from_scopes(local_scope, global_scope, var2).clone();

    get_from_scopes_mut(local_scope, global_scope, assign_to).set_value(
        if variable1 != variable2 {
            "true"
        } else {
            "false"
        },
    );
}

pub fn is_less_than(
    global_scope: &mut Scope,
    local_scope: &mut Scope,
    var1: &str,
    var2: &str,
    assign_to: &str,
) {
    let variable1 = get_from_scopes(local_scope, global_scope, var1).clone();
    let variable2 = get_from_scopes(local_scope, global_scope, var2).clone();

    get_from_scopes_mut(local_scope, global_scope, assign_to).set_value(if variable1 < variable2 {
        "true"
    } else {
        "false"
    });
}

pub fn is_less_than_equal(
    global_scope: &mut Scope,
    local_scope: &mut Scope,
    var1: &str,
    var2: &str,
    assign_to: &str,
) {
    let variable1 = get_from_scopes(local_scope, global_scope, var1).clone();
    let variable2 = get_from_scopes(local_scope, global_scope, var2).clone();

    get_from_scopes_mut(local_scope, global_scope, assign_to).set_value(
        if variable1 <= variable2 {
            "true"
        } else {
            "false"
        },
    );
}

pub fn is_greater_than(
    global_scope: &mut Scope,
    local_scope: &mut Scope,
    var1: &str,
    var2: &str,
    assign_to: &str,
) {
    let variable1 = get_from_scopes(local_scope, global_scope, var1).clone();
    let variable2 = get_from_scopes(local_scope, global_scope, var2).clone();

    get_from_scopes_mut(local_scope, global_scope, assign_to).set_value(if variable1 > variable2 {
        "true"
    } else {
        "false"
    });
}

pub fn is_greater_than_equal(
    global_scope: &mut Scope,
    local_scope: &mut Scope,
    var1: &str,
    var2: &str,
    assign_to: &str,
) {
    let variable1 = get_from_scopes(local_scope, global_scope, var1).clone();
    let variable2 = get_from_scopes(local_scope, global_scope, var2).clone();

    get_from_scopes_mut(local_scope, global_scope, assign_to).set_value(
        if variable1 >= variable2 {
            "true"
        } else {
            "false"
        },
    );
}

fn get_from_scopes<'a>(scope1: &'a Scope, scope2: &'a Scope, variable: &str) -> &'a DataType {
    if scope1.contains_key(variable) {
        scope1.get(variable).unwrap()
    } else {
        scope2.get(variable).unwrap()
    }
}

fn get_from_scopes_mut<'a>(
    scope1: &'a mut Scope,
    scope2: &'a mut Scope,
    variable: &str,
) -> &'a mut DataType {
    if scope1.contains_key(variable) {
        scope1.get_mut(variable).unwrap()
    } else {
        scope2.get_mut(variable).unwrap()
    }
}
