use actix_web::{get, post, web::{self, Json}, App, HttpResponse, HttpServer, Responder, body::{BodyStream, BoxBody}};
use diesel::PgConnection;

use crate::{Pool, diesel_stex::models::{Dummy, User}};

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

#[get("auto/u")]
pub async fn get_names(db: web::Data<Pool>) -> impl Responder {
    let all = crate::diesel_stex::get_all_dnames(&mut db.get().unwrap()).iter().map(|s| Json(s.to_string())).collect::<Vec<Json<String>>>();
    HttpResponse::Ok().body(all.iter().flat_map(|s| {let mut l = s.as_bytes().to_vec(); l.append(&mut vec![10 as u8]); l}).collect::<Vec<u8>>())
}

#[get("auto/t")]
pub async fn get_tags(db: web::Data<Pool>) -> impl Responder {
    let all = crate::diesel_stex::get_all_tagnames(&mut db.get().unwrap()).iter().map(|s| Json(s.to_string())).collect::<Vec<Json<String>>>();
    HttpResponse::Ok().body(all.iter().flat_map(|s| {let mut l = s.as_bytes().to_vec(); l.append(&mut vec![10 as u8]); l}).collect::<Vec<u8>>())
}