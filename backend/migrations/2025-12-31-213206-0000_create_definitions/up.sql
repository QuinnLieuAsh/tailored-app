-- Your SQL goes here
CREATE TABLE definitions (
    id SERIAL PRIMARY KEY,
    term VARCHAR NOT NULL,
    formal_def VARCHAR(1000) NOT NULL,
    useful_def VARCHAR(1000) NOT NULL,
    simple_def VARCHAR(1000) NOT NULL,
    date_created TIMESTAMP NOT NULL DEFAULT NOW(),
    date_updated TIMESTAMP NOT NULL DEFAULT NOW()
);