use std::sync::Arc;

use super::{mw, PoolConnection};
use axum::http::StatusCode;
use axum::{
    extract::State,
    response::{IntoResponse, Json},
    routing::get,
    routing::post,
};
use axum::{middleware, Router};
use bcrypt::verify;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

pub fn router(state_pool: PoolConnection) -> Router {
    Router::new()
        .route("/", get(get_users))
        .route_layer(middleware::from_fn_with_state(
            Arc::clone(&state_pool),
            mw::mw_require_auth,
        ))
        .route("/logout", post(logout))
        .route("/login", post(login))
        .route("/revalidate-session", post(revalidate_session))
        .with_state(state_pool)
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

#[derive(Serialize)]
struct LoginResponse {
    session_token: String,
    username: String,
}

async fn login(
    State(pool): State<PoolConnection>,
    payload: Json<LoginPayload>,
) -> Result<(StatusCode, Json<LoginResponse>), StatusCode> {
    let result = sqlx::query!(
        "
        SELECT password_hash, password_salt, id 
        FROM user_tbl 
        WHERE username = ?
        ",
        &payload.username
    )
    .fetch_one(&(*pool))
    .await;

    if result.is_err() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let result = result.unwrap();

    if !verify_password(
        &payload.password,
        &result.password_hash,
        &result.password_salt,
    ) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let session_token = generate_csprng();
    let expiration_date = Utc::now() + Duration::days(30);
    let x = expiration_date.format("%Y-%m-%d %H:%M:%S").to_string();

    sqlx::query!(
        "
        INSERT INTO session_tbl
        VALUES
        (DEFAULT, ?, ?, ?)
        ",
        &session_token,
        &result.id,
        &x
    )
    .execute(&(*pool))
    .await
    .unwrap();

    Ok((
        StatusCode::OK,
        Json(LoginResponse {
            session_token: session_token,
            username: payload.username.clone(),
        }),
    ))
}

#[derive(Deserialize)]
struct LogoutPayload {
    session_token: String,
}

async fn logout(State(pool): State<PoolConnection>, payload: Json<LogoutPayload>) -> StatusCode {
    let result = sqlx::query!(
        "
        DELETE FROM session_tbl
        WHERE token = ?;
        ",
        &payload.session_token
    )
    .execute(&(*pool))
    .await;

    if result.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}

#[derive(Deserialize)]
struct RevalidateSessionPayload {
    session_token: String,
}

#[derive(Serialize)]
struct RevalidateSessionResponse {
    username: String,
}

async fn revalidate_session(
    State(pool): State<PoolConnection>,
    payload: Json<RevalidateSessionPayload>,
) -> Result<(StatusCode, Json<RevalidateSessionResponse>), StatusCode> {
    let new_expiration_date = Utc::now() + Duration::days(30);
    let x = new_expiration_date.format("%Y-%m-%d %H:%M:%S").to_string();

    let result = sqlx::query!(
        "
        UPDATE session_tbl
        SET expiration_date = ?
        WHERE token = ?;
        ",
        &x,
        &payload.session_token
    )
    .execute(&(*pool))
    .await;

    if result.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let record = sqlx::query!(
        "
        SELECT username
        FROM user_tbl
        RIGHT JOIN session_tbl
        ON session_tbl.user_id = user_tbl.id
        WHERE token = ?;
        ",
        &payload.session_token
    )
    .fetch_one(&(*pool))
    .await
    .unwrap();

    Ok((
        StatusCode::OK,
        Json(RevalidateSessionResponse {
            username: record.username.unwrap(),
        }),
    ))
}

fn generate_csprng() -> String {
    let mut rng = urandom::csprng();
    let value: i128 = rng.next();

    format!("{:x}", value)
}

fn verify_password(entered_password: &str, db_hashed: &str, db_salt: &str) -> bool {
    verify(format!("{}{}", entered_password, db_salt), db_hashed).unwrap_or(false)
}
