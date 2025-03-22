use std::time::SystemTime;

use horse_stable::User;
use libsql::{params, Connection, Result};

pub async fn create_user(user: User, conn: &Connection) -> Result<Option<User>> {
    let uuid = uuid7::uuid7();

    let created_at = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|e| e.as_secs());
    let updated_at = created_at.clone();

    let mut response =conn.query(r#"
    INSERT INTO User (id, username, password, email,created_at,updated_at) VALUES (?1, ?2, ?3, ?4,?5,?6) RETURNING *;
    "#, params![    
    uuid.to_string(),
    user.username,
    user.password,
    user.email,
    created_at.unwrap_or(0),
    updated_at.unwrap_or(0)
    ]).await?;

    response.next().await?.map(User::try_from).transpose()
}

pub async fn get_user_by_id(id: String, conn: &Connection) -> Result<Option<User>> {
    let mut stmt = conn
        .prepare(
            r#"
    SELECT * FROM User WHERE id = ?1;
    "#,
        )
        .await?;

    let mut response = stmt.query(params![id]).await?;

    response.next().await?.map(User::try_from).transpose()
}

pub async fn has_user(email: String, conn: &Connection) -> Result<bool> {
    println!("checking if user exists {:?}", email);
    let mut response = conn
        .query(
            r#"
    SELECT id FROM User WHERE email = ?1;
    "#,
            params![email],
        )
        .await?;

    let x = response.next().await.map(|x| x.is_some());
    println!("x {:?}", x);

    x
}

pub async fn get_user_by_login(
    email: String,
    password: String,
    conn: &Connection,
) -> Result<Option<User>> {
    let mut stmt = conn
        .prepare(
            r#"
    SELECT * FROM User WHERE email = ?1 AND password = ?2;
    "#,
        )
        .await?;

    let mut response = stmt.query(params![email, password]).await?;

    response.next().await?.map(User::try_from).transpose()
}
