use horse_stable::Stable;
use libsql::params;

fn from_row(row: libsql::Row) -> Stable {
    Stable::from_db(
        row.get(0).unwrap(),
        row.get(1).unwrap(),
        row.get(2).unwrap(),
        row.get(3).unwrap(),
        row.get(4).unwrap(),
        row.get(5).unwrap(),
        row.get(6).unwrap(),
    )
}

pub async fn get_stable(id: u32, conn: &libsql::Connection) -> Option<horse_stable::Stable> {
    let mut stmt = conn
        .prepare(
            r#"
    SELECT * FROM Stable WHERE id = ?1;
    "#,
        )
        .await
        .unwrap();

    let mut response = stmt.query(params![id]).await.unwrap();

    match response.next().await.unwrap() {
        Some(row) => Some(from_row(row)),
        None => None,
    }
}

pub async fn create_stable(
    stable: Stable,
    conn: &libsql::Connection,
) -> Option<horse_stable::Stable> {
    let mut stmt = conn
        .prepare(
            r#"
            insert into Stable (name, address, monthly_fee, created_at, updated_at, horse_count) values (?1, ?2, ?3, ?4, ?5, ?6) returning *;
    "#,
        )
        .await
        .unwrap();

    let mut response = stmt
        .query(params![
            stable.name(),
            stable.address(),
            stable.monthly_fee(),
            stable.created_at(),
            stable.updated_at(),
            stable.horse_count().into_inner(),
        ])
        .await
        .unwrap();

    match response.next().await.unwrap() {
        Some(row) => Some(from_row(row)),
        None => None,
    }
}
