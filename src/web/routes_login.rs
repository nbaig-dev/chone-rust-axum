use axum::{Json, Router};
use axum::routing::{post};
use serde::Deserialize;
use serde_json::{json, Value};
use crate::{Error, Result, web};
use tower_cookies::{Cookies, Cookie};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    //TODO: Implement real db/auth logic
    if payload.username != "demo1" || payload.pwd != "welcome" {
        println!("->> {:<12} - api_login", "HANDLER");
        return Err(Error::LoginFail);
    }

    // FIXME: Implement real auth-token generation/signature.
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    //TODO: Set cookies

    // Create the success body.
    let body = Json(json!({
        "result": {
            "success": true,
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}