CREATE TABLE users
(
    id SERIAL PRIMARY KEY,
    username VARCHAR UNIQUE NOT NULL,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    email VARCHAR UNIQUE NOT NULL,
    password VARCHAR NOT NULL
);

CREATE TABLE posts
(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    title VARCHAR UNIQUE NOT NULL,
    date TIMESTAMP NOT NULL DEFAULT(NOW()),
    content TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT 'f',
    FOREIGN KEY (user_id) REFERENCES users (id)
);