use crate::cache;
use crate::cache::Cache;
use crate::config::app_context::AppContext;
use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DbErr};
use std::env;

pub async fn db_connection() -> Result<AppContext, DbErr> {
  dotenv().ok();

  let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let mut opt = ConnectOptions::new(db_url.to_owned());
  opt
      .sqlx_logging(true) // Disable SQLx logging
      .sqlx_logging_level(log::LevelFilter::Trace) // Enable Debug level for SQLx
      .max_connections(10) // Optimize connection pool
      .min_connections(2)  // Minimum connections to maintain
      .connect_timeout(std::time::Duration::from_secs(30)) // Connection timeout
      .acquire_timeout(std::time::Duration::from_secs(30)) // Acquire timeout
      .idle_timeout(std::time::Duration::from_secs(600)); // Idle timeout

  let db = Database::connect(opt).await?;

  // Apply PRAGMA settings for optimization
  db.execute_unprepared("PRAGMA journal_mode=WAL2;").await?;
  db.execute_unprepared("PRAGMA synchronous=NORMAL;").await?;
  db.execute_unprepared("PRAGMA temp_store=MEMORY;").await?;
  db.execute_unprepared("PRAGMA cache_size=-20000;").await?; // Negative value sets size in KB
  db.execute_unprepared("PRAGMA locking_mode=EXCLUSIVE;").await?;
  db.execute_unprepared("PRAGMA foreign_keys=ON;").await?; // Ensure foreign key checks
  db.execute_unprepared("PRAGMA busy_timeout=5000;").await?; // Ensure foreign key checks


  // Run migrations
  Migrator::up(&db, None)
    .await
    .expect("Failed to run migrations");

  Ok(AppContext::new(
    db,
    Cache::new(cache::drivers::inmem::new()).into()
  ))
  
}
