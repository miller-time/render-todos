-- Your SQL goes here

CREATE TABLE todos (
    id SERIAL PRIMARY KEY,
    text TEXT NOT NULL DEFAULT '',
    completed BOOLEAN NOT NULL DEFAULT FALSE
)
