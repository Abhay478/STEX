#![allow(non_snake_case)]
#![recursion_limit = "512"]
use actix_cors::Cors;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
pub mod actix_stex;
pub mod auth_stex;
pub mod diesel_stex;
pub mod schema;
use actix_stex::auth::*;
use actix_stex::handlers::*;
use auth_stex::models::{State, Config};
use diesel::{r2d2::ConnectionManager, PgConnection};
// use log::{debug, error, log_enabled, info, Level};

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn before() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "debug");
    }

    env_logger::init();
}

fn corses() -> Cors {
    Cors::default()
        .allowed_origin("http://localhost:3000")
        .allow_any_method()
        .allow_any_header()
        .supports_credentials()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = diesel_stex::connect();
    before();
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(State {
                pool: pool.clone(),
                env: Config::init(),
            }))
            .service(register_user_handler)
            .service(login_user_handler)
            .service(logout_handler)
            .service(get_names)
            .service(get_tags)
            .service(get_qa)
            .service(get_question_by_title)
            .service(get_questions_by_owner)
            .service(get_answers_by_owner)
            .service(get_qa_by_tags)
            .service(ask_question)
            .service(give_answer)
            .service(delete_qa)
            .service(rephrase_qa)
            .service(get_page)
            .service(whoami)
            .service(bio)
            .wrap(corses())
            .wrap(Logger::default())
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
