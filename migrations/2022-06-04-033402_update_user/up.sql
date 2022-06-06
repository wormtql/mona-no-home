-- Your SQL goes here


alter table "account"
add column "artifact_modified" timestamp with time zone not null default now(),
add column "preset_modified" timestamp with time zone not null default now(),
add column "kumi_modified" timestamp with time zone not null default now();
