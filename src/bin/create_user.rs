use bcrypt::DEFAULT_COST;
use mona_no_home::common::utils::get_pg_connection;
use diesel::prelude::*;
use mona_no_home::models::user::{NewUser, User};

pub fn main() {
    let conn = get_pg_connection();

    use mona_no_home::schema::user;

    let password = "123456";
    let pwhash = bcrypt::hash(password, DEFAULT_COST).unwrap();

    let new_user = NewUser {
        username: String::from("new_user_name"),
        pwhash,
        email: None,
        admin: false
    };
    
    let user: User = diesel::insert_into(user::table)
        .values(&new_user)
        .get_result(&conn)
        .unwrap();
    println!("new user id: {}", user.id);
}