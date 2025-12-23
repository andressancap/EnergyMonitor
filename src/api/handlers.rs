use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;
use crate::domain::models::ElectricityPrice;
use crate::utils::error::AppError;
use crate::domain::models::GeoZone;
use rust_decimal::Decimal;
use chrono::NaiveDate;
use axum::extract::Query;


#[derive(serde::Serialize)]
pub struct ErrorResponse {
    error: String,
}


#[derive(serde::Deserialize)]
pub struct StatsQuery {
    pub date: NaiveDate,
}

#[derive(serde::Serialize, sqlx::FromRow)] 
pub struct DailyStats {
    pub max_price: Option<Decimal>,
    pub min_price: Option<Decimal>,
    pub avg_price: Option<Decimal>,
    pub sample_count: Option<i64>,
}

// Handler: GET /prices
pub async fn get_prices(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<ElectricityPrice>>, (StatusCode, Json<ErrorResponse>)> {
    tracing::info!("Fetching latest electricity prices");
    
    let rows = sqlx::query!(
        r#"
        SELECT datetime, value, geo_zone
        FROM electricity_prices
        ORDER BY datetime DESC
        LIMIT 100
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        // Transformamos el error interno a una respuesta HTTP 500
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: e.to_string() }),
        )
    })?;

    let prices: Vec<ElectricityPrice> = rows.into_iter().map(|row| {

        let zone = match row.geo_zone.as_str() {
            "Peninsula" => GeoZone::Peninsula,
            "Canarias" => GeoZone::Canarias,
            "Baleares" => GeoZone::Baleares,
            "Ceuta" => GeoZone::Ceuta,
            "Melilla" => GeoZone::Melilla,
            _ => GeoZone::Peninsula, // Fallback. Podríamos manejar esto mejor.
        };

        ElectricityPrice {
            datetime: row.datetime,
            value: row.value,
            geo_zone: zone,
            meta_tags: None,
        }
    }).collect();

    Ok(Json(prices))
}


// GET /stats?date=2024-01-01
pub async fn get_daily_stats(
    State(pool): State<PgPool>,
    Query(params): Query<StatsQuery>,
) -> Result<Json<DailyStats>, (StatusCode, Json<ErrorResponse>)> {

    let date = params.date;

    tracing::info!("Calculando estadísticas para: {}", date);

    let stats = sqlx::query_as!(
        DailyStats,
        r#"
        SELECT 
            MAX(value) as max_price,
            MIN(value) as min_price,
            AVG(value) as avg_price,
            COUNT(*) as sample_count
        FROM electricity_prices
        WHERE datetime::date = $1
        "#,
        date
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Error DB: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: "Error calculando estadísticas".into() }),
        )
    })?;

    Ok(Json(stats))
}