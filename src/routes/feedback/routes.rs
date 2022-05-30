use crate::common::error::MyStringError;
use crate::common::response_wrapper::ResponseWrapper;
use crate::db_pool::DBConn;
use crate::guards::admin_user::AdminUserGuard;
use crate::routes::feedback::dto::{CreateFeedbackRequest, FeedbackResponseItem};
use diesel::prelude::*;
use rocket::serde::json::Json;
use crate::models::feedback::{Feedback, NewFeedback};

#[get("/feedbacks")]
pub async fn get_feedbacks(conn: DBConn, admin: Result<AdminUserGuard<'_>, MyStringError>) -> ResponseWrapper<Vec<FeedbackResponseItem>> {
    if admin.is_err() {
        return ResponseWrapper::from_error(&admin.err().unwrap());
    }

    let query_result: Vec<Feedback> = match conn.run(|c| {
        use crate::schema::feedback::dsl::*;

        feedback.load::<Feedback>(c)
    }).await {
        Ok(v) => v,
        Err(e) => return ResponseWrapper::from_error(&e)
    };

    let mut result: Vec<_> = query_result.iter().map(|x| x.into()).collect();


    ResponseWrapper::ok(result)
}

#[post("/feedbacks/create", data = "<input>")]
pub async fn create_feedback(conn: DBConn, input: Json<CreateFeedbackRequest>) -> ResponseWrapper<()> {
    let result = match conn.run(move |c| {
        let new_feedback = NewFeedback {
            text: input.text.as_str()
        };
        diesel::insert_into(crate::schema::feedback::table)
            .values(&new_feedback)
            .execute(c)
    }).await {
        Ok(v) => v,
        Err(e) => return ResponseWrapper::from_error(&e)
    };

    ResponseWrapper::ok(())
}
