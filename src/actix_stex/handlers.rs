

use actix_web::{get, post, web::{self}, HttpResponse, Responder, delete};

use crate::{Pool, diesel_stex::models::*, auth_stex::{models::AppState, jwt_auth::JwtMiddleware}};

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

#[get("/auto/u")]
pub async fn get_names(state: web::Data<AppState>, prefix: web::Query<AutocParams>, _ : JwtMiddleware) -> impl Responder {
    let db = &state.pool;
    let all = crate::diesel_stex::get_all_dnames(&mut db.get().unwrap(), &prefix.q);
    HttpResponse::Ok().json(all)
}

#[get("/auto/t")]
pub async fn get_tags(state: web::Data<AppState>, prefix: web::Query<AutocParams>, _ : JwtMiddleware) -> impl Responder {
    let db = &state.pool;
    let all = crate::diesel_stex::get_all_tagnames(&mut db.get().unwrap(), &prefix.q);
    HttpResponse::Ok().json(all)
}

#[get("/auto/p")]
pub async fn get_posts(state: web::Data<AppState>, prefix: web::Query<AutocParams>, _ : JwtMiddleware) -> impl Responder {
    let db = &state.pool;
    let all = crate::diesel_stex::get_all_pnames(&mut db.get().unwrap(), &prefix.q);
    HttpResponse::Ok().json(all)
}

#[get("/search/post_title")]
pub async fn get_post_by_title(state: web::Data<AppState>, title: web::Query<AutocParams>, _ : JwtMiddleware) -> impl Responder {
    let db = &state.pool;
    let post = crate::diesel_stex::post_search_title(&mut db.get().unwrap(), &title.q);
    HttpResponse::Ok().json(post)
}

#[get("/search/post_owner")]
pub async fn get_post_by_owner(state: web::Data<AppState>, oid: web::Query<AutocParams>, _ : JwtMiddleware) -> impl Responder {
    let db = &state.pool;
    let post = crate::diesel_stex::post_search_owner(&mut db.get().unwrap(), oid.q.parse().unwrap_or(-1));
    HttpResponse::Ok().json(post)
}

#[get("/search/post_tag")]
pub async fn get_post_by_tag(state: web::Data<AppState>, tag: web::Query<AutocParams>, _ : JwtMiddleware) -> impl Responder {
    let db = &state.pool;
    let post = crate::diesel_stex::post_search_tags(&mut db.get().unwrap(), &tag.q);
    HttpResponse::Ok().json(post)
}

#[post("/post/new")]
pub async fn insert_post(state: web::Data<AppState>, new: web::Json<NewPost>, _ : JwtMiddleware) -> impl Responder {
    let db = &state.pool;
    let post = crate::diesel_stex::new_post(&mut db.get().unwrap(), &new.0);
    match post {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(e) => HttpResponse::Ok().json(format!("Can't do that: {}.", e.to_string()))
    }
}

#[post("/post/answer")]
pub async fn answer_to_post(state: web::Data<AppState>, new: web::Json<AnswerPost>, _ : JwtMiddleware) -> impl Responder {
    let db = &state.pool;
    let post = crate::diesel_stex::answer(&mut db.get().unwrap(), &new.0);
    match post {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(e) => HttpResponse::Ok().json(format!("Can't do that: {}.", e.to_string()))
    }
}

#[post("/post/update")]
pub async fn update_post(state: web::Data<AppState>, new: web::Json<NewPost>, _ : JwtMiddleware) -> impl Responder {
    let db = &state.pool;
    let post = crate::diesel_stex::update(&mut db.get().unwrap(), &new.0);
    match post {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(e) => HttpResponse::NotFound().json(format!("Can't do that: {}.", e.to_string()))
    }
}

#[delete("/post/delete")]
pub async fn delete_post(state: web::Data<AppState>, kill: web::Query<AutocParamsInt>, _ : JwtMiddleware) -> impl Responder {
    let db = &state.pool;
    let post = crate::diesel_stex::delete(&mut db.get().unwrap(), &(kill.q as i32));
    match post {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(e) => HttpResponse::NotFound().json(format!("Can't do that: {}.", e.to_string()))
    }
}
