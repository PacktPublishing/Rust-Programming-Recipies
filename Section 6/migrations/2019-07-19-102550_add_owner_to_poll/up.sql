-- Your SQL goes here
ALTER TABLE polls
    ADD COLUMN owner INTEGER REFERENCES users (id);
