-- Add migration script here
CREATE TABLE members(
    id UUID NOT NULL,
    PRIMARY KEY (id),
    firstname TEXT,
    surname TEXT,
    created_at TIMESTAMPTZ NOT NULL
);