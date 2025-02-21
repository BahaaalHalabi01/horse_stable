use libsql::{Builder, Connection, Result};
use std::{fs::create_dir_all, path::PathBuf};
mod tables;

pub use tables::*;

use crate::AppState;

pub async fn get_main_db_conn() -> Result<Connection> {
    let db_dir = get_db_dir();

    (if let Ok(url) = std::env::var("LIBSQL_URL") {
        let token = std::env::var("LIBSQL_AUTH_TOKEN").unwrap_or_else(|_| {
            println!("LIBSQL_TOKEN not set, using empty token...");
            "".to_string()
        });
        Builder::new_remote(url, token).build().await
    } else {
        Builder::new_local(db_dir.join("main.db")).build().await
    })?
    .connect()
}


pub async fn get_db_conn(user_id: String) -> Result<Connection> {
    if user_id.is_empty() {
        panic!("User id is empty,how did you get here?");
    }
    let db_dir = get_db_dir().join(&user_id).with_extension("db");

    println!("creating db_dir: {:?}", db_dir);
    Builder::new_local(db_dir)
        .build()
        .await?
        .connect()
}

pub async fn init_user_table(conn: &Connection) -> Result<u64> {
    create_user_table(conn).await
}

pub async fn get_horse_db(app_state: AppState<'_>) -> Result<Connection> {
    let user_id = app_state.lock().await.user_id.clone();

    let conn = get_db_conn(user_id).await?;

    // this is bad i guess,
    // i need to just use migrations
    create_horse_table(&conn).await?;

    Ok(conn)
}

pub async fn get_stable_db(app_state: AppState<'_>) -> Result<Connection> {
    let user_id = app_state.lock().await.user_id.clone();
    let conn = get_db_conn(user_id).await?;

    // this is bad i guess,
    // i need to just use migrations
    create_stable_table(&conn).await?;

    Ok(conn)
}

fn get_db_dir() -> PathBuf {
    //just hard coded for now
    let db_dir = PathBuf::from("/home/bh2/Desktop/Rust/horse-stable/dbs/");

    create_dir_all(&db_dir).expect("could not create dbs folder");

    db_dir

    // match std::env::current_dir() {
    //     Ok(mut cwd) => {
    //         cwd.pop();
    //         cwd.join("dbs")
    //     }
    //     Err(_) => PathBuf::from("../dbs"),
    // }
}
