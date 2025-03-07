CREATE TABLE IF NOT EXISTS admins
(
    id                   uuid        NOT NULL PRIMARY KEY,
    email                text        NOT NULL,
    password_hash        text        NOT NULL,
    last_password_change timestamptz NOT NULL,
    deleted              bool        NOT NULL
);