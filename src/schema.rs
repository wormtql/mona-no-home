table! {
    account (id) {
        id -> Int4,
        user_id -> Int4,
        created -> Timestamp,
        name -> Varchar,
        uid -> Nullable<Varchar>,
        artifact -> Nullable<Text>,
        preset -> Nullable<Text>,
        kumi -> Nullable<Text>,
        note -> Nullable<Varchar>,
        artifact_modified -> Timestamptz,
        preset_modified -> Timestamptz,
        kumi_modified -> Timestamptz,
    }
}

table! {
    compute_result (id) {
        id -> Int4,
        created -> Timestamp,
        config_json -> Nullable<Text>,
        artifacts_json -> Nullable<Text>,
    }
}

table! {
    feedback (id) {
        id -> Int4,
        created -> Timestamp,
        text -> Nullable<Text>,
    }
}

table! {
    preset (id) {
        id -> Int4,
        created -> Timestamptz,
        name -> Varchar,
        config_json -> Varchar,
        note -> Nullable<Varchar>,
        is_dsl -> Bool,
        genre -> Varchar,
        image -> Nullable<Varchar>,
    }
}

table! {
    repo (id) {
        id -> Int4,
        created -> Timestamptz,
        content -> Varchar,
        expire -> Timestamptz,
        code -> Varchar,
    }
}

table! {
    user (id) {
        id -> Int4,
        created -> Timestamp,
        username -> Varchar,
        pwhash -> Varchar,
        email -> Varchar,
        admin -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    account,
    compute_result,
    feedback,
    preset,
    repo,
    user,
);
