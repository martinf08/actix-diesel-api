use db::DbConnection;
use diesel::{MysqlConnection, RunQueryDsl};
use bcrypt::{DEFAULT_COST, hash};
use db::models::Role;

const ADMIN_USER: &str = "admin";
const ADMIN_PASSWORD: &str = "admin";

pub fn create_user(conn: &MysqlConnection, name: &str, password: &str) {
    use db::schema::users::dsl::users;

    let new_user = db::models::NewUser { name, password, role_id: &1 };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new user");
}

pub fn init() {
    let connection = DbConnection::new().pool.get().unwrap();

    clean_db(&connection);

    create_roles(&connection);

    create_user(&connection, ADMIN_USER, &*hash(ADMIN_PASSWORD, DEFAULT_COST).unwrap());
}

fn clean_db(conn: &MysqlConnection) {
    diesel::sql_query("SET FOREIGN_KEY_CHECKS = 0")
        .execute(conn)
        .expect(&format!("Couldn't  FOREIGN_KEY_CHECK"));

    diesel::sql_query("TRUNCATE TABLE users")
        .execute(conn)
        .expect(&format!("Couldn't truncate table users"));

    diesel::sql_query("TRUNCATE TABLE roles")
        .execute(conn)
        .expect(&format!("Couldn't truncate table roles"));

    diesel::sql_query("SET FOREIGN_KEY_CHECKS = 1")
        .execute(conn)
        .expect(&format!("Couldn't  FOREIGN_KEY_CHECK"));
}

fn create_roles(conn: &MysqlConnection) {
    use db::schema::roles::dsl::roles;

    let new_roles = vec![
        Role { id: 1, name: "ROLE_ADMIN".parse().unwrap() },
        Role { id: 2, name: "ROLE_USER".parse().unwrap() },
    ];

    diesel::insert_into(roles)
        .values(&new_roles)
        .execute(conn)
        .expect("Error saving role");
}