use std::net::SocketAddr;
use chrono::Utc;
use diesel::prelude::*;
use rocket::State;
use crate::db_pool::DBConn;
use crate::models::repo::{NewRepo, Repo, RepoMeta};
use crate::state::create_repo_count::CreateRepoCount;
use nanoid::nanoid;
use rocket::serde::json::Json;

#[post("/repo/create", data = "<data>")]
pub async fn route_create_repo(db: DBConn, remote_addr: SocketAddr, data: String, repo_count: &State<CreateRepoCount>) -> Result<Json<RepoMeta>, String> {
    println!("{}", remote_addr);
    if !repo_count.try_ip(remote_addr) {
        return Err(String::from("请稍后再试"));
    }

    let now = Utc::now();
    let expire = now + chrono::Duration::days(1);
    let code = nanoid!();
    // let code2 = code.clone();
    let result = match db.run(move |c| {
        let new_repo = NewRepo {
            content: data,
            created: now,
            expire,
            code
        };

        use crate::schema::repo;
        use crate::schema::repo::dsl as r;
        diesel::insert_into(repo::table).values(&new_repo)
            .returning((r::expire, r::code))
            .get_result::<RepoMeta>(c)
    }).await {
        Err(e) => return Err(e.to_string()),
        Ok(v) => v,
    };

    return Ok(Json(result))
}

#[get("/repo/<code>")]
pub async fn route_get_and_delete(db: DBConn, code: String) -> Result<Json<Repo>, String> {
    let mut result: Vec<Repo> = match db.run(move |c| -> Result<Vec<Repo>, String> {
        use crate::schema::repo::dsl as r;
        let now = Utc::now();
        let result: Vec<Repo> = match r::repo.filter(r::code.eq(&code)).filter(r::expire.gt(now))
            .load::<Repo>(c) {
            Ok(v) => v,
            Err(e) => return Err(e.to_string())
        };

        // if result.len() > 0 {
        //     diesel::delete(r::repo.filter(r::code.eq(&code))).execute(c);
        // }

        Ok(result)
    }).await {
        Ok(v) => v,
        Err(e) => return Err(e.to_string())
    };

    if result.len() == 0 {
        Err(String::from("不存在"))
    } else {
        Ok(Json(result.pop().unwrap()))
    }
}