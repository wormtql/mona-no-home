use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::models::feedback::Feedback;

#[derive(Serialize)]
pub struct FeedbackResponseItem {
    pub id: i32,
    pub created: NaiveDateTime,
    pub text: Option<String>
}

impl From<&Feedback> for FeedbackResponseItem {
    fn from(a: &Feedback) -> Self {
        FeedbackResponseItem {
            id: a.id,
            created: a.created.clone(),
            text: a.text.clone()
        }
    }
}

// #[derive(Serialize)]
// pub struct CreateFeedbackResponse {
//     pub text: String,
// }

#[derive(Deserialize)]
pub struct CreateFeedbackRequest {
    pub text: String,
}
