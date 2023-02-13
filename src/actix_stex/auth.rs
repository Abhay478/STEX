/* 
// // use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
// // use actix_web_httpauth::extractors::AuthenticationError;
// // use actix_web_httpauth::middleware::HttpAuthentication;
// // use actix_web::dev::ServiceRequest;
// // use actix_web::Error;
// // use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
// // use futures::TryFutureExt;
// // use serde_derive::{Deserialize, Serialize};
// // use reqwest::{Response, Request};

// // pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
// //     let config = req
// //         .app_data::<Config>()
// //         .map(|data| data.clone())
// //         .unwrap_or_else(Default::default);
// //     match validate_token(credentials.token()) {
// //         Ok(res) => {
// //             if res == true {
// //                 Ok(req)
// //             } else {
// //                 Err(AuthenticationError::from(config).into())
// //             }
// //         }
// //         Err(_) => Err(AuthenticationError::from(config).into()),
// //     }
// // }



// // #[derive(Debug, Serialize, Deserialize)]
// // struct Claims {
// //     sub: String,
// //     company: String,
// //     exp: usize,
// // }

// // pub fn validate_token(token: &str) -> Result<bool, ServiceError> {
// //     let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");
// //     let jwks = fetch_jwks(&format!("{}{}", authority.as_str(), ".well-known/jwks.json"))
// //         .expect("failed to fetch jwks");
// //     let validations = vec![Validation::Issuer(authority), Validation::SubjectPresent];
// //     let kid = match token_kid(&token) {
// //         Ok(res) => res.expect("failed to decode kid"),
// //         Err(_) => return Err(ServiceError::JWKSFetchError),
// //     };
// //     let jwk = jwks.find(&kid).expect("Specified key not found in set");
// //     let res = validate(token, jwk, validations);
// //     Ok(res.is_ok())
// // }



// // #[derive(Debug, Display)]
// // pub enum ServiceError {
// //     #[display(fmt = "Internal Server Error")]
// //     InternalServerError,

// //     #[display(fmt = "BadRequest: {}", _0)]
// //     BadRequest(String),

// //     #[display(fmt = "JWKSFetchError")]
// //     JWKSFetchError,
// // }

// // // impl ResponseError trait allows to convert our errors into http responses with appropriate data
// // impl ResponseError for ServiceError {
// //     fn error_response(&self) -> HttpResponse {
// //         match self {
// //             ServiceError::InternalServerError => {
// //                 HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
// //             }
// //             ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
// //             ServiceError::JWKSFetchError => {
// //                 HttpResponse::InternalServerError().json("Could not fetch JWKS")
// //             }
// //         }
// //     }
// // }

// // pub async fn fetch_jwks(uri: &str) -> Result<JWKS, reqwest::Error> {
//     // let mut res = reqwest::get(uri).await.unwrap();
//     // let val = res.json::<JWKS>()?;
//     // return Ok(val);
// // }

// /* 
// use anyhow;
// use oauth2::{
//     AuthorizationCode,
//     AuthUrl,
//     ClientId,
//     ClientSecret,
//     CsrfToken,
//     PkceCodeChallenge,
//     RedirectUrl,
//     Scope,
//     TokenResponse,
//     TokenUrl
// };
// use oauth2::basic::BasicClient;
// use oauth2::reqwest::async_http_client;
// use url::Url;

// // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
// // token URL.
// let client = BasicClient::new(
//         ClientId::new("client_id".to_string()),
//         Some(ClientSecret::new("client_secret".to_string())),
//         AuthUrl::new("http://authorize".to_string())?,
//         Some(TokenUrl::new("http://token".to_string())?)
//     )
//     // Set the URL the user will be redirected to after the authorization process.
//     .set_redirect_uri(RedirectUrl::new("http://redirect".to_string())?);

// // Generate a PKCE challenge.
// let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

// // Generate the full authorization URL.
// let (auth_url, csrf_token) = client
//     .authorize_url(CsrfToken::new_random)
//     // Set the desired scopes.
//     .add_scope(Scope::new("read".to_string()))
//     .add_scope(Scope::new("write".to_string()))
//     // Set the PKCE code challenge.
//     .set_pkce_challenge(pkce_challenge)
//     .url();

// // This is the URL you should redirect the user to, in order to trigger the authorization
// // process.
// println!("Browse to: {}", auth_url);

// // Once the user has been redirected to the redirect URL, you'll have access to the
// // authorization code. For security reasons, your code should verify that the `state`
// // parameter returned by the server matches `csrf_state`.

// // Now you can trade it for an access token.
// let token_result = client
//     .exchange_code(AuthorizationCode::new("some authorization code".to_string()))
//     // Set the PKCE code verifier.
//     .set_pkce_verifier(pkce_verifier)
//     .request_async(async_http_client)
//     .await?;

// */
// use anyhow;
// use oauth2::*;
// use oauth2::basic::{BasicClient, BasicTokenType};
// use oauth2::reqwest::async_http_client;
// use oauth2::url::*;
// use actix_web::get;
// use serde_derive::{Serialize, Deserialize};

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Web {
//     client_id: String, // ": "906591928750-3gksa42oad11qt7hlokthhanmb68fknl.apps.googleusercontent.com",
//     project_id: String, //"forward-logic-377503",
//     auth_uri: String, // "https://accounts.google.com/o/oauth2/auth",
//     token_uri: String, // "https://oauth2.googleapis.com/token",
//     auth_provider_x509_cert_url: String, //"https://www.googleapis.com/oauth2/v1/certs",
//     client_secret: String, //"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",
//     redirect_uris: [String; 1], //["https://cqa-auth.com"],
//     javascript_origins: [String; 1]//["https://cqa-auth.com"]
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Secrets {
//     web: Web
// }
// pub async fn authit() -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, anyhow::Error>{
//     // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
//     // token URL.

//     let file = serde_json::from_str::<Secrets>(&*std::fs::read_to_string("secret.json").unwrap()).unwrap().web;
//     let client =
//         BasicClient::new(
//             ClientId::new(file.client_id),
//             Some(ClientSecret::new(file.client_secret)),
//             AuthUrl::new(file.auth_uri)?,
//             Some(TokenUrl::new(file.token_uri)?)
//         )
//         // Set the URL the user will be redirected to after the authorization process.
//         .set_redirect_uri(RedirectUrl::new(file.redirect_uris[0].clone())?);

//     // Generate a PKCE challenge.
//     let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

//     // Generate the full authorization URL.
//     let (auth_url, csrf_token) = client
//         .authorize_url(CsrfToken::new_random)
//         // Set the desired scopes.
//         .add_scope(Scope::new("read".to_string()))
//         .add_scope(Scope::new("write".to_string()))
//         // Set the PKCE code challenge.
//         .set_pkce_challenge(pkce_challenge)
//         .url();

//     // This is the URL you should redirect the user to, in order to trigger the authorization
//     // process.
//     // println!("Browse to: {}", auth_url);

//     // Once the user has been redirected to the redirect URL, you'll have access to the
//     // authorization code. For security reasons, your code should verify that the `state`
//     // parameter returned by the server matches `csrf_state`.

//     // Now you can trade it for an access token.
//     let token_result = client
//         .exchange_code(AuthorizationCode::new("some authorization code".to_string()))
//         // Set the PKCE code verifier.
//         .set_pkce_verifier(pkce_verifier)
//         .request_async(async_http_client)
//         .await?;

//     Ok(token_result)
// }
*/

// use crate::{
//     jwt_auth,
//     model::{LoginUserSchema, RegisterUserSchema, TokenClaims, User},
//     response::FilteredUser,
//     AppState,
// };

use crate::{auth_stex::jwt_auth::{self, TokenClaims}, AppState, actix_stex::models::{Account, AccountID}};
use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{prelude::*, Duration};
use diesel::{QueryDsl, RunQueryDsl, prelude::*};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
// use crate::schema::*;

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "JWT Authentication in Rust using Actix-web, Postgres, and SQLX";

    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE}))
}

