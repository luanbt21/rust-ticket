use crate::{web, Error, Result};
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLED");

    if payload.username != "user1" || payload.pwd != "pass1" {
        return Err(Error::LoginFail);
    }

    let cookie = Cookie::build((web::AUTH_TOKEN, "user-1.exp.sign"))
        .http_only(true)
        .path("/")
        .build();
    cookies.add(cookie);

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
