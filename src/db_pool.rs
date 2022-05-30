use dotenv::dotenv;
use redis::Client;
use rocket_sync_db_pools::database;
use std::env;
use std::error::Error;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Build, Data, Orbit, Request, Response, Rocket, State};
use rocket::request::{FromRequest, Outcome};

#[database("mona_db")]
pub struct DBConn(diesel::PgConnection);


pub struct RedisConn {
    pub pool: r2d2::Pool<Client>
}

impl RedisConn {
    pub fn new() -> Result<RedisConn, Box<dyn Error>> {
        dotenv().ok();

        let client_connection_string = env::var("REDIS_CONNECTION")?;
        let client = redis::Client::open(client_connection_string.as_str())?;

        let pool = r2d2::Pool::builder()
            .max_size(15)
            .build(client)?;

        Ok(RedisConn {
            pool
        })
    }
}

pub struct RedisConnFairing;

#[rocket::async_trait]
impl Fairing for RedisConnFairing {
    fn info(&self) -> Info {
        Info {
            name: "Redis connection pool",
            kind: Kind::Ignite
        }
    }
    
    async fn on_ignite(&self, rocket: Rocket<Build>) -> rocket::fairing::Result {
        let conn = match RedisConn::new() {
            Ok(v) => v,
            Err(e) => return Err(rocket)
        };

        Ok(rocket.manage(conn))
    }
}

