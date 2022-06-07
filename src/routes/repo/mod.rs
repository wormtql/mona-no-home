use rocket::Route;

pub mod routes;


fn get_common_routes() -> Vec<Route> {
    routes![
        routes::route_create_repo,
        routes::route_get_and_delete,
        routes::route_options_create_repo,
    ]
}

pub fn get_routes() -> Vec<Route> {
    let mut temp = get_common_routes();

    temp
}
