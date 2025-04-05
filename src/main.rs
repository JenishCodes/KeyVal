mod server;
mod store;
mod command;
mod value;

#[tokio::main]
async fn main() {
    println!("Starting Redis server...");
    let result = server::run("127.0.0.1:6379").await;
    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}