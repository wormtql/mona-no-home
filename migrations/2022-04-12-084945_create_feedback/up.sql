-- Your SQL goes here
CREATE TABLE feedback (
    id serial,
    created timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    text Text,
    PRIMARY KEY (id)
)