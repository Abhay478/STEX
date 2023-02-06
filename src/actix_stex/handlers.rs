

use actix_web::{get, post, web::{self, Json}, HttpResponse, Responder};

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

#[post("/auto/u")]
pub async fn get_names(db: web::Data<Pool>, prefix: String) -> impl Responder {
    let all = crate::diesel_stex::get_all_dnames(&mut db.get().unwrap(), prefix);
    // HttpResponse::Ok().body(all.iter().flat_map(|s| {let mut l = s.as_bytes().to_vec(); l.append(&mut vec![10 as u8]); l}).collect::<Vec<u8>>())
    HttpResponse::Ok().body(serde_json::to_string(&all).unwrap())
}

#[post("/auto/t")]
pub async fn get_tags(db: web::Data<Pool>, prefix: String) -> impl Responder {
    let all = crate::diesel_stex::get_all_tagnames(&mut db.get().unwrap(), prefix);
    // HttpResponse::Ok().body(all.iter().flat_map(|s| {let mut l = s.as_bytes().to_vec(); l.append(&mut vec![10 as u8]); l}).collect::<Vec<u8>>())
    HttpResponse::Ok().body(serde_json::to_string(&all).unwrap())
}

#[post("/auto/p")]
pub async fn get_posts(db: web::Data<Pool>, prefix: String) -> impl Responder {
    let all = crate::diesel_stex::get_all_pnames(&mut db.get().unwrap(), prefix);
    // let all = all.iter().filter(|&mut s| s.is_some()).map(|&mut s| s.take().unwrap()).collect::<Vec<String>>();
    HttpResponse::Ok().body(serde_json::to_string(&all).unwrap())
}

#[get("/auto/u")]
pub async fn get_all_names(db: web::Data<Pool>) -> impl Responder {
    let all = crate::diesel_stex::get_all_dnames(&mut db.get().unwrap(), String::new());
    // HttpResponse::Ok().body(all.iter().flat_map(|s| {let mut l = s.as_bytes().to_vec(); l.append(&mut vec![10 as u8]); l}).collect::<Vec<u8>>())
    HttpResponse::Ok().body(serde_json::to_string(&all).unwrap())
}

#[get("/auto/t")]
pub async fn get_all_tags(db: web::Data<Pool>) -> impl Responder {
    let all = crate::diesel_stex::get_all_tagnames(&mut db.get().unwrap(), String::new());
    // HttpResponse::Ok().body(all.iter().flat_map(|s| {let mut l = s.as_bytes().to_vec(); l.append(&mut vec![10 as u8]); l}).collect::<Vec<u8>>())
    HttpResponse::Ok().body(serde_json::to_string(&all).unwrap())
}

#[get("/auto/p")]
pub async fn get_all_posts(db: web::Data<Pool>) -> impl Responder {
    let all = crate::diesel_stex::get_all_pnames(&mut db.get().unwrap(), String::new());
    // let all = all.iter().filter(|&mut s| s.is_some()).map(|&mut s| s.take().unwrap()).collect::<Vec<String>>();
    HttpResponse::Ok().body(serde_json::to_string(&all).unwrap())
}

#[post("/sr/ptitle")]
pub async fn get_post_by_title(db: web::Data<Pool>, title: String) -> impl Responder {
    let post = crate::diesel_stex::post_search_title(&mut db.get().unwrap(), title);
    HttpResponse::Ok().body(serde_json::to_string(&post).unwrap())
}

#[post("/sr/powner")]
pub async fn get_post_by_owner(db: web::Data<Pool>, oid: Json<i32>) -> impl Responder {
    let post = crate::diesel_stex::post_search_owner(&mut db.get().unwrap(), *oid);
    HttpResponse::Ok().body(serde_json::to_string(&post).unwrap())
}