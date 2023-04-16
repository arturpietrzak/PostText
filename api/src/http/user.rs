use super::PoolConnection;
use axum::Router;
use axum::{
    extract::State,
    response::{IntoResponse, Json},
    routing::get,
};
use serde::Serialize;

pub fn router(state_pool: PoolConnection) -> Router {
    Router::new()
        .route("/", get(get_users))
        .with_state(state_pool)
}

#[derive(Serialize)]
struct TestResponse {
    users: Vec<String>,
}

async fn get_users(State(pool): State<PoolConnection>) -> impl IntoResponse {
    let row = sqlx::query!("SELECT username FROM user_tbl LIMIT 100")
        .fetch_all(&(*pool))
        .await
        .unwrap();

    Json(TestResponse {
        users: row.iter().map(|record| record.username.clone()).collect(),
    })
}
