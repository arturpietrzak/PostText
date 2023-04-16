use axum::{
    extract::{Path, State},
    response::{IntoResponse, Json},
    routing::get,
    Router, Server,
};
use serde::Serialize;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Serialize)]
struct TestResponse {
    id: String,
    username: String,
}
type Conn = Arc<MySqlPool>;

async fn get_note(State(pool): State<Conn>) -> impl IntoResponse {
    let row = sqlx::query!("SELECT username FROM user_tbl")
        .fetch_one(&(*pool))
        .await
        .unwrap();

    Json(TestResponse {
        id: String::from(&row.username),
        username: String::from(&row.username),
    })
}

#[tokio::main]
async fn main() {
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect("mysql://root:@localhost:3306/post_text")
        .await
        .unwrap();

    let app = Router::new()
        .route("/", get(get_note))
        .with_state(Arc::new(pool));

    let address = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on {address}");

    Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
