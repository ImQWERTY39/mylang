#[derive(Debug, Clone)]
pub enum DataType {
    Integer(Option<i64>),
    Float(Option<f64>),
    Boolean(Option<bool>),
    String(Option<String>),
    Character(Option<char>),
    Callable(Option<Vec<Box<DataType>>>),
}

impl DataType {
    pub fn same_type(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(_), Self::Integer(_)) => true,
            (Self::Float(_), Self::Float(_)) => true,
            (Self::Boolean(_), Self::Boolean(_)) => true,
            (Self::String(_), Self::String(_)) => true,
            (Self::Character(_), Self::Character(_)) => true,
            (Self::Callable(_), Self::Callable(_)) => true,
            _ => false,
        }
    }
}

pub enum ValueType {
    Identifier(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Character(char),
    String(String),
}

pub fn get_value_type(value: &str) -> ValueType {
    if value.contains('.') {
        match value.parse::<f64>() {
            Ok(i) => return ValueType::Float(i),
            Err(_) => (),
        };
    }

    match value.parse::<i64>() {
        Ok(i) => return ValueType::Integer(i),
        Err(_) => (),
    };

    if value == "true" {
        ValueType::Boolean(true)
    } else if value == "false" {
        ValueType::Boolean(false)
    } else if value.starts_with("'") {
        ValueType::Character(value.chars().nth(1).unwrap())
    } else if value.starts_with('"') {
        ValueType::String(value[1..(value.len() - 1)].to_string())
    } else {
        ValueType::Identifier(value.to_string())
    }
}
