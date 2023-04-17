use super::PoolConnection;
use crate::pdt_to_dt;
use axum::extract::State;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use chrono::Utc;
use tower_cookies::Cookies;

pub async fn mw_require_auth<T>(
    State(pool): State<PoolConnection>,
    cookies: Cookies,
    req: Request<T>,
    next: Next<T>,
) -> Result<Response, StatusCode> {
    let session_token = cookies.get("session-token").map(|c| c.value().to_string());

    if session_token.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let result = sqlx::query!(
        "
        SELECT expiration_date 
        FROM session_tbl 
        WHERE token = ?
        ",
        &session_token
    )
    .fetch_one(&(*pool))
    .await;

    if result.is_err() || pdt_to_dt(&result.unwrap().expiration_date) < Utc::now() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}
