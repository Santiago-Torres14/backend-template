use std::{error::Error, str::FromStr};

use axum::{Router, routing::get};
use bb8_postgres::{PostgresConnectionManager, bb8::Pool};
use tokio_postgres::NoTls;


#[derive(Clone)]
struct AppState {
    pub pg_pool: Pool<PostgresConnectionManager<NoTls>>
}

type BoxedError = Box<dyn Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), BoxedError>{

    dotenv::dotenv().ok();

    let host = std::env::var("DB_HOST").expect("NO ENV HOST SET");
    let username = std::env::var("DB_USERNAME").expect("NO ENV USERNAME SET");
    let password = std::env::var("DB_PASSWORD").expect("NO ENV PASSWORD SET");
    let postgres_config = tokio_postgres::Config::from_str(&format!("postgres://${username}:${password}@${host}:5432"))?;
    let pg_cm = PostgresConnectionManager::new(postgres_config, NoTls);

    let pool = Pool::builder().build(pg_cm).await?;

    let app_state = AppState {
        pg_pool: pool
    };

    let router = Router::new().route("/", get(|| async { "hello, world"})).with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;

    axum::serve(listener, router).await?;

    Ok(())
} 
