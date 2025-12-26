use crate::AppState;
use axum::{ Router, routing::get};
use crate::api::handlers::{get_prices, get_daily_stats};

// Ahora recibe AppState, no PgPool
pub fn app_router(state: AppState) -> Router {
    Router::new()
        .route("/prices", get(get_prices))
        .route("/stats", get(get_daily_stats))
        .with_state(state)
}