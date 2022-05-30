use rocket::Route;

pub mod routes;
pub mod dto;

pub fn get_routes() -> Vec<Route> {
    routes![
        routes::create_compute_result,
        routes::get_compute_result_analysis,
    ]
}