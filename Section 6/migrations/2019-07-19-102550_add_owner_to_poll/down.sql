-- This file should undo anything in `up.sql`
ALTER TABLE polls
    DROP COLUMN owner;
