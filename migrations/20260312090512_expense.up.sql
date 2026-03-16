-- Add up migration script here
create extension if not exists "uuid-ossp";

create table if not exists expenses (
    id uuid primary key default uuid_generate_v4(),
    amount NUMERIC(10, 2) not null,
    user_id  uuid not null references users(id) on delete cascade,
    category_id  uuid not null references categories(id) on delete cascade,
    expense_date date not null,
    created_at timestamptz default now(),
    updated_at timestamptz default now()
);

create index if not exists idx_expenses_user_id on expenses (user_id);
create index if not exists idx_expenses_category_id on expenses (category_id);
create index if not exists idx_expenses_expense_date on expenses (expense_date);

