use crate::store::Store;
use std::sync::{Arc, Mutex};

pub type DB = Arc<Mutex<Store>>;

pub fn handle_command(input: &str, db: &DB) -> String {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.is_empty() {
        return "-ERR empty command\r\n".to_string();
    }

    let command = parts[0].to_uppercase();
    let args = &parts[1..];

    if command == "PING" {
        return "+PONG\r\n".to_string();
    }

    let mut store = db.lock().unwrap();

    match command.as_str() {
        "SET" if args.len() == 2 => {
            store.set(args[0], args[1]);
            "+OK\r\n".to_string()
        }
        "GET" if args.len() == 1 => match store.get(args[0]) {
            Some(value) => format!("${}\r\n{}\r\n", value.len(), value),
            None => "$-1\r\n".to_string(),
        },
        "DEL" if args.len() == 1 => {
            format!(":{}\r\n", if store.del(args[0]) { 1 } else { 0 })
        }
        "EXPIRE" if args.len() == 2 => match args[1].parse::<u64>() {
            Ok(time) => format!("{}\r\n", if store.expire(args[0], time) { 1 } else { 0 }),
            Err(_) => "-ERR value is not an integer or out of range\r\n".to_string(),
        },
        "TTL" if args.len() == 1 => {
            format!(":{}\r\n", store.ttl(args[0]).unwrap())
        }
        "EXISTS" if args.len() == 1 => {
            format!(":{}\r\n", if store.get(args[0]).is_some() { 1 } else { 0 })
        }
        "STRLEN" if args.len() == 1 => match store.get(args[0]) {
            Some(value) => format!(":{}\r\n", value.len()),
            None => "$-1\r\n".to_string(),
        },
        "INCRBY" if args.len() == 2 => args[1]
            .parse::<i64>()
            .ok()
            .and_then(|value| store.incr_by(args[0], value))
            .map(|v| format!("{}\r\n", v))
            .unwrap_or_else(|| "-ERR value is not an integer or out of range\r\n".to_string()),

        "DECRBY" if args.len() == 2 => args[1]
            .parse::<i64>()
            .ok()
            .and_then(|value| store.incr_by(args[0], -value))
            .map(|v| format!("{}\r\n", v))
            .unwrap_or_else(|| "-ERR value is not an integer or out of range\r\n".to_string()),
        "INCR" if args.len() == 1 => store
            .incr_by(args[0], 1)
            .map(|v| format!("{}\r\n", v))
            .unwrap_or_else(|| "-ERR value is not an integer or out of range\r\n".to_string()),
        "DECR" if args.len() == 1 => store
            .incr_by(args[0], -1)
            .map(|v| format!("{}\r\n", v))
            .unwrap_or_else(|| "-ERR value is not an integer or out of range\r\n".to_string()),
        "SET" | "GET" | "DEL" | "EXPIRE" | "TTL" | "INCR" | "DECR" | "INCRBY" | "DECRBY"
        | "EXISTS" | "STRLEN" => {
            format!(
                "-ERR wrong number of arguments for '{}'\r\n",
                command.to_lowercase()
            )
        }
        _ => format!("-ERR unknown command `{}`\r\n", command),
    }
}
