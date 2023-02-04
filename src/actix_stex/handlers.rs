use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::PgConnection;

use crate::{Pool, diesel_stex::models::Dummy};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body.to_uppercase())
}

#[get("/toot")]
pub async fn hey() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/dummy/123")]
pub async fn pfft(db: web::Data<Pool>) -> impl Responder {
    let _junk = crate::diesel_stex::push(&mut db.get().unwrap());
    HttpResponse::Ok().body("Done.")
}

#[post("/dummy/123")]
pub async fn meh(db: web::Data<Pool>, req: web::Json<Dummy>) -> impl Responder {
    // let _junk = crate::diesel_stex::accept(&mut db.get().unwrap(), req.0);
    let _junk = crate::diesel_stex::accept_struct(&mut db.get().unwrap(), req.0);
    HttpResponse::Ok().body("Done.")
}