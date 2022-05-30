use mona_no_home::common::utils::get_pg_connection;
use mona_no_home::models::compute_result::{ComputeResult, ComputeResultInDB};
use diesel::prelude::*;
use mona_no_home::result_analysis::result_analysis::result_analysis;

fn main() {
    let conn = get_pg_connection().unwrap();

    use mona_no_home::schema::compute_result::dsl::*;
    let all_results: Vec<ComputeResultInDB> = compute_result.load(&conn).unwrap();

    let mut temp: Vec<ComputeResult> = Vec::new();
    for item in all_results.iter() {
        if let Some(x) = item.to_compute_result() {
            temp.push(x);
        }
    }

    let analysis_result = result_analysis(&temp);

    println!("{:?}", analysis_result.character_result);
}
