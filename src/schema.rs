// @generated automatically by Diesel CLI.

diesel::table! {
    groups (id) {
        id -> Int4,
        #[max_length = 128]
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 64]
        username -> Varchar,
        #[max_length = 128]
        password -> Varchar,
        #[max_length = 128]
        email -> Varchar,
        #[max_length = 256]
        first_name -> Nullable<Varchar>,
        #[max_length = 256]
        last_name -> Nullable<Varchar>,
        is_active -> Bool,
        is_staff -> Bool,
        is_superuser -> Bool,
        last_login -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users_groups (id) {
        id -> Int4,
        user_id -> Int4,
        group_id -> Int4,
    }
}

diesel::joinable!(users_groups -> groups (group_id));
diesel::joinable!(users_groups -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    groups,
    users,
    users_groups,
);
