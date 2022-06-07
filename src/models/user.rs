use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::schema::user;

#[derive(Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub created: NaiveDateTime,
    pub username: String,
    pub pwhash: String,
    pub email: String,
    pub admin: bool,
}

#[derive(Insertable)]
#[table_name="user"]
pub struct NewUser {
    pub username: String,
    pub pwhash: String,
    pub email: String,
    pub admin: bool
}
