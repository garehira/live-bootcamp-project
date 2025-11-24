-- Add up migration script here
-- sudo systemctl stop postgresql22
CREATE TABLE IF NOT EXISTS users
(
    email
    TEXT
    NOT
    NULL
    PRIMARY
    KEY,
    password_hash
    TEXT
    NOT
    NULL,
    requires_2fa
    BOOLEAN
    NOT
    NULL
    DEFAULT
    FALSE
);