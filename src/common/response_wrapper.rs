use std::error::Error;
use rocket::Request;
use rocket::response::Responder;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Debug)]
pub struct ResponseWrapper<T: Serialize> {
    pub data: Option<T>,
    pub msg: Option<String>,
    pub success: bool,
}

impl<'r, 'o: 'r, T: Serialize> Responder<'r, 'o> for ResponseWrapper<T> {
    fn respond_to(self, request: &'r Request) -> rocket::response::Result<'o> {
        Json(self).respond_to(request)
    }
}

impl<T: Serialize> ResponseWrapper<T> {
    pub fn ok(data: T) -> ResponseWrapper<T> {
        ResponseWrapper {
            success: true,
            data: Some(data),
            msg: None,
        }
    }

    pub fn err(msg: &str) -> ResponseWrapper<T> {
        ResponseWrapper {
            success: false,
            data: None,
            msg: Some(String::from(msg))
        }
    }

    pub fn from_error<U: Error>(e: &U) -> ResponseWrapper<T> {
        ResponseWrapper {
            success: false,
            data: None,
            msg: Some(String::from(&e.to_string()))
        }
    }
}
