-- Add up migration script here
create extension if not exists "uuid-ossp";

create table if not exists budgets (
    id uuid primary key default uuid_generate_v4(),
    amount NUMERIC not null check (amount > 0),
    user_id  uuid not null references users(id) on delete cascade,
    category_id  uuid not null references categories(id) on delete cascade,
    created_at timestamptz default now(),
    updated_at timestamptz default now()
);

create index if not exists idx_budgets_user_id on budgets (user_id);
create index if not exists idx_budgets_category_id on budgets (category_id);    