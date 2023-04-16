use super::PoolConnection;
use crate::error::Error;
use axum::http::StatusCode;
use axum::Router;
use axum::{
    extract::State,
    response::{IntoResponse, Json},
    routing::get,
    routing::post,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

pub fn router(state_pool: PoolConnection) -> Router {
    Router::new()
        .route("/", get(get_users))
        .route("/login", post(login))
        .with_state(state_pool)
        .layer(CookieManagerLayer::new())
}

#[derive(Serialize)]
struct GetUsersResponse {
    users: Vec<String>,
}

async fn get_users(State(pool): State<PoolConnection>) -> impl IntoResponse {
    let row = sqlx::query!("SELECT username FROM user_tbl LIMIT 100")
        .fetch_all(&(*pool))
        .await
        .unwrap();

    Json(GetUsersResponse {
        users: row.iter().map(|record| record.username.clone()).collect(),
    })
}

#[derive(Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}

async fn login(
    cookies: Cookies,
    State(pool): State<PoolConnection>,
    payload: Json<LoginPayload>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        "SELECT password_hash, password_salt FROM user_tbl WHERE username = ?",
        payload.username
    )
    .fetch_one(&(*pool))
    .await
    .unwrap();

    // TODO: Add response with session cookie after succesful login
    if !verify_password(
        &payload.password,
        &result.password_hash,
        &result.password_salt,
    ) {
        return StatusCode::UNAUTHORIZED;
    }

    cookies.add(Cookie::new("session-token", generate_csprng()));

    return StatusCode::OK;
}

fn generate_csprng() -> String {
    let mut rng = urandom::csprng();
    let value: i128 = rng.next();

    format!("{:x}", value)
}

fn verify_password(entered_password: &str, db_hashed: &str, db_salt: &str) -> bool {
    verify(format!("{}{}", entered_password, db_salt), db_hashed).unwrap_or(false)
}
