CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    body TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT 'f'
);

CREATE TABLE users (
    username TEXT PRIMARY KEY NOT NULL,
    passwd TEXT NOT NULL
);
