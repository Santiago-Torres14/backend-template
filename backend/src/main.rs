use chrono::offset::Utc;
use serde_json;


#[tokio::main]
async fn main() {
    let now = serde_json::to_string(&Utc::now()).unwrap();
    println!("{now}");
}
