use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use crate::AppState; // Usamos nuestro nuevo Estado agnóstico
use crate::domain::{models::ElectricityPrice, repository::DailyStatsDto}; // Importamos los DTOs
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

// --- DTOs ---

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

#[derive(Deserialize)]
pub struct StatsQuery {
    pub date: NaiveDate,
}

// --- HANDLERS ---

// GET /prices
pub async fn get_prices(
    State(state): State<AppState>, // <-- Recibimos AppState, NO PgPool
) -> Result<Json<Vec<ElectricityPrice>>, (StatusCode, Json<ErrorResponse>)> {
    
    // CAMBIO RADICAL:
    // Antes: Hacíamos SQL query aquí.
    // Ahora: Le pedimos al repo que nos dé los datos. No sabemos si vienen de Postgres o Redis.
    let prices = state.repo.get_last_prices(100).await
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: e.to_string() }),
        ))?;

    Ok(Json(prices))
}

// GET /stats
pub async fn get_daily_stats(
    State(state): State<AppState>, // <-- Recibimos AppState
    Query(params): Query<StatsQuery>,
) -> Result<Json<DailyStatsDto>, (StatusCode, Json<ErrorResponse>)> {
    
    let date = params.date;

    // CAMBIO RADICAL:
    // Delegamos el cálculo al repositorio.
    let stats = state.repo.get_stats_by_date(date).await
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: e.to_string() }),
        ))?;

    Ok(Json(stats))
}