use crate::{
    actix_stex::models::{Account, AccountID},
    auth_stex::jwt_auth::{self, TokenClaims},
    diesel_stex::{acc_by_id, dupe_acc, makeme},
    AppState,
};
use actix_web::{
    cookie::{time::Duration as AWD, Cookie},
    get, post, web, HttpResponse, Responder,
};
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "JWT Authentication in Rust using Actix-web, Postgres, and Diesel";

    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE}))
}

#[post("/auth/register")]
pub async fn register_user_handler(
    body: web::Json<Account>,
    data: web::Data<AppState>,
) -> impl Responder {
    use crate::actix_stex::models::NewUser;
    // use crate::schema::accounts::dsl::*;
    let db = &mut data.pool.get().unwrap();
    let exists = dupe_acc(db, &body.username);
    if exists {
        return HttpResponse::Conflict().json(
            serde_json::json!({"status": "fail","message": "Doppleganger alert."}),
        );
    }

    let res = makeme(
        db,
        NewUser {
            display_name: body.username.clone(),
            hash: body.password_hash.clone(),
            crnd: chrono::offset::Local::now().naive_utc(),
        },
    );
    match res {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","message": format!("{:?}", e)})),
    }
}

#[post("/auth/login")]
pub async fn login_user_handler(
    body: web::Json<AccountID>,
    data: web::Data<AppState>,
) -> impl Responder {
    let db = &mut data.pool.get().unwrap();

    let query_result = acc_by_id(db, &body.id);

    match &query_result {
        Ok(user) => {
            let othertemp = body.clone().password_hash.unwrap();
            let temp = user.clone().password_hash.unwrap();
            let parsed_hash = PasswordHash::new(temp.as_str()).unwrap();
            let mut is_valid = Argon2::default()
                .verify_password(othertemp.as_bytes(), &parsed_hash)
                .map_or(false, |_| true);

            is_valid = is_valid && user.username == body.username;
            if !is_valid {
                return HttpResponse::BadRequest()
                    .json(json!({"status": "fail", "message": "These are not the droids we are looking for."}));
            }
        }
        Err(_e) => {
            return HttpResponse::BadRequest()
                .json(json!({"status": "fail", "message": "No record."}));
        }
    }

    let user = query_result.unwrap();

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(AWD::new(60 * 60, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success", "token": token}))
}

#[get("/auth/logout")]
pub async fn logout_handler(_: jwt_auth::JwtMiddleware) -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(AWD::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success"}))
}
