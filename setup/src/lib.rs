use db::DbConnection;
use diesel::{MysqlConnection, RunQueryDsl};
use bcrypt::{DEFAULT_COST, hash};

const ADMIN_USER: &str = "admin";
const ADMIN_PASSWORD: &str = "admin";

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

    create_user(&connection, ADMIN_USER, &*hash(ADMIN_PASSWORD, DEFAULT_COST).unwrap());
}

fn clean_db(conn: &MysqlConnection) {
    diesel::sql_query("TRUNCATE TABLE users")
        .execute(conn)
        .expect(&format!("Couldn't truncate table users"));
}