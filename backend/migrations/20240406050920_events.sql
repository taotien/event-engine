-- Add migration script here
CREATE TABLE IF NOT EXISTS events
(
    id          INTEGER PRIMARY KEY NOT NULL,
    name        TEXT NOT NULL,
    start_time  TIMESTAMP NOT NULL,
    end_time    TIMESTAMP NOT NULL,
    location    TEXT NOT NULL,
    description TEXT NOT NULL,
    price       INTEGER NOT NULL,
    tags        TEXT NOT NULL,
    source      TEXT NOT NULL
    start_time  TEXT,
    end_time    TEXT,
    location    TEXT,
    description TEXT,
    price       TEXT,
    tags        TEXT,
    source      TEXT,
    check_list  TEXT
);
