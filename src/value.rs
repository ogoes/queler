use std::fmt::Display;

pub enum Value {
    NULL,
    Boolean(bool),
    String(String),
    Int(i64),
    UInt(u64),
    Float(f32),
    Double(f64),
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::String(v.to_string())
    }
}

impl<T: Into<Value> + Clone> From<&T> for Value {
    fn from(v: &T) -> Self {
        v.clone().into()
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::String(v.to_string())
    }
}

impl From<u8> for Value {
    fn from(v: u8) -> Self {
        Value::UInt(v as u64)
    }
}

impl From<u16> for Value {
    fn from(v: u16) -> Self {
        Value::UInt(v as u64)
    }
}

impl From<u32> for Value {
    fn from(v: u32) -> Self {
        Value::UInt(v as u64)
    }
}

impl From<u64> for Value {
    fn from(v: u64) -> Self {
        Value::UInt(v)
    }
}

impl From<i8> for Value {
    fn from(v: i8) -> Self {
        Value::Int(v as i64)
    }
}

impl From<i16> for Value {
    fn from(v: i16) -> Self {
        Value::Int(v as i64)
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Value::Int(v as i64)
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::Int(v)
    }
}

impl From<f32> for Value {
    fn from(v: f32) -> Self {
        Value::Float(v)
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Double(v)
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Boolean(v)
    }
}

impl<T> From<Option<T>> for Value
where
    T: Into<Value>,
{
    fn from(v: Option<T>) -> Self {
        if let Some(v) = v {
            v.into()
        } else {
            Value::NULL
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::NULL => write!(f, "NULL"),
            Value::String(v) => {
                if v.find(|c| c == ':').is_some() {
                    write!(f, "{}", v.replace(":", ""))
                } else {   
                    write!(f, "'{}'", v)
                }
            }
            Value::Int(v) => write!(f, "{}", v),
            Value::UInt(v) => write!(f, "{}", v),
            Value::Float(v) => write!(f, "{}", v),
            Value::Double(v) => write!(f, "{}", v),
            Value::Boolean(v) => write!(f, "{}", if *v { 1 } else { 0 }),
        }
    }
}
