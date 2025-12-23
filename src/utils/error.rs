use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    
    #[error("Error de red o HTTP: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Error parseando datos: {0}")]
    ParseError(String),
    
    #[error("Error de base de datos: {0}")]
    DatabaseError(#[from] sqlx::Error),
}