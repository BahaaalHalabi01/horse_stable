use horse_stable::{Gender, Horse};
use libsql::{de, params, Connection};
// add diesel orm
pub async fn add_horse_query( horse: Horse, conn: &Connection) -> Horse{
    create_horse_table(&conn).await;

    let mut stmt =conn.prepare(r#"
    INSERT INTO Horse (name, breed, color, nationality, age, gender, weight, height, length) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9);
    RETURNING id
    "#).await.unwrap();

    let mut response  =stmt.query(params![
        horse.name,
        horse.breed,
        horse.color,
        horse.nationality,
        horse.age as u64,
        horse.gender.to_string(),
        horse.weight,
        horse.length
    ])
    .await
    .unwrap();

    let ret = match response.next().await.unwrap(){
        Some(row) => Horse{
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            breed: row.get(2).unwrap(),
            color: row.get(3).unwrap(),
            nationality: row.get(4).unwrap(),
            age: row.get(5).unwrap(),
            gender: Gender::from(row.get::<String>(6).unwrap()),
            weight: row.get(7).unwrap(),
            height: row.get(8).unwrap(),
            length: row.get(9).unwrap(),
        },
        None => panic!("No horse found")
    };

    ret
}

pub async fn create_horse_table(conn: &Connection) {
    conn.execute_batch(
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
    )
    .await
    .unwrap();
}
