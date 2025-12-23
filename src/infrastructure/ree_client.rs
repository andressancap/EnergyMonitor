use crate::domain::models::{ElectricityPrice, GeoZone};
use crate::utils::error::AppError;
use rust_decimal::Decimal;
use serde::Deserialize;
use chrono::{DateTime, Utc};


// { "included": [ { "attributes": { "values": [ { "value": 12.3, "datetime": "..." } ] } } ] }
#[derive(Deserialize, Debug)]
struct ReeResponseDto {
    included: Vec<ReeIncludedDto>,
}

#[derive(Deserialize, Debug)]
struct ReeIncludedDto {
    attributes: ReeAttributesDto,
}

#[derive(Deserialize, Debug)]
struct ReeAttributesDto {
    values: Vec<ReeValueDto>,
}

#[derive(Deserialize, Debug)]
struct ReeValueDto {
    value: f64,
    datetime: DateTime<Utc>,
}

#[derive(Clone)]
pub struct ReeClient {
    base_url: String,
    http_client: reqwest::Client,
}

impl ReeClient {

    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn get_spot_prices(&self) -> Result<Vec<ElectricityPrice>, AppError> {

        let url = "https://apidatos.ree.es/es/datos/mercados/precios-mercados-tiempo-real?start_date=2024-01-01T00:00&end_date=2024-01-01T23:59&time_trunc=hour";

        let response = self.http_client
            .get(url)
            .header("User-Agent", "EnergyMonitor/1.0")
            .header("Accept", "application/json")
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            tracing::error!("La API falló. Status: {}. Body: {}", status, error_text);
            return Err(AppError::ParseError(format!("API Error {}: {}", status, error_text)));
        }

        let dto: ReeResponseDto = response.json().await?;

        let mut domain_prices = Vec::new();

        for item in dto.included {
            for val in item.attributes.values {
                
                let price_value = Decimal::from_f64_retain(val.value)
                    .unwrap_or(Decimal::ZERO); 

                domain_prices.push(ElectricityPrice {
                    datetime: val.datetime,
                    value: price_value,
                    geo_zone: GeoZone::Peninsula, 
                    meta_tags: None,
                });
            }
        }
        
        Ok(domain_prices)
    }
}