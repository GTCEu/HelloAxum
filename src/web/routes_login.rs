use axum::routing::post;
use axum::{Json, Router};
use serde::{de::value, Deserialize};
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use crate::{web, Error, Result};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies,payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("!! {:<12} - api_login", "HANDLER");

    // impl later 
    if payload.username != "tester" || payload.pwd != "account" {
        return Err(Error::LoginFail);
    }

    // impl real token etc
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    // b
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