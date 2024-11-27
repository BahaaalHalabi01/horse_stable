use anyhow::anyhow;
use horse_stable::Horse;
use libsql::{params, Connection};

pub fn from_row(row: libsql::Row) -> Option<Horse> {
    Some(Horse::from_db(
        row.get(0).unwrap(),
        row.get(1).unwrap(),
        row.get(2).unwrap(),
        row.get(3).unwrap(),
        row.get(4).unwrap(),
        row.get(5).unwrap(),
        row.get(6).unwrap(),
        row.get(7).unwrap(),
        row.get(8).unwrap(),
        row.get(9).unwrap(),
    ))
}

pub async fn create_horse(horse: Horse, conn: &Connection) -> anyhow::Result<Option<Horse>> {
    let uuid = uuid7::uuid7();
    let mut stmt = conn.prepare(r#"
    INSERT INTO Horse (id, name, breed, color, nationality, gender, weight, age, height, length) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10) RETURNING *;
    "#).await.unwrap();

    let mut response = stmt
        .query(params![
            uuid.to_string(),
            horse.name(),
            horse.breed(),
            horse.color(),
            horse.nationality(),
            horse.gender().to_string(),
            horse.weight(),
            horse.age(),
            horse.height(),
            horse.length(),
        ])
        .await
        .unwrap();

    match response.next().await.unwrap() {
        Some(row) => Ok(from_row(row)),
        None => Err(anyhow!("No horse found")),
    }
}

pub async fn get_horse_by_id(id: u32, conn: &Connection) -> anyhow::Result<Option<Horse>> {
    let mut stmt = conn
        .prepare(
            r#"
            SELECT * from Horse where id = ?1;
    "#,
        )
        .await
        .unwrap();

    let mut response = stmt.query(params![id]).await.unwrap();

    match response.next().await.map_err(|e| anyhow::anyhow!(e))? {
        Some(row) => Ok(from_row(row)),
        None => Ok(None),
    }
}

pub async fn get_all_horses(conn: &Connection) -> Vec<Horse> {
    let mut stmt = conn
        .prepare(
            r#"
SELECT * from Horse;
"#,
        )
        .await
        .unwrap();

    let mut response = stmt.query(()).await.unwrap();

    let mut horses = Vec::new();

    while let Some(row) = response.next().await.unwrap() {
        horses.push(from_row(row).unwrap());
    }

    horses
}

pub async fn delete_horse(id: String, conn: &Connection) -> anyhow::Result<u64> {
    conn.execute("DELETE FROM Horse  WHERE id = ?1", params![id])
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn update_horse(horse: Horse, conn: &Connection) -> Horse {
    let mut stmt =conn
        .prepare(
            r#"
    UPDATE OR IGNORE Horse  
    SET name = ?1, breed = ?2, color = ?3, nationality = ?4, age = ?5, gender = ?6, weight = ?7, height = ?8, length = ?9
    WHERE id = ?10
    RETURNING *
    "#,).await.unwrap();

    let res = match stmt
        .query(params![
            horse.name(),
            horse.breed(),
            horse.color(),
            horse.nationality(),
            horse.age() as u64,
            horse.gender().to_string(),
            horse.weight(),
            horse.height(),
            horse.length(),
            horse.id()
        ])
        .await
    {
        Ok(mut response) => match response.next().await.unwrap() {
            Some(row) => from_row(row),
            None => panic!("No horse found"),
        },
        Err(_) => panic!("Could not update horse"),
    };

    res.unwrap()
}

#[cfg(test)]
mod tests {
    use tauri::Manager as _;

    fn create_app<R: tauri::Runtime>(builder: tauri::Builder<R>) -> tauri::App<R> {
        let app=
        builder
            // remove the string argument to use your app's config file
            .build(tauri::generate_context!())
            .expect("failed to build app");

            app.manage(tokio::sync::Mutex::new(crate::AppStateInner::default()));

        app
    }

    #[tokio::test]
    async fn list_all_horses() {

        let app = create_app(tauri::test::mock_builder());
        let state = app.state();
        let conn = crate::db::get_horse_db(state).await.unwrap();
        let horses = crate::services::get_all_horses(&conn).await;
        assert_eq!(horses.len(), 0);


    }
}
