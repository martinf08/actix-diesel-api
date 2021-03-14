use db::{DbConnection, DbPool};
use diesel::{MysqlConnection, RunQueryDsl, QueryDsl, ExpressionMethods};
use fake::Fake;
use db::models::{User, SlimUser};
use diesel::result::Error;

pub fn create_user(conn: &MysqlConnection, name: &str, password: &str) {
    use db::schema::users::dsl::users;

    let new_user = db::models::NewUser { name, password };

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
    use fake::faker::lorem::raw::*;

    for _ in 1..2000 {
        let name: String = Name(EN).fake();
        let password: String = Word(EN).fake();
        create_user(&connection, &name, &password)
    }
}

fn clean_db(conn: &MysqlConnection) {
    diesel::sql_query("TRUNCATE TABLE users")
        .execute(conn)
        .expect(&format!("Couldn't truncate table users"));
}