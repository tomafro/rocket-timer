-- Your SQL goes here

CREATE TABLE stopwatches (
  id SERIAL PRIMARY KEY,
  identifier VARCHAR NOT NULL,
  name VARCHAR,
  created_at TIMESTAMPTZ DEFAULT current_timestamp,
  updated_at TIMESTAMPTZ DEFAULT current_timestamp
);

CREATE UNIQUE INDEX ON stopwatches (identifier);
SELECT diesel_manage_updated_at('stopwatches');
