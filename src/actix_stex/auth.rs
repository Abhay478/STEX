// use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
// use actix_web_httpauth::extractors::AuthenticationError;
// use actix_web_httpauth::middleware::HttpAuthentication;
// use actix_web::dev::ServiceRequest;
// use actix_web::Error;
// use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
// use futures::TryFutureExt;
// use serde_derive::{Deserialize, Serialize};
// use reqwest::{Response, Request};

// pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
//     let config = req
//         .app_data::<Config>()
//         .map(|data| data.clone())
//         .unwrap_or_else(Default::default);
//     match validate_token(credentials.token()) {
//         Ok(res) => {
//             if res == true {
//                 Ok(req)
//             } else {
//                 Err(AuthenticationError::from(config).into())
//             }
//         }
//         Err(_) => Err(AuthenticationError::from(config).into()),
//     }
// }



// #[derive(Debug, Serialize, Deserialize)]
// struct Claims {
//     sub: String,
//     company: String,
//     exp: usize,
// }

// pub fn validate_token(token: &str) -> Result<bool, ServiceError> {
//     let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");
//     let jwks = fetch_jwks(&format!("{}{}", authority.as_str(), ".well-known/jwks.json"))
//         .expect("failed to fetch jwks");
//     let validations = vec![Validation::Issuer(authority), Validation::SubjectPresent];
//     let kid = match token_kid(&token) {
//         Ok(res) => res.expect("failed to decode kid"),
//         Err(_) => return Err(ServiceError::JWKSFetchError),
//     };
//     let jwk = jwks.find(&kid).expect("Specified key not found in set");
//     let res = validate(token, jwk, validations);
//     Ok(res.is_ok())
// }



// #[derive(Debug, Display)]
// pub enum ServiceError {
//     #[display(fmt = "Internal Server Error")]
//     InternalServerError,

//     #[display(fmt = "BadRequest: {}", _0)]
//     BadRequest(String),

//     #[display(fmt = "JWKSFetchError")]
//     JWKSFetchError,
// }

// // impl ResponseError trait allows to convert our errors into http responses with appropriate data
// impl ResponseError for ServiceError {
//     fn error_response(&self) -> HttpResponse {
//         match self {
//             ServiceError::InternalServerError => {
//                 HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
//             }
//             ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
//             ServiceError::JWKSFetchError => {
//                 HttpResponse::InternalServerError().json("Could not fetch JWKS")
//             }
//         }
//     }
// }

// pub async fn fetch_jwks(uri: &str) -> Result<JWKS, reqwest::Error> {
    // let mut res = reqwest::get(uri).await.unwrap();
    // let val = res.json::<JWKS>()?;
    // return Ok(val);
// }

/* 
use anyhow;
use oauth2::{
    AuthorizationCode,
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    PkceCodeChallenge,
    RedirectUrl,
    Scope,
    TokenResponse,
    TokenUrl
};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use url::Url;

// Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
// token URL.
let client = BasicClient::new(
        ClientId::new("client_id".to_string()),
        Some(ClientSecret::new("client_secret".to_string())),
        AuthUrl::new("http://authorize".to_string())?,
        Some(TokenUrl::new("http://token".to_string())?)
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(RedirectUrl::new("http://redirect".to_string())?);

// Generate a PKCE challenge.
let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

// Generate the full authorization URL.
let (auth_url, csrf_token) = client
    .authorize_url(CsrfToken::new_random)
    // Set the desired scopes.
    .add_scope(Scope::new("read".to_string()))
    .add_scope(Scope::new("write".to_string()))
    // Set the PKCE code challenge.
    .set_pkce_challenge(pkce_challenge)
    .url();

// This is the URL you should redirect the user to, in order to trigger the authorization
// process.
println!("Browse to: {}", auth_url);

// Once the user has been redirected to the redirect URL, you'll have access to the
// authorization code. For security reasons, your code should verify that the `state`
// parameter returned by the server matches `csrf_state`.

// Now you can trade it for an access token.
let token_result = client
    .exchange_code(AuthorizationCode::new("some authorization code".to_string()))
    // Set the PKCE code verifier.
    .set_pkce_verifier(pkce_verifier)
    .request_async(async_http_client)
    .await?;

*/