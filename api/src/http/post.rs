use axum::{extract::State, response::IntoResponse, routing::put, Json, Router};
use serde::{Deserialize, Serialize};

use super::PoolConnection;

pub fn router(state_pool: super::PoolConnection) -> Router {
    Router::new()
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
