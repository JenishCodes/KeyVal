use std::sync::{Arc, Mutex};


use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

use crate::command::{DB, handle_command};
use crate::store::Store;

pub async fn run(addr: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    let db: DB = Arc::new(Mutex::new(Store::new()));

    loop {
        let (socket, _) = listener.accept().await?;
        let db = db.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, db).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}

async fn handle_connection(mut socket: TcpStream, db: DB) -> std::io::Result<()> {
    let (reader, mut writer) = socket.split();
    let mut buffer = BufReader::new(reader);
    let mut line = String::new();

    loop {
        line.clear();

        let bytes = buffer.read_line(&mut line).await?;
        if bytes == 0 {
            break;
        }

        let response = handle_command(&line, &db);
        writer.write_all(response.as_bytes()).await?;
    }

    writer.flush().await?;
    Ok(())
}
