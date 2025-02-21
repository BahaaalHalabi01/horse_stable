use horse_stable::User;
use libsql::{params, Connection, Result};

pub async fn create_user(user: User, conn: &Connection) -> Result<Option<User>> {
    let uuid = uuid7::uuid7();
    let mut stmt =conn.prepare(r#"
    INSERT INTO User (id, username, password, email,created_at,updated_at) VALUES (?1, ?2, ?3, ?4,?5,?6) RETURNING *;
    "#).await.unwrap();

    let mut response = stmt
        .query(params![
            uuid.to_string(),
            user.username,
            user.password,
            user.email,
            user.created_at,
            user.updated_at
        ])
        .await?;

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

pub async fn has_user(email: String, conn: &Connection) -> bool {
    let mut stmt = conn
        .prepare(
            r#"
    SELECT id FROM User WHERE email = ?1;
    "#,
        )
        .await
        .unwrap();

    let mut response = stmt.query(params![email]).await.unwrap();

    match response.next().await.unwrap() {
        Some(row) => row.get::<u64>(0).is_ok(),
        None => false,
    }
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
