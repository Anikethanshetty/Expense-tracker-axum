-- Add up migration script here
create extension if not exists "uuid-ossp";

create table if not exists categories (
    id uuid primary key default uuid_generate_v4(),
    category_name varchar(255) not null ,
    description varchar(255) not null,
    user_id uuid not null references users(id) on delete cascade,
    created_at timestamptz default now(),
    updated_at timestamptz default now()
);

create index if not exists idx_categories_user_id on categories (user_id);