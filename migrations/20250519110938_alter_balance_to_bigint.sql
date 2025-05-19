-- Add migration script here

ALTER TABLE account_balances
    ALTER COLUMN balance TYPE BIGINT
    USING (balance::BIGINT);