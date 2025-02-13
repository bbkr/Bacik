use tokio;
mod sqlite;
use sqlite::SQLite;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let x = SQLite::init().await.unwrap();
}
