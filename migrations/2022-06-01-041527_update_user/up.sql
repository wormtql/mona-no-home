-- Your SQL goes here
alter table "user"
alter column "email" set not null;

alter table "user"
add constraint email_unique unique (email);