use actix_web::{
    delete, get, post,
    web::{self},
    HttpResponse, Responder,
};

use crate::{
    auth_stex::{jwt_auth::JwtMiddleware, models::AppState},
    diesel_stex::{handlers::*, models::*},
};

#[get("/auto/u")]
pub async fn get_names(
    state: web::Data<AppState>,
    prefix: web::Query<AutocParams>,
    _: JwtMiddleware,
) -> impl Responder {
    let db = &state.pool;
    let all = get_all_dnames(&mut db.get().unwrap(), &prefix.q);
    HttpResponse::Ok().json(all)
}

#[get("/auto/t")]
pub async fn get_tags(
    state: web::Data<AppState>,
    prefix: web::Query<AutocParams>,
    _: JwtMiddleware,
) -> impl Responder {
    let db = &state.pool;
    let all = get_all_tagnames(&mut db.get().unwrap(), &prefix.q);
    HttpResponse::Ok().json(all)
}

#[get("/auto/p")]
pub async fn get_posts(
    state: web::Data<AppState>,
    prefix: web::Query<AutocParams>,
    _: JwtMiddleware,
) -> impl Responder {
    let db = &state.pool;
    let all = get_all_pnames(&mut db.get().unwrap(), &prefix.q);
    HttpResponse::Ok().json(all)
}

#[get("/search/post_title")]
pub async fn get_post_by_title(
    state: web::Data<AppState>,
    title: web::Query<AutocParams>,
    _: JwtMiddleware,
) -> impl Responder {
    let db = &state.pool;
    let post = post_search_title(&mut db.get().unwrap(), &title.q);
    HttpResponse::Ok().json(post)
}

#[get("/search/post_owner")]
pub async fn get_post_by_owner(
    state: web::Data<AppState>,
    oid: web::Query<AutocParams>,
    _: JwtMiddleware,
) -> impl Responder {
    let db = &state.pool;
    let post = post_search_owner(&mut db.get().unwrap(), oid.q.parse().unwrap_or(-1));
    HttpResponse::Ok().json(post)
}

#[get("/search/post_tag")]
pub async fn get_post_by_tag(
    state: web::Data<AppState>,
    tag: web::Query<AutocParams>,
    _: JwtMiddleware,
) -> impl Responder {
    let db = &state.pool;
    let post = post_search_tags(&mut db.get().unwrap(), &tag.q);
    HttpResponse::Ok().json(post)
}

#[post("{id}/post/new")]
pub async fn insert_post(
    state: web::Data<AppState>,
    mut new: web::Json<NewPost>,
    _idd: web::Path<i32>,
    _: JwtMiddleware,
) -> impl Responder {
    let db = &state.pool;
    let post = new_post(&mut db.get().unwrap(), &mut new.0);
    match post {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(e) => HttpResponse::Ok().json(format!("Can't do that: {}.", e.to_string())),
    }
}

#[post("{id}/post/answer")]
pub async fn answer_to_post(
    state: web::Data<AppState>,
    new: web::Json<AnswerPost>,
    _idd: web::Path<i32>,
    _: JwtMiddleware,
) -> impl Responder {
    let db = &state.pool;
    let post = answer(&mut db.get().unwrap(), &new.0);
    match post {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(e) => HttpResponse::Ok().json(format!("Can't do that: {}.", e.to_string())),
    }
}

#[post("{id}/post/update")]
pub async fn update_post(
    state: web::Data<AppState>,
    new: web::Json<OldPost>,
    idd: web::Path<i32>,
    me: JwtMiddleware,
) -> impl Responder {
    let db = &state.pool;
    if me.user_id != idd.to_string() {
        return HttpResponse::BadRequest().body("Invalid creds.");
    }
    let post = update(&mut db.get().unwrap(), &new.0, &idd);
    match post {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(e) => HttpResponse::NotFound().json(format!("Can't do that: {}.", e.to_string())),
    }
}

#[delete("{id}/post/delete")]
pub async fn delete_post(
    state: web::Data<AppState>,
    kill: web::Query<AutocParamsInt>,
    idd: web::Path<i32>,
    me: JwtMiddleware,
) -> impl Responder {
    let db = &state.pool;
    if me.user_id != idd.to_string() {
        return HttpResponse::BadRequest().body("Invalid creds.");
    }
    let post = delete(&mut db.get().unwrap(), &kill.q, &idd);
    match post {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(e) => HttpResponse::NotFound().json(format!("Can't do that: {}.", e.to_string())),
    }
}

/// Route responds to a get request with struct containing the post corresponding to that id, and all answers to that post.
#[get("/question/{id}")]
pub async fn get_question(state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    use crate::actix_stex::models::Page;
    let db = &state.pool;
    let qn = get_post_by_id(&mut db.get().unwrap(), &id);
    match qn {
        Ok(q) => {
            let out = Page {
                q,
                a: all_answers(&mut db.get().unwrap(), &id).unwrap(),
            };
            HttpResponse::Ok().json(out)
        }
        Err(e) => HttpResponse::NotFound().json(format!("Can't do that: {}.", e.to_string())),
    }
}

#[get("/{id}")]
pub async fn whoami(
    state: web::Data<AppState>, 
    id: web::Path<i32>
) -> impl Responder {
    let db = &state.pool;
    let me = iam(&mut db.get().unwrap(), &id);
    match me {
        Ok(q) => {
            // let out = Page {q, a: all_answers(&mut db.get().unwrap(), &id).unwrap()};
            HttpResponse::Ok().json(q)
        }
        Err(e) => HttpResponse::NotFound().json(format!("Can't do that: {}.", e.to_string())),
    }
}
