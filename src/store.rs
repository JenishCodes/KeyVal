use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

use crate::value::Value;

pub struct Store {
    data: HashMap<String, Value>,
    expiry: HashMap<String, Instant>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            data: HashMap::new(),
            expiry: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &Value) {
        self.data.insert(key.to_string(), value.clone());
        self.expiry.remove(key);
    }

    pub fn get(&mut self, key: &str) -> Option<Value> {
        if let Some(expiry_time) = self.expiry.get(key) {
            if Instant::now() > *expiry_time {
                self.data.remove(key);
                self.expiry.remove(key);
                return None;
            }
        }
        self.data.get(key).cloned()
    }

    pub fn del(&mut self, key: &str) -> bool {
        self.expiry.remove(key);
        self.data.remove(key).is_some()
    }

    pub fn expire(&mut self, key: &str, duration: u64) -> bool {
        if self.data.contains_key(key) {
            self.expiry.insert(
                key.to_string(),
                Instant::now() + Duration::from_secs(duration),
            );
            true
        } else {
            false
        }
    }

    pub fn ttl(&mut self, key: &str) -> Option<i64> {
        if let Some(expiry_time) = self.expiry.get(key) {
            if Instant::now() < *expiry_time {
                return Some(expiry_time.duration_since(Instant::now()).as_secs() as i64);
            }

            self.data.remove(key);
            self.expiry.remove(key);
            return Some(-1);
        }

        if self.data.contains_key(key) {
            Some(-1)
        } else {
            Some(-2)
        }
    }

    pub fn incr_by(&mut self, key: &str, by: i64) -> Option<i64> {
        let current = self.get(key)?;
        if !current.is_string() {
            return None;
        }
        let current = current.as_string().unwrap();

        match current.parse::<i64>() {
            Ok(n) => {
                let new_value = n + by;
                self.set(key, &Value::from(new_value.to_string()));
                Some(new_value)
            }
            Err(_) => None,
        }
    }

    pub fn lpush(&mut self, key: &str, value: Vec<String>) -> usize {
        let current = self.get(key);
        let mut list = match current {
            Some(Value::List(list)) => list,
            _ => VecDeque::new(),
        };

        for v in value.clone() {
            list.push_front(v);
        }

        let len = list.len();
        self.set(key, &Value::from(list));

        len
    }

    pub fn rpush(&mut self, key: &str, value: Vec<String>) -> usize {
        let current = self.get(key);
        let mut list = match current {
            Some(Value::List(list)) => list,
            _ => VecDeque::new(),
        };

        for v in value {
            list.push_back(v.to_string());
        }

        let len = list.len();
        self.set(key, &Value::from(list));

        len
    }

    pub fn lpop(&mut self, key: &str) -> Option<String> {
        let current = self.get(key);
        if let Some(Value::List(mut list)) = current {
            let value = list.pop_front();
            self.set(key, &Value::from(list));
            value
        } else {
            None
        }
    }
    pub fn rpop(&mut self, key: &str) -> Option<String> {
        let current = self.get(key);
        if let Some(Value::List(mut list)) = current {
            let value = list.pop_back();
            self.set(key, &Value::from(list));
            value
        } else {
            None
        }
    }

    pub fn llen(&mut self, key: &str) -> Option<usize> {
        let current = self.get(key);
        if let Some(Value::List(list)) = current {
            Some(list.len())
        } else {
            None
        }
    }

    pub fn lindex(&mut self, key: &str, index: usize) -> Option<String> {
        let current = self.get(key);
        if let Some(Value::List(list)) = current {
            if index < list.len() {
                return Some(list[index].clone());
            }
        }
        None
    }

    pub fn lset(&mut self, key: &str, index: usize, value: String) -> bool {
        let current = self.get(key);
        if let Some(Value::List(mut list)) = current {
            if index < list.len() {
                list[index] = value;
                self.set(key, &Value::from(list));
                return true;
            }
        }
        false
    }

    pub fn lrange(&mut self, key: &str, start: usize, end: usize) -> Option<Vec<String>> {
        let current = self.get(key);
        if let Some(Value::List(list)) = current {
            if start < end && start < list.len() && end < list.len() {
                return Some(list.range(start..=end).cloned().collect());
            }
        }
        None
    }

    pub fn lrem(&mut self, key: &str, count: i64, value: String) -> usize {
        let current = self.get(key);
        if let Some(Value::List(mut list)) = current {
            let mut removed_count = 0;
            if count > 0 {
                while let Some(pos) = list.iter().position(|x| *x == value) {
                    list.remove(pos);
                    removed_count += 1;
                    if removed_count == count as usize {
                        break;
                    }
                }
            } else if count < 0 {
                while let Some(pos) = list.iter().rposition(|x| *x == value) {
                    list.remove(pos);
                    removed_count += 1;
                    if removed_count == (-count) as usize {
                        break;
                    }
                }
            } else {
                removed_count = list.iter().filter(|x| **x == value).count();
                list.retain(|x| *x != value);
            }

            self.set(key, &Value::from(list));
            return removed_count;
        }
        0
    }

    pub fn hset(&mut self, key: &str, field: &str, value: &str) -> bool {
        let current = self.get(key);
        let mut hash = match current {
            Some(Value::Hash(hash)) => hash,
            _ => HashMap::new(),
        };

        let res = match hash.insert(field.to_string(), value.to_string()) {
            Some(_) => true,
            None => false,
        };

        self.set(key, &Value::from(hash));

        res
    }

    pub fn hget(&mut self, key: &str, field: &str) -> Option<String> {
        let current = self.get(key);
        if let Some(Value::Hash(hash)) = current {
            return hash.get(field).cloned();
        }
        None
    }

    pub fn hdel(&mut self, key: &str, field: &str) -> bool {
        let current = self.get(key);
        if let Some(Value::Hash(mut hash)) = current {
            let res = hash.remove(field).is_some();
            self.set(key, &Value::from(hash));
            return res;
        }
        false
    }

    pub fn hlen(&mut self, key: &str) -> Option<usize> {
        let current = self.get(key);
        if let Some(Value::Hash(hash)) = current {
            return Some(hash.len());
        }
        None
    }

    pub fn hget_all(&mut self, key: &str) -> Option<HashMap<String, String>> {
        let current = self.get(key);
        if let Some(Value::Hash(hash)) = current {
            return Some(hash.clone());
        }
        None
    }

    pub fn hincr_by(&mut self, key: &str, field: &str, by: i64) -> Option<i64> {
        let current = self.get(key);
        if let Some(Value::Hash(mut hash)) = current {
            if let Some(value) = hash.get_mut(field) {
                match value.parse::<i64>() {
                    Ok(n) => {
                        let new_value = n + by;
                        *value = new_value.to_string();
                        self.set(key, &Value::from(hash));
                        return Some(new_value);
                    }
                    Err(_) => return None,
                }
            } else {
                hash.insert(field.to_string(), by.to_string());
                self.set(key, &Value::from(hash));
                return Some(by);
            }
        }
        None
    }
}
