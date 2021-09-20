-- Add migration script here
drop table if exists users;
create table users
(
    id             serial primary key,
    username       varchar     not null unique,
    email          varchar     not null unique,
    password_hash  varchar     not null,
    nickname       varchar     not null,
    bio            varchar null,
    image          varchar null,
    email_verified boolean     not null default false,
    active         boolean     not null default true,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT current_timestamp
);
