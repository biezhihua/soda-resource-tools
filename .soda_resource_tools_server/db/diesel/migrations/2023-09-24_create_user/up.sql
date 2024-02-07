CREATE TABLE table_user
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    name       VARCHAR NOT NULL UNIQUE,
    password   VARCHAR NOT NULL,
    permission VARCHAR NOT NULL,
    email      VARCHAR,
    avatar     VARCHAR
);