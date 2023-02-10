#![allow(non_snake_case)]
#![recursion_limit = "256"]
use actix_web::{HttpServer, App, web::Data};

pub mod diesel_stex;
pub mod actix_stex;
pub mod schema;
use actix_stex::handlers::*;
use diesel::{PgConnection, r2d2::ConnectionManager};

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = diesel_stex::connect();
    HttpServer::new(move || {
        App::new().app_data(Data::new(pool.clone()))
            .service(hello)
            .service(echo)
            .service(hey)
            .service(get_names)
            .service(get_tags)
            .service(get_posts)
            .service(get_post_by_title)
            .service(get_post_by_owner)
            .service(get_post_by_tag)
            .service(insert_post)
            .service(answer_to_post)
            .service(delete_post)
            .service(update_post)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}