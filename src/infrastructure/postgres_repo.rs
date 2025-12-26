use async_trait::async_trait;
use sqlx::PgPool;
use chrono::NaiveDate;
use crate::domain::{
    models::{ElectricityPrice, GeoZone},
    repository::{PriceRepository, DailyStatsDto},
};
use crate::utils::error::AppError;

// Esta struct implementará el Trait
#[derive(Clone)]
pub struct PostgresPriceRepository {
    pool: PgPool,
}

impl PostgresPriceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PriceRepository for PostgresPriceRepository {
    async fn save_prices(&self, prices: &[ElectricityPrice]) -> Result<(), AppError> {
        let mut transaction = self.pool.begin().await?;

        for price in prices {
            let zone_str = match price.geo_zone {
                GeoZone::Peninsula => "Peninsula",
                GeoZone::Canarias => "Canarias",
                GeoZone::Baleares => "Baleares",
                GeoZone::Ceuta => "Ceuta",
                GeoZone::Melilla => "Melilla",
            };

            sqlx::query!(
                r#"
                INSERT INTO electricity_prices (datetime, value, geo_zone)
                VALUES ($1, $2, $3)
                ON CONFLICT (datetime, geo_zone) DO NOTHING
                "#,
                price.datetime,
                price.value,
                zone_str
            )
            .execute(&mut *transaction)
            .await?;
        }
        transaction.commit().await?;
        Ok(())
    }

    async fn get_last_prices(&self, limit: i32) -> Result<Vec<ElectricityPrice>, AppError> {
        // Lógica de query movida aquí
        let rows = sqlx::query!(
            r#"SELECT datetime, value, geo_zone FROM electricity_prices ORDER BY datetime DESC LIMIT $1"#,
            limit as i64 
        )
        .fetch_all(&self.pool)
        .await?;

        let prices = rows.into_iter().map(|row| {
             let zone = match row.geo_zone.as_str() {
                "Canarias" => GeoZone::Canarias,
                "Baleares" => GeoZone::Baleares,
                "Ceuta" => GeoZone::Ceuta,
                "Melilla" => GeoZone::Melilla,
                _ => GeoZone::Peninsula,
            };
            ElectricityPrice {
                datetime: row.datetime,
                value: row.value,
                geo_zone: zone,
                meta_tags: None,
            }
        }).collect();

        Ok(prices)
    }

    async fn get_stats_by_date(&self, date: NaiveDate) -> Result<DailyStatsDto, AppError> {
        let stats = sqlx::query!(
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
        .fetch_one(&self.pool)
        .await?;

        Ok(DailyStatsDto {
            max: stats.max_price,
            min: stats.min_price,
            avg: stats.avg_price,
            count: stats.sample_count.unwrap_or(0),
        })
    }
}