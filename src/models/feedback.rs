use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::feedback;


#[derive(Serialize, Deserialize, Queryable)]
pub struct Feedback {
    pub id: i32,
    pub created: NaiveDateTime,
    pub text: Option<String>
}

#[derive(Insertable)]
#[table_name="feedback"]
pub struct NewFeedback<'a> {
    pub text: &'a str,
}
