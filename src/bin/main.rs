#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;

use std::collections::HashMap;
use dotenv::dotenv;
use figment::providers::{Toml, Format};
use rocket::Config;
use mona_no_home::db_pool;
use mona_no_home::routes;
// use diesel_migrations::embed_migrations;
use mona_no_home::common::utils::get_pg_connection;
use mona_no_home::fairings::schedule_analysis::ScheduleAnalysisFairing;
use mona_no_home::result_analysis::{get_analysis_result, write_result_to_redis};
// use diesel_migrations::EmbedMigrations;
use figment::Figment;
use serde::Deserialize;
use figment::util::map;
use std::env;

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
    let figment = rocket::Config::figment()
        .merge(Toml::file("Rocket.toml").nested())
        .merge(("databases", map!["mona_db" => map!["url" => database_url]]));


    rocket::custom(figment)
        .attach(db_pool::DBConn::fairing())
        .attach(db_pool::RedisConnFairing)
        .attach(ScheduleAnalysisFairing)
        .mount("/api", routes::auth::get_routes())
        .mount("/api", routes::feedback::get_routes())
        .mount("/api", routes::compute_result::get_routes())
        .mount("/api", routes![hello_route])
}
