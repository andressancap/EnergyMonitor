use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum GeoZone {
    Peninsula,
    Canarias,
    Baleares,
    Ceuta,
    Melilla,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ElectricityPrice {

    pub datetime: DateTime<Utc>,

    pub value: Decimal,

    pub geo_zone: GeoZone,

    pub meta_tags: Option<String>, 
}