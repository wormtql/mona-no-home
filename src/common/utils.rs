use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::Utc;
use diesel::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn get_current_timestamp() -> usize {
    // let now = SystemTime::now();
    // let since_epoch = now.duration_since(UNIX_EPOCH)?;
    // Ok(since_epoch.as_millis() as usize)
    let utc = Utc::now();
    let time = utc.timestamp();
    time as usize
}

/// get one pg connection without connection pool, this is for some discrete operations
pub fn get_pg_connection() -> Result<PgConnection, Box<dyn Error>> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")?;
    Ok(PgConnection::establish(&db_url)?)
}

/// get one redis connection without connection pool
pub fn get_redis_connection() -> Result<redis::Connection, Box<dyn Error>> {
    dotenv::dotenv().ok();
    let client_connection_string = env::var("REDIS_CONNECTION")?;
    let client = redis::Client::open(client_connection_string.as_str())?;
    let mut con = client.get_connection()?;

    Ok(con)
}
