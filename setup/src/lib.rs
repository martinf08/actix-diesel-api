use bcrypt::{DEFAULT_COST, hash};
use db::DbConnection;
use db::models::Role;
use diesel::query_dsl::filter_dsl::FilterDsl;
use diesel::query_dsl::select_dsl::SelectDsl;
use diesel::result::Error;
use diesel::{MysqlConnection, RunQueryDsl, ExpressionMethods};

const ADMIN_USER: &str = "admin";
const ADMIN_PASSWORD: &str = "admin";

pub fn create_user(conn: &MysqlConnection, name: &str, password: &str) {
    use db::schema::users::dsl::users;
    use db::schema::roles::dsl::{id, name as role_name, roles};

    let role_id: Result<i32, Error> = roles
        .filter(role_name.eq("ROLE_ADMIN"))
        .select(id)
        .first(conn);

    let new_user = db::models::NewUser { name, password, role_id: &role_id.unwrap() };

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
    let queries = vec![
        "SET FOREIGN_KEY_CHECKS = 0",
        "TRUNCATE TABLE users",
        "TRUNCATE TABLE roles",
        "SET FOREIGN_KEY_CHECKS = 1",
    ];

    for query in queries {
        diesel::sql_query(query)
            .execute(conn)
            .expect(&format!("Couldn't run {}", query));
    }
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