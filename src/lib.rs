pub mod api;
pub mod domain;
pub mod infrastructure;
pub mod utils;
use std::sync::Arc;
use crate::domain::repository::PriceRepository;

// Este es el estado que Axum pasará a todos los handlers.
// Usamos Arc<dyn ...> para que sea barato de clonar y dinámico.
#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<dyn PriceRepository>, 
}