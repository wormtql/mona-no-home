#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate rocket;

use std::collections::HashMap;
use std::env;

use chrono::Duration;
use dotenv::dotenv;
use figment::Figment;
use figment::providers::{Format, Toml};
use figment::util::map;
use rocket::Config;
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins, CorsOptions};
use serde::Deserialize;

use mona_no_home::common::utils::get_pg_connection;
use mona_no_home::db_pool;
use mona_no_home::fairings::cleanup_expired_repo::CleanupExpiredRepo;
use mona_no_home::fairings::schedule_analysis::ScheduleAnalysisFairing;
use mona_no_home::result_analysis::{get_analysis_result, write_result_to_redis};
use mona_no_home::routes;
use mona_no_home::state::create_repo_count::CreateRepoCount;

embed_migrations!();

#[get("/hello")]
fn hello_route() -> String {
    String::from("hello")
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    // on startup, do db migrations
    let connection = get_pg_connection();
    if let Ok(c) = connection {
        embedded_migrations::run(&c);
    }

    // on startup, do the first time computing result analysis
    // let result = get_analysis_result().unwrap();
    // write_result_to_redis(&result);

    let database_url = env::var("DATABASE_URL").expect("`DATABASE_URL` env is required");
    let rocket_toml = include_str!("../../Rocket.toml");
    let figment = rocket::Config::figment()
        .merge(Toml::string(&rocket_toml).nested())
        .merge(("databases", map!["mona_db" => map!["url" => database_url]]));

    // let cors = CorsOptions {
    //     allowed_origins: AllowedOrigins::All,
    //     // allowed_methods: AllowedMethods::,
    //     allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
    //     allowed_headers: AllowedHeaders::All,
    //     allow_credentials: true,
    //
    //     ..Default::default()
    // }.to_cors()?;

    rocket::custom(figment)
        .attach(db_pool::DBConn::fairing())
        .attach(db_pool::RedisConnFairing)
        // .attach(ScheduleAnalysisFairing)
        .attach(CleanupExpiredRepo)
        .mount("/api", routes::auth::get_routes())
        .mount("/api", routes::feedback::get_routes())
        .mount("/api", routes::compute_result::get_routes())
        .mount("/api", routes::preset::get_routes())
        .mount("/api", routes::repo::get_routes())
        .mount("/api", routes![hello_route])
        .manage(CreateRepoCount::new(Duration::seconds(3)))
}
