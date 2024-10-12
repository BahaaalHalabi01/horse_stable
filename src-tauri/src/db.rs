use libsql::{Builder, Connection, Error};
mod queries;

pub use queries::*;

pub async fn init_db() -> Result<Connection, Error> {

    let db = if let Ok(url) = std::env::var("LIBSQL_URL") {
        let token = std::env::var("LIBSQL_AUTH_TOKEN").unwrap_or_else(|_| {
            println!("LIBSQL_TOKEN not set, using empty token...");
            "".to_string()
        });

        Builder::new_remote(url, token).build().await.unwrap()
    } else {
        Builder::new_local("local.db").build().await.unwrap()
    };

    let conn = db.connect()?;

    Ok(conn)
}
