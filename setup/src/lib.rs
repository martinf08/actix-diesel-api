use db::DbConnection;
use diesel::{MysqlConnection, RunQueryDsl};
use fake::Fake;

pub fn create_user(conn: &MysqlConnection, name: &str) {
    use db::schema::users::dsl::users;

    let new_user = db::models::NewUser { name };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new user");
}

pub fn init() {
    let connection = DbConnection::new().pool.get().unwrap();

    clean_db(&connection);

    use fake::faker::name::raw::*;
    use fake::locales::EN;

    for _ in 1..2000 {
        let name: String = Name(EN).fake();
        create_user(&connection, &name)
    }
}

fn clean_db(conn: &MysqlConnection) {
    diesel::sql_query("TRUNCATE TABLE users")
        .execute(conn)
        .expect(&format!("Couldn't truncate table users"));
}