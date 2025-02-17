use horse_stable::{Horse, HorseCreate};
use libsql::{params, Connection, Result};

pub async fn create_horse(stable_id:u32,horse: HorseCreate, conn: &Connection) -> Result<Option<Horse>> {
    let uuid = uuid7::uuid7();
    let mut stmt = conn.prepare(r#"
    INSERT INTO Horse (id, name, breed, color, nationality, gender, weight, age, height, length,stable_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,?11) RETURNING *;
    "#).await?;

    let mut response = stmt
        .query(params![
            uuid.to_string(),
            horse.name,
            horse.breed,
            horse.color,
            horse.nationality,
            horse.gender.to_string(),
            horse.weight,
            horse.age,
            horse.height,
            horse.length,
            stable_id
        ])
        .await?;

    response.next().await?.map(Horse::try_from).transpose()
}

pub async fn get_horse_by_id(id: u32, conn: &Connection) -> Result<Option<Horse>> {
    let mut stmt = conn
        .prepare(
            r#"
            SELECT * from Horse where id = ?1;
    "#,
        )
        .await
        .unwrap();

    let mut response = stmt.query(params![id]).await.unwrap();

    response.next().await?.map(Horse::try_from).transpose()
}

pub async fn get_all_horses(conn: &Connection) -> Result<Vec<Horse>> {
    let mut stmt = conn
        .prepare(
            r#"
SELECT * from Horse;
"#,
        )
        .await?;

    let mut response = stmt.query(()).await?;

    let mut horses = Vec::new();

    while let Ok(Some(row)) = response.next().await {
        horses.push(Horse::try_from(row)?);
    }

    Ok(horses)
}

pub async fn delete_horse(id: String, conn: &Connection) -> anyhow::Result<u64> {
    if id.is_empty() {
        return Err(anyhow::anyhow!("id is empty"));
    }
    println!("deleting horse with id {}", id);
    conn.execute("DELETE FROM Horse  WHERE id = ?1", params![id])
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn update_horse(horse: Horse, conn: &Connection) -> Result<Option<Horse>> {
    let mut stmt =conn
        .prepare(
            r#"
    UPDATE OR IGNORE Horse  
    SET name = ?1, breed = ?2, color = ?3, nationality = ?4, age = ?5, gender = ?6, weight = ?7, height = ?8, length = ?9
    WHERE id = ?10
    RETURNING *
    "#,).await?;

    let mut res = stmt
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
        .await?;

    res.next().await?.map(Horse::try_from).transpose()
}

#[cfg(test)]
mod tests {
    use tauri::Manager as _;

    fn create_app<R: tauri::Runtime>(builder: tauri::Builder<R>) -> tauri::App<R> {
        let app = builder
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
        assert_eq!(horses.unwrap().len(), 0);
    }
}
