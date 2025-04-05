use core::fmt;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    List(VecDeque<String>),
    Hash(HashMap<String, String>),
    Set(HashSet<String>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "{}", s),
            Value::List(l) => write!(f, "{:?}", l),
            Value::Hash(h) => write!(f, "{:?}", h),
            Value::Set(s) => write!(f, "{:?}", s),
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}
impl From<VecDeque<String>> for Value {
    fn from(value: VecDeque<String>) -> Self {
        Value::List(value)
    }
}
impl From<HashMap<String, String>> for Value {
    fn from(value: HashMap<String, String>) -> Self {
        Value::Hash(value)
    }
}
impl From<HashSet<String>> for Value {
    fn from(value: HashSet<String>) -> Self {
        Value::Set(value)
    }
}

impl Value {
    pub fn as_string(&self) -> Option<&String> {
        if let Value::String(ref s) = *self {
            Some(s)
        } else {
            None
        }
    }

    pub fn as_list(&self) -> Option<&VecDeque<String>> {
        if let Value::List(ref l) = *self {
            Some(l)
        } else {
            None
        }
    }

    pub fn as_hash(&self) -> Option<&HashMap<String, String>> {
        if let Value::Hash(ref h) = *self {
            Some(h)
        } else {
            None
        }
    }

    pub fn as_set(&self) -> Option<&HashSet<String>> {
        if let Value::Set(ref s) = *self {
            Some(s)
        } else {
            None
        }
    }
}
impl Value {
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    pub fn is_list(&self) -> bool {
        matches!(self, Value::List(_))
    }

    pub fn is_hash(&self) -> bool {
        matches!(self, Value::Hash(_))
    }

    pub fn is_set(&self) -> bool {
        matches!(self, Value::Set(_))
    }
}

impl Value {
    pub fn len(&self) -> usize {
        match *self {
            Value::String(ref s) => s.len(),
            Value::List(ref l) => l.len(),
            Value::Hash(ref h) => h.len(),
            Value::Set(ref s) => s.len(),
        }
    }
}
