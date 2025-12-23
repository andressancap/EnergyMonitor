use axum::{routing::get, Router};
use sqlx::PgPool;
use crate::api::handlers::{get_prices, get_daily_stats};

pub fn app_router(pool: PgPool) -> Router {
    Router::new()
        .route("/prices", get(get_prices))
        .route("/stats", get(get_daily_stats))
        .with_state(pool) 
}