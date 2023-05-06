-- Add up migration script here

CREATE TABLE error_table (
    id SERIAL PRIMARY KEY,
    error_message TEXT NOT NULL,
    error_data JSONB NOT NULL,
    error_code INT NOT NULL,
    error_category TEXT NOT NULL,
    error_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

COMMENT ON TABLE error_table IS 'Table to store errors';
