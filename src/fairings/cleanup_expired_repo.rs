use std::time::Duration;
use chrono::Utc;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Build, Data, Orbit, Request, Response, Rocket};
use crate::common::utils::get_pg_connection;
use diesel::prelude::*;

pub struct CleanupExpiredRepo;

#[rocket::async_trait]
impl Fairing for CleanupExpiredRepo {
    fn info(&self) -> Info {
        Info {
            name: "Cleanup Expired Repo",
            kind: Kind::Ignite
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> rocket::fairing::Result {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(86400));
            // let mut interval = tokio::time::interval(Duration::from_secs(10));

            loop {
                interval.tick().await;

                {
                    let conn = get_pg_connection();
                    if conn.is_err() {
                        println!("schedule: error: cannot get pg connection");
                        continue;
                    }
                    let conn = conn.unwrap();

                    let result = {
                        use crate::schema::repo::dsl as r;
                        let now = Utc::now();
                        diesel::delete(
                            r::repo.filter(r::expire.lt(now))
                        ).execute(&conn)
                    };

                    let count = match result {
                        Ok(v) => v,
                        Err(e) => {
                            println!("schedule: error: cannot delete entries, {}", e);
                            continue;
                        }
                    };

                    println!("schedule: delete {} expired repos", count);
                }
            }
        });

        Ok(rocket)
    }
}