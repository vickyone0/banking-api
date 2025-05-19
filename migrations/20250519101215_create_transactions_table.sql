-- Add migration script here
-- migrations/20230520123456_create_transactions_table.sql
CREATE TYPE transaction_type AS ENUM ('debit', 'credit');

CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    amount DECIMAL(19,4) NOT NULL,
    transaction_type transaction_type NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);