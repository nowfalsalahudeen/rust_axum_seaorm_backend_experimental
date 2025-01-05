use pos_rust_local_backend::config::app_context::AppContext;
use pos_rust_local_backend::config::db::db_connection;
use pos_rust_local_backend::config::routes_config::AppRoutes;
use pos_rust_local_backend::controllers;
use std::net::SocketAddr;
use axum_core::__private::tracing;
use sea_orm::DatabaseConnection;
use tokio::signal;

pub fn routes(_ctx: &AppContext) -> AppRoutes {
  AppRoutes::with_default_routes().add_route(controllers::tasks_controller::routes())
}

#[tokio::main]
async fn main() {
  // Initialize the database connection
  let ctx = db_connection()
    .await
    .expect("Failed to connect to the database");

  // Create a new router with the shared state
  let app = routes(&ctx).into_router(&ctx);

  // Initialize tracing
  setup_logging(&ctx.config.debug_mode);

  // Print server information
  // println!("Server running on port {}", 3000);
  

  // Create a new listener
  let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
  let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
  println!("Server running on {:?}", addr);

  // Start the server with graceful shutdown
  axum::serve(listener, app)
    .with_graceful_shutdown(shutdown_signal(ctx.db.clone()))
    .await
    .unwrap();
}

/// Handles graceful shutdown by listening for SIGINT (Ctrl+C) and SIGTERM signals.
async fn shutdown_signal(db: DatabaseConnection) {
  let ctrl_c = async {
    signal::ctrl_c()
      .await
      .expect("Failed to install Ctrl+C handler");
  };

  #[cfg(unix)]
  let terminate = async {
    signal::unix::signal(signal::unix::SignalKind::terminate())
      .expect("Failed to install SIGTERM handler")
      .recv()
      .await;
  };

  #[cfg(not(unix))]
  let terminate = std::future::pending::<()>();

  tokio::select! {
      _ = ctrl_c => {},
      _ = terminate => {},
  }

  if let Err(e) = db.close().await {
    tracing::error!("Error during database closure: {}", e);
  } else {
    tracing::info!("Database connection closed successfully");
  }

  tracing::info!("Shutting down gracefully...");
  
  println!("Shutting down gracefully...");
}

fn setup_logging(debug_mode: &bool) {
  if *debug_mode {
    // Enable logging in debug mode
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
    println!(
      "{} ({}) {}",
      env!("CARGO_PKG_VERSION"),
      option_env!("BUILD_SHA")
          .or(option_env!("GITHUB_SHA"))
          .unwrap_or("dev"),
      env!("CARGO_CRATE_NAME")
    );
    println!("Logging enabled {}",debug_mode);
    println!(
      "Compilation mode: {}",
      if *debug_mode { "Debug" } else { "Release" }
    );
  } else {
    // Disable logging in release mode
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::ERROR) // Minimal logging
        .without_time() // Optional: omit timestamps in release
        .init();
  }
}

