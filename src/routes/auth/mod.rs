use rocket::Route;

pub mod routes;
pub mod dto;


pub fn get_routes() -> Vec<Route> {
    routes![routes::login]
}
