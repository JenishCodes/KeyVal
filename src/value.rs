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

impl Value {
    pub fn new_string() -> Self {
        Value::String(String::new())
    }

    pub fn new_list() -> Self {
        Value::List(VecDeque::new())
    }

    pub fn new_hash() -> Self {
        Value::Hash(HashMap::new())
    }

    pub fn new_set() -> Self {
        Value::Set(HashSet::new())
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
impl From<Value> for String {
    fn from(value: Value) -> Self {
        match value {
            Value::String(s) => s,
            _ => panic!("Cannot convert non-string value to string"),
        }
    }
}
impl From<Value> for VecDeque<String> {
    fn from(value: Value) -> Self {
        match value {
            Value::List(l) => l,
            _ => panic!("Cannot convert non-list value to list"),
        }
    }
}
impl From<Value> for HashMap<String, String> {
    fn from(value: Value) -> Self {
        match value {
            Value::Hash(h) => h,
            _ => panic!("Cannot convert non-hash value to hash"),
        }
    }
}
impl From<Value> for HashSet<String> {
    fn from(value: Value) -> Self {
        match value {
            Value::Set(s) => s,
            _ => panic!("Cannot convert non-set value to set"),
        }
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
    pub fn push(&mut self, value: String) {
        if let Value::List(ref mut list) = *self {
            list.push_back(value);
        } else {
            panic!("Cannot push to non-list value");
        }
    }

    pub fn pop(&mut self) -> Option<String> {
        if let Value::List(ref mut list) = *self {
            list.pop_front()
        } else {
            panic!("Cannot pop from non-list value");
        }
    }
}

impl Value {
    pub fn insert(&mut self, key: String, value: String) {
        if let Value::Hash(ref mut hash) = *self {
            hash.insert(key, value);
        } else {
            panic!("Cannot insert into non-hash value");
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        if let Value::Hash(ref hash) = *self {
            hash.get(key)
        } else {
            panic!("Cannot get from non-hash value");
        }
    }
}
impl Value {
    pub fn add(&mut self, value: String) {
        if let Value::Set(ref mut set) = *self {
            set.insert(value);
        } else {
            panic!("Cannot add to non-set value");
        }
    }

    pub fn contains(&self, value: &str) -> bool {
        if let Value::Set(ref set) = *self {
            set.contains(value)
        } else {
            panic!("Cannot check containment in non-set value");
        }
    }

    pub fn remove(&mut self, value: &str) {
        if let Value::Set(ref mut set) = *self {
            set.remove(value);
        } else {
            panic!("Cannot remove from non-set value");
        }
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
