use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::repo;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Repo {
    pub id: i32,
    pub created: DateTime<Utc>,
    pub content: String,
    pub expire: DateTime<Utc>,
    pub code: String,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct RepoMeta {
    pub expire: DateTime<Utc>,
    pub code: String
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "repo"]
pub struct NewRepo {
    pub content: String,
    pub created: DateTime<Utc>,
    pub expire: DateTime<Utc>,
    pub code: String,
}
