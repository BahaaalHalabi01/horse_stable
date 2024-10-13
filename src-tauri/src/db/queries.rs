use horse_stable::{Gender, Horse};
use libsql::{params, Connection};
// add diesel orm
pub async fn add_horse_query(horse: Horse, conn: &Connection) -> u32 {
    create_horse_table(&conn).await;

    println!("adding horse {:?}", horse);

    let mut stmt =conn.prepare(r#"
    INSERT INTO Horse (name, breed, color, nationality, age, gender, weight, height, length) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9) RETURNING *;
    "#).await.unwrap();

    let mut response = stmt
        .query(params![
            horse.name,
            horse.breed,
            horse.color,
            horse.nationality,
            horse.age as u64,
            horse.gender.to_string(),
            horse.weight,
            horse.height,
            horse.length
        ])
        .await
        .unwrap();

    let ret: u32 = match response.next().await.unwrap() {
        Some(row) => row.get(0).unwrap(),
        None => panic!("No horse found"),
    };

    ret
}

pub async fn get_all_horses_query(conn: &Connection) -> Vec<Horse> {
    create_horse_table(&conn).await;

    let mut stmt = conn.query("SELECT * FROM Horse", params![]).await.unwrap();

    let mut horses: Vec<Horse> = vec![];

    while let Some(row) = stmt.next().await.unwrap() {
        horses.push(Horse {
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
        });
    }

    horses
}

pub async fn delete_horse_query(id: u32, conn: &Connection) {
    create_horse_table(&conn).await;

    if let Ok(res) = conn
        .execute("DELETE FROM Horse  WHERE id = ?1", params![id])
        .await
    {
        println!("deleted  {} horse(s) with id {}", res, id);
    } else {
        panic!("Could not delete horse");
    }

    ()
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
