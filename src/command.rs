use tokio::sync::{Mutex, MutexGuard};

use crate::store::Store;
use crate::value::Value;
use std::sync::Arc;

pub type DB = Arc<Mutex<Store>>;

#[derive(Debug, Clone)]
pub enum Command {
    Ping,
    Quit,

    Set(String, Value),
    Get(String),
    Del(String),
    Expire(String, u64),
    TTL(String),
    Exists(String),
    Strlen(String),
    IncrBy(String, i64),
    DecrBy(String, i64),
    Incr(String),
    Decr(String),

    LPush(String, Vec<String>),
    RPush(String, Vec<String>),
    LPop(String),
    RPop(String),
    LRange(String, usize, usize),
    LRem(String, i64, String),
    LIndex(String, usize),
    LSet(String, usize, String),
    LLen(String),
}

impl Command {
    pub fn parse(input: &String) -> Result<Command, String> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            return Err("Empty command".to_string());
        }

        let cmd = parts[0].to_uppercase();
        let args = &parts[1..];

        match cmd.as_str() {
            "SET" if args.len() == 2 => Ok(Command::Set(
                args[0].to_string(),
                Value::from(args[1].to_string()),
            )),
            "GET" if args.len() == 1 => Ok(Command::Get(args[0].to_string())),
            "DEL" if args.len() == 1 => Ok(Command::Del(args[0].to_string())),
            "EXPIRE" if args.len() == 2 => match args[1].parse::<u64>() {
                Ok(time) => Ok(Command::Expire(args[0].to_string(), time)),
                Err(_) => Err("Invalid time".to_string()),
            },
            "TTL" if args.len() == 1 => Ok(Command::TTL(args[0].to_string())),
            "EXISTS" if args.len() == 1 => Ok(Command::Exists(args[0].to_string())),
            "STRLEN" if args.len() == 1 => Ok(Command::Strlen(args[0].to_string())),
            "INCRBY" if args.len() == 2 => match args[1].parse::<i64>() {
                Ok(value) => Ok(Command::IncrBy(args[0].to_string(), value)),
                Err(_) => Err("Invalid value".to_string()),
            },
            "DECRBY" if args.len() == 2 => match args[1].parse::<i64>() {
                Ok(value) => Ok(Command::DecrBy(args[0].to_string(), value)),
                Err(_) => Err("Invalid value".to_string()),
            },
            "INCR" if args.len() == 1 => Ok(Command::Incr(args[0].to_string())),
            "DECR" if args.len() == 1 => Ok(Command::Decr(args[0].to_string())),

            "LPUSH" if args.len() >= 2 => {
                let values = args[1..].iter().map(|&s| s.to_string()).collect();
                Ok(Command::LPush(args[0].to_string(), values))
            }
            "RPUSH" if args.len() >= 2 => {
                let values = args[1..].iter().map(|&s| s.to_string()).collect();
                Ok(Command::RPush(args[0].to_string(), values))
            }
            "LPOP" if args.len() == 1 => Ok(Command::LPop(args[0].to_string())),
            "RPOP" if args.len() == 1 => Ok(Command::RPop(args[0].to_string())),
            "LRANGE" if args.len() == 3 => {
                let start = args[1]
                    .parse::<usize>()
                    .map_err(|_| "Invalid start".to_string())?;
                let end = args[2]
                    .parse::<usize>()
                    .map_err(|_| "Invalid end".to_string())?;
                Ok(Command::LRange(args[0].to_string(), start, end))
            }
            "LREM" if args.len() == 3 => {
                let count = args[1]
                    .parse::<i64>()
                    .map_err(|_| "Invalid count".to_string())?;
                Ok(Command::LRem(
                    args[0].to_string(),
                    count,
                    args[2].to_string(),
                ))
            }
            "LINDEX" if args.len() == 2 => {
                let index = args[1]
                    .parse::<usize>()
                    .map_err(|_| "Invalid index".to_string())?;
                Ok(Command::LIndex(args[0].to_string(), index))
            }
            "LSET" if args.len() == 3 => {
                let index = args[1]
                    .parse::<usize>()
                    .map_err(|_| "Invalid index".to_string())?;
                Ok(Command::LSet(
                    args[0].to_string(),
                    index,
                    args[2].to_string(),
                ))
            }
            "LLEN" if args.len() == 1 => Ok(Command::LLen(args[0].to_string())),

            "PING" if args.is_empty() => Ok(Command::Ping),
            "QUIT" if args.is_empty() => Ok(Command::Quit),

            _ => Err(format!("Unknown or malformed command: {}", cmd)),
        }
    }

    pub fn execute(&self, store: &mut MutexGuard<Store>) -> String {
        match self {
            Command::Ping => format!("+PONG\r\n"),
            Command::Quit => format!("+OK\r\n"),
            Command::Set(key, value) => {
                store.set(key, value);
                format!("+OK\r\n")
            }
            Command::Get(key) => match store.get(key) {
                Some(value) => format!("${}\r\n{}\r\n", value.len(), value),
                None => "$-1\r\n".to_string(),
            },
            Command::Del(key) => {
                format!(":{}\r\n", if store.del(key) { 1 } else { 0 })
            }
            Command::Expire(key, time) => {
                format!(":{}\r\n", if store.expire(key, *time) { 1 } else { 0 })
            }
            Command::TTL(key) => {
                format!(":{}\r\n", store.ttl(key).unwrap())
            }
            Command::Exists(key) => {
                format!(":{}\r\n", if store.get(key).is_some() { 1 } else { 0 })
            }
            Command::Strlen(key) => match store.get(key) {
                Some(value) => format!(":{}\r\n", value.len()),
                None => "$-1\r\n".to_string(),
            },
            Command::IncrBy(key, value) => match store.incr_by(key, *value) {
                Some(v) => format!("{}\r\n", v),
                None => "-ERR value is not an integer or out of range\r\n".to_string(),
            },
            Command::DecrBy(key, value) => match store.incr_by(key, -value) {
                Some(v) => format!("{}\r\n", v),
                None => "-ERR value is not an integer or out of range\r\n".to_string(),
            },
            Command::Incr(key) => match store.incr_by(key, 1) {
                Some(v) => format!("{}\r\n", v),
                None => "-ERR value is not an integer or out of range\r\n".to_string(),
            },
            Command::Decr(key) => match store.incr_by(key, -1) {
                Some(v) => format!("{}\r\n", v),
                None => "-ERR value is not an integer or out of range\r\n".to_string(),
            },

            Command::LPush(key, values) => {
                format!("{}\r\n", store.lpush(key, values.clone()))
            }
            Command::RPush(key, values) => {
                format!("{}\r\n", store.rpush(key, values.clone()))
            }
            Command::LPop(key) => match store.lpop(key) {
                Some(value) => format!("{}\r\n", value),
                None => "$-1\r\n".to_string(),
            },
            Command::RPop(key) => match store.rpop(key) {
                Some(value) => format!("{}\r\n", value),
                None => "$-1\r\n".to_string(),
            },
            Command::LRange(key, start, end) => {
                match store.lrange(key, *start as usize, *end as usize) {
                    Some(result) => {
                        format!("*{}\r\n", result.len())
                            + &result
                                .iter()
                                .map(|v| format!("${}\r\n{}\r\n", v.len(), v))
                                .collect::<String>()
                    }
                    None => return "-ERR index out of range\r\n".to_string(),
                }
            }
            Command::LRem(key, count, value) => {
                format!(":{}\r\n", store.lrem(key, *count, value.to_string()))
            }
            Command::LIndex(key, index) => match store.lindex(key, *index as usize) {
                Some(value) => format!("{}\r\n", value),
                None => "-ERR index out of range\r\n".to_string(),
            },
            Command::LSet(key, index, value) => {
                if store.lset(key, *index, value.clone()) {
                    format!("+OK\r\n")
                } else {
                    "-ERR index out of range\r\n".to_string()
                }
            }
            Command::LLen(key) => match store.llen(key) {
                Some(len) => format!("{}\r\n", len),
                None => "$-1\r\n".to_string(),
            },
        }
    }

    pub fn is_quit(&self) -> bool {
        matches!(self, Command::Quit)
    }
}
