//use std::str::FromStr;

use dotenv::dotenv;
use tokio_postgres::NoTls;
//use refinery::config::Config;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let host = std::env::var("DB_HOST").expect("NO ENV HOST SET");
    let username = std::env::var("DB_USERNAME").expect("NO ENV USERNAME SET");
    let password = std::env::var("DB_PASSWORD").expect("NO ENV PASSWORD SET");
    let (mut client, connection) =
        tokio_postgres::connect(&format!("host={host} user={username} password={password}"), NoTls).await?;


    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // there is another way to set up a connection, in where we directly use
    // refinery tools since Config implements AsyncMigrate
    //let db_connection = format!("postgres://{username}:{password}@{host}:5432");
    //let mut config = Config::from_str(&db_connection)?;

    embedded::migrations::runner()
        .run_async(&mut client)
        .await?;
    Ok(())
}
