use axum::Router;

pub fn router(state_pool: super::PoolConnection) -> Router {
    Router::new().with_state(state_pool)
}
