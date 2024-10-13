use libsql::Connection;

pub async fn create_horse_table(conn: &Connection) {
    conn.execute(
        r#"
    CREATE TABLE IF NOT EXISTS Horse (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    breed TEXT NOT NULL,
    color TEXT NOT NULL,
    nationality TEXT NOT NULL,
    age INTEGER NOT NULL,
    gender TEXT NOT NULL,
    weight INTEGER NOT NULL,
    height INTEGER NOT NULL,
    length INTEGER NOT NULL
)"#,
        (),
    )
    .await
    .unwrap();
}

pub async fn create_user_table(conn: &Connection) {
    conn.execute(
        r#"
    CREATE TABLE IF NOT EXISTS User (
    id TEXT PRIMARY KEY ,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    email TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
)"#,
        (),
    )
    .await
    .unwrap();
}
