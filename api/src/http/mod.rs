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
    users: Vec<String>,
}

type Connection = Arc<MySqlPool>;

async fn get_test(State(pool): State<Connection>) -> impl IntoResponse {
    let row = sqlx::query!("SELECT username FROM user_tbl")
        .fetch_all(&(*pool))
        .await
        .unwrap();

    Json(TestResponse {
        users: row.iter().map(|record| record.username.clone()).collect(),
    })
}

pub async fn serve(config: Config) {
    let pool = MySqlPoolOptions::new()
        .max_connections(config.max_db_connections)
        .connect(&config.database_url)
        .await
        .unwrap();

    let app = Router::new()
        .route("/test", get(get_test))
        .with_state(Arc::new(pool));

    println!("Listening on {}", &config.address);

    Server::bind(&config.address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
