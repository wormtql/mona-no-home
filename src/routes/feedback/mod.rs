use rocket::Route;

pub mod routes;
pub mod dto;

pub fn get_routes() -> Vec<Route> {
    routes![
        routes::get_feedbacks,
        routes::create_feedback
    ]
}