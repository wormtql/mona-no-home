use rocket::{Request, Response};
use rocket::response::Responder;

pub struct CorsResponder;

impl<'r, 'o: 'r> Responder<'r, 'o> for CorsResponder {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'o> {
        Response::build_from(().respond_to(request)?)
            .raw_header("Access-Control-Allow-Origin", "*")
            .raw_header("Access-Control-Allow-Methods", "POST,GET,OPTIONS")
            .raw_header("Access-Control-Allow-Headers", "*")
            .raw_header("Access-Control-Allow-Credentials", "true")
            .ok()
    }
}