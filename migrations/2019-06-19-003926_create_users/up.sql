-- Your SQL goes here
CREATE TABLE users(
    id uuid PRIMARY KEY,
    email VARCHAR UNIQUE NOT NULL,
    email_verified BOOLEAN NOT NULL DEFAULT false,
    username VARCHAR UNIQUE NOT NULL, 
    password_hash VARCHAR NOT NULL,
    created_on TIMESTAMP NOT NULL DEFAULT now(),
    updated_on TIMESTAMP NOT NULL DEFAULT current_timestamp,
    deleted_on TIMESTAMP
);