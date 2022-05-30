create table "compute_result" (
    id serial,
    created timestamp not null default current_timestamp,
    config_json text,
    artifacts_json text,
    primary key (id)
)