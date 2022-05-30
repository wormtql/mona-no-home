-- Your SQL goes here
CREATE TABLE "user" (
    id serial,
    created timestamp not null default current_timestamp,
    username varchar not null,
    pwhash varchar not null,
    email varchar,
    admin boolean not null,
    primary key (id)
)