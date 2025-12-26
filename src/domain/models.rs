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

// Función para probar testing
impl ElectricityPrice {
    pub fn is_expensive(&self) -> bool {
        self.value > Decimal::from(150)
    }
}


#[cfg(test)] // Solo compila al correr 'cargo test'
mod tests {
    use super::*; // Importamos todo lo del módulo padre
    use chrono::Utc;
    use rust_decimal::Decimal;

    #[test] // Indica que esta función es un test
    fn test_is_expensive_returns_true_when_high() {
        // Arrange
        let price = ElectricityPrice {
            datetime: Utc::now(),
            value: Decimal::from(200),
            geo_zone: GeoZone::Peninsula,
            meta_tags: None,
        };

        // Act & Assert
        assert!(price.is_expensive(), "Debería ser caro porque es 200");
    }

    #[test]
    fn test_is_expensive_returns_false_when_low() {
        let price = ElectricityPrice {
            datetime: Utc::now(),
            value: Decimal::from(50),
            geo_zone: GeoZone::Peninsula,
            meta_tags: None,
        };

        assert!(!price.is_expensive());
    }
}