#[post("/auth/register")]
async fn register_user_handler(body: web::Json<Account>, data: web::Data<AppState>,) -> impl Responder {
    use crate::schema::accounts::dsl::*;
    let db = &mut data.pool.get().unwrap();
    let exists = !accounts.filter(username.eq(body.username.as_ref())).get_results::<AccountID>(db).unwrap().is_empty();
    if exists {
        return HttpResponse::Conflict().json(
            serde_json::json!({"status": "fail","message": "User with that email already exists"}),
        );
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password_hash.as_ref().unwrap().as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string();

    let res = diesel::insert_into(accounts).values(body.clone()).get_results::<AccountID>(db);
    match res {
        Ok(user) => {
            // let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
            //     "user": filter_user_record(&user)
            // })});

            return HttpResponse::Ok().body("TODO");
        }
        Err(e) => {
            // return HttpResponse::InternalServerError()
            //     .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
            return HttpResponse::InternalServerError().body("TODO");
        }
    }
}

// fn gen_cookie<>(user: &AccountID, data: &'a AppState) -> (Cookie, String) {
    

//     (cookie, token)
// } 

#[post("/auth/login")]
async fn login_user_handler(body: web::Json<AccountID>, data: web::Data<AppState>,) -> impl Responder {
    // let query_result = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", body.email)
    //     .fetch_optional(&data.db)
    //     .await
    //     .unwrap();
    use crate::schema::accounts::dsl::*;
    let db = &mut data.pool.get().unwrap();

    let query_result = accounts.filter(id.eq(body.id)).get_result::<AccountID>(db);
    // let user = query_result.map_or(None, |u| Some(u));
    // let is_valid = query_result.map_or(false, |user| {
    //     let temp = user.password_hash.unwrap();
    //     let othertemp = body.clone().password_hash.unwrap();
    //     let parsed_hash = PasswordHash::new(temp.as_str()).unwrap();
    //     Argon2::default()
    //         .verify_password(othertemp.as_bytes(), &parsed_hash)
    //         .map_or(false, |_| true)
    // });

    

    match &query_result {
        Ok(user) => {
            let othertemp = body.clone().password_hash.unwrap();
            let temp = user.clone().password_hash.unwrap();
            let parsed_hash = PasswordHash::new(temp.as_str()).unwrap();
            let is_valid = Argon2::default()
                .verify_password(othertemp.as_bytes(), &parsed_hash)
                .map_or(false, |_| true);
            if !is_valid {
                return HttpResponse::BadRequest()
                    .json(json!({"status": "fail", "message": "Invalid email or password"}));
            }
        }
        Err(e) => {return HttpResponse::BadRequest().json(json!({"status": "fail", "message": "Invalid email or password"}));}
    }

    let user = query_result.unwrap();

    // let (cookie, token) = gen_cookie(&user, &data);
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
        .max_age(ActixWebDuration::new(60 * 60, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success", "token": token}))
}

#[get("/auth/logout")]
async fn logout_handler(_: jwt_auth::JwtMiddleware) -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success"}))
}

/*
#[get("/users/me")]
async fn get_me_handler(req: HttpRequest, data: web::Data<AppState>, _: jwt_auth::JwtMiddleware,) -> impl Responder {
    let ext = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();

    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(&data.db)
        .await
        .unwrap();

    let json_response = serde_json::json!({
        "status":  "success",
        "data": serde_json::json!({
            "user": filter_user_record(&user)
        })
    });
    

    HttpResponse::Ok().json(json_response)
}
 */
// fn filter_user_record(user: &User) -> FilteredUser {
//     FilteredUser {
//         id: user.id.to_string(),
//         email: user.email.to_owned(),
//         name: user.name.to_owned(),
//         photo: user.photo.to_owned(),
//         role: user.role.to_owned(),
//         verified: user.verified,
//         createdAt: user.created_at.unwrap(),
//         updatedAt: user.updated_at.unwrap(),
//     }
// }

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/")
        .service(health_checker_handler)
        .service(register_user_handler)
        .service(login_user_handler)
        .service(logout_handler);

    conf.service(scope);
}
