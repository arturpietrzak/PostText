use super::{mw, PoolConnection};
use axum::{
    extract::State,
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_cookies::Cookies;

pub fn router(state_pool: super::PoolConnection) -> Router {
    Router::new()
        .route("/post", post(create_post))
        .route_layer(middleware::from_fn_with_state(
            Arc::clone(&state_pool),
            mw::mw_require_auth,
        ))
        .route("/", put(get_posts))
        .with_state(state_pool)
}

#[derive(Serialize)]
struct Post {
    id: String,
    username: String,
    content: String,
}

#[derive(Deserialize)]
struct GetPostsPayload {
    page: Option<u32>,
}

#[derive(Serialize)]
struct GetPostsResponse {
    has_next: bool,
    posts: Vec<Post>,
}

async fn get_posts(
    State(pool): State<PoolConnection>,
    payload: Json<GetPostsPayload>,
) -> impl IntoResponse {
    let offset = payload.page.unwrap_or(0) * 10;

    let row = sqlx::query!(
        "
        SELECT post_tbl.id, post_tbl.content, user_tbl.username
        FROM post_tbl
        LEFT JOIN user_tbl
        ON post_tbl.user_id = user_tbl.id
        ORDER BY post_tbl.created_at DESC
        LIMIT ?,10
        ",
        offset
    )
    .fetch_all(&(*pool))
    .await
    .unwrap();

    let count = sqlx::query!(
        "
        SELECT COUNT(*) as count FROM post_tbl
        "
    )
    .fetch_one(&(*pool))
    .await
    .unwrap();

    Json(GetPostsResponse {
        has_next: count.count > (offset + 10).into(),
        posts: row
            .iter()
            .map(|record| Post {
                content: record.content.clone().unwrap_or(String::from("")),
                id: record.id.clone(),
                username: record.username.clone().unwrap_or(String::from("")),
            })
            .collect(),
    })
}

#[derive(Deserialize)]
struct CreatePostPayload {
    content: String,
}

async fn create_post(
    State(pool): State<PoolConnection>,
    cookies: Cookies,
    payload: Json<CreatePostPayload>,
) -> Result<StatusCode, StatusCode> {
    let session_token = cookies.get("session-token").map(|c| c.value().to_string());

    if payload.content.len() > 280 {
        return Err(StatusCode::PAYLOAD_TOO_LARGE);
    }

    if session_token.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let row = sqlx::query!(
        "
        SELECT user_tbl.id
        FROM session_tbl
        RIGHT JOIN user_tbl
        ON session_tbl.user_id = user_tbl.id
        "
    )
    .fetch_one(&(*pool))
    .await;

    if row.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    sqlx::query!(
        "
        INSERT INTO post_tbl
        VALUES
        (DEFAULT, DEFAULT, ?, ?)
        ",
        row.unwrap().id,
        &payload.content,
    )
    .execute(&(*pool))
    .await
    .unwrap();

    Ok(StatusCode::OK)
}
