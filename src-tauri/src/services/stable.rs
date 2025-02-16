use std::time::SystemTime;

use horse_stable::{Stable, StableCreate};
use libsql::{params, Result};

pub async fn get_stable(
    id: u32,
    conn: &libsql::Connection,
) -> Result<Option<horse_stable::Stable>> {
    let mut stmt = conn
        .prepare(
            r#"
    SELECT * FROM Stable WHERE id = ?1;
    "#,
        )
        .await?;

    let mut response = stmt.query(params![id]).await?;

    response.next().await?.map(Stable::try_from).transpose()
}

pub async fn create_stable(
    stable: StableCreate,
    conn: &libsql::Connection,
) -> Result<Option<horse_stable::Stable>> {
    let created_at = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|e| e.as_secs());
    let updated_at = created_at.clone();

    if created_at.is_err() {
        panic!("Could not get current time, please try again");
    }

    let mut response = conn
        .query(r#"
        insert into Stable (name, address, monthly_fee, created_at, updated_at) values (?1, ?2, ?3, ?4, ?5) returning *;
        "#,
        params![
            stable.name,
            stable.address,
            stable.monthly_fee,
            created_at.unwrap_or(0),
            updated_at.unwrap_or(0),
        ]
        )
        .await?;

    response.next().await?.map(Stable::try_from).transpose()
}

pub async fn list_stables(conn: &libsql::Connection) -> Result<Vec<Stable>> {
    let mut stmt = conn
        .prepare(
            r#"
    SELECT * FROM Stable;
    "#,
        )
        .await?;

    let mut response = stmt.query(()).await?;

    let mut stables = Vec::new();

    while let Ok(Some(row)) = response.next().await {
        stables.push(Stable::try_from(row)?);
    }

    Ok(stables)
}
