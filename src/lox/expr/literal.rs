use super::{Expr, Expression};

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Nil,
    Str(String),
    Num(f32),
    Bool(bool),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Bool(v) => write!(f, "{v}"),
            Value::Num(v) => write!(f, "{v}"),
            Value::Str(v) => write!(f, "{v}"),
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::Str(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self::Num(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl<T: Into<Value>> From<Option<T>> for Value {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(t) => t.into(),
            None => Value::Nil,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Literal {
    pub value: Value,
}

impl Literal {
    pub fn new<T: Into<Value>>(value: T) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl From<Literal> for Expr {
    fn from(value: Literal) -> Self {
        Self::Literal(value)
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Expression for Literal {}
