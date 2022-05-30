use std::time::Duration;
use tokio::time;
use tokio_cron_scheduler::{JobScheduler, JobToRun, Job};
use tokio::time::Interval;

#[tokio::main]
async fn main() {
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(5));
        loop {
            interval.tick().await;
            println!("123");
        }
    }).await;
}
