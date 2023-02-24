pub mod handlers;
pub mod models;

use diesel::{pg::*, r2d2::ConnectionManager};
use dotenvy::dotenv;
use std::env;

use crate::Pool;

pub fn connect() -> Pool {
    dotenv().ok();

    let url = env::var("DATABASE_URL").unwrap();
    let cn = ConnectionManager::<PgConnection>::new(url);
    println!("Connected to database.");
    r2d2::Pool::builder().build(cn).unwrap()
}
