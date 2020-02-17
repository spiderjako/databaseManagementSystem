CREATE TABLE users(
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    user_type BOOLEAN NOT NULL DEFAULT 'f'
)