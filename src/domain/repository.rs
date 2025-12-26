// src/domain/repository.rs

use async_trait::async_trait;
use rust_decimal::Decimal;
use chrono::NaiveDate;
use crate::domain::models::ElectricityPrice;
use crate::utils::error::AppError;

// IMPORTANTE: Necesitamos importar esto para que Axum pueda convertir el struct a JSON
use serde::{Serialize, Deserialize}; 

// EL DTO
// Añade Serialize y Deserialize aquí 👇
#[derive(Debug, Serialize, Deserialize)] 
pub struct DailyStatsDto {
    pub max: Option<Decimal>,
    pub min: Option<Decimal>,
    pub avg: Option<Decimal>,
    pub count: i64,
}

// EL CONTRATO
#[async_trait]
pub trait PriceRepository: Send + Sync {
    async fn save_prices(&self, prices: &[ElectricityPrice]) -> Result<(), AppError>;
    async fn get_last_prices(&self, limit: i32) -> Result<Vec<ElectricityPrice>, AppError>;
    async fn get_stats_by_date(&self, date: NaiveDate) -> Result<DailyStatsDto, AppError>;
}