CREATE TABLE users (
    id INTEGER NOT NULL PRIMARY KEY,
    username VARCHAR NOT NULL,
    hashed_password VARCHAR NOT NULL,
    creation_date VARCHAR
)