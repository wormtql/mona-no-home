use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::preset;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Preset {
    pub id: i32,
    pub created: DateTime<Utc>,
    pub name: String,
    pub config_json: String,
    pub note: Option<String>,
    pub is_dsl: bool,
    pub genre: String,
    pub image: Option<String>
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct PresetMeta {
    pub id: i32,
    pub name: String,
    pub config_json: String,
    pub note: Option<String>,
    pub is_dsl: bool,
    pub genre: String,
    pub image: Option<String>
}

#[derive(Insertable)]
#[table_name="preset"]
pub struct NewPreset<'a> {
    pub name: &'a str,
    pub config_json: &'a str,
    pub note: Option<&'a str>,
    pub is_dsl: bool,
    pub genre: &'a str,
    pub image: Option<&'a str>
}
