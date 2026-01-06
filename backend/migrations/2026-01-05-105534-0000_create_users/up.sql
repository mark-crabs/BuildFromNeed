CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT,
    give_name TEXT,
    email TEXT UNIQUE,
    email_verified BOOLEAN,
    picture TEXT,
    role TEXT NOT NULL CHECK (role IN ('Admin', 'Casual', 'Moderator')),
    archive BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);