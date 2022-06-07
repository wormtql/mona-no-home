use rocket::Route;

pub mod routes;


fn get_debug_routes() -> Vec<Route> {
    routes![
        routes::debug_route_create_preset
    ]
}

fn get_common_routes() -> Vec<Route> {
    routes![
        routes::route_get_all_presets
    ]
}

pub fn get_routes() -> Vec<Route> {
    let mut routes = get_common_routes();

    #[cfg(debug_assertions)]
    for r in get_debug_routes() {
        routes.push(r)
    }

    routes
}