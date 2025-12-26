use energy_monitor::api::routes::app_router;
use energy_monitor::infrastructure::db::{get_database_pool, save_prices};
use energy_monitor::infrastructure::ree_client::ReeClient;


use std::env;
use std::net::SocketAddr;
use dotenvy::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::time::Duration;
use tokio::time;



async fn spawn_etl_background_task(pool: sqlx::PgPool, client: ReeClient) {

    tokio::spawn(async move {

        let mut interval = time::interval(Duration::from_secs(60)); 

        loop {

            interval.tick().await;

            tracing::info!("Ejecutando tarea programada: Actualización de precios...");

            match client.get_spot_prices().await {
                Ok(prices) => {
                    tracing::info!("Descargados {} precios.", prices.len());

                    match save_prices(&pool, &prices).await {
                        Ok(_) => tracing::info!("Datos actualizados en DB."),
                        Err(e) => tracing::error!("Error guardando en DB: {:?}", e),
                    }
                },
                Err(e) => tracing::error!("Error en request a REE: {:?}", e),
            }
            
        }
    });
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(env::var("RUST_LOG").unwrap_or("info".into())))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Iniciando EnergyMonitor System...");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL no seteada");
    let pool = get_database_pool(&db_url).await?;
    let ree_client = ReeClient::new("https://apidatos.ree.es/es/datos");

    spawn_etl_background_task(pool.clone(), ree_client.clone()).await;
    
    tracing::info!("Tarea de background iniciada.");
    
    let app = app_router(pool);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    tracing::info!("Servidor API escuchando en http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}