mod entities;
mod handlers;

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sea_orm::{Database, DatabaseConnection};
use std::env;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState {
    db: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_seaorm=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Get database URL from environment
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment");

    // Connect to database
    tracing::info!("Connecting to database...");
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Database connected successfully");

    // Create application state
    let state = AppState { db };

    // Build router
    let app = Router::new()
        .route("/", get(handlers::health_check))
        .route("/users", post(handlers::create_user))
        .route("/users", get(handlers::list_users))
        .route("/users/{id}", get(handlers::get_user))
        .route("/users/{id}", put(handlers::update_user))
        .route("/users/{id}", delete(handlers::delete_user))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    tracing::info!("Server listening on 0.0.0.0:3000");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

