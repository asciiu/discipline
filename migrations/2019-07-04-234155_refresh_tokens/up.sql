-- Your SQL goes here
-- REMEMBER ME TOKENS
CREATE TABLE refresh_tokens(
  id UUID PRIMARY KEY, 
  user_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
  selector VARCHAR UNIQUE NOT NULL,
  token_hash VARCHAR NOT NULL,
  expires_on TIMESTAMP NOT NULL
);