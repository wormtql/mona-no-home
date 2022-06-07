-- Your SQL goes here
create table "preset" (
    id serial,
    created timestamptz not null default current_timestamp,
    name varchar(50) not null unique,
    config_json varchar not null,
    note varchar,
    is_dsl boolean not null,
    genre varchar(50) not null,
    image varchar(50),
    primary key (id)
);