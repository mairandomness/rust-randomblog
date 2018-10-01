CREATE TABLE users
(
    id SERIAL PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    password VARCHAR NOT NULL
);

CREATE TABLE posts
(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    title VARCHAR NOT NULL,
    content TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT 'f'
);
