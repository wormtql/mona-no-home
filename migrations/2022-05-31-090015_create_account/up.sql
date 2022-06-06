-- Your SQL goes here
create table "account" (
    id serial,
    user_id integer not null,
    created timestamp not null default current_timestamp,
    name varchar(255) not null,
    uid varchar(32),
    artifact text,
    preset text,
    kumi text,
    note varchar(255),
    primary key (id)
)
