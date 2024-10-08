// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        #[max_length = 36]
        id -> Varchar,
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
    }
}
