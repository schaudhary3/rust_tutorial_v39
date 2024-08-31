// @generated automatically by Diesel CLI.

diesel::table! {
    _sqlx_migrations (version) {
        version -> Int8,
        description -> Text,
        installed_on -> Timestamptz,
        success -> Bool,
        checksum -> Bytea,
        execution_time -> Int8,
    }
}

diesel::table! {
    items (uuid) {
        uuid -> Int4,
        name -> Nullable<Varchar>,
        number -> Nullable<Int4>,
    }
}

diesel::table! {
    my_table (uuid) {
        uuid -> Int4,
        name -> Nullable<Char>,
        number -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    _sqlx_migrations,
    items,
    my_table,
    users,
);
