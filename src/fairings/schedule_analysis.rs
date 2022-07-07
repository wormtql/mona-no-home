use std::time::Duration;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Build, Data, Orbit, Request, Response, Rocket};
use crate::result_analysis::{get_analysis_result, write_result_to_redis};
use serde_json;
use dotenv::dotenv;
use std::env;

pub struct ScheduleAnalysisFairing;

#[derive(Debug)]
struct Dummy(usize);

impl Drop for Dummy {
    fn drop(&mut self) {
        println!("drop: {}", self.0);
    }
}

#[rocket::async_trait]
impl Fairing for ScheduleAnalysisFairing {
    fn info(&self) -> Info {
        Info {
            name: "Redis connection pool",
            kind: Kind::Ignite
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> rocket::fairing::Result {
        dotenv().ok();

        // let interval_secs = env::var("DAMAGE_ANALYSIS_INTERVAL").unwrap_or(String::from("86400")).parse::<u64>().unwrap();
        let interval_secs = 10;
        let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));

        // do an initial analysis
        // {
        //     let a = match get_analysis_result() {
        //         Ok(v) => v,
        //         Err(_) => return Err(rocket)
        //     };
        //     // let dummy = Dummy(10);
        //     // println!("{:?}", dummy);
        //     write_result_to_redis(&a);
        // }

        tokio::spawn(async move {
            loop {
                // to drop memory
                {
                    let analysis = match get_analysis_result() {
                        Ok(v) => v,
                        Err(e) => {
                            println!("schedule analysis error: {}", e);
                            continue;
                        }
                    };
                    // let dummy = Dummy(11);

                    write_result_to_redis(&analysis);
                    println!("successfully write analysis to redis");
                }

                interval.tick().await;
            }
        });

        Ok(rocket)
    }
}
