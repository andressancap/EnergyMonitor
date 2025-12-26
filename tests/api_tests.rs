use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt; // Nos da el método .oneshot()
use energy_monitor::api::routes::app_router;
use energy_monitor::infrastructure::db::get_database_pool;
use dotenvy::dotenv;
use std::env;

// Usamos tokio::test para que el test sea asíncrono
#[tokio::test]
async fn test_get_prices_endpoint_returns_200() {
    // 1. Setup (Cargamos entorno y conectamos a DB real o de test)
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL necesaria");
    let pool = get_database_pool(&db_url).await.unwrap();

    // 2. Iniciamos el Router (La App)
    let app = app_router(pool);

    // 3. Crear la petición (Mock Request)
    // Fíjate: No hay localhost:3000, es una petición interna.
    let response = app
        .oneshot(Request::builder().uri("/prices").body(Body::empty()).unwrap())
        .await
        .unwrap();

    // 4. Assertions
    assert_eq!(response.status(), StatusCode::OK);

    // Opcional: Leer el body y verificar que es JSON
    // Esto requiere un poco más de boilerplate con http-body-util, 
    // pero para empezar el status code es suficiente.
}

#[tokio::test]
async fn test_get_stats_with_invalid_date_returns_400() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL necesaria");
    let pool = get_database_pool(&db_url).await.unwrap();
    let app = app_router(pool);

    // Enviamos una fecha mal formada
    let response = app
        .oneshot(Request::builder().uri("/stats?date=bad-date").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}