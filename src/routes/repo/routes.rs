use std::net::SocketAddr;
use chrono::Utc;
use diesel::prelude::*;
use rocket::State;
use crate::db_pool::DBConn;
use crate::models::repo::{NewRepo, Repo, RepoMeta};
use crate::state::create_repo_count::CreateRepoCount;
use nanoid::nanoid;
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::guards::my_socket_addr::MyRemoteAddr;
use crate::responder::CorsResponder;

#[options("/repo/create")]
pub async fn route_options_create_repo() -> CorsResponder {
    CorsResponder
}

#[post("/repo/create", data = "<data>")]
pub async fn route_create_repo(db: DBConn, remote_addr: MyRemoteAddr, data: String, repo_count: &State<CreateRepoCount>) -> Result<Json<RepoMeta>, String> {
    println!("{}", remote_addr.addr);
    if !repo_count.try_ip(remote_addr.addr) {
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
pub async fn route_get_and_delete(db: DBConn, code: String) -> Result<Json<Repo>, rocket::response::status::NotFound<String>> {
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
        // Err(e) => return Err(e.to_string())
        Err(e) => {
            println!("error: {}", e);
            vec![]
        }
    };

    if result.len() == 0 {
        Err(rocket::response::status::NotFound(String::from("不存在")))
    } else {
        Ok(Json(result.pop().unwrap()))
    }
}