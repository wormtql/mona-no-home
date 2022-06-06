-- This file should undo anything in `up.sql`
alter table "account"
drop column "artifact_modified",
drop column "preset_modified",
drop column "kumi_modified";
