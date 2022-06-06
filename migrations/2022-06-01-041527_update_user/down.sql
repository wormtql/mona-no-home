-- This file should undo anything in `up.sql`
alter table "user"
alter column "email" drop not null;

alter table "user"
drop constraint email_unique;
