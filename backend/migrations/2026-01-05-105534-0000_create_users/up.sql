CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    first_name TEXT,
    last_name TEXT,
    email TEXT UNIQUE,
    password TEXT,
    role TEXT NOT NULL CHECK (role IN ('Admin', 'Casual', 'Moderator')),
    registration_option TEXT NOT NULL CHECK (registration_option IN ('Manual', 'Google')),
    archive BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);