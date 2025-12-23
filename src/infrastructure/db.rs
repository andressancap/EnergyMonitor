use sqlx::{PgPool, Postgres, Pool};
use crate::domain::models::ElectricityPrice;
use crate::utils::error::AppError;
use crate::domain::models::GeoZone;


pub async fn get_database_pool(connection_string: &str) -> Result<Pool<Postgres>, AppError> {
    let pool = PgPool::connect(connection_string).await?; 
    Ok(pool)
}

pub async fn save_prices(pool: &PgPool, prices: &[ElectricityPrice]) -> Result<(), AppError> {
    
    let mut transaction = pool.begin().await?;

    for price in prices {
        // Mapear Enum a String para la BD
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