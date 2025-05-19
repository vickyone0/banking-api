-- Add migration script here

ALTER TABLE transactions
    ALTER COLUMN amount TYPE BIGINT
    USING (amount::BIGINT);