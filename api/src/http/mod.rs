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

pub struct Config {
    pub database_url: String,
    pub address: SocketAddr,
    pub max_db_connections: u32,
}

#[derive(Serialize)]
struct TestResponse {
    id: String,
    username: String,
}

type Connection = Arc<MySqlPool>;

async fn get_note(State(pool): State<Connection>) -> impl IntoResponse {
    let row = sqlx::query!("SELECT username, id FROM user_tbl")
        .fetch_one(&(*pool))
        .await
        .unwrap();

    Json(TestResponse {
        id: String::from(&row.id),
        username: String::from(&row.username),
    })
}

pub async fn serve(config: Config) {
    let pool = MySqlPoolOptions::new()
        .max_connections(config.max_db_connections)
        .connect(&config.database_url)
        .await
        .unwrap();

    let app = Router::new()
        .route("/", get(get_note))
        .with_state(Arc::new(pool));

    println!("Listening on {}", &config.address);

    Server::bind(&config.address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
