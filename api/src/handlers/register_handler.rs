use actix_web::{web, HttpResponse};
use bcrypt::{hash, DEFAULT_COST};
use db::models::{PartialUser, SlimUser, User};
use db::DbPool;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/create").route(web::post().to(register)));
}

pub async fn register(slim_user: web::Json<SlimUser>, pool: web::Data<DbPool>) -> HttpResponse {
    use db::schema::roles::dsl::{id, name as role_name, roles};
    use db::schema::users::dsl::{name as user_name, users};

    let conn = &pool.get().unwrap();

    let existing_user: QueryResult<User> = users.filter(user_name.eq(&*slim_user.name)).first(conn);

    match existing_user {
        Ok(user) => HttpResponse::Ok().json(format!("User {} already exists", user.name.unwrap())),
        _err => {
            let role_id: QueryResult<i32> = roles
                .filter(role_name.eq("ROLE_USER"))
                .select(id)
                .first(conn);

            let new_user = PartialUser {
                name: slim_user.name.clone(),
                password: hash(slim_user.password.clone(), DEFAULT_COST).unwrap(),
                role_id: role_id.unwrap(),
            };

            diesel::insert_into(users)
                .values(&new_user)
                .execute(conn)
                .expect("Error saving new user");
            HttpResponse::Ok().json({ "Done" })
        }
    }
}
