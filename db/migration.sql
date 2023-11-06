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
                                               child_cannon_time Timestamp DEFAULT NULL, -- Null unless the story part is "cannon" -- NOW() + INTERVAL '24 hours'
                                               first UUID REFERENCES story_parts(id), -- optimization for going to the top, not sure if to keep or not
                                               parent UUID REFERENCES story_parts(id), -- used to search for "child" stories
                                               score Int NOT NULL DEFAULT 0, -- cache for upvote - downvote
                                               is_final bool NOT NULL DEFAULT FALSE,
                                               index smallint NOT NULL DEFAULT 0,
                                               child UUID REFERENCES story_parts(id) -- update when a child becomes cannon
    );

CREATE INDEX IF NOT EXISTS idx_cannon_expire ON story_parts(child_cannon_time DESC); -- Index for searching for which expires first
-- CREATE INDEX IF NOT EXISTS idx_parent_part_id ON story_parts(parent);
CREATE INDEX IF NOT EXISTS idx_user_token ON users USING hash(token); -- Helps search for an user with a hash1
CREATE INDEX IF NOT EXISTS idx_parent_part_id ON story_parts USING hash(parent); -- This needs to be tested, to see if hashing has collisions

CREATE INDEX IF NOT EXISTS idx_story_score ON story_parts(score);

CREATE TABLE IF NOT EXISTS comments(
                                    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
                                    writer Text NOT NULL,
                                    body Text NOT NULL,
                                    created Timestamp NOT NULL DEFAULT NOW(),
                                    modified Timestamp DEFAULT NULL,
                                    score Int NOT NULL DEFAULT 0,
                                    snippet UUID NOT NULL REFERENCES story_parts(id)
);

CREATE INDEX IF NOT EXISTS idx_comment_score ON comments(score);

CREATE TABLE IF NOT EXISTS snippet_votes (
                       users Text NOT NULL REFERENCES users(username),
                       snippet Uuid NOT NULL REFERENCES story_parts(id),
                       vote_type smallint NOT NULL, -- 1 for upvote, -1 for downvote
                       created_at TIMESTAMP NOT NULL DEFAULT NOW(),
                       CONSTRAINT pk_snipp_votes PRIMARY KEY (users, snippet)
);

CREATE OR REPLACE FUNCTION update_snippet_score()
    RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        -- Increment or decrement the post score based on the type of vote
        UPDATE story_parts
        SET score = score + NEW.vote_type
        WHERE id = NEW.snippet;
    ELSIF TG_OP = 'DELETE' THEN
        -- Reverse the increment or decrement on vote deletion
        UPDATE story_parts
        SET score = score - OLD.vote_type
        WHERE id = OLD.snippet;
    END IF;

    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER after_vote_operation
    AFTER INSERT OR DELETE ON snippet_votes
    FOR EACH ROW
EXECUTE FUNCTION update_snippet_score();

-- "CANONIZATION QUERY"

-- Call a stored procedure every 5 seconds


CREATE OR REPLACE FUNCTION update_story_parts() RETURNS VOID AS $$
BEGIN
    WITH updated AS (
        UPDATE story_parts p
            SET child = c.id, child_cannon_time = NULL
            FROM (
                SELECT id, parent, score,
                       ROW_NUMBER() OVER (PARTITION BY parent ORDER BY score DESC) AS rnk
                FROM story_parts
                WHERE parent IS NOT NULL
            ) c
            WHERE p.child_cannon_time IS NOT NULL
                AND p.id = c.parent
                AND p.child_cannon_time < NOW()
                AND c.rnk = 1
            RETURNING p.child
    )
    UPDATE story_parts
    SET child_cannon_time = NOW() + interval '24 HOURS'
    FROM updated
    WHERE story_parts.id = updated.child;
END;
$$ LANGUAGE plpgsql;

SELECT cron.schedule('process-updates', '5 minutes', 'CALL update_story_parts()');

-- TODO: Indexes for child_cannon_time, c.parent

CREATE TABLE IF NOT EXISTS email_confirmation (
                                            email Text NOT NULL REFERENCES users(email) PRIMARY KEY,
                                             code Text NOT NULL,
                                             expire Timestamp NOT NULL DEFAULT NOW() + INTERVAL '5 MINUTES'
);

CREATE INDEX IF NOT EXISTS idx_email_code ON email_confirmation using hash(code);
CREATE INDEX IF NOT EXISTS idx_email_email ON email_confirmation using hash(email);
CREATE INDEX IF NOT EXISTS idx_users_email ON users using hash(email);

CREATE TABLE IF NOT EXISTS notifications (
                               id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
                               users Text NOT NULL references users(username),
                               kind smallint NOT NULL,
                               data JSONB NOT NULL,
                               created TIMESTAMP NOT NULL DEFAULT NOW(),
                               read BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE OR REPLACE FUNCTION delete_oldest_notification()
    RETURNS TRIGGER AS $$
BEGIN
    IF (SELECT COUNT(*) FROM notifications) > 50 THEN
        -- Delete the oldest notification based on the created timestamp
        DELETE FROM notifications
        WHERE id = (SELECT id FROM notifications ORDER BY created ASC LIMIT 1);
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_delete_oldest
    AFTER INSERT ON notifications
    FOR EACH ROW
EXECUTE FUNCTION delete_oldest_notification();


-- TODO: comments, caching, "place" system,
