use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::State;
use redis::Commands;
use crate::common::response_wrapper::ResponseWrapper;
use crate::db_pool::{DBConn, RedisConn};
use crate::models::compute_result::NewComputeResult;
use crate::result_analysis::result_analysis::AnalysisResult;
use crate::routes::compute_result::dto::CreateComputeResultRequest;

#[post("/compute_result/create", data = "<data>")]
pub async fn create_compute_result(data: Json<CreateComputeResultRequest>, db: DBConn) -> ResponseWrapper<()> {
    let config_json = match serde_json::to_string(&data.config) {
        Ok(v) => v,
        Err(e) => return ResponseWrapper::from_error(&e)
    };

    let result_artifacts = match serde_json::to_string(&data.result_artifacts) {
        Ok(v) => v,
        Err(e) => return ResponseWrapper::from_error(&e)
    };

    let new_compute_result = NewComputeResult {
        config_json,
        artifacts_json: result_artifacts,
    };

    match db.run(move |c| {
        use crate::schema::compute_result;

        diesel::insert_into(compute_result::table)
            .values(&new_compute_result)
            .execute(c)
    }).await {
        Err(e) => return ResponseWrapper::from_error(&e),
        _ => ()
    };

    ResponseWrapper::ok(())
}

#[get("/compute_result/analysis")]
pub async fn get_compute_result_analysis(redis_conn: &State<RedisConn>) -> ResponseWrapper<AnalysisResult> {
    let mut conn = match redis_conn.pool.get() {
        Ok(v) => v,
        Err(e) => return ResponseWrapper::from_error(&e)
    };

    let analysis: String = match (*conn).get("analysis") {
        Ok(v) => v,
        Err(e) => return ResponseWrapper::from_error(&e)
    };

    let result: AnalysisResult = match serde_json::from_str(&analysis) {
        Ok(v) => v,
        Err(e) => return ResponseWrapper::from_error(&e)
    };

    ResponseWrapper::ok(result)
}
