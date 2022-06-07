-- Your SQL goes here
create table "repo" (
    id serial,
    created timestamptz not null default current_timestamp,
    content varchar not null,
    expire timestamptz not null,
    code varchar(50) not null,
    primary key (id)
)