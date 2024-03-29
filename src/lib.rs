#![feature(never_type)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
pub mod schema;
pub mod models;
pub mod db_pool;
pub mod routes;
pub mod common;
pub mod guards;
pub mod result_analysis;
pub mod fairings;
pub mod state;
pub mod responder;
