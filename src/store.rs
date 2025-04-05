use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct Store {
    data: HashMap<String, String>,
    expiry: HashMap<String, Instant>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            data: HashMap::new(),
            expiry: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
        self.expiry.remove(key);
    }

    pub fn get(&mut self, key: &str) -> Option<String> {
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
        let existed = self.data.remove(key).is_some();
        self.expiry.remove(key);
        existed
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
        let current = self.get(key).unwrap_or("0".to_string());

        match current.parse::<i64>() {
            Ok(n) => {
                let new_value = n + by;
                self.set(key, &new_value.to_string());
                Some(new_value)
            }
            Err(_) => None,
        }
    }
}
