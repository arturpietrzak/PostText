use axum::{Router, Server};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;

mod mw;
mod post;
mod user;

pub type PoolConnection = Arc<MySqlPool>;

pub struct Config {
    pub database_url: String,
    pub address: SocketAddr,
    pub max_db_connections: u32,
}

pub async fn serve(config: Config) {
    let state_pool = Arc::new(
        MySqlPoolOptions::new()
            .max_connections(config.max_db_connections)
            .connect(&config.database_url)
            .await
            .unwrap(),
    );

    let app = api_router(state_pool);

    println!("Listening on {}", &config.address);

    Server::bind(&config.address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn api_router(state_pool: Arc<MySqlPool>) -> Router {
    Router::new()
        .nest("/user", user::router(Arc::clone(&state_pool)))
        .nest("/post", post::router(Arc::clone(&state_pool)))
        .layer(CorsLayer::permissive())
        .layer(CookieManagerLayer::new())
}
