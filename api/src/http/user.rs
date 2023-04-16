use super::PoolConnection;
use axum::Router;
use axum::{
    extract::State,
    response::{IntoResponse, Json},
    routing::get,
    routing::post,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};

pub fn router(state_pool: PoolConnection) -> Router {
    Router::new()
        .route("/", get(get_users))
        .route("/login", post(login))
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

async fn login(
    State(pool): State<PoolConnection>,
    payload: Json<LoginPayload>,
) -> impl IntoResponse {
    let hashed = hash("hunter2", DEFAULT_COST).unwrap_or(String::from(""));
    let valid = verify("hunter2", &hashed).unwrap_or(false);

    println!("{} {}", hashed, valid);

    Json(GetUsersResponse { users: vec![] })
}
