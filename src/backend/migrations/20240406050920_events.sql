-- Add migration script here
CREATE TABLE IF NOT EXISTS events
(
    id          TEXT PRIMARY KEY NOT NULL,
    name        TEXT NOT NULL,
    start_time  TIMESTAMP NOT NULL,
    end_time    TIMESTAMP NOT NULL,
    location    TEXT NOT NULL,
    description TEXT NOT NULL,
    price       SMALLINT NOT NULL,
    tags        TEXT NOT NULL,
    source      TEXT NOT NULL
);
