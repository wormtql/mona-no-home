use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use diesel::prelude::*;
use crate::common::error::MyStringError;
use crate::common::response_wrapper::ResponseWrapper;
use crate::common::token::decode_login_token;
use crate::db_pool::DBConn;
use crate::models::user::User;

pub struct AdminUserGuard<'a> {
    pub user: &'a User
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUserGuard<'r> {
    type Error = MyStringError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token_header = request.headers().get_one("Authorization");
        if token_header.is_none() {
            #[cfg(debug_assertions)]
            return Outcome::Failure((Status::Unauthorized, MyStringError::from("缺少Header")));
            #[cfg(not(debug_assertions))]
            return Outcome::Failure((Status::Unauthorized, MyStringError::from("需要授权")));
        }

        let token_str = token_header.unwrap();
        let temp: Vec<_> = token_str.split(" ").collect();
        if temp.len() != 2 {
            #[cfg(debug_assertions)]
            return Outcome::Failure((Status::Unauthorized, MyStringError::from("Header格式不正确")));
            #[cfg(not(debug_assertions))]
            return Outcome::Failure((Status::Unauthorized, MyStringError::from("需要授权")));
        }

        let token = temp[temp.len() - 1];
        let login_token = decode_login_token(token);
        if let Err(e) = login_token {
            #[cfg(debug_assertions)]
            return Outcome::Failure((Status::Unauthorized, MyStringError::from(&format!("token不正确：{}", e))));
            #[cfg(not(debug_assertions))]
            return Outcome::Failure((Status::Unauthorized, MyStringError::from("需要授权")));
        }

        let login_token = login_token.unwrap();
        if !login_token.admin {
            #[cfg(debug_assertions)]
            return Outcome::Failure((Status::Unauthorized, MyStringError::from("不是admin")));
            #[cfg(not(debug_assertions))]
            return Outcome::Failure((Status::Unauthorized, MyStringError::from("需要授权")));
        }

        let user_id = login_token.user_id;
        let mut user_query_result = request.local_cache_async(async {
            let db_pool = request.guard::<DBConn>().await.succeeded()?;

            use crate::schema::user::dsl as user_dsl;
            let mut user: Vec<User> = match db_pool.run(move |c| {
                user_dsl::user.filter(user_dsl::id.eq(user_id))
                    .load::<User>(c)
            }).await {
                Err(e) => return None,
                Ok(v) => v
            };

            Some(user.remove(0))
        }).await;

        if user_query_result.is_none() {
            #[cfg(debug_assertions)]
            return Outcome::Failure((Status::Unauthorized, MyStringError::from("数据库错误")));
            #[cfg(not(debug_assertions))]
            return Outcome::Failure((Status::Unauthorized, MyStringError::from("需要授权")));
        }

        Outcome::Success(AdminUserGuard {
            user: user_query_result.as_ref().unwrap(),
        })
    }
}
