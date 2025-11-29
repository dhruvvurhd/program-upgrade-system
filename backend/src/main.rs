mod api;
mod services;
mod clients;
mod db;
mod models;
mod config;

use axum::{
    Router,
    routing::{get, post},
};
use tower_http::cors::CorsLayer;
use tracing_subscriber;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    // Load configuration
    dotenvy::dotenv().ok();
    let config = config::Config::from_env()?;
    
    // Initialize database
    let db_pool = db::init_pool(&config.database_url).await?;
    
    // Initialize Anchor client
    let anchor_client = clients::anchor_client::AnchorClient::new(
        &config.rpc_url,
        &config.program_id,
        &config.payer_keypair_path,
    )?;
    
    // Initialize services
    let services = Arc::new(services::Services::new(
        db_pool.clone(),
        anchor_client,
    ));
    
    // Build router
    let app = Router::new()
        .route("/health", get(api::health))
        .route("/proposals", get(api::upgrade::list_proposals))
        .route("/proposals/:id", get(api::upgrade::get_proposal))
        .route("/proposals/propose", post(api::upgrade::propose_upgrade))
        .route("/proposals/:id/approve", post(api::upgrade::approve_upgrade))
        .route("/proposals/:id/execute", post(api::upgrade::execute_upgrade))
        .route("/proposals/:id/cancel", post(api::upgrade::cancel_upgrade))
        .route("/migration/start", post(api::migration::start_migration))
        .route("/migration/:id/progress", get(api::migration::get_progress))
        .layer(CorsLayer::permissive())
        .with_state(services);
    
    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    tracing::info!("Server listening on {}", addr);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
