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
    user (id) {
        id -> Int4,
        created -> Timestamp,
        username -> Varchar,
        pwhash -> Varchar,
        email -> Nullable<Varchar>,
        admin -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    compute_result,
    feedback,
    user,
);
