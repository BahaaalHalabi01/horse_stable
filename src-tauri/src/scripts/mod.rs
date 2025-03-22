use fake::{Fake, Faker};
use libsql::Connection;

use crate::{
    db::{create_horse_table, create_stable_table, get_db_conn},
    services::{create_horse, create_stable, get_user_by_login},
};

pub async fn init(main_conn: &Connection) {
    let email = String::from("hello");
    let pass = String::from("127[]3");
    let user = get_user_by_login(email, pass, main_conn)
        .await
        .unwrap()
        .unwrap();

    let conn = get_db_conn(user.id).await.unwrap();

    create_stable_table(&conn).await.unwrap();
    create_horse_table(&conn).await.unwrap();

    for _ in 0..2 {
        let x = create_stable(Faker.fake::<crate::StableCreate>(), &conn)
            .await
            .unwrap()
            .unwrap();

        let id = x.id();

        for _ in 0..20 {
            create_horse(id, Faker.fake::<crate::HorseCreate>(), &conn)
                .await
                .unwrap();
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn seed_test() {}
}
