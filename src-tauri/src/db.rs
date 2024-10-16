use std::fs::create_dir_all;
use libsql::{Builder, Connection, Error};
mod queries;
mod tables;

pub use queries::*;
pub use tables::*;

use crate::AppState;

pub async fn init_main_db() -> Result<Connection, Error> {
    let db = if let Ok(url) = std::env::var("LIBSQL_URL") {
        let token = std::env::var("LIBSQL_AUTH_TOKEN").unwrap_or_else(|_| {
            println!("LIBSQL_TOKEN not set, using empty token...");
            "".to_string()
        });

        Builder::new_remote(url, token).build().await.unwrap()
    } else {
        create_dir_all("./dbs").expect("could not create dbs folder");
        Builder::new_local("./dbs/main.db").build().await.expect("could not create db")
    };

    let conn = db.connect()?;

    create_user_table(&conn).await;
    Ok(conn)
}

pub async fn get_db(app_state: AppState<'_>) -> Result<Connection, Error> {
    let app_state = app_state.lock().await;

    let user_id = app_state.user_id.clone();


        create_dir_all("./dbs").expect("could not create dbs folder");
    let db = Builder::new_local(format!("./dbs/{}.db", user_id))
        .build()
        .await
        .expect(&format!("could not create db for {}", user_id));

    let conn = db.connect()?;

    // this is bad i guess,
    // i need to just use migrations
    create_horse_table(&conn).await;

    Ok(conn)
}
