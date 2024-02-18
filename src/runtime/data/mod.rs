use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Clone, Debug)]
pub enum DataType {
    Integer(Option<i64>),
    Boolean(Option<bool>),
    Float(Option<f64>),
    Character(Option<char>),
    String(Option<String>),
}

impl DataType {
    pub fn set_value(&mut self, value: &str) {
        *self = match self {
            DataType::Integer(_) => DataType::Integer(Some(value.parse().unwrap())),
            DataType::Boolean(_) => DataType::Boolean(Some(if value == "true" {
                true
            } else if value == "false" {
                false
            } else {
                panic!()
            })),
            DataType::Float(_) => DataType::Float(Some(value.parse().unwrap())),
            DataType::Character(_) => DataType::Character(Some(value.chars().nth(1).unwrap())),
            DataType::String(_) => DataType::String(Some(value[1..value.len() - 1].to_owned())),
        }
    }
}

impl Add for DataType {
    type Output = DataType;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (DataType::Integer(i), DataType::Integer(j)) => {
                DataType::Integer(Some(i.unwrap() + j.unwrap()))
            }
            (DataType::Integer(i), DataType::Float(j)) => {
                DataType::Float(Some(i.unwrap() as f64 + j.unwrap()))
            }
            (DataType::Float(i), DataType::Integer(j)) => {
                DataType::Float(Some(i.unwrap() + j.unwrap() as f64))
            }
            (DataType::Float(i), DataType::Float(j)) => {
                DataType::Float(Some(i.unwrap() + j.unwrap()))
            }
            (DataType::String(i), DataType::Character(j)) => {
                let mut string = i.clone().unwrap();
                string.push(j.unwrap());
                DataType::String(Some(string))
            }
            (DataType::String(i), DataType::String(j)) => {
                let mut string = i.clone().unwrap();
                string.push_str(j.unwrap().as_str());
                DataType::String(Some(string))
            }
            _ => panic!(),
        }
    }
}

impl Sub for DataType {
    type Output = DataType;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (DataType::Integer(i), DataType::Integer(j)) => {
                DataType::Integer(Some(i.unwrap() - j.unwrap()))
            }
            (DataType::Integer(i), DataType::Float(j)) => {
                DataType::Float(Some(i.unwrap() as f64 - j.unwrap()))
            }
            (DataType::Float(i), DataType::Integer(j)) => {
                DataType::Float(Some(i.unwrap() - j.unwrap() as f64))
            }
            (DataType::Float(i), DataType::Float(j)) => {
                DataType::Float(Some(i.unwrap() - j.unwrap()))
            }
            _ => panic!(),
        }
    }
}

impl Mul for DataType {
    type Output = DataType;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (DataType::Integer(i), DataType::Integer(j)) => {
                DataType::Integer(Some(i.unwrap() * j.unwrap()))
            }
            (DataType::Integer(i), DataType::Float(j)) => {
                DataType::Float(Some(i.unwrap() as f64 * j.unwrap()))
            }
            (DataType::Float(i), DataType::Integer(j)) => {
                DataType::Float(Some(i.unwrap() * j.unwrap() as f64))
            }
            (DataType::Float(i), DataType::Float(j)) => {
                DataType::Float(Some(i.unwrap() * j.unwrap()))
            }
            _ => panic!(),
        }
    }
}

impl Div for DataType {
    type Output = DataType;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (DataType::Integer(i), DataType::Integer(j)) => {
                DataType::Integer(Some(i.unwrap() / j.unwrap()))
            }
            (DataType::Integer(i), DataType::Float(j)) => {
                DataType::Float(Some(i.unwrap() as f64 / j.unwrap()))
            }
            (DataType::Float(i), DataType::Integer(j)) => {
                DataType::Float(Some(i.unwrap() / j.unwrap() as f64))
            }
            (DataType::Float(i), DataType::Float(j)) => {
                DataType::Float(Some(i.unwrap() / j.unwrap()))
            }
            _ => panic!(),
        }
    }
}

impl Rem for DataType {
    type Output = DataType;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (DataType::Integer(i), DataType::Integer(j)) => {
                DataType::Integer(Some(i.unwrap() % j.unwrap()))
            }
            _ => panic!(),
        }
    }
}

impl PartialEq for DataType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(l0), Self::Integer(r0)) => l0.unwrap() == r0.unwrap(),
            (Self::Integer(l0), Self::Float(r0)) => l0.unwrap() as f64 == r0.unwrap(),
            (Self::Boolean(l0), Self::Boolean(r0)) => l0.unwrap() == r0.unwrap(),
            (Self::Float(l0), Self::Float(r0)) => l0.unwrap() == r0.unwrap(),
            (Self::Float(l0), Self::Integer(r0)) => l0.unwrap() == r0.unwrap() as f64,
            (Self::Character(l0), Self::Character(r0)) => l0.unwrap() == r0.unwrap(),
            (Self::String(l0), Self::String(r0)) => l0.clone().unwrap() == r0.clone().unwrap(),
            _ => false,
        }
    }
}

impl PartialOrd for DataType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match (self, other) {
            (DataType::Integer(i), DataType::Integer(j)) => i.unwrap().cmp(&j.unwrap()),
            (DataType::Integer(i), DataType::Float(j)) => {
                (i.unwrap() as f64).total_cmp(&j.unwrap())
            }
            (DataType::Float(i), DataType::Integer(j)) => {
                i.unwrap().total_cmp(&(j.unwrap() as f64))
            }
            (DataType::Float(i), DataType::Float(j)) => i.unwrap().total_cmp(&j.unwrap()),
            (DataType::Character(i), DataType::Character(j)) => i.unwrap().cmp(&j.unwrap()),
            (DataType::String(i), DataType::String(j)) => {
                i.clone().unwrap().cmp(&j.clone().unwrap())
            }
            _ => panic!(),
        })
    }
}
