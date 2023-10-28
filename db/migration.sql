-- This file does NOT get update the database automatically, nor does it get updated when the db gets updated, so make sure to take care when changing stuff here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp"; -- for uuid_generate_v4
-- TODO check if citext works with sqlx, if not the collation method works fine
CREATE EXTENSION IF NOT EXISTS citext;
CREATE COLLATION IF NOT EXISTS case_insensitive (
    provider = icu,
    locale = 'und-u-ks-level2',
    deterministic = false
);


CREATE TABLE users (
    username Text PRIMARY KEY COLLATE case_insensitive,
    pw Text NOT NULL,
    perm smallint NOT NULL DEFAULT 1,
    pw_changed Timestamp NOT NULL DEFAULT NOW(),
    token Text NOT NULL,
    score int NOT NULL DEFAULT 0
);

CREATE TABLE story_parts (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    writer Text NOT NULL REFERENCES users(username),
    body Text NOT NULL,
    child_cannon_time timestamp DEFAULT NOW() + INTERVAL '24 hours', -- Null unless the story part is "cannon"
    first UUID REFERENCES story_parts(id), -- optimization for going to the top
    parent UUID REFERENCES story_parts(id), -- used to search for "child" stories
    child UUID REFERENCES story_parts(id) -- update when a child becomes cannon
);

CREATE INDEX idx_cannon_expire ON story_parts(child_cannon_time DESC); -- Index for searching for which expires first
CREATE INDEX idx_parent_part_id ON story_parts(parent);
-- CREATE INDEX idx_parent_part_id ON story_parts USING hash(parent); -- This needs to be tested, to see if hashing has collisions

-- TODO: Vote system, comments, caching, "place" system,