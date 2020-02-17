CREATE TABLE appointments(
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    doctor VARCHAR NOT NULL,
    time_of_app VARCHAR NOT NULL
)