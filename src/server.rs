use std::sync::Arc;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

use crate::command::{Command, DB};
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

        let command = match Command::parse(&line) {
            Ok(cmd) => cmd,
            Err(err) => {
                writer
                    .write_all(format!("ERR {}\n", err).as_bytes())
                    .await?;
                continue;
            }
        };

        let mut store = db.lock().await;
        let response = command.execute(&mut store);

        writer.write_all(response.as_bytes()).await?;

        if command.is_quit() {
            break;
        }
    }

    writer.flush().await?;
    Ok(())
}
