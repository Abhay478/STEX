use actix_web::{
    delete, get, post,
    web::{self},
    HttpResponse, Responder,
};

use crate::{
    auth_stex::{jwt_auth::JwtMiddleware, models::AppState},
    diesel_stex::{handlers::*, models::*},
};

/// Autocomplete for users: Provide query thus: "/auto/u?q=<prefix>"
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

/// Autocomplete for tags: Provide query thus: "/auto/t?q=<prefix>"
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

/// Autocomplete for posts: Provide query thus: "/auto/p?q=<prefix>"
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

/// Post data dump: Provide query thus: "/search/post_title?q=<title>"
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


/// Post data dump: Provide query thus: "/search/post_owner?q=<owner_user_id>". If not provided, defaults to -1 (Community).
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

/// Post data dump: Provide query thus: "/search/post_owner?q=<tag_name>".
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

/// Post body requires owner id (same as in path), title, tags and body and date-time optional.
#[post("{id}/post/new")]
pub async fn insert_post(
    state: web::Data<AppState>,
    mut new: web::Json<NewPost>,
    idd: web::Path<i32>,
    _: JwtMiddleware,
) -> impl Responder {
    let db = &state.pool;
    let post = new_post(&mut db.get().unwrap(), &mut new.0, &idd);
    match post {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(e) => HttpResponse::Ok().json(format!("Can't do that: {}.", e.to_string())),
    }
}

/// Post body requires owner id (same as in path), title, tags and body and parent post (question) id.
#[post("{id}/post/answer")]
pub async fn answer_to_post(
    state: web::Data<AppState>,
    new: web::Json<AnswerPost>,
    idd: web::Path<i32>,
    _: JwtMiddleware,
) -> impl Responder {
    let db = &state.pool;
    let post = answer(&mut db.get().unwrap(), &new.0, &idd);
    match post {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(e) => HttpResponse::Ok().json(format!("Can't do that: {}.", e.to_string())),
    }
}

/// Post body requires post id (not same as in path), title, tags and body.
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

/// Query parameter thus: "{id}/post/delete/q=<id>"
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

#[get("/me")]
pub async fn whoami(
    state: web::Data<AppState>, 
    me: JwtMiddleware
) -> impl Responder {
    let db = &state.pool;
    // let me = ;
    // match me {
    //     Ok(q) => {
    //         // let out = Page {q, a: all_answers(&mut db.get().unwrap(), &id).unwrap()};
    //         HttpResponse::Ok().json(q)
    //     }
    //     Err(e) => HttpResponse::NotFound().json(format!("Can't do that: {}.", e.to_string())),
    // }

    HttpResponse::Ok().json(iam(&mut db.get().unwrap(), &me.user_id.parse().unwrap()).unwrap())
}


// to update about_me
#[post("/{id}/bio")]
pub async fn bio(state: web::Data<AppState>, id: web::Path<i32>, new: String) -> impl Responder {
    let db = &state.pool;
    let res = make_bio(&mut db.get().unwrap(), &new, &id);

    HttpResponse::Ok().json(res.unwrap())
}

