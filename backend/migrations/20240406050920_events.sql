-- Add migration script here
CREATE TABLE IF NOT EXISTS events
(
    id          INTEGER PRIMARY KEY NOT NULL,
    name        TEXT NOT NULL,
    start_time  TEXT,
    end_time    TEXT,
    location    TEXT,
    description TEXT,
    price       TEXT,
    tags        TEXT,
    source      TEXT,
    check_list  TEXT
);
