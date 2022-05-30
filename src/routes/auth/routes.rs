use bcrypt::verify;
use rocket::serde::json::Json;
use crate::common::response_wrapper::{ResponseWrapper};
use crate::db_pool::DBConn;
use crate::routes::auth::dto::{LoginData, LoginResponse};
use diesel::prelude::*;
use crate::common::token::encode_login_token;
use crate::models::user::User;

#[post("/login", data = "<input>")]
pub async fn login(input: Json<LoginData>, conn: DBConn) -> ResponseWrapper<LoginResponse> {
    use crate::schema::user::dsl as user_dsl;

    let input_username = input.username.clone();
    let input_password = &input.password;

    let result: Vec<User> = match conn.run(|c| {
        user_dsl::user.filter(user_dsl::username.eq(input_username))
            .load::<User>(c)
    }).await {
        Ok(v) => v,
        Err(e) => return ResponseWrapper::from_error(&e)
    };

    if result.len() > 1 {
        return ResponseWrapper::err("error");
    }
    if result.len() == 0 {
        return ResponseWrapper::err("用户不存在");
    }

    let user = &result[0];
    let valid = match verify(&input_password, &user.pwhash) {
        Ok(v) => v,
        Err(e) => return ResponseWrapper::from_error(&e)
    };

    if !valid {
        return ResponseWrapper::err("用户名或密码不正确");
    }

    let token = match encode_login_token(user) {
        Ok(v) => v,
        Err(e) => return ResponseWrapper::from_error(&e)
    };

    let response = LoginResponse {
        token
    };

    ResponseWrapper::ok(response)
    // Ok(ResponseWrapper::new(response))
}
