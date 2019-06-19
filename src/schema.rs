table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        email_verified -> Bool,
        username -> Varchar,
        password_hash -> Varchar,
        created_on -> Timestamp,
        updated_on -> Timestamp,
        deleted_on -> Nullable<Timestamp>,
    }
}
