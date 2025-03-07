CREATE TABLE IF NOT EXISTS clients
(
    id                   uuid        NOT NULL PRIMARY KEY,
    name                 text        NOT NULL,
    surname              text        NOT NULL,
    passport             text        NOT NULL,
    email                text        NOT NULL,
    password_hash        text        NOT NULL,
    last_password_change timestamptz NOT NULL,
    send_notifications   bool        NOT NULL,
    is_internal          bool        NOT NULL,
    verified             bool        NOT NULL,
    deleted              bool        NOT NULL
);