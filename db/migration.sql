-- This file does NOT get update the database automatically, nor does it get updated when the db gets updated, so make sure to take care when changing stuff here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp"; -- for uuid_generate_v4
-- TODO check if citext works with sqlx, if not the collation method works fine
CREATE EXTENSION IF NOT EXISTS citext;
CREATE COLLATION IF NOT EXISTS case_insensitive (
    provider = icu,
    locale = 'und-u-ks-level2',
    deterministic = false
);


CREATE TABLE IF NOT EXISTS users (
                                     username Text PRIMARY KEY COLLATE case_insensitive,
                                     email Text NOT NULL UNIQUE,
                                     pw Text NOT NULL,
                                     perm smallint NOT NULL DEFAULT 1,
                                     pw_changed Timestamp NOT NULL DEFAULT NOW(),
                                     token Text UNIQUE,
                                     tok_expire Timestamp NOT NULL DEFAULT NOW() + INTERVAL '7 days',
                                     score int NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS story_parts (
                                           id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
                                           writer Text NOT NULL REFERENCES users(username),
                                           body Text NOT NULL,
                                           created Timestamp NOT NULL DEFAULT NOW(),
                                           modified Timestamp DEFAULT NULL,
                                           child_cannon_time Timestamp DEFAULT NOW() + INTERVAL '24 hours', -- Null unless the story part is "cannon"
                                           first UUID REFERENCES story_parts(id), -- optimization for going to the top, not sure if to keep or not
                                           parent UUID REFERENCES story_parts(id), -- used to search for "child" stories
                                           score Int NOT NULL DEFAULT 0, -- cache for upvote - downvote
                                           child UUID REFERENCES story_parts(id) -- update when a child becomes cannon
);

CREATE INDEX IF NOT EXISTS idx_cannon_expire ON story_parts(child_cannon_time DESC); -- Index for searching for which expires first
-- CREATE INDEX IF NOT EXISTS idx_parent_part_id ON story_parts(parent);
CREATE INDEX IF NOT EXISTS idx_user_token ON users USING hash(token); -- Helps search for an user with a hash1
CREATE INDEX idx_parent_part_id ON story_parts USING hash(parent); -- This needs to be tested, to see if hashing has collisions

CREATE INDEX idx_story_points ON story_parts(points);

CREATE TABLE IF NOT EXISTS comments(
                                    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
                                    writer Text NOT NULL,
                                    body Text NOT NULL,
                                    created Timestamp NOT NULL DEFAULT NOW(),
                                    modified Timestamp DEFAULT NULL,
                                    score Int NOT NULL DEFAULT 0,
                                    snippet UUID NOT NULL REFERENCES story_parts(id)
);

CREATE INDEX idx_comment_points ON comments(points);

-- TODO: Vote system, comments, caching, "place" system,
