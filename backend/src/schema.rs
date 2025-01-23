// @generated automatically by Diesel CLI.

diesel::table! {
    permission (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    user (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 50]
        first_name -> Varchar,
        #[max_length = 50]
        last_name -> Varchar,
        #[max_length = 50]
        username -> Nullable<Varchar>,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        status -> Nullable<Int4>,
        ask_for_new_password -> Int4,
        birth_date -> Nullable<Date>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(permission, user);
