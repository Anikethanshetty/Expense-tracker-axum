-- Add up migration script here
create extension if not exists "uuid-ossp";

create table users (
    id uuid primary key default uuid_generate_v4(),
    username varchar(255) not null,
    email varchar(255) not null unique,
    password varchar(255) not null,
    created_at timestamptz default now(),
    updated_at timestamptz default now()
);

create index if not exists idx_users_email on users (email);
