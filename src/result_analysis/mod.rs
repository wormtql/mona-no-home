use std::error::Error;
use crate::common::utils::{get_pg_connection, get_redis_connection};
use crate::models::compute_result::{ComputeResult, ComputeResultInDB};
use diesel::prelude::*;
use crate::result_analysis::result_analysis::AnalysisResult;
use serde_json;
use redis::Commands;

pub mod result_analysis;


pub fn get_analysis_result() -> Result<AnalysisResult, Box<dyn Error>> {
    let conn = get_pg_connection()?;

    use crate::schema::compute_result::dsl::*;
    let all_results: Vec<ComputeResultInDB> = compute_result.load(&conn)?;

    let mut temp: Vec<ComputeResult> = Vec::new();
    for item in all_results.iter() {
        if let Some(x) = item.to_compute_result() {
            temp.push(x);
        }
    }

    let analysis_result = result_analysis::result_analysis(&temp);

    Ok(analysis_result)
}

pub fn write_result_to_redis(result: &AnalysisResult) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string(&result)?;

    let mut redis_conn = get_redis_connection()?;

    redis_conn.set("analysis", &json)?;

    Ok(())
}
