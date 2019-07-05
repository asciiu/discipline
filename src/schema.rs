table! {
    refresh_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        selector -> Varchar,
        token_hash -> Varchar,
        expires_on -> Timestamp,
    }
}

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

joinable!(refresh_tokens -> users (user_id));

allow_tables_to_appear_in_same_query!(
    refresh_tokens,
    users,
);
