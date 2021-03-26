table! {
    roles (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        role_id -> Integer,
    }
}

joinable!(users -> roles (role_id));

allow_tables_to_appear_in_same_query!(
    roles,
    users,
);
