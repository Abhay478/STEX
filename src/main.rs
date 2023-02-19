#![allow(non_snake_case)]
#![recursion_limit = "256"]
use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web::Data, App, HttpServer};
pub mod actix_stex;
pub mod auth_stex;
pub mod diesel_stex;
pub mod schema;
use actix_stex::auth::*;
use actix_stex::handlers::*;
use auth_stex::models::{AppState, Config};
use diesel::{r2d2::ConnectionManager, PgConnection};

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn before() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "debug");
    }
}

fn corses() -> Cors {
    Cors::default()
        .allowed_origin("http://localhost:3000")
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
        ])
        .supports_credentials()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = diesel_stex::connect();
    before();
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                pool: pool.clone(),
                env: Config::init(),
            }))
            .service(register_user_handler)
            .service(login_user_handler)
            .service(logout_handler)
            // .service(hello)
            // .service(echo)
            // .service(hey)
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
            .service(get_question)
            .service(whoami)
            .wrap(corses())
            .wrap(Logger::default())
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
