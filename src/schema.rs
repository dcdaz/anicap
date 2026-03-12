// ############## //
// ANICAP SCHEMA //
// ############# //

table! {
    app_user (id) {
        id -> SmallInt,
        first_name -> Varchar,
        last_name -> Varchar,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        register_at -> Timestamp,
    }
}

table! {
    serie (id) {
        id -> SmallInt,
        user_id -> SmallInt,
        name -> Varchar,
        season -> SmallInt,
        chapter -> SmallInt,
        score -> Float,
        favorite -> Bool,
        wish_to_see -> Bool,
        watch_status -> SmallInt,
    }
}

joinable!(serie -> app_user(user_id));

table! {
    migration(id) {
        id -> SmallInt,
        migration_name -> VarChar,
        migrated_at -> Timestamp,
    }
}